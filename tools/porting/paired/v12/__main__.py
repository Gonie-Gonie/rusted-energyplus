"""Authenticate the closed v12 typed-warning/parent/V2 metadata release before paired imports."""
from pathlib import Path
import hashlib, importlib.util, json, sys, types

FROZEN_V1 = dict(path='tools/porting/paired/v1/release.json', sha256='bc74f5e874b22bfb373ff2bcda55c8961ab078c284f774fe85276b64332cff61', bytes=7167)
FROZEN_V2 = dict(path='tools/porting/paired/v2/release.json', sha256='9de2f4016a3081c070dacc8144788444ce817c2d23ed84f051405ef95563c3fa', bytes=8007)
FROZEN_V3 = dict(path='tools/porting/paired/v3/release.json', sha256='7db47ee8c24d7742e8f69317876135515602b37bd059d6b529de42b7278adfd0', bytes=7398)
FROZEN_V4 = {'path': 'tools/porting/paired/v4/release.json', 'sha256': 'a68819407a41647cb506cc9a959242e0b592073e7b043c9c68bfdeb7ce4eb8f5', 'bytes': 8742}
FROZEN_V5 = {'path': 'tools/porting/paired/v5/release.json', 'sha256': '89aabe3af3993118c4f0f38c073648d746e424ed4d4a27ccdfcde78a8c5953ea', 'bytes': 8924}
FROZEN_V6 = {'path': 'tools/porting/paired/v6/release.json', 'sha256': 'fad05d195e3969372afbcbff8451aff70b70484e3b12683c1890c51ac5c01754', 'bytes': 9106}
FROZEN_V7 = {'path': 'tools/porting/paired/v7/release.json', 'sha256': '22b0757b755b71403d2bf5bd65b1a25b647b36c790c8477e96a4fdd622b7de15', 'bytes': 10330}
FROZEN_V8 = {'path': 'tools/porting/paired/v8/release.json', 'sha256': 'fa2d45b0ab6696e726f97ad1555589d138112ab151f0f753dc0b15b9ab476dc0', 'bytes': 11634}
SOURCE_RECORDER_INVOCATION_CONTRACT = {'schema': 'paired.SourceRecorderInvocation.v1', 'owner': 'source_production.main.record', 'invocation': 'fixed_trusted_runpy_same_process', 'recorder_schema': 'recorded-porting-command.v1', 'recorder': 'tools/porting/record_command.py', 'recorder_bytes': 'unchanged_frozen_v8_recorder', 'recorder_arguments': 'full_real_sys.argv_with_every_ordered_explicit_input', 'recorder_OS_subprocess_created': False, 'command_launch': 'unchanged_recorder_subprocess.run', 'Python_preconditions': ['assertions_enabled', 'utf8_mode_1', 'dont_write_bytecode_true'], 'stdin_precondition': 'producer_outer_OS_stdin_DEVNULL_inherited_by_unchanged_recorder_command_child', 'stdin_precondition_runtime_enforced': False, 'environment': 'temporary_exact_prior_child_env_including_PATH_restored_in_finally', 'arguments_restoration': 'original_sys.argv_object_restored_in_finally', 'diagnostic_print_capture': 'UTF8_text_newline_empty_exclusive_recorder.stdout_and_recorder.stderr', 'command_stdout_stderr': 'unchanged_binary_recorder_owned_files', 'exit': 'integer_SystemExit_code_only_other_SystemExit_failure_no_return_is_error', 'fresh_input_authentication': 'all_existing_ordered_leaf_auth_calls_after_epoch_close_unchanged', 'epoch_cleanup': 'close_clear_setNone_detach_before_input_authentication', 'schema_science_actor_or_gate_change': False}
FROZEN_V10 = {'path': 'tools/porting/paired/v10/release.json', 'sha256': '4ee07caa27b87a028178c26f5e7d48a5210d20ba5e5664ebdd3358531aa7cb0b', 'bytes': 13469}
NORMALIZATION_EPOCH_V2_CONTRACT = {'schema': 'paired.NormalizationValidationEpoch.v2', 'owner': 'authority_from_evidence', 'lifetime': 'one_normalization_invocation_only', 'stable_read_only_path_layout_required': True, 'stable_pinned_evidence_and_in_epoch_descriptors_required': True, 'snapshot_metadata_key': 'freshly_authenticated_actual_read_SHA256_and_length', 'snapshot_metadata_seed': 'only_after_complete_existing_Source_reference_boundary_checks', 'snapshot_cold_miss': 'original_schema_member_checks_and_archive_loop_no_early_index_validation', 'snapshot_metadata_storage': 'private_immutable_scalar_tuples_with_detached_owner_tables', 'per_stage_and_binding_snapshot_tables': 'existing_independent_AuthorityPins_compartments', 'pin_membership': 'per_collection_exact_descriptor_set_after_unconditional_auth_preserving_first_seen_and_Ref2_Ref3', 'descriptor_mutation_or_list_bypass_during_epoch': 'unsupported_no_such_current_Authority_route', 'bytes_authentication': 'every_existing_auth_call_and_archive_loop_unchanged', 'current_Source_checks': 'existing_inspect_snapshot_current_membership_and_command_boundaries_unchanged', 'general_load_receipt_parse_memo': 'deferred', 'whole_Authority_success_or_cross_invocation_actor_command_reuse': False, 'success_cleanup': 'clear_metadata_and_pin_sets_close_detach_before_return', 'concurrent_path_replacement_or_ABA': 'unsupported', 'exception_lifetime': 'unreturned_local_or_traceback_only_no_prompt_erasure_guarantee'}
FROZEN_V11 = {'path': 'tools/porting/paired/v11/release.json', 'sha256': 'b0222c884418cbad11e8478ff0942087b4897b9078c7746abbf27e8e50ad24af', 'bytes': 14731}
WARNING_PHASE_CONTRACT = {'path': '.runtime/object-expansion/paired-warning-phase-contract-v2.json', 'sha256': '9d2abce1cc108cfb473e423500b8ad4ff49eda90169f0d2e71d7b9eabea82a18', 'bytes': 3476}
NEW_CONSTANT_SCIENTIFIC_SHA256 = 'db5faa295c450692ce2552581020292f2e4cf31eafbc82867296ca5b9e0ae245'
QUALITY_PREFLIGHT_EPOCH_CONTRACT = {'schema': 'paired.QAPreflightObservationEpoch.v1', 'owner': 'source_production.main_quality_preflight', 'lifetime': 'one_invocation_read_only_prefix_until_first_record', 'stable_read_only_path_layout_required': True, 'stable_pinned_evidence_required': True, 'observations': 'existing_path_JSON_Rust_text_memo_only', 'per_stage_and_binding_snapshot_tables': 'independent', 'bytes_authentication': 'fresh_at_every_existing_auth_call', 'current_Source_checks': 'no_epoch_for_source_ref_or_snapshot', 'cleanup': 'close_clear_setNone_detach_before_every_record_and_input_authentication', 'build_and_evidence_epoch': None, 'after_first_record_epoch': None, 'cross_invocation_actor_or_command_reuse': False, 'concurrent_path_replacement_or_ABA': 'unsupported', 'exception_lifetime': 'unreturned_local_or_traceback_only_no_prompt_erasure_guarantee'}
NORMALIZATION_EPOCH_CONTRACT = {'schema': 'paired.NormalizationValidationEpoch.v1', 'owner': 'authority_from_evidence', 'lifetime': 'one_invocation_only', 'stable_read_only_path_layout_required': True, 'stable_pinned_evidence_required': True, 'path_resolution': 'first_observation_per_logical_name', 'JSON_parse_key': 'actual_read_SHA256_and_length', 'Rust_mask_key': 'exact_authenticated_source_text', 'bytes_authentication': 'fresh_at_every_existing_auth_call', 'current_Source_membership_and_executable_checks': 'fresh_existing_inspect_snapshot_and_command_boundaries', 'cross_invocation_actor_or_command_reuse': False, 'concurrent_path_replacement_or_ABA': 'unsupported', 'success_cleanup': 'clear_close_detach', 'exception_lifetime': 'unreturned_local_or_traceback_only_no_prompt_erasure_guarantee'}


def load_package(release_pin):
    if not __debug__:
        raise RuntimeError('Assertions are required before release authentication/imports')
    home = Path(__file__).resolve().parent
    root = home.parents[3]
    assert Path.cwd().resolve() == root and home == root / 'tools/porting/paired/v12'

    def auth(pin):
        assert type(pin) is dict and set(pin) == {'path', 'sha256', 'bytes'}
        assert type(pin['path']) is str and pin['path'] and '\\' not in pin['path'] and ':' not in pin['path']
        assert all(part and part not in ['.', '..'] for part in pin['path'].split('/'))
        assert type(pin['sha256']) is str and len(pin['sha256']) == 64 and all(c in '0123456789abcdef' for c in pin['sha256'])
        assert type(pin['bytes']) is int and pin['bytes'] >= 0
        path = (root / pin['path']).resolve()
        assert path.is_relative_to(root) and path != root and pin['path'] == path.relative_to(root).as_posix()
        raw = path.read_bytes()
        assert hashlib.sha256(raw).hexdigest() == pin['sha256'] and len(raw) == pin['bytes']
        return path

    old1 = json.loads(auth(FROZEN_V1).read_bytes())
    json.loads(auth(FROZEN_V2).read_bytes())
    json.loads(auth(FROZEN_V3).read_bytes())
    old4 = json.loads(auth(FROZEN_V4).read_bytes())
    for pin in [*old4['files'].values(), *old4['tools'].values(), old4['recorder']]:
        auth(pin)
    old5 = json.loads(auth(FROZEN_V5).read_bytes())
    for pin in [*old5['files'].values(), *old5['tools'].values(), old5['recorder']]:
        auth(pin)
    old6 = json.loads(auth(FROZEN_V6).read_bytes())
    for pin in [*old6['files'].values(), *old6['tools'].values(), old6['recorder']]:
        auth(pin)
    old7 = json.loads(auth(FROZEN_V7).read_bytes())
    for pin in [*old7['files'].values(), *old7['tools'].values(), old7['recorder']]:
        auth(pin)
    old8 = json.loads(auth(FROZEN_V8).read_bytes())
    for pin in [*old8['files'].values(), *old8['tools'].values(), old8['recorder']]:
        auth(pin)
    old10 = json.loads(auth(FROZEN_V10).read_bytes())
    for pin in [*old10['files'].values(), *old10['tools'].values(), old10['recorder']]:
        auth(pin)
    old11 = json.loads(auth(FROZEN_V11).read_bytes())
    for pin in [*old11['files'].values(), *old11['tools'].values(), old11['recorder']]: auth(pin)
    release = json.loads(auth(release_pin).read_bytes())
    assert set(release) == set(old11) | {'frozen_v11_release', 'original_IDF_input_schema', 'original_IDF_structure_schema', 'original_IDF_gate_ids', 'runtime_metadata_binder'}
    assert release['schema'] == 'paired.Release.v12' and type(release['version']) is int and release['version'] == 12
    assert release['profile'] == 'ideal_loads.oa_topology.v1'
    assert type(release['Source_metadata_schema']) is int and release['Source_metadata_schema'] == 2
    assert type(release['Source_evidence_schema']) is int and release['Source_evidence_schema'] == 3
    assert release['QA_descriptor_schema'] == 'paired.SourceQAProfile.v1'
    assert release['QA_binding_schema'] == 'paired.SourceQAProfileBinding.v1'
    assert release['qa_reuse_mode'] == 'disabled' and release['compatible_parent_releases'] == [FROZEN_V5, FROZEN_V6, FROZEN_V7, FROZEN_V8, FROZEN_V10, FROZEN_V11]
    assert release['frozen_v1_release'] == FROZEN_V1 and release['frozen_v2_release'] == FROZEN_V2 and release['frozen_v3_release'] == FROZEN_V3 and release['frozen_v4_release'] == FROZEN_V4 and release['frozen_v5_release'] == FROZEN_V5 and release['frozen_v6_release'] == FROZEN_V6 and release['frozen_v7_release'] == FROZEN_V7
    assert release['frozen_v10_release'] == FROZEN_V10 and release['frozen_v11_release'] == FROZEN_V11
    assert release['original_IDF_input_schema'] == 'paired.OriginalIDFInput.v1' and release['original_IDF_structure_schema'] == 'paired.OATopologyStructure.v2'
    ids = json.loads(json.dumps(old11['gate_ids'])); ids['paired'][6] = 'actual_Source_original_IDF_input_projection'
    assert release['gate_ids'] == old11['gate_ids'] and release['original_IDF_gate_ids'] == ids
    assert auth(release['runtime_metadata_binder']) == root / '.runtime/object-expansion/Root-bind-paired-metadata-child09-v12.py'
    assert release['warning_phase_contract'] == WARNING_PHASE_CONTRACT
    assert release['constant_scientific_sha256'] == NEW_CONSTANT_SCIENTIFIC_SHA256
    assert release['frozen_v8_release'] == FROZEN_V8
    assert release['Source_recorder_invocation'] == SOURCE_RECORDER_INVOCATION_CONTRACT
    assert release['normalization_validation_epoch'] == NORMALIZATION_EPOCH_V2_CONTRACT
    assert release['quality_preflight_observation_epoch'] == QUALITY_PREFLIGHT_EPOCH_CONTRACT
    changed = {'schema', 'version', 'files', 'tools', 'compatible_parent_releases', 'normalization_validation_epoch', 'constant_scientific_sha256'}
    assert all(release[key] == old11[key] for key in old11 if key not in changed)
    assert set(release['files']) == {'__main__.py', 'contracts.py', 'original_idf.py', 'catalogue.py', 'runner.py', 'common116.py'}
    paths = {}
    for name, pin in release['files'].items():
        target = root / 'tools/porting/paired/v1/common116.py' if name == 'common116.py' else home / name
        paths[name] = auth(pin)
        assert paths[name] == target
    assert release['files']['common116.py'] == old1['files']['common116.py']
    assert release['files']['catalogue.py']['path'] == 'tools/porting/paired/v12/catalogue.py'
    assert release['files']['original_idf.py']['path'] == 'tools/porting/paired/v12/original_idf.py'
    assert set(release['tools']) == {'source_production.py', 'coordinate.py', 'publish.py'}
    for name, pin in release['tools'].items():
        assert auth(pin) == home / name
    assert auth(release['recorder']) == root / 'tools/porting/record_command.py'
    package = types.ModuleType('_frozen_paired_v12')
    package.__path__ = [str(home)]
    sys.modules[package.__name__] = package
    for name in ['common116', 'contracts', 'original_idf', 'catalogue', 'runner']:
        fullname = package.__name__ + '.' + name
        spec = importlib.util.spec_from_file_location(fullname, paths[name + '.py'])
        assert spec and spec.loader
        module = importlib.util.module_from_spec(spec)
        sys.modules[fullname] = module
        spec.loader.exec_module(module)
        setattr(package, name, module)
    return package, release, root


def bootstrap(argv):
    if not __debug__:
        raise RuntimeError('Optimized Python cannot import release modules')
    root = Path.cwd().resolve()

    def option(name):
        assert argv.count(name) == 1
        at = argv.index(name)
        assert at + 1 < len(argv)
        return argv[at + 1]

    def pin(value):
        name, sha = value.rsplit('#', 1)
        assert name and '\\' not in name and ':' not in name and all(p and p not in ['.', '..'] for p in name.split('/'))
        assert len(sha) == 64 and all(c in '0123456789abcdef' for c in sha)
        path = (root / name).resolve()
        assert path.is_relative_to(root) and path != root and path.relative_to(root).as_posix() == name
        raw = path.read_bytes()
        assert hashlib.sha256(raw).hexdigest() == sha
        return dict(path=name, sha256=sha, bytes=len(raw))

    assert option('--mode') == 'fresh', 'OA topology profile is fresh-only'
    if argv and argv[0] == 'authority':
        release_pin = pin(option('--release'))
    else:
        binding_pin = pin(option('--bindings'))
        binding = json.loads((root / binding_pin['path']).read_bytes())
        assert binding['schema'] == 'paired.RuntimeBinding.v1' and binding['mode'] == 'fresh'
        release_pin = binding['release']
    package, release, root = load_package(release_pin)
    return package.runner.main(argv)


if __name__ == '__main__':
    raise SystemExit(bootstrap(sys.argv[1:]))
