"""Closed original-IDF input custody; no IDF parser or physics producer.

Actor operands are the same original bytes. Scientific inspection uses the
whole official conversion, including every emitted member and numeric kind.
Native InputProcessor observations supply declaration order/metadata only;
getter defaults and numeric buffers never become simulation input.
"""
from pathlib import Path
import copy, json, re
from . import contracts as d

SCHEMA = 'paired.OriginalIDFInput.v1'
FAMILIES = ('NodeList', 'SetpointManager:Scheduled', 'OutdoorAir:NodeList',
            'OutdoorAir:Node', 'SizingPeriod:DesignDay')
FLAGS = dict(isEpJSON=False, outputEpJSONConversion=False,
             outputEpJSONConversionOnly=False, preserveIDFOrder=True)
SOURCE_METADATA = {'NodeList': (), 'SetpointManager:Scheduled':
                   ('idf_order', 'idf_max_fields', 'idf_max_extensible_fields')}
OFFICIAL_CONVERTER = {'path': '.runtime/energyplus/26.1.0/ConvertInputFormat.exe', 'sha256': '04f1327fef7148118ea1d79d9e990fb2447b9731e9c1b28a9a8276b840d7cbfe', 'bytes': 6048768}
OFFICIAL_API = {'path': '.runtime/energyplus/26.1.0/energyplusapi.dll', 'sha256': 'bbe6f66240d108df9f51a03b9beb968808a2173234253574d27a31605aec2aa2', 'bytes': 35054080}
CONVERTER_PRIMARY = {'path': '.reference/energyplus-src/26.1.0/src/ConvertInputFormat/main.cpp', 'sha256': '64c7308d3fe77bc0e32d234849c0d00bd1f78bd1aef8f9290a25d74c746570a7', 'bytes': 29219}
DOCUMENTARY_SCHEMA = {'path': '.runtime/energyplus/26.1.0/Energy+.schema.epJSON', 'sha256': '60b24d904e6fa553f773b38037a8a09730e6da1538647df552cd77db8d6704a9', 'bytes': 10568550}
NATIVE_VERSION = 'EnergyPlus, Version 26.1.0-6f2e40d102'


def load_contract(pin):
    pins = {}

    def take(value):
        path = d._canonical_pin(value)
        if value['path'] in pins:
            d.require(pins[value['path']] == value, 'Conflicting original-IDF pin')
        pins[value['path']] = copy.deepcopy(value)
        return path

    def data(value):
        return json.loads(take(value).read_bytes())

    def recorded(value, required):
        receipt = data(value)
        d.require(receipt['schema'] == 'recorded-porting-command.v1'
                  and type(receipt['exit_code']) is int and receipt['exit_code'] == 0
                  and receipt['launch_error'] is None, 'Input-only command failed')
        d.require(type(receipt['command']) is list and receipt['command']
                  and all(type(x) is str for x in receipt['command']), 'Input command argv')
        custody = receipt['explicit_input_custody']
        d.require(custody['command_process_started'] is True
                  and custody['all_explicit_inputs_unchanged'] is True
                  and custody['read_errors_before'] == custody['read_errors_after'] == []
                  and d.typed(custody['before']) == d.typed(custody['after']),
                  'Input-only explicit custody changed')
        for row in custody['before']:
            take(row)
        d.require(all(row in custody['before'] for row in required),
                  'Input-only command omitted a required operand')
        request = data(custody['request'])
        d.require(request['schema'] == 'recorded-command-explicit-input-custody-request.v1'
                  and request['command'] == receipt['command']
                  and request['cwd'] == receipt['cwd']
                  and request['declared_input_arguments'] == custody['declared_input_arguments']
                  and d.typed(request['inputs_before']) == d.typed(custody['before'])
                  and request['input_read_errors_before'] == [], 'Input request join')
        for key in ['stdout', 'stderr']:
            # V1 recorder stdio descriptors are Ref2; authenticate their actual
            # complete bytes and retain a canonical Ref3 in the actor pin set.
            take(d.ref(d.auth(receipt[key])))
        return receipt

    contract = data(pin)
    d.exact_keys(contract, {'schema', 'input', 'baseline_input', 'scientific_dom',
                           'raw_edit_ledger', 'semantic_edit_ledger',
                           'semantic_inverse_operations', 'conversion', 'native_ip'}, SCHEMA)
    d.require(contract['schema'] == SCHEMA, 'Original-IDF contract schema')
    original = take(contract['input'])
    d.require(original.suffix.lower() == '.idf', 'Original-IDF operand extension')
    raw = original.read_bytes()
    d.require(raw and b'\x00' not in raw, 'Empty/NUL original-IDF operand')
    raw.decode('utf8')
    document = data(contract['scientific_dom'])
    d.require(type(document) is dict and document, 'Complete converted DOM required')
    baseline = take(contract['baseline_input']).read_bytes()
    ledger = data(contract['raw_edit_ledger'])
    d.exact_keys(ledger, {'schema', 'baseline_input', 'input', 'forward', 'inverse',
                         'forward_patch', 'inverse_patch'}, 'IDF raw-byte ledger')
    d.require(ledger['schema'] == 'paired.ByteInputEditLedger.v1'
              and ledger['baseline_input'] == contract['baseline_input']
              and ledger['input'] == contract['input'], 'IDF byte ancestry identity')
    d.require(d.verify_byte_edits(baseline, raw, ledger['forward'])
              and d.verify_byte_edits(raw, baseline, ledger['inverse'])
              and d.apply_unified(baseline, take(ledger['forward_patch']).read_bytes()) == raw
              and d.apply_unified(raw, take(ledger['inverse_patch']).read_bytes()) == baseline,
              'Whole original-IDF byte inverse')
    # The existing JSON numeric-kind/string/typed inverse kernel is used only
    # on JSON operands, never on original IDF tokens or a stripped DOM.
    semantic = data(contract['semantic_edit_ledger'])
    for value in d.reference_closure(semantic):
        take(value)
    shared = data(semantic['numeric_baseline'])
    for value in d.reference_closure(shared):
        take(value)
    d.verify_ledger(dict(baseline_input=contract['baseline_input'],
                         input=contract['scientific_dom'], scientific_dom=contract['scientific_dom'],
                         edit_ledger=contract['semantic_edit_ledger'],
                         inverse_operations=contract['semantic_inverse_operations']))

    conversion = contract['conversion']
    d.exact_keys(conversion, {'schema', 'input', 'output', 'executable', 'api',
                             'native_version', 'receipt', 'version_receipt',
                             'schema_provenance'}, 'Official conversion')
    d.require(conversion['schema'] == 'paired.NativeInputConversion.v1'
              and conversion['input'] == contract['input']
              and conversion['output'] == contract['scientific_dom']
              and conversion['executable'] == OFFICIAL_CONVERTER
              and conversion['api'] == OFFICIAL_API, 'Conversion input/DOM/official-tool identity')
    exe, api = [take(conversion[key]) for key in ['executable', 'api']]
    d.require(exe.name == 'ConvertInputFormat.exe' and api.name == 'energyplusapi.dll'
              and exe.parent == api.parent, 'Official converter/API custody')
    version = conversion['native_version']
    d.require(type(version) is str and version == NATIVE_VERSION,
              'Closed Native input schema version')
    receipt = recorded(conversion['receipt'], [contract['input'], conversion['executable'], conversion['api']])
    converted = take(contract['scientific_dom'])
    d.require(Path(receipt['cwd']).resolve() == d.ROOT
              and receipt['command'] == [str(exe), '--output', str(converted.parent),
                                         '--format', 'epJSON', str(original)]
              and converted.name == original.stem + '.epJSON', 'Official conversion command/output')
    version_receipt = recorded(conversion['version_receipt'], [conversion['executable'], conversion['api']])
    d.require(version_receipt['command'] == [str(exe), '--version']
              and Path(version_receipt['cwd']).resolve() == d.ROOT
              and d.auth(version_receipt['stdout']).read_text('utf8').strip() == version,
              'Actual converter version')
    provenance = conversion['schema_provenance']
    d.exact_keys(provenance, {'embedded_schema', 'primary', 'documentary_external_schema',
                            'external_schema_role'}, 'Converter schema provenance')
    d.require(provenance['embedded_schema'] is True
              and provenance['external_schema_role'] == 'documentary_not_consumed'
              and provenance['primary'] == CONVERTER_PRIMARY
              and provenance['documentary_external_schema'] == DOCUMENTARY_SCHEMA,
              'Do not relabel external schema/IDD as a converter operand')
    take(provenance['primary']); take(provenance['documentary_external_schema'])

    witnesses = contract['native_ip']
    d.require(type(witnesses) is list and [row['family'] for row in witnesses] == list(FAMILIES),
              'Complete five-family Native input-order witnesses')
    orders, metadata = {}, {}
    for witness in witnesses:
        d.exact_keys(witness, {'family', 'observation', 'receipt', 'observer_executable'},
                     'Native IP witness')
        family = witness['family']; observed = data(witness['observation'])
        observer = take(witness['observer_executable'])
        command = recorded(witness['receipt'], [contract['input'], witness['observer_executable']])
        d.require(command['command'][0] == str(observer)
                  and Path(command['cwd']).resolve() == d.ROOT, 'Native IP command executable/cwd')
        argv = command['command']
        d.require(len(argv) == 9 and set(argv[1::2]) == {'--idf', '--family', '--output-dir', '--log-dir'}
                  and len(set(argv[1::2])) == 4, 'Native IP exact option set')
        options = dict(zip(argv[1::2], argv[2::2], strict=True))
        d.require(options['--idf'] == str(original) and options['--family'] == family
                  and all(Path(options[key]).resolve().is_relative_to(d.ROOT / '.runtime')
                          for key in ['--output-dir', '--log-dir']), 'Native IP exact input/family command')
        d.require(take(witness['observation']) == Path(options['--output-dir']).resolve() / 'native-ip-observation.json',
                  'Native IP output identity')
        d.require(observed['schema'] == 'native.InputProcessorObserver.v1'
                  and observed['canonical_family'] == observed['requested_family'] == family
                  and Path(observed['original_idf']).resolve() == original
                  and observed['native_version'] == version
                  and observed['input_route'] == 'production_processInput_original_idf'
                  and observed['processInput_returned'] is observed['observation_complete'] is True
                  and observed['physics_executed'] is observed['observer_supplies_inputs'] is False
                  and observed['scientific_acceptance'] is False, 'Genuine input-only Native route')
        for field in ['format_flags_before', 'format_flags_after_processInput', 'format_flags_after_getters']:
            d.require(d.typed(observed[field]) == d.typed(FLAGS), 'Native original-IDF flags changed')
        keys = observed['native_original_order_keys']; rows = observed['objects']
        d.require(type(keys) is list and all(type(x) is str for x in keys)
                  and len(keys) == len(set(keys)) == len(document.get(family, {}))
                  and set(keys) == set(document.get(family, {}))
                  and type(observed['object_count']) is int and observed['object_count'] == len(keys)
                  and type(rows) is list and len(rows) == len(keys), 'Complete Native family inventory')
        table = {}
        for number, (key, row) in enumerate(zip(keys, rows, strict=True), 1):
            d.require(row['native_parser_key'] == key and type(row['original_order_number']) is int
                      and row['original_order_number'] == number and type(row['status']) is int
                      and row['status'] == 1, 'Actual Native row/order/status')
            fields = row['raw_idf_metadata']
            d.exact_keys(fields, {'idf_order', 'idf_max_fields', 'idf_max_extensible_fields'}, 'Native IDF metadata')
            d.require(all(type(x) is int and 0 <= x <= 2147483647 for x in fields.values())
                      and fields['idf_order'] > 0, 'Native IDF metadata integer domain')
            table[key] = copy.deepcopy(fields)
        d.require([table[k]['idf_order'] for k in keys] == sorted({table[k]['idf_order'] for k in keys}),
                  'Actual global Native declaration order')
        orders[family], metadata[family] = list(keys), table

    source_exports = {}
    for family, fields in SOURCE_METADATA.items():
        source_exports[family] = copy.deepcopy(document[family])
        for key, row in source_exports[family].items():
            d.require(type(row) is dict, 'Converted selected object must be an object')
            for field in fields:
                d.require(field not in row, 'Unexpected official converted metadata; new profile required')
                # geometry_trace::raw_value publishes these three RawModel
                # numeric metadata tokens as f64. This explicit representation
                # conversion is confined to expected observer metadata.
                row[field] = float(metadata[family][key][field])
    return dict(contract=contract, document=document, orders=orders, metadata=metadata,
                source_exports=source_exports, pins=list(pins.values()))


def verify_case(case):
    bundle = load_contract(case['input_contract']); contract = bundle['contract']
    d.require(case['schema'] == 'paired.CaseConfig.v2'
              and all(case[key] == contract[key] for key in ['input', 'baseline_input', 'scientific_dom'])
              and case['edit_ledger'] == contract['raw_edit_ledger']
              and d.typed(case['inverse_operations']) == d.typed(contract['semantic_inverse_operations']),
              'Original-IDF case/contract ancestry')
    return bundle


def _initial_tooling_dependency(pin, release_pin, release, authority):
    """Validate the original closed adoption/validation against its own actual Source epoch.

    This is not a Python-only publication under the preceding Rust build.
    The supplied Authority is current for installation, historical for published reuse.
    """
    def data(value):
        return json.loads(d._canonical_pin(value).read_bytes())
    dependency = data(pin)
    d.require(release['schema'] == 'paired.Release.v14', 'Exact v14 tooling consumer required')
    keys = {'schema', 'release', 'prepared_manifest', 'proposal', 'adoption',
            'Root_review', 'independent_review', 'validation'}
    d.exact_keys(dependency, keys, 'Versioned tooling dependency')
    d.require(dependency['schema'] == 'paired.VersionedToolingDependency.v1'
              and dependency['release'] == release_pin, 'Tooling release identity')
    prepared, proposal = [data(dependency[key]) for key in ['prepared_manifest', 'proposal']]
    d.require(prepared['CLOSED'] is proposal['CLOSED'] is True
              and prepared['proposal'] == dependency['proposal']
              and d.typed(prepared['files']) == d.typed(proposal['files']), 'Reviewed tooling file mapping')
    owned = sorted([value['path'] for value in release['files'].values()
                    if value['path'].startswith('tools/porting/paired/v14/')]
                   + [value['path'] for value in release['tools'].values()]
                   + [release_pin['path'], release['runtime_metadata_binder']['path']])
    rows = prepared['files']
    d.require(type(rows) is list and [row['path'] for row in rows] == sorted(set(owned))
              and len(owned) == len(set(owned)) == 9, 'Exact nine original tooling endpoints')
    current = []
    for row in rows:
        d.require(row['before'] is None, 'Only new versioned tooling leaves may be added')
        planned = d._canonical_pin(row['after'])
        actual = d.ref(d.ROOT / row['path'])
        d.require(actual['sha256'] == row['after']['sha256']
                  and actual['bytes'] == row['after']['bytes']
                  and (d.ROOT / row['path']).read_bytes() == planned.read_bytes(), 'Installed tooling afterimage differs')
        current.append(actual)
    adoption = data(dependency['adoption'])
    d.exact_keys(adoption, {'schema', 'release', 'prepared_manifest', 'proposal', 'files',
                           'Root_review', 'independent_review', 'accepted', 'source_snapshot'},
                 'Actual v14 tooling adoption')
    d.require(adoption['schema'] == 'Root-paired-v14-tooling-adoption.v1'
              and adoption['accepted'] is True
              and adoption['release'] == release_pin
              and adoption['prepared_manifest'] == dependency['prepared_manifest']
              and adoption['proposal'] == dependency['proposal']
              and d.typed(adoption['files']) == d.typed(rows), 'Actual v14 tooling adoption identity')
    d.require(d._source_ref(adoption['source_snapshot'], None) == d._source_ref(authority['snapshot'], None),
              'Tooling adoption must retain the actual final Source epoch')
    for role in ['Root', 'independent']:
        key = role + '_review'; review = data(dependency[key])
        d.require(adoption[key] == dependency[key]
                  and review['review_role'] == role and review['accepted'] is True
                  and review['accepted_static_only'] is True and review['findings'] == []
                  and review['prepared_manifest'] == dependency['prepared_manifest']
                  and review['proposal'] == dependency['proposal'], 'Tooling genuine review binding')
    d.require(dependency['Root_review'] != dependency['independent_review'], 'Distinct tooling reviewers')
    validation = data(dependency['validation'])
    d.exact_keys(validation, {'schema', 'release', 'adoption', 'helper', 'helper_review',
                             'checks', 'accepted', 'all_checks_passed', 'source_snapshot',
                             'new_Source_builds', 'new_Rust_QA_commands',
                             'actual_Native_engine_calls', 'actual_Source_engine_calls',
                             'scientific_acceptance', 'retained_format_validation',
                             'inherited_archive_basis'}, 'Actual v14 tooling validation')
    d.require(validation['schema'] == 'Root-paired-v14-tooling-validation.v1'
              and validation['release'] == release_pin and validation['adoption'] == dependency['adoption']
              and validation['accepted'] is validation['all_checks_passed'] is True
              and validation['scientific_acceptance'] is False
              and d._source_ref(validation['source_snapshot'], None) == d._source_ref(authority['snapshot'], None),
              'Actual generic v14 validation identity')
    helper_review = data(validation['helper_review'])
    d._canonical_pin(validation['helper'])
    d.require(helper_review['review_role'] == 'independent' and helper_review['helper'] == validation['helper']
              and helper_review['accepted'] is helper_review['accepted_static_only'] is True
              and helper_review['findings'] == [], 'Generic validation helper peer')
    checks = validation['checks']
    names = ['release_closure', 'current_qualified_QA_held', 'original_execution_custody',
             'retained_format_after_witness', 'retained_context_eligibility_negatives',
             'plan_stage_custody_negatives', 'legacy_v3_compatibility']
    d.require(type(checks) is list and [row['name'] for row in checks] == names,
              'Required generic validation coverage')
    for row in checks:
        d.exact_keys(row, {'name', 'passed'}, 'Generic metadata check')
        d.require(row['passed'] is True, 'Generic v14 validation failed')
    for key in ['new_Source_builds', 'new_Rust_QA_commands', 'actual_Native_engine_calls', 'actual_Source_engine_calls']:
        d.require(type(validation[key]) is int and validation[key] == 0, 'Metadata-only validation scope')
    # The old archive report is documentary provenance, not a current result.
    basis = validation['inherited_archive_basis']
    d.exact_keys(basis, {'release', 'report', 'status'}, 'Inherited archive validation basis')
    d.require(basis['release'] == d.FROZEN_V13_RELEASE
              and basis['status'] == 'historical_only_not_rerun', 'Inherited archive basis identity')
    data(basis['report'])
    retained = data(validation['retained_format_validation'])
    d.exact_keys(retained, {'schema', 'release', 'source_snapshot', 'evidence', 'authority',
                            'helper', 'helper_review', 'checks', 'accepted', 'all_checks_passed',
                            'elapsed_seconds', 'comparison', 'new_Source_builds',
                            'new_Rust_QA_commands', 'actual_Native_engine_calls',
                            'actual_Source_engine_calls', 'scientific_acceptance'},
                 'Current retained-format metadata correctness validation')
    d.require(retained['schema'] == 'paired.RetainedFormatValidation.v1'
              and retained['release'] == release_pin
              and retained['authority'] == d.ref(d.auth(retained['authority']))
              and data(retained['authority']) == authority
              and retained['evidence'] == authority['evidence']
              and retained['source_snapshot'] == authority['snapshot']
              and retained['accepted'] is retained['all_checks_passed'] is True
              and retained['comparison'] is None
              and type(retained['elapsed_seconds']) in [int, float]
              and d.math.isfinite(retained['elapsed_seconds'])
              and retained['elapsed_seconds'] >= 0,
              'Actual retained-format validation identity/timing differs')
    retained_peer = data(retained['helper_review'])
    d._canonical_pin(retained['helper'])
    d.require(retained_peer['review_role'] == 'independent'
              and retained_peer['helper'] == retained['helper']
              and retained_peer['accepted'] is retained_peer['accepted_static_only'] is True
              and retained_peer['findings'] == [], 'Retained-format auxiliary helper peer')
    required = ['actual_external_format_and_check', 'qualified_current_QA_held',
                'original_v13_execution_custody', 'missing_duplicate_unused_context_rejected',
                'failed_or_nonformat_command_rejected', 'plan_profile_environment_custody_rejected',
                'stage_endpoint_after_witness_rejected', 'final_current_format_credit_guard',
                'v3_rule_and_output_held', 'metadata_only_no_comparison']
    checks = retained['checks']
    d.require(type(checks) is list and [row['name'] for row in checks] == required,
              'Required retained-format correctness coverage')
    for row in checks:
        d.exact_keys(row, {'name', 'passed'}, 'Retained-format correctness check')
        d.require(row['passed'] is True, 'Retained-format correctness validation failed')
    for key in ['new_Source_builds', 'new_Rust_QA_commands', 'actual_Native_engine_calls', 'actual_Source_engine_calls']:
        d.require(type(retained[key]) is int and retained[key] == 0, 'Retained-format metadata-only scope')
    d.require(retained['scientific_acceptance'] is False, 'No scientific credit from retained-format validation')
    pins = [pin, *d.reference_closure(dependency), *d.reference_closure(adoption),
            *d.reference_closure(validation), *d.reference_closure(retained), *current]
    for value in pins: d._canonical_pin(value)
    return dict(files=current, tracked_files=[row for row in current if row['path'].startswith('tools/')],
                pins=pins, dependency=copy.deepcopy(pin), adoption=dependency['adoption'], validation=dependency['validation'],
                state='initial', publication=None, historical_Source_authority=None)

def tooling_dependency(pin, release_pin, release, authority):
    """One initial installation, or its actual published proof under later Rust epochs.

    No current Rust identity is inferred from a historical installation. The
    caller independently normalizes its current Source Authority. Whole past
    publication/Authority JSON bytes are authenticated, but their old live
    Source file references are not recursively relabeled as current inputs.
    """
    reads = []

    def data(value):
        value = copy.deepcopy(value)
        raw = d._canonical_pin(value).read_bytes()
        reads.append(value)
        return json.loads(raw)

    dependency = data(pin)
    d.require(authority['schema'] in ['paired.SourceAuthority.v3', 'paired.SourceAuthority.v4'] and authority['mode'] == 'fresh',
              'Current fresh Source Authority required independently of tooling state')
    d.require(dependency.get('schema') in release['tooling_dependency_schemas'],
              'Unknown tooling dependency schema')
    if dependency['schema'] == 'paired.VersionedToolingDependency.v1':
        return _initial_tooling_dependency(pin, release_pin, release, authority)
    d.exact_keys(dependency, {'schema', 'release', 'publication', 'initial_dependency'},
                 'Published tooling dependency')
    d.require(dependency['schema'] == 'paired.PublishedToolingDependency.v1'
              and dependency['release'] == release_pin
              and release['schema'] == 'paired.Release.v14', 'Published tooling release differs')
    publication = data(dependency['publication'])
    d.require(publication['schema'] == 'Root-paired-domain-publication.v2'
              and publication['release'] == release_pin
              and publication['worktree_clean'] is True
              and type(publication['head']) is str
              and re.fullmatch(r'[0-9a-f]{40}|[0-9a-f]{64}', publication['head']) is not None
              and publication['head'] == publication['remote_head'],
              'Actual initial tooling publication required')
    d.require(publication['tooling_dependency'] == dependency['initial_dependency']
              and publication['tooling_state'] == 'initial'
              and publication['tooling_publication'] is None
              and publication['tooling_published_atomically_with_actual_Source'] is True,
              'Published proof must identify the original atomic installation')
    historical_ref = publication['Source_authority']
    d.require(publication['tooling_historical_Source_authority'] == historical_ref
              and publication['metadata_loader'] == release['files']['__main__.py'],
              'Original publication tooling epoch/loader differs')
    historical = data(historical_ref)
    d.require(historical['schema'] in ['paired.SourceAuthority.v3', 'paired.SourceAuthority.v4']
              and historical['mode'] == 'fresh'
              and publication['actual_Source_build'] == historical['build'],
              'Historical published Source Authority/build differs')
    historical_build = data(historical['build'])
    d.require(historical_build['source_snapshot'] == historical['snapshot'],
              'Historical installation snapshot differs')
    original = _initial_tooling_dependency(dependency['initial_dependency'], release_pin,
                                           release, historical)
    d.require(publication['tooling_adoption'] == original['adoption']
              and publication['tooling_validation'] == original['validation'],
              'Published installation/validation identity differs')
    complete = data(publication['completed'])
    scientific = data(publication['Root_scientific_review'])
    for owner, keys in [(publication, ['new_qualifying_cases', 'new_qualifying_values',
                                      'actual_Native_engine_calls', 'actual_Source_engine_calls', 'engine_reruns']),
                        (complete, ['qualifying_cases', 'qualifying_values'])]:
        d.require(all(type(owner[key]) is int for key in keys), 'Published installation counter type')
    d.require(publication['new_qualifying_cases'] == complete['qualifying_cases'] == 1
              and publication['new_qualifying_values'] == complete['qualifying_values'] == release['counts']['total_values']
              and complete['schema'] == 'paired.Completed.v1' and complete['mode'] == 'fresh'
              and complete['scientific_acceptance'] is True
              and complete['release'] == release_pin and complete['source_authority'] == historical_ref
              and publication['actual_Native_engine_calls'] == publication['actual_Source_engine_calls'] == 1
              and publication['engine_reruns'] == 0,
              'Original installation publication is not an accepted actual Source publication')
    d.require(scientific['accepted'] is True and scientific['scientific_acceptance'] is True
              and scientific['completion'] == publication['completed']
              and scientific['release'] == release_pin
              and scientific['Source_authority'] == historical_ref
              and scientific['case_config'] == complete['case_config']
              and scientific['readback'] == complete['paired']
              and scientific['binding'] == complete['bindings'],
              'Published installation scientific provenance differs')
    tracked = original['tracked_files']
    d.require(len(tracked) == 8 and len(original['files']) == 9,
              'Exact original tracked/ignored tooling set differs')
    endpoints = [dict(path=row['path'], before=None, after=row) for row in tracked]
    d.require(d.typed(publication['tooling_endpoints']) == d.typed(endpoints),
              'Published tooling endpoints differ from original installation')
    d.require(all(row in publication['files'] for row in tracked),
              'Published Git files omit an installed tooling leaf')
    custody = publication['Git_canonical_custody']
    d.require(type(custody) is list and all(type(row) is dict and type(row.get('path')) is str for row in custody)
              and len({row['path'] for row in custody}) == len(custody), 'Duplicate/malformed published Git custody')
    by_path = {row['path']: row for row in custody}
    for installed in tracked:
        row = by_path.get(installed['path'])
        d.require(row is not None and row['actual_Source_bytes'] == installed
                  and type(row['Git_blob']) is str
                  and re.fullmatch(r'[0-9a-f]{40}|[0-9a-f]{64}', row['Git_blob']) is not None,
                  'Published tooling Git identity differs')
        attrs = row['attributes']
        d.exact_keys(attrs, {'text', 'eol', 'filter', 'working-tree-encoding'}, 'Published tooling Git attributes')
        d.require(attrs['text'] in ['auto', 'set', 'unset'] and attrs['eol'] == 'lf'
                  and attrs['filter'] == attrs['working-tree-encoding'] == 'unspecified',
                  'Published tooling Git transformation differs')
        raw = d._canonical_pin(installed).read_bytes()
        raw.decode('utf8')
        d.require(b'\x00' not in raw, 'NUL in installed tooling')
        canonical = raw if attrs['text'] == 'unset' else raw.replace(b'\r\n', b'\n')
        d.require(row['Git_canonical_sha256'] == d.hashlib.sha256(canonical).hexdigest()
                  and type(row['Git_canonical_bytes']) is int and row['Git_canonical_bytes'] == len(canonical)
                  and row['CRLF_only_builtin_normalization'] is (raw != canonical),
                  'Published canonical tooling bytes differ')
    # The original validator freshly authenticates all nine installed leaves.
    # Do not append publication.files or historical.pins: they include old live
    # Rust references and are historical facts, not this case's current Source.
    pins = [pin, dependency['release'], dependency['initial_dependency'],
            dependency['publication'], *reads, *original['pins']]
    for value in pins:
        d._canonical_pin(value)
    return dict(files=original['files'], tracked_files=tracked, pins=pins,
                dependency=copy.deepcopy(pin), adoption=original['adoption'],
                validation=original['validation'], state='published',
                publication=dependency['publication'],
                historical_Source_authority=historical_ref)
