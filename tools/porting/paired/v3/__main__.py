"""Authenticate the closed OA topology release before importing paired code."""
from pathlib import Path
import hashlib, importlib.util, json, sys, types

FROZEN_V1 = dict(path='tools/porting/paired/v1/release.json', sha256='bc74f5e874b22bfb373ff2bcda55c8961ab078c284f774fe85276b64332cff61', bytes=7167)


def load_package(release_pin):
    if not __debug__:
        raise RuntimeError('Assertions are required before topology release imports')
    home = Path(__file__).resolve().parent
    root = home.parents[3]
    assert Path.cwd().resolve() == root and home == root / 'tools/porting/paired/v3'

    def auth(pin):
        assert type(pin) is dict and {'path', 'sha256'} <= set(pin) <= {'path', 'sha256', 'bytes'}
        assert type(pin['path']) is str and pin['path']
        assert type(pin['sha256']) is str and len(pin['sha256']) == 64
        assert all(char in '0123456789abcdef' for char in pin['sha256'])
        assert 'bytes' not in pin or type(pin['bytes']) is int and pin['bytes'] >= 0
        path = (root / pin['path']).resolve()
        assert path.is_relative_to(root) and path != root
        assert pin['path'] == path.relative_to(root).as_posix()
        data = path.read_bytes()
        assert hashlib.sha256(data).hexdigest() == pin['sha256']
        assert len(data) == pin.get('bytes', len(data))
        return path

    frozen = json.loads(auth(FROZEN_V1).read_bytes())
    release = json.loads(auth(release_pin).read_bytes())
    assert set(release) == set(frozen) | {'frozen_v1_release', 'structure_schema', 'Source_metadata_schema'}
    assert release['schema'] == 'paired.Release.v3' and type(release['version']) is int and release['version'] == 3
    assert release['profile'] == 'ideal_loads.oa_topology.v1'
    assert release['frozen_v1_release'] == FROZEN_V1
    assert release['structure_schema'] == 'paired.OATopologyStructure.v1'
    assert type(release['Source_metadata_schema']) is int and release['Source_metadata_schema'] == 1
    assert release['full_retained_migration_units'] == release['partial_shared_kernel_units'] == []
    changed = {'schema', 'version', 'profile', 'files', 'full_retained_migration_units', 'partial_shared_kernel_units'}
    assert all(release[key] == frozen[key] for key in frozen if key not in changed)
    names = {'common116.py', 'contracts.py', 'catalogue.py', 'runner.py', '__main__.py'}
    assert set(release['files']) == names
    paths = {}
    for name, pin in release['files'].items():
        expected = root / 'tools/porting/paired/v1/common116.py' if name == 'common116.py' else home / name
        paths[name] = auth(pin)
        assert paths[name] == expected
    assert release['files']['common116.py'] == frozen['files']['common116.py']
    assert auth(release['recorder']) == root / 'tools/porting/record_command.py'
    package = types.ModuleType('_frozen_paired_v3')
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
        raise RuntimeError('Optimized Python cannot import topology release modules')
    root = Path.cwd().resolve()

    def option(name):
        assert argv.count(name) == 1
        at = argv.index(name)
        assert at + 1 < len(argv)
        return argv[at + 1]

    def pin(value):
        path, sha = value.rsplit('#', 1)
        return dict(path=path, sha256=sha)

    assert option('--mode') == 'fresh', 'OA topology profile is fresh-only'
    if argv and argv[0] == 'authority':
        release_pin = pin(option('--release'))
    else:
        binding_pin = pin(option('--bindings'))
        path = (root / binding_pin['path']).resolve()
        assert path.is_relative_to(root) and path != root
        raw = path.read_bytes()
        assert hashlib.sha256(raw).hexdigest() == binding_pin['sha256']
        binding = json.loads(raw)
        assert binding['schema'] == 'paired.RuntimeBinding.v1' and binding['mode'] == 'fresh'
        release_pin = binding['release']
    package, release, root = load_package(release_pin)
    return package.runner.main(argv)


if __name__ == '__main__':
    raise SystemExit(bootstrap(sys.argv[1:]))
