"""Root-only precheck/verify transport; pin projection is not validation.

The genuine frozen v6 child performs the full Source/physics validation. This
parent only projects the child's ordered recorder inputs without normalizing
SourceAuthority again. It never starts a model actor or changes frozen files.
"""
if not __debug__:
    raise RuntimeError('Assertions are required before reader prototype activity')

from pathlib import Path
import argparse, hashlib, importlib.util, json, re, subprocess, sys, types

FROZEN6 = dict(path='tools/porting/paired/v6/release.json', sha256='fad05d195e3969372afbcbff8451aff70b70484e3b12683c1890c51ac5c01754', bytes=9106)
R = Path.cwd().resolve()


def ref(path):
 b=path.read_bytes();return dict(path=path.relative_to(R).as_posix(),sha256=hashlib.sha256(b).hexdigest(),bytes=len(b))

def auth(pin):
 path=(R/pin['path']).resolve();assert path.is_relative_to(R);actual=ref(path);assert actual['sha256']==pin['sha256']and actual['bytes']==pin.get('bytes',actual['bytes']);return path

def token(pin):return pin['path']+'#'+pin['sha256']

def read(path):return json.loads(path.read_bytes())

def save(path,value):
 assert not path.exists();path.write_text(json.dumps(value,indent=2)+'\n',encoding='utf8',newline='\n');return ref(path)


def canonical_pin(pin):
    """Authenticate one actual ref; legacy optional size is never a boolean."""
    assert type(pin) is dict and set(pin) in [{'path', 'sha256'}, {'path', 'sha256', 'bytes'}]
    assert type(pin['path']) is str and pin['path'] and '\\' not in pin['path'] and ':' not in pin['path']
    assert all(part and part not in ['.', '..'] for part in pin['path'].split('/'))
    assert type(pin['sha256']) is str and re.fullmatch('[0-9a-f]{64}', pin['sha256']) is not None
    assert 'bytes' not in pin or type(pin['bytes']) is int and pin['bytes'] >= 0
    path = auth(pin)
    assert pin['path'] == path.relative_to(R).as_posix()
    return ref(path)


def load_metadata(release_pin):
    """Use the actual, authenticated frozen package; never spoof its location."""
    if not __debug__:
        raise RuntimeError('Assertions are required before release imports')
    assert release_pin == FROZEN6 and canonical_pin(release_pin) == FROZEN6
    release = read(auth(release_pin))
    assert release['schema'] == 'paired.Release.v6' and type(release['version']) is int and release['version'] == 6
    entry = auth(canonical_pin(release['files']['__main__.py']))
    assert entry == R / 'tools/porting/paired/v6/__main__.py'
    for pin in [*release['files'].values(), *release['tools'].values(), release['recorder']]:
        canonical_pin(pin)
    assert auth(release['recorder']) == R / 'tools/porting/record_command.py'
    spec = importlib.util.spec_from_file_location('_reader_projection_v6_bootstrap', entry)
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    package, loaded, root = module.load_package(release_pin)
    assert root == R and loaded == release
    return package, release, root


def raw_snapshot_rows(d, snapshot_pin):
    """Index authenticated JSON only, without claiming live/archive validation.

    The child still performs every frozen member/path/archive/current-file
    check. No rows, path resolutions or prior acceptance are cached here.
    """
    value = d.load(canonical_pin(snapshot_pin))
    assert value['schema'] == 'rust-command-source-snapshot.v1' and value['bytes_normalized'] is False
    rows = value['files']
    assert type(rows) is list
    members = {}
    for row in rows:
        assert type(row) is dict and set(row) == {'historical_path', 'size_bytes', 'archive'}
        name = row['historical_path']
        assert type(name) is str and name and name not in members
        assert '\\' not in name and ':' not in name and all(p and p not in ['.', '..'] for p in name.split('/'))
        assert type(row['size_bytes']) is int and row['size_bytes'] >= 0
        archive = row['archive']
        assert type(archive) is dict and set(archive) in [{'path', 'sha256'}, {'path', 'sha256', 'bytes'}]
        assert type(archive['path']) is str and archive['path'] and '\\' not in archive['path'] and ':' not in archive['path']
        assert all(p and p not in ['.', '..'] for p in archive['path'].split('/'))
        assert type(archive['sha256']) is str and re.fullmatch('[0-9a-f]{64}', archive['sha256']) is not None
        assert 'bytes' not in archive or type(archive['bytes']) is int and archive['bytes'] == row['size_bytes']
        members[name] = row
    return members


def ordered_inputs(pins):
    """Canonical three-key refs, first path order, and explicit conflicts."""
    result = {}
    for pin in pins:
        actual = canonical_pin(pin)
        path = actual['path']
        if path not in result:
            result[path] = actual
        else:
            assert result[path] == actual, 'Conflicting projected input identity'
    return tuple(result.values())


def project_context(package, release, binding_ref):
    """Reproduce contracts653-660 pin order, not load_context authorization."""
    d = package.contracts
    binding = d.load(binding_ref)
    assert binding['schema'] == 'paired.RuntimeBinding.v1' and binding['mode'] == 'fresh' and binding['release'] == FROZEN6
    case = d.load(binding['case_config'])
    d.exact_keys(case, d.CASE_KEYS, 'CaseConfig projection')
    assert case['schema'] == 'paired.CaseConfig.v1' and case['profile'] == release['profile']
    assert case['source_authority'] == binding['source_authority']
    for key in ['Root_review', 'independent_review']:
        review = d.load(binding[key])
        assert review['schema'] == 'paired.RuntimeReview.v1' and review['accepted'] is True and review['mode'] == 'fresh'
        assert review['release'] == FROZEN6 and review['case_config'] == binding['case_config'] and review['source_authority'] == binding['source_authority']
    authority = d.load(binding['source_authority'])  # Serialized data, NEVER normalized by this parent.
    assert authority['schema'] == 'paired.SourceAuthority.v3' and authority['mode'] == 'fresh'
    assert type(authority['pins']) is list and type(authority['inventory_count']) is int
    source_rows = raw_snapshot_rows(d, authority['snapshot'])
    assert len(source_rows) == authority['inventory_count']
    bundle = d._topology_contract(case['structure'])
    parent_rows = raw_snapshot_rows(d, bundle['parent_authority']['snapshot'])
    parent_derivation = d.load(bundle['parent_case']['structure']['derivation'])
    parent_pins = d._current_source_citations(
        [bundle['parent_case']['scientific'], parent_derivation['primary'], parent_derivation['report_cutoff']],
        parent_rows, False)
    recipe = case['scientific']
    pp = d.load(recipe['primary_pins'])
    # Exact sequence in frozen load_context, with no helper/baseline pin added.
    pins = bundle['pins'] + [binding_ref, binding['release'], binding['case_config'], binding['source_authority'], binding['Root_review'], binding['independent_review']] + d.reference_closure(case) + list(release['files'].values()) + authority['pins']
    for key in ['edit_ledger', 'clock']:
        pins += d.reference_closure(d.load(case[key]))
    ledger = d.load(case['edit_ledger'])
    pins += d.reference_closure(d.load(ledger['numeric_baseline']))
    pins += d.reference_closure(d.load(case['aliases']['science_contract'])) + [release['recorder']]
    for p in pp['Native_bundle'].values():
        d.auth(p)
        pins.append(p)
    pins += [bundle['parent_authority']['snapshot'], *parent_pins.values()]
    return binding, case, ordered_inputs(pins)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('phase', choices=['precheck', 'verify'])
    parser.add_argument('--home', required=True)
    parser.add_argument('--release', required=True)
    parser.add_argument('--projection-output')
    args = parser.parse_args(argv)
    assert args.release == token(FROZEN6), 'Only the exact frozen6 release is supported'
    home = (R / args.home).resolve()
    assert home.is_relative_to(R / '.runtime/porting') and home != R / '.runtime/porting' and home.is_dir()
    package, release, _ = load_metadata(FROZEN6)
    binding_ref = ref(home / 'binding.json')
    binding, case, pins = project_context(package, release, binding_ref)
    assert binding['case_config'] == ref(home / 'case-config.json')
    actors = [canonical_pin(ref(home / 'native/launch.json'))]
    output = home / ('native-precheck.json' if args.phase == 'precheck' else 'readback.json')
    base = [sys.executable, '-X', 'utf8', '-B', str(auth(release['files']['__main__.py']))]
    common = ['--bindings', token(binding_ref), '--case', case['id'], '--mode', 'fresh']
    if args.phase == 'verify':
        actors.append(canonical_pin(ref(home / 'source/launch.json')))
        # This object supplies command fields only; it is NOT a validated Context.
        command_fields = types.SimpleNamespace(binding_ref=binding_ref, case=case, mode='fresh')
        command = package.runner.verify_command(command_fields, *actors, output)
    else:
        command = base + ['precheck'] + common + ['--native-launch', token(actors[0]), '--output', str(output)]
    inputs = list(pins) + actors
    recorder = auth(release['recorder'])
    command_dir = home / (args.phase + '-command')
    recorder_argv = [sys.executable, '-X', 'utf8', '-B', str(recorder), '--output-dir', str(command_dir)]
    for pin in inputs:
        recorder_argv += ['--input', str(auth(pin))]
    recorder_argv += ['--', *command]
    if args.projection_output:
        target = (R / args.projection_output).resolve()
        assert target.is_relative_to(R / '.runtime') and not target.is_relative_to(home)
        assert target.parent.is_dir() and not target.exists()
        result = dict(schema='Root-frozen-v6-reader-input-projection.v1', non_authorizing=True,
                      phase=args.phase, home=home.relative_to(R).as_posix(), release=FROZEN6,
                      binding=binding_ref, case_id=case['id'], command=command, inputs_before=inputs,
                      recorder_argv=recorder_argv, full_validation_deferred_to_genuine_frozen_child=True,
                      recorder_or_child_processes_started=0, model_actor_calls=0)
        print(json.dumps(dict(projection=save(target, result))))
        return 0
    # Genuine execution only in normal mode. Preserve the coordinate basenames
    # and strengthen its fresh guards to avoid any partial log overwrite.
    stdout = home / (args.phase + '-recorder.stdout')
    stderr = home / (args.phase + '-recorder.stderr')
    assert not output.exists() and not command_dir.exists() and not stdout.exists() and not stderr.exists()
    with stdout.open('xb') as out, stderr.open('xb') as err:
        result = subprocess.run(recorder_argv, cwd=R, stdin=subprocess.DEVNULL, stdout=out, stderr=err)
    receipt = command_dir / 'receipt.json'
    print(json.dumps(dict(phase=args.phase, exit_code=result.returncode,
                          output=ref(output) if output.exists() else None,
                          receipt=ref(receipt) if receipt.exists() else None)))
    return result.returncode


if __name__ == '__main__':
    raise SystemExit(main())
