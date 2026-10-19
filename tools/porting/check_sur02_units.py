#!/usr/bin/env python3
"""One bounded, explicit SUR-02 unit comparison; not part of the default suite.

The Native adapter receives a byte-exact Original block. The Rust adapter includes
the entire unchanged production suffix and a byte-exact production group/index
helper excerpt. Both receive only the same declared raw16 primary input file.
No Cargo, EnergyPlus engine, normal CLI or final15 case is launched.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import shutil
import struct
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
ORIGINAL = ROOT / '.reference/energyplus-src/26.1.0/src/EnergyPlus/HeatBalanceSurfaceManager.cc'
SUFFIX = ROOT / 'crates/ep_runtime/src/heat_balance/ordinary_surface_initialization/suffix.rs'
PREFIX = ROOT / 'crates/ep_runtime/src/heat_balance/ordinary_surface_initialization/prefix.rs'
PLAN = ROOT / '.runtime/porting/SRC-06/Source-Surface-live-kickoff-observer-Peer-59-01/complete-proposed-compiler-Source-plan.json'
PINNED = {
    ORIGINAL: '0d50903b9cebab76608a76c8643a4686f94f053c82030d7577d652971b67f73c',
    SUFFIX: '0e48e22d49512892f912a54ccea9271dd0846632c196fcec62ecad9604cef41c',
    PREFIX: '621d4b5db3dbe482871d2832df7b92fd49ea6a6ac17bd24ab850c68127d914eb',
    PLAN: 'ae1b1d0bbb0df3a27ff348efa8016dcac0a27db056ad1271f4bafe81f6373fe2',
}


def sha(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def ref(path: Path) -> dict:
    raw = path.read_bytes()
    try:
        name = path.resolve().relative_to(ROOT).as_posix()
    except ValueError:
        name = path.resolve().as_posix()
    return {'path': name, 'sha256': sha(raw), 'bytes': len(raw)}


def write_json(path: Path, value: object) -> dict:
    path.write_bytes((json.dumps(value, ensure_ascii=True, indent=2, allow_nan=False) + '\n').encode('utf-8'))
    return ref(path)


def raw16(value: float) -> str:
    assert math.isfinite(value)
    return struct.pack('>d', value).hex()


def source_guard() -> dict:
    for path, expected in PINNED.items():
        assert sha(path.read_bytes()) == expected, f'pinned source changed: {path}'
    plan = json.loads(PLAN.read_bytes())
    assert len(plan['entries']) == 4562
    ledger = []
    for entry in plan['entries']:
        path = ROOT / entry['path']
        actual = ref(path)
        assert actual['sha256'] == entry['final_sha256'] and actual['bytes'] == entry['final_bytes'], entry['path']
        ledger.append(actual)
    return {'count': len(ledger), 'all_match_complete_Source59_plan': True,
            'ordered_raw_file_ledger_sha256': sha(json.dumps(ledger, sort_keys=True).encode('utf-8'))}


def coefficients(terms: int, current_sign: int = 1) -> list[list[float]]:
    # Authored primary coefficients, never an expected result from either backend.
    out = [[0.0] * 19 for _ in range(4)]
    for channel in range(4):
        out[channel][0] = current_sign * math.ldexp(float(channel + 1), 450)
    for term in range(1, 19):
        out[0][term] = (term + 1) / 8.0
        out[1][term] = -(term + 2) / 16.0
        out[2][term] = (term + 3) / 32.0
        out[3][term] = (-1 if term % 2 else 1) * (term + 1) / 64.0
    return out


def histories(surfaces: int, current_sign: int = 1) -> list[list[list[float]]]:
    a = [[[0.0] * surfaces for _ in range(19)] for _ in range(4)]
    for channel in range(4):
        for surface in range(surfaces):
            a[channel][0][surface] = current_sign * math.ldexp(float(2 + channel + surface), 450)
    return a


def life(name: str, terms: int, calls: list[tuple[str, list]], current_sign: int = 1) -> dict:
    return {'name': name, 'zones': [[1]], 'spaces': [[1, 1]],
            'surfaces': [[40, 0, 1]], 'construction_terms': [terms],
            'coefficients': [coefficients(terms, current_sign)], 'history_ids': [40],
            'initial': [[37.0, -41.0]], 'calls': calls}


def build_inputs() -> tuple[bytes, list[dict], list[dict]]:
    lives = []
    for terms in (0, 1):
        a = histories(1)
        for channel in range(4):
            for term in range(1, 19):
                a[channel][term][0] = (channel + 1) * (term + 1) / 8.0
        lives.append(life(f'N{terms}_reset', terms, [('nonzero_primary', a), ('repeat_without_output_reset', a)]))
    for terms in (2, 18):
        for channel, label in enumerate(('outside_temperature', 'inside_temperature', 'outside_flux', 'inside_flux')):
            calls = []
            for term in range(1, terms + 1):
                for sign, suffix in ((1, 'positive'), (-1, 'negative')):
                    a = histories(1)
                    a[channel][term][0] = sign * (term + 3) / 4.0
                    if terms == 2:
                        # Inactive tail is a canary, not another selected past term.
                        for tail_channel in range(4):
                            a[tail_channel][18][0] = (tail_channel + 9) / 8.0
                    calls.append((f'term{term}_{suffix}', a))
            lives.append(life(f'N{terms}_{label}_isolated', terms, calls))
    dense = histories(1)
    for channel in range(4):
        for term in range(1, 19):
            # Dyadic cancellation inputs exercise fixed accumulation order.
            dense[channel][term][0] = ((-1.0) ** (term + channel)) * (math.ldexp(1.0, 48) + (term + 1) / 8.0)
    variant = [[row[:] for row in channel] for channel in dense]
    for channel in range(4):
        variant[channel][0][0] = -math.ldexp(float(channel + 7), 450)
    lives.append(life('dense_cancellation', 18, [('dense', dense), ('repeat', dense), ('current_history_variant', variant), ('zero_past_reset', histories(1))]))
    lives.append(life('current_coefficient_variant', 18, [('dense', dense)], -1))
    impulse = []
    for term in (0, 1, 2, 3, 0):
        a = histories(1)
        if term:
            a[0][term][0] = 1.0
        impulse.append((f'impulse_step{len(impulse)}_past_term{term}', a))
    lives.append(life('persistent_impulse_shift', 3, impulse))
    zero = histories(1)
    lives.append(life('zero_past', 18, [('zero', zero)]))
    multi = histories(3)
    for channel in range(4):
        for term in range(1, 19):
            for surface in range(3):
                multi[channel][term][surface] = (-1 if channel % 2 else 1) * (100 * surface + 10 * channel + term) / 16.0
    lives.append({'name': 'reordered_owners', 'zones': [[2], [1]], 'spaces': [[1, 1], [2, 3]],
                  'surfaces': [[40, 0, 2], [8, 0, 1], [91, 2, 3]],
                  'construction_terms': [18, 2, 2],
                  'coefficients': [coefficients(18), coefficients(2, -1), coefficients(2)],
                  'history_ids': [8, 91, 40], 'initial': [[3.0, -4.0], [5.0, -6.0], [7.0, -8.0]],
                  'calls': [('first', multi), ('repeat', multi), ('zero_past_reset', histories(3))]})
    lines = ['SUR02_INPUT_V1', f'LIFETIMES {len(lives)}']
    matrix = []
    for v in lives:
        assert len(v['calls']) < 100
        ns = len(v['surfaces'])
        lines.append(f"LIFE {v['name']} {len(v['zones'])} {len(v['spaces'])} {ns} {len(v['construction_terms'])} {len(v['history_ids'])} {len(v['calls'])}")
        for z in v['zones']:
            lines.append('ZONE ' + ' '.join(map(str, [len(z), *z])))
        for s in v['spaces']:
            lines.append('SPACE ' + ' '.join(map(str, s)))
        for s in v['surfaces']:
            lines.append('SURFACE ' + ' '.join(map(str, s)))
        for terms, arrays in zip(v['construction_terms'], v['coefficients'], strict=True):
            lines.append('CONSTRUCTION ' + str(terms) + ' ' + ' '.join(raw16(x) for a in arrays for x in a))
        lines.append('HISTORY_IDS ' + ' '.join(map(str, v['history_ids'])))
        lines.append('INITIAL ' + ' '.join(raw16(x) for row in v['initial'] for x in row))
        for name, arrays in v['calls']:
            lines.append('CALL ' + name + ' ' + ' '.join(raw16(x) for a in arrays for row in a for x in row))
        matrix.append({'life': v['name'], 'calls': [x[0] for x in v['calls']], 'surfaces': ns,
                       'construction_terms': v['construction_terms'], 'physical_to_typed': [x[0] for x in v['surfaces']],
                       'history_typed_order': v['history_ids'], 'zone_space_order': v['zones'],
                       'space_surface_ranges': v['spaces'], 'shared_primary_input_only': True})
    lines.append('END')
    return ('\n'.join(lines) + '\n').encode('ascii'), matrix, lives


def command(argv: list[str], stem: str, out: Path, artifacts: list[dict], timeout: int = 120) -> dict:
    start = time.time()
    launched = None
    launch_error = None
    try:
        launched = subprocess.run(argv, cwd=ROOT, capture_output=True, timeout=timeout, check=False)
        stdout, stderr, code = launched.stdout, launched.stderr, launched.returncode
    except (OSError, subprocess.TimeoutExpired) as exc:
        stdout, stderr, code = b'', str(exc).encode('utf-8'), None
        launch_error = type(exc).__name__ + ': ' + str(exc)
    stdout_path = out / (stem + '.stdout')
    stderr_path = out / (stem + '.stderr')
    stdout_path.write_bytes(stdout)
    stderr_path.write_bytes(stderr)
    receipt = {'argv': argv, 'cwd': ROOT.as_posix(), 'exit_code': code, 'launch_error': launch_error,
               'started_utc_epoch': start, 'finished_utc_epoch': time.time(),
               'stdout': ref(stdout_path), 'stderr': ref(stderr_path)}
    receipt_path = out / (stem + '.receipt.json')
    receipt_ref = write_json(receipt_path, receipt)
    artifacts.extend([receipt_ref, receipt['stdout'], receipt['stderr']])
    assert code == 0 and launch_error is None, f'command failed: {stem}; see {receipt_path}'
    return receipt_ref


def parse_output(raw: bytes) -> list[dict]:
    lines = raw.decode('ascii').splitlines()
    assert len(lines) % 6 == 0
    rows = []
    for start in range(0, len(lines), 6):
        header, before, pre, after, post, identity = lines[start:start + 6]
        pieces = header.split()
        assert len(pieces) == 6 and pieces[0] == 'CASE' and pieces[2] == 'CALL' and pieces[4] == 'INDEX'
        assert before.startswith('BEFORE ') and after.startswith('AFTER ')
        assert pre.startswith('READSET_BEFORE ') and post.startswith('READSET_AFTER ')
        assert pre.removeprefix('READSET_BEFORE ') == post.removeprefix('READSET_AFTER '), header
        assert identity == 'OWNER_STORAGE_STABLE 1'
        outputs = after.split()[1:]
        for x in outputs:
            assert len(x) == 16 and int(x, 16) & 0x7ff0000000000000 != 0x7ff0000000000000
        rows.append({'life': pieces[1], 'call': pieces[3], 'index': int(pieces[5]),
                     'before_output_bits': before.split()[1:], 'after_output_bits': outputs,
                     'whole_readset_before': pre.removeprefix('READSET_BEFORE '),
                     'whole_readset_after': post.removeprefix('READSET_AFTER '), 'owner_storage_stable': True})
    return rows


def compare(native: bytes, rust: bytes, matrix: list[dict], lives: list[dict]) -> dict:
    a, b = parse_output(native), parse_output(rust)
    assert len(a) == len(b) == sum(len(v['calls']) for v in matrix)
    results, scalar_count, offset = [], 0, 0
    by_name = {}
    for m, life_data in zip(matrix, lives, strict=True):
        previous = [raw16(x) for row in life_data['initial'] for x in row]
        for call_index, name in enumerate(m['calls']):
            nr, sr = a[offset], b[offset]
            assert nr == sr, f"literal/actual Source difference: {m['life']}/{name}"
            assert nr['life'] == m['life'] and nr['call'] == name
            assert nr['index'] == call_index
            assert len(nr['after_output_bits']) == 2 * m['surfaces']
            assert nr['before_output_bits'] == previous, 'previous actual output must remain next input'
            previous = nr['after_output_bits']
            scalar_count += len(previous)
            by_name[(m['life'], name)] = previous
            results.append({'life': m['life'], 'call': name, 'output_binary64_fields': len(previous),
                            'all_output_bits_equal': True, 'whole_readsets_unchanged': True,
                            'owner_storage_stable': True})
            offset += 1
    for n in (0, 1):
        assert by_name[(f'N{n}_reset', 'nonzero_primary')] == ['0000000000000000'] * 2
    assert by_name[('zero_past', 'zero')] == ['0000000000000000'] * 2
    dense = by_name[('dense_cancellation', 'dense')]
    assert dense == by_name[('dense_cancellation', 'repeat')] == by_name[('dense_cancellation', 'current_history_variant')]
    assert dense == by_name[('current_coefficient_variant', 'dense')]
    assert by_name[('dense_cancellation', 'zero_past_reset')] == ['0000000000000000'] * 2
    for m in matrix:
        if m['life'].endswith('_isolated'):
            for name in m['calls']:
                bits = by_name[(m['life'], name)]
                magnitude = [int(x, 16) & 0x7fffffffffffffff for x in bits]
                if '_outside_flux_' in m['life']:
                    assert magnitude[0] == 0 and magnitude[1] != 0, 'outside flux affects only the outside constant'
                elif '_inside_flux_' in m['life']:
                    assert magnitude[0] != 0 and magnitude[1] == 0, 'inside flux affects only the inside constant'
                else:
                    assert all(magnitude), 'temperature impulse must affect both constants'
                if name.endswith('_positive'):
                    other = by_name[(m['life'], name.removesuffix('_positive') + '_negative')]
                    for positive, negative in zip(bits, other, strict=True):
                        if int(positive, 16) & 0x7fffffffffffffff:
                            assert int(positive, 16) ^ int(negative, 16) == 0x8000000000000000, 'single impulse sign contrast'
    assert by_name[('reordered_owners', 'first')][-2:] == [raw16(7.0), raw16(-8.0)], 'nonCTF skipped outputs retain actual incoming values'
    return {'schema': 'sur02-strict-unit-comparison.v1', 'accepted': True, 'findings': [],
            'life_count': len(matrix), 'call_count': len(a), 'compared_output_binary64_fields': scalar_count,
            'whole_readsets_unchanged': True, 'all_Cpp_Rust_output_bits_equal': True,
            'raw_stdout_bytes_equal': native == rust, 'per_case_results': results,
            'current_coefficient_and_history_zero_index_canaries_ignored': True,
            'repeat_call_resets_output_instead_of_accumulating': True,
            'nonCTF_skip_retains_incoming_outputs': True,
            'literal_Original_whole_parent_calls': 0, 'actual_Rust_selected_method_calls': len(a)}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', required=True, type=Path)
    parser.add_argument('--cxx', type=Path)
    parser.add_argument('--rustc', type=Path)
    args = parser.parse_args()
    out = args.output.resolve()
    assert not out.exists(), 'fresh output directory required'
    assert out.is_relative_to(ROOT / '.runtime/porting'), 'output must stay within ignored .runtime/porting'
    out.mkdir(parents=True)
    artifacts: list[dict] = []
    report = {'schema': 'sur02-literal-and-current-producer-units.v1', 'accepted': False, 'findings': [],
              'command_argv': [sys.executable, *sys.argv], 'publication_artifacts': artifacts}
    try:
        before = source_guard()
        head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT).decode('ascii').strip()
        original_raw, prefix_raw = ORIGINAL.read_bytes(), PREFIX.read_bytes()
        selected = b''.join(original_raw.splitlines(keepends=True)[488:536])
        assert sha(selected) == 'c1f79a50269d0378139a19d6bfda67c824e99ea4d19699a15c78270925e9c268' and len(selected) == 3005
        start = prefix_raw.index(b'pub(super) fn index(')
        end = prefix_raw.index(b'\nimpl SourceSurfaceHeatBalanceArrays', start)
        prefix_selected = prefix_raw[start:end]
        first = prefix_raw[:start].count(b'\n') + 1
        last = prefix_raw[:end].count(b'\n')
        original_extent = out / 'Original-HeatBalanceSurfaceManager-489-536.cpp'
        prefix_extent = out / 'actual-prefix-selected.rs'
        frozen_suffix = out / 'actual-suffix-whole.rs'
        original_extent.write_bytes(selected)
        prefix_extent.write_bytes(prefix_selected)
        frozen_suffix.write_bytes(SUFFIX.read_bytes())
        cpp_template = Path(__file__).with_name('sur02_reference.cpp')
        rs_template = Path(__file__).with_name('sur02_reference.rs')
        cpp = cpp_template.read_bytes()
        rs = rs_template.read_bytes()
        assert cpp.count(b'/*__ORIGINAL_BLOCK__*/') == 1 and rs.count(b'/*__PRODUCTION_PREFIX__*/') == 1
        cpp = cpp.replace(b'/*__ORIGINAL_BLOCK__*/', selected)
        rs = rs.replace(b'/*__PRODUCTION_PREFIX__*/', prefix_selected)
        assert rs.count(b'__ACTUAL_SUFFIX_PATH__') == 1
        rs = rs.replace(b'__ACTUAL_SUFFIX_PATH__', SUFFIX.as_posix().encode('utf-8'))
        cpp_path, rs_path = out / 'reference-generated.cpp', out / 'source-generated.rs'
        cpp_path.write_bytes(cpp)
        rs_path.write_bytes(rs)
        assert cpp.replace(selected, b'/*__ORIGINAL_BLOCK__*/', 1) == cpp_template.read_bytes()
        restored_rs = rs.replace(prefix_selected, b'/*__PRODUCTION_PREFIX__*/', 1).replace(SUFFIX.as_posix().encode('utf-8'), b'__ACTUAL_SUFFIX_PATH__', 1)
        assert restored_rs == rs_template.read_bytes()
        input_bytes, matrix, lives = build_inputs()
        input_path = out / 'primary-input.txt'
        input_path.write_bytes(input_bytes)
        policy_path, matrix_path = out / 'policy.json', out / 'case-matrix.json'
        write_json(policy_path, {'schema': 'sur02-frozen-exact-unit-policy.v1', 'atol': 0, 'rtol': 0,
                   'binary64_bits': 'exact including signed zero', 'IDs_shapes_selectors_call_order': 'exact',
                   'finite_primary_inputs_only': True, 'whole_readsets_before_after': 'exact unchanged',
                   'all_outputs_observed': ['SurfCTFConstInPart', 'SurfCTFConstOutPart'],
                   'adapters': 'Explicit minimal owner/array types; not whole Native/Rust parent storage equivalence.',
                   'Original_arithmetic': 'Unmodified literal489-536; no copied formula elsewhere.',
                   'Rust_arithmetic': 'Entire actual production suffix included unchanged; exact production index/source_groups excerpt.',
                   'reference_answers_enter_Source_owners': False, 'frozen_before_compiler_execution': True})
        write_json(matrix_path, {'schema': 'sur02-declared-primary-case-matrix.v1', 'lifetimes': matrix,
                   'case_families': ['N0/1 reset', 'N2/N18 four channels all past terms both signs', 'zero',
                                    'dense cancellation/order', '0th coefficient/history canaries', 'retained repeated calls',
                                    'persistent shifted impulse', 'multiple reordered zone/space/surface/history owners',
                                    'nonCTF skip'], 'literal_parent_calls': 0})
        report.update({'observed_HEAD': head, 'source_contract': {
            'Original_commit': '6f2e40d10250a105b49966baa24d843711e61048', 'Original_whole': ref(ORIGINAL),
            'Original_selected_extent': {'first': 489, 'last': 536, **ref(original_extent)},
            'actual_Rust_suffix': ref(SUFFIX), 'actual_Rust_prefix': ref(PREFIX),
            'frozen_whole_Rust_suffix': ref(frozen_suffix),
            'selected_prefix_extent': {'first': first, 'last': last, **ref(prefix_extent)},
            'complete_Source59_plan': ref(PLAN)},
            'input': ref(input_path), 'policy': ref(policy_path), 'case_matrix': ref(matrix_path),
            'generated_wrappers': [ref(cpp_path), ref(rs_path)],
            'author_sources': [ref(Path(__file__)), ref(cpp_template), ref(rs_template)],
            'template_and_literal_inverse_verified': True})
        artifacts.extend([ref(input_path), ref(policy_path), ref(matrix_path), ref(cpp_path), ref(rs_path),
                          ref(original_extent), ref(prefix_extent), ref(frozen_suffix),
                          *report['author_sources']])
        frozen_inputs = [(p, ref(p)) for p in [input_path, policy_path, matrix_path, cpp_path, rs_path, ORIGINAL, SUFFIX, PREFIX]]
        cxx_default = ROOT / '.runtime/reference-tools/winlibs-gcc13.2.0-ucrt-r3/mingw64/bin/g++.exe'
        cxx = args.cxx or (cxx_default if cxx_default.is_file() else Path(shutil.which('g++') or ''))
        rustc = args.rustc or Path(shutil.which('rustc') or '')
        assert cxx.is_file() and rustc.is_file(), 'g++ and rustc must be available'
        report['compiler_executables'] = {'cxx': ref(cxx), 'rustc': ref(rustc)}
        report['tool_version_receipts'] = [command([str(cxx), '--version'], 'cxx-version', out, artifacts),
                                            command([str(rustc), '--version', '--verbose'], 'rustc-version', out, artifacts)]
        ext = '.exe' if os.name == 'nt' else ''
        native_exe, source_exe = out / ('reference' + ext), out / ('source' + ext)
        cpp_argv = [str(cxx), '-std=c++17', '-O0', '-ffp-contract=off', '-Wall', '-Wextra', '-pedantic',
                    str(cpp_path), '-o', str(native_exe)]
        if os.name == 'nt':
            cpp_argv[1:1] = ['-static', '-static-libgcc', '-static-libstdc++']
        report['compile_receipts'] = [command(cpp_argv, 'compile-cpp', out, artifacts),
                                      command([str(rustc), '--crate-name', 'sur02_current_producer_reference', '--edition=2024', '-C', 'opt-level=0', '-C', 'overflow-checks=yes',
                                               str(rs_path), '-o', str(source_exe)], 'compile-rust', out, artifacts)]
        report['runtime_binaries_retained_not_published'] = [ref(native_exe), ref(source_exe)]
        report['execution_receipts'] = [command([str(native_exe), str(input_path)], 'run-cpp', out, artifacts),
                                        command([str(source_exe), str(input_path)], 'run-rust', out, artifacts)]
        comparison = compare((out / 'run-cpp.stdout').read_bytes(), (out / 'run-rust.stdout').read_bytes(), matrix, lives)
        comparison_ref = write_json(out / 'comparison.json', comparison)
        artifacts.append(comparison_ref)
        for p, before_ref in frozen_inputs:
            assert ref(p) == before_ref, f'input/source/policy changed during execution: {p}'
        after = source_guard()
        assert before == after
        assert subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT).decode('ascii').strip() == head
        report.update({'accepted': True, 'comparison': comparison_ref,
                       'counts': {key: comparison[key] for key in ['life_count', 'call_count', 'compared_output_binary64_fields']},
                       'before_after_compiler_inputs': {'before': before, 'after': after, 'count': 4562, 'unchanged': True},
                       'all_shared_input_source_policy_bytes_unchanged': True,
                       'qualifications': ['Selected Original literal-block unit and actual Rust selected-method arithmetic with explicit minimal owner adapters.',
                                          'No full InitSurfaceHeatBalance call or full production owner-type numerical certificate.',
                                          'Actual current P3 real-owner cadence/constants/handoff connection is retained separately in final60.',
                                          'All4562 production compiler inputs match Source59 before/after. No Cargo/default-suite/normalCLI/final15 execution.',
                                          'No tolerance changes, expected-value injection or numeric normalization. Raw command streams are retained.'],
                       'new_Cargo_or_engine_or_normal_CLI_or_final15_executions': 0})
    except Exception as exc:
        report['findings'].append(type(exc).__name__ + ': ' + str(exc))
    unique = {x['path']: x for x in artifacts}
    report['publication_artifacts'] = list(unique.values())
    review_ref = write_json(out / 'review.json', report)
    print(json.dumps({'accepted': report['accepted'], 'review': review_ref, 'findings': report['findings']}, ensure_ascii=True))
    return 0 if report['accepted'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
