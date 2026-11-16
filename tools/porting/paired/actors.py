"""Root-only actor transport; projected input fields are not a validated Context.

The genuine frozen record child performs full load_context and actor_plan_check
before the trusted recorder can launch an engine. Frozen runner.act retains its
actual Native proof checks and all postlaunch Source/input/environment custody.
"""
if not __debug__:
    raise RuntimeError('Assertions are required before actor wrapper activity')

from pathlib import Path
import argparse, copy, hashlib, importlib.util, json, re, sys, types

R = Path.cwd().resolve()
READERS = dict(path='tools/porting/paired/readers.py', sha256='9ca150ccd888e3cd1e2d42632a5accd1e695738c6e2161ab278a7bb195a94f1f', bytes=11935)
FROZEN6 = dict(path='tools/porting/paired/v6/release.json', sha256='fad05d195e3969372afbcbff8451aff70b70484e3b12683c1890c51ac5c01754', bytes=9106)


def ref(path):
    raw = path.read_bytes()
    return dict(path=path.relative_to(R).as_posix(), sha256=hashlib.sha256(raw).hexdigest(), bytes=len(raw))


def authenticate_module(pin):
    assert type(pin) is dict and set(pin) == {'path', 'sha256', 'bytes'}
    assert type(pin['path']) is str and pin['path'] and '\\' not in pin['path'] and ':' not in pin['path']
    assert all(part and part not in ['.', '..'] for part in pin['path'].split('/'))
    assert type(pin['sha256']) is str and re.fullmatch('[0-9a-f]{64}', pin['sha256']) is not None
    assert type(pin['bytes']) is int and pin['bytes'] >= 0
    path = (R / pin['path']).resolve()
    assert path.is_relative_to(R) and path != R and path.relative_to(R).as_posix() == pin['path']
    assert ref(path) == pin
    return path


def load_projection_runtime():
    if not __debug__:
        raise RuntimeError('Assertions are required before authenticated module imports')
    path = authenticate_module(READERS)
    assert path == R / 'tools/porting/paired/readers.py'
    authenticate_module(FROZEN6)
    spec = importlib.util.spec_from_file_location('_actor_input_projection_readers', path)
    assert spec and spec.loader
    readers = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(readers)
    assert Path(readers.__file__).resolve() == path and readers.R == R and readers.FROZEN6 == FROZEN6
    package, release, root = readers.load_metadata(FROZEN6)
    assert root == R and release['schema'] == 'paired.Release.v6'
    for name in ['common116', 'contracts', 'catalogue', 'runner']:
        module = getattr(package, name)
        assert module.__name__ == '_frozen_paired_v6.' + name
        assert Path(module.__file__).resolve() == authenticate_module(release['files'][name + '.py'])
    assert Path(package.runner.__file__).resolve() == R / 'tools/porting/paired/v6/runner.py'
    return readers, package, release


def project_actor_inputs(readers, package, release, binding_ref, case_id):
    """Project actual fields only; this never calls or claims load_context."""
    d = package.contracts
    binding, case, pins = readers.project_context(package, release, binding_ref)
    assert type(case_id) is str and case_id and case['id'] == case_id
    d.exact_keys(case['aliases'], {'fuel_case', 'gate_case', 'original_exhaust_recipe', 'science_contract'}, 'profile aliases projection')
    assert set(case['scientific']) == set(release['scientific_fields'])
    authority = d.load(binding['source_authority'])
    assert authority['schema'] == 'paired.SourceAuthority.v3' and authority['mode'] == 'fresh'
    # Literal construction from frozen contracts638-639; no placeholder policy,
    # document, environments, validated flag, Source normalization or cache.
    recipe = copy.deepcopy(case['scientific'])
    recipe.update(input=case['input'], baseline_input=case['baseline_input'], scientific_dom=case['scientific_dom'], derived_policy=case['policy'], counts=case['counts'], case_kind=case['case_kind'], inherited_heating_efficiency_case_kind=case['aliases']['fuel_case'], inherited_exhaust_baseline_recipe=case['aliases']['original_exhaust_recipe'], structure_config=case['structure'], science_contract=case['aliases']['science_contract'])
    recipe['exact_empty_selector_science_contract'] = recipe['science_contract']
    recipe.update(scope='B', format='epJSON', weather=None, weather_cli_argument=None, hours_cli_argument=None, DDOnly_cli_argument=False, Native_convert_argument=False)
    assert recipe['inherited_heating_efficiency_case_kind'] == 'POSITIVE' and case['aliases']['gate_case'] == 'SUM'
    primary = d.load(recipe['primary_pins'])
    native_bundle = primary['Native_bundle']
    assert type(native_bundle) is dict
    # This private field record is NOT d.Context and carries no authorization.
    return types.SimpleNamespace(binding_ref=binding_ref, binding=binding, release_ref=binding['release'], release=release, case_ref=binding['case_config'], case=case, authority_ref=binding['source_authority'], authority=authority, mode='fresh', recipe=recipe, native_bundle=native_bundle, pins=pins)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('engine', choices=['native', 'source'])
    parser.add_argument('--home', required=True)
    parser.add_argument('--release', required=True)
    parser.add_argument('--case', required=True)
    parser.add_argument('--mode', choices=['fresh'], required=True)
    parser.add_argument('--projection-output')
    args = parser.parse_args(argv)
    assert args.release == FROZEN6['path'] + '#' + FROZEN6['sha256']
    home = (R / args.home).resolve()
    assert home.is_relative_to(R / '.runtime/porting') and home != R / '.runtime/porting' and home.is_dir()
    run_dir = home / args.engine
    readers, package, release = load_projection_runtime()
    binding_ref = readers.canonical_pin(ref(home / 'binding.json'))
    fields = project_actor_inputs(readers, package, release, binding_ref, args.case)
    assert fields.case_ref == readers.canonical_pin(ref(home / 'case-config.json'))
    if args.projection_output:
        target = (R / args.projection_output).resolve()
        assert target.is_relative_to(R / '.runtime') and not target.is_relative_to(home)
        assert target.parent.is_dir() and not target.exists()
        # No actor_plan/_proof/launch read, actor home creation, synthetic plan
        # pin or fabricated argv. Root can compare these actual fields against
        # one genuine full Context without repeating any accepted model actor.
        result = dict(schema='Root-frozen-v6-actor-input-projection.v1', non_authorizing=True,
                      engine=args.engine, home=home.relative_to(R).as_posix(), run_dir=run_dir.relative_to(R).as_posix(),
                      release=FROZEN6, reader_module=READERS, binding=fields.binding_ref,
                      case_config=fields.case_ref, case_id=fields.case['id'], source_authority=fields.authority_ref,
                      mode=fields.mode, recipe=fields.recipe, native_bundle=fields.native_bundle,
                      inputs_before=list(fields.pins), projected_field_names=list(vars(fields)),
                      full_validation_deferred_to_genuine_frozen_record_child=True,
                      actor_plan=None, record_argv=None, actor_home_created=False, model_actor_calls=0)
        print(json.dumps(dict(projection=readers.save(target, result))))
        return 0
    assert not run_dir.exists(), 'Actor home must be genuinely fresh'
    native_proof = native_precheck_receipt = None
    if args.engine == 'source':
        native_proof = readers.token(readers.canonical_pin(ref(home / 'native-precheck.json')))
        native_precheck_receipt = readers.token(readers.canonical_pin(ref(home / 'precheck-command/receipt.json')))
    actor_args = types.SimpleNamespace(engine=args.engine, run_dir=str(run_dir), native_proof=native_proof, native_precheck_receipt=native_precheck_receipt)
    # The authenticated frozen runner retains actor_plan/_proof/check_launch,
    # genuine record subprocess/argv and all postlaunch checks unmodified.
    return package.runner.act(fields, actor_args)


if __name__ == '__main__':
    raise SystemExit(main())
