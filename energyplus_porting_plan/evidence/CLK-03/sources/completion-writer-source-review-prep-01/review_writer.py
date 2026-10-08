"""Independent static/metadata review; never import or execute the proposal writer."""
from __future__ import annotations

import argparse
import ast
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[4]
BASE = '.runtime/porting/CLK-03/'
BUNDLE = BASE + 'completion-prep-02/'
SOURCES = {
    'common.py': 'f966ec1a2f28e28a3fdbb193d67bf39ce498cc1987ac2af833e47aa34bb2cc46',
    'proofs.py': 'c3758c935ebb3450449273e9bc3debf0b3b39368343dcf87de2931d8375ae17b',
    'quality.py': '1b7c74702f24926d00211942ba53df95890c32e75c46da651895a2a6e547be4a',
    'document.py': '8e007bcc72f484b13c7458e3793f296b3507a9affd7df8d90d23dc3ee8fdc07c',
    'propose_closure.py': '28b8cd33ce99435c8181b41cb9a53cc803f15c5ff6239709c548da60cadd9d88',
}
IMPORTS = {'__future__', 'argparse', 'copy', 'datetime', 'hashlib', 'json',
           'pathlib', 're', 'subprocess', 'sys', 'common', 'proofs', 'quality', 'document'}
FORBIDDEN = {'exec', 'eval', '__import__', 'compile', 'import_module', 'run_path',
             'run_module', 'Popen', 'run', 'call', 'check_call', 'check_output',
             'system', 'unlink', 'remove', 'rmtree', 'replace_file', 'rename'}


def require(ok, message):
    if not ok:
        raise ValueError(message)


def now():
    return datetime.now(timezone.utc).isoformat()


def path_of(value):
    path = Path(value)
    path = (path if path.is_absolute() else ROOT / path).resolve()
    require(path.is_relative_to(ROOT) and path.is_file(), 'Contained actual file required: ' + str(value))
    return path


def ref(value):
    path = path_of(value)
    with path.open('rb') as stream:
        digest = hashlib.file_digest(stream, 'sha256').hexdigest()
    return {'path': path.relative_to(ROOT).as_posix(), 'sha256': digest}


def read(value):
    def pairs(rows):
        out = {}
        for key, item in rows:
            require(key not in out, 'Duplicate JSON metadata key')
            out[key] = item
        return out
    def invalid(token):
        raise ValueError('Nonstandard JSON token: ' + token)
    return json.loads(path_of(value).read_text(encoding='utf8'), object_pairs_hook=pairs, parse_constant=invalid)


def literal(tree, name):
    return next(ast.literal_eval(node.value) for node in tree.body
                if isinstance(node, ast.Assign)
                and any(isinstance(target, ast.Name) and target.id == name for target in node.targets))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output-dir', type=Path, required=True)
    args = parser.parse_args()
    require(Path.cwd().resolve() == ROOT, 'Actual repository cwd required')
    output = (ROOT / args.output_dir).resolve()
    require(output.is_relative_to(ROOT / BASE) and output != ROOT / BASE and not output.exists(),
            'Fresh ignored review output required')
    started = now()
    trees = {}
    source_bytes = {}
    for name, expected in SOURCES.items():
        source = BUNDLE + name
        require(ref(source)['sha256'] == expected, 'Held source hash differs: ' + name)
        raw = path_of(source).read_bytes()
        tree = ast.parse(raw, filename=source)
        trees[name] = tree
        source_bytes[name] = raw
        require(len(raw.splitlines()) < 500, 'Source module exceeds bounded size')
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                require(all(alias.name in IMPORTS for alias in node.names), 'Unexpected import')
            elif isinstance(node, ast.ImportFrom):
                require(node.level == 0 and node.module in IMPORTS, 'Unexpected module import')
            elif isinstance(node, ast.Call):
                called = node.func.id if isinstance(node.func, ast.Name) else node.func.attr if isinstance(node.func, ast.Attribute) else None
                require(called not in FORBIDDEN, 'Execution/removal capability found: ' + str(called))
                if isinstance(node.func, ast.Attribute) and isinstance(node.func.value, ast.Name) and node.func.value.id == 'subprocess':
                    require(called == 'list2cmdline', 'Only command rendering allowed')
    require(literal(trees['propose_closure.py'], 'NAMES') == list(SOURCES), 'Exact five source bindings differ')
    contracts = {}
    for key, expected in literal(trees['common.py'], 'CONTRACTS').items():
        value = ref('energyplus_porting_plan/contracts/CLK-03-' + key + '.json')
        require(value['sha256'] == expected, 'Current R3 packet hash differs')
        contracts[key] = value
    packet = read(contracts['source']['path'])
    cases = read(contracts['cases']['path'])
    require(packet['packet_revision'] == cases['packet_revision'] == 3
            and len(packet['source_files']) == 15 and len(packet['selected_ranges']) == 109, 'R3 packet shape differs')
    require(cases['counts'] == {'handoff_sequences': 4, 'weather_sequences': 10, 'handoff_operations': 7,
                              'weather_operations': 44, 'total_sequences': 14, 'total_operations': 51}, 'Unit requests changed')
    pins = {}
    for file_name, assignment in [('proofs.py', 'PINNED'), ('quality.py', 'QUALITY')]:
        for key, (relative, expected) in literal(trees[file_name], assignment).items():
            actual = ref(BASE + relative)
            require(actual['sha256'] == expected, 'Actual fixed prerequisite bytes differ: ' + key)
            read(actual['path'])
            pins[assignment + '.' + key] = actual
    canonical = [ref('energyplus_porting_plan/plan.json'), ref('energyplus_porting_plan/cards/CLK-03.md')]
    plan = read(canonical[0]['path'])
    require(len(plan['tasks']) == 64 and len({row['id'] for row in plan['tasks']}) == 64, 'Canonical task uniqueness differs')
    gates = literal(trees['common.py'], 'GATES')
    for card, expected in [('CON-01', '\ud1b5\uacfc'), ('CLK-01', '\ud1b5\uacfc'), ('CLK-02', '\ud1b5\uacfc'), ('CLK-03', '\ubbf8\ud655\uc778')]:
        row = next(item for item in plan['tasks'] if item['id'] == card)
        require(all(row[key] == expected for key in gates), 'Canonical prerequisite/current gate state differs')
    card = path_of(canonical[1]['path']).read_text(encoding='utf8')
    placeholders = ['\uad6c\ud604 \ucee4\ubc0b:  \n', '\uc2dc\ud5d8 \uba85\ub839:  \n', '\uc99d\uac70 \uacbd\ub85c:  \n',
                    '\ucd5c\ub300\uc624\ucc28/RMSE/\uc0c1\ud0dc \ubd88\uc77c\uce58:  \n', '\ucd94\uac00 \uac80\ud1a0\ud560 helper:\n']
    require(card.count('- [ ] ') == 4 and all(card.count(item) == 1 for item in placeholders), 'Exact unclosed card placeholders differ')
    require(not (ROOT / 'energyplus_porting_plan/evidence/CLK-03.json').exists(), 'Canonical closure already exists')
    output.mkdir()
    archive_dir = output / 'source'
    archive_dir.mkdir()
    archives = []
    for name, raw in source_bytes.items():
        destination = archive_dir / name
        with destination.open('xb') as stream:
            stream.write(raw)
        archive = ref(destination)
        require(archive['sha256'] == SOURCES[name] == ref(BUNDLE + name)['sha256'], 'Held source/archive changed')
        archives.append({'historical_path': BUNDLE + name, 'archive': archive})
    own_archive = archive_dir / 'executed_review_writer.py'
    with own_archive.open('xb') as stream:
        stream.write(Path(__file__).read_bytes())
    require(all(ref(item['path']) == item for item in [*contracts.values(), *pins.values(), *canonical]), 'Reviewed metadata changed')
    result = {
        'schema': 'clk03-independent-completion-writer-source-review.v1',
        'status': 'pass-source-before-proposed-closure-metadata',
        'review_started_utc': started, 'review_completed_utc': now(),
        'reviewed_sources': archives, 'reviewed_contracts': contracts,
        'reviewed_fixed_prerequisites': pins, 'reviewed_unclosed_canonical_files': canonical,
        'executed_reader_source_archives': [{'historical_path': Path(__file__).relative_to(ROOT).as_posix(), 'archive': ref(own_archive)}],
        'source_review_scope': [
            'Only fresh ignored three-file proposals; canonical apply and native retirement are separately required.',
            'Required actual unit239178/0, matched production positive/0, full independent reviews, six CLI0, matched483 data and final-source QA0.',
            'Original14/51 chronology remains distinct from supplemental3/483; old1647956/1 remains failed.',
            'Strict actual stdout protocols: full report/production QA; compact unit, Original, failure and lineage; ordered six child log tails/receipts/progress/final matrix.',
            'Final scientific df88/47e/ca7b sources and separate actual precommit quality heads; no compiler-selected inventory claim.',
            'Typed exact comparisons remain the existing scientific readers; this writer transcribes reports without recomputing operands.',
            'Other63 plan tasks/root fields and prior card history preserved; four proposed gates do not constitute actual closure.'
        ],
        'authorship_disclosure': 'Reviewer did not author the five proposal sources or scientific comparers. Reviewer authored header/consumer and native glue and separately reviewed actual provenance; this static review does not independently certify those implementations.',
        'scientific_execution_performed': False, 'target_sources_imported_or_executed': False,
        'future_production_PASS_or_actual_card_closure_claimed': False,
        'canonical_files_written': False, 'gates_updated': False, 'native_retirement_admitted': False,
        'engines_Cargo_Git_or_comparers_executed_by_reviewer': False,
    }
    with (output / 'review.json').open('xb') as stream:
        stream.write((json.dumps(result, ensure_ascii=False, indent=2, allow_nan=False) + '\n').encode('utf8'))
    print(json.dumps(result, ensure_ascii=False, allow_nan=False))


if __name__ == '__main__':
    main()
