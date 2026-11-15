"""Authenticate the closed Source-metadata/QA release before paired imports."""
from pathlib import Path
import hashlib, importlib.util, json, sys, types

FROZEN_V1 = dict(path='tools/porting/paired/v1/release.json', sha256='bc74f5e874b22bfb373ff2bcda55c8961ab078c284f774fe85276b64332cff61', bytes=7167)
FROZEN_V2 = dict(path='tools/porting/paired/v2/release.json', sha256='9de2f4016a3081c070dacc8144788444ce817c2d23ed84f051405ef95563c3fa', bytes=8007)
FROZEN_V3 = dict(path='tools/porting/paired/v3/release.json', sha256='7db47ee8c24d7742e8f69317876135515602b37bd059d6b529de42b7278adfd0', bytes=7398)


def load_package(release_pin):
    if not __debug__:
        raise RuntimeError('Assertions are required before release authentication/imports')
    home = Path(__file__).resolve().parent
    root = home.parents[3]
    assert Path.cwd().resolve() == root and home == root / 'tools/porting/paired/v4'

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
    old3 = json.loads(auth(FROZEN_V3).read_bytes())
    release = json.loads(auth(release_pin).read_bytes())
    added = {'tools', 'frozen_v2_release', 'frozen_v3_release', 'Source_evidence_schema', 'QA_descriptor_schema', 'QA_binding_schema', 'qa_reuse_mode', 'compatible_parent_releases'}
    assert set(release) == set(old3) | added
    assert release['schema'] == 'paired.Release.v4' and type(release['version']) is int and release['version'] == 4
    assert release['profile'] == 'ideal_loads.oa_topology.v1'
    assert type(release['Source_metadata_schema']) is int and release['Source_metadata_schema'] == 2
    assert type(release['Source_evidence_schema']) is int and release['Source_evidence_schema'] == 3
    assert release['QA_descriptor_schema'] == 'paired.SourceQAProfile.v1'
    assert release['QA_binding_schema'] == 'paired.SourceQAProfileBinding.v1'
    assert release['qa_reuse_mode'] == 'disabled' and release['compatible_parent_releases'] == [FROZEN_V3]
    assert release['frozen_v1_release'] == FROZEN_V1 and release['frozen_v2_release'] == FROZEN_V2 and release['frozen_v3_release'] == FROZEN_V3
    changed = {'schema', 'version', 'files', 'Source_metadata_schema'}
    assert all(release[key] == old3[key] for key in old3 if key not in changed)
    assert set(release['files']) == {'__main__.py', 'contracts.py', 'catalogue.py', 'runner.py', 'common116.py'}
    paths = {}
    for name, pin in release['files'].items():
        target = root / 'tools/porting/paired/v1/common116.py' if name == 'common116.py' else home / name
        paths[name] = auth(pin)
        assert paths[name] == target
    assert release['files']['common116.py'] == old1['files']['common116.py']
    for name in ['runner.py', 'catalogue.py']:
        assert all(release['files'][name][key] == old3['files'][name][key] for key in ['sha256', 'bytes'])
    assert set(release['tools']) == {'source_production.py', 'coordinate.py', 'publish.py'}
    for name, pin in release['tools'].items():
        assert auth(pin) == home / name
    assert auth(release['recorder']) == root / 'tools/porting/record_command.py'
    package = types.ModuleType('_frozen_paired_v4')
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
