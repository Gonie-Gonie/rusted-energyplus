"""Authenticate the closed v7 Source-metadata/QA epoch release before paired imports."""
from pathlib import Path
import hashlib, importlib.util, json, sys, types

FROZEN_V1 = dict(path='tools/porting/paired/v1/release.json', sha256='bc74f5e874b22bfb373ff2bcda55c8961ab078c284f774fe85276b64332cff61', bytes=7167)
FROZEN_V2 = dict(path='tools/porting/paired/v2/release.json', sha256='9de2f4016a3081c070dacc8144788444ce817c2d23ed84f051405ef95563c3fa', bytes=8007)
FROZEN_V3 = dict(path='tools/porting/paired/v3/release.json', sha256='7db47ee8c24d7742e8f69317876135515602b37bd059d6b529de42b7278adfd0', bytes=7398)
FROZEN_V4 = {'path': 'tools/porting/paired/v4/release.json', 'sha256': 'a68819407a41647cb506cc9a959242e0b592073e7b043c9c68bfdeb7ce4eb8f5', 'bytes': 8742}
FROZEN_V5 = {'path': 'tools/porting/paired/v5/release.json', 'sha256': '89aabe3af3993118c4f0f38c073648d746e424ed4d4a27ccdfcde78a8c5953ea', 'bytes': 8924}
FROZEN_V6 = {'path': 'tools/porting/paired/v6/release.json', 'sha256': 'fad05d195e3969372afbcbff8451aff70b70484e3b12683c1890c51ac5c01754', 'bytes': 9106}
NORMALIZATION_EPOCH_CONTRACT = {'schema': 'paired.NormalizationValidationEpoch.v1', 'owner': 'authority_from_evidence', 'lifetime': 'one_invocation_only', 'stable_read_only_path_layout_required': True, 'stable_pinned_evidence_required': True, 'path_resolution': 'first_observation_per_logical_name', 'JSON_parse_key': 'actual_read_SHA256_and_length', 'Rust_mask_key': 'exact_authenticated_source_text', 'bytes_authentication': 'fresh_at_every_existing_auth_call', 'current_Source_membership_and_executable_checks': 'fresh_existing_inspect_snapshot_and_command_boundaries', 'cross_invocation_actor_or_command_reuse': False, 'concurrent_path_replacement_or_ABA': 'unsupported', 'success_cleanup': 'clear_close_detach', 'exception_lifetime': 'unreturned_local_or_traceback_only_no_prompt_erasure_guarantee'}


def load_package(release_pin):
    if not __debug__:
        raise RuntimeError('Assertions are required before release authentication/imports')
    home = Path(__file__).resolve().parent
    root = home.parents[3]
    assert Path.cwd().resolve() == root and home == root / 'tools/porting/paired/v7'

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
    release = json.loads(auth(release_pin).read_bytes())
    assert set(release) == set(old6) | {'frozen_v6_release', 'normalization_validation_epoch'}
    assert release['schema'] == 'paired.Release.v7' and type(release['version']) is int and release['version'] == 7
    assert release['profile'] == 'ideal_loads.oa_topology.v1'
    assert type(release['Source_metadata_schema']) is int and release['Source_metadata_schema'] == 2
    assert type(release['Source_evidence_schema']) is int and release['Source_evidence_schema'] == 3
    assert release['QA_descriptor_schema'] == 'paired.SourceQAProfile.v1'
    assert release['QA_binding_schema'] == 'paired.SourceQAProfileBinding.v1'
    assert release['qa_reuse_mode'] == 'disabled' and release['compatible_parent_releases'] == [FROZEN_V5, FROZEN_V6]
    assert release['frozen_v1_release'] == FROZEN_V1 and release['frozen_v2_release'] == FROZEN_V2 and release['frozen_v3_release'] == FROZEN_V3 and release['frozen_v4_release'] == FROZEN_V4 and release['frozen_v5_release'] == FROZEN_V5 and release['frozen_v6_release'] == FROZEN_V6
    assert release['normalization_validation_epoch'] == NORMALIZATION_EPOCH_CONTRACT
    changed = {'schema', 'version', 'files', 'tools', 'compatible_parent_releases'}
    assert all(release[key] == old6[key] for key in old6 if key not in changed)
    assert set(release['files']) == {'__main__.py', 'contracts.py', 'catalogue.py', 'runner.py', 'common116.py'}
    paths = {}
    for name, pin in release['files'].items():
        target = root / 'tools/porting/paired/v1/common116.py' if name == 'common116.py' else root / 'tools/porting/paired/v4/catalogue.py' if name == 'catalogue.py' else home / name
        paths[name] = auth(pin)
        assert paths[name] == target
    assert release['files']['common116.py'] == old1['files']['common116.py']
    assert release['files']['catalogue.py'] == old4['files']['catalogue.py']
    assert all(release['files']['runner.py'][key] == old4['files']['runner.py'][key] for key in ['sha256', 'bytes'])
    assert set(release['tools']) == {'source_production.py', 'coordinate.py', 'publish.py'}
    for name, pin in release['tools'].items():
        assert auth(pin) == home / name
    assert auth(release['recorder']) == root / 'tools/porting/record_command.py'
    package = types.ModuleType('_frozen_paired_v7')
    package.__path__ = [str(home)]
    sys.modules[package.__name__] = package
    for name in ['common116', 'contracts', 'catalogue', 'runner']:
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
