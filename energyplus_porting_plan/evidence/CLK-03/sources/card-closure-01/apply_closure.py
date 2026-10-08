"""Root-reviewed metadata application; runs no numerical engine or comparer."""
import hashlib
import json
import subprocess
import sys
import tomllib
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
sys.path.insert(0, str(ROOT / 'tools/porting'))
from check_plan import validate_plan

BASE = '.runtime/porting/CLK-03/'
PLAN = 'energyplus_porting_plan/'
PROPOSAL = BASE + 'completion-proposed-01/proposal.json'
EXECUTION = BASE + 'completion-proposed-command-01/receipt.json'
TARGETS = [PLAN + 'evidence/CLK-03.json', PLAN + 'plan.json', PLAN + 'cards/CLK-03.md']
GATES = ['scope_review', 'unit_gate', 'integration_gate', 'production_gate']

def require(value, message):
    if not value:
        raise ValueError(message)

def now():
    return datetime.now(timezone.utc).isoformat()

def path(value):
    p = (ROOT / value).resolve()
    require(p.is_relative_to(ROOT), 'Path escapes repository')
    return p

def ref(value):
    p = path(value)
    with p.open('rb') as stream:
        digest = hashlib.file_digest(stream, 'sha256').hexdigest()
    return {'path': p.relative_to(ROOT).as_posix(), 'sha256': digest}

def bound(value):
    require(type(value) is dict and set(value) == {'path', 'sha256'}, 'Exact binding required')
    require(ref(value['path']) == value, 'Artifact bytes differ: ' + value['path'])
    return value

def read(value):
    def pairs(items):
        result = {}
        for key, item in items:
            require(key not in result, 'Duplicate JSON key')
            result[key] = item
        return result
    return json.loads(path(value).read_text(encoding='utf8'), object_pairs_hook=pairs,
                      parse_constant=lambda value: (_ for _ in ()).throw(ValueError(value)))

def command(value):
    receipt = read(value)
    require(receipt['schema'] == 'recorded-porting-command.v1'
            and type(receipt['exit_code']) is int and receipt['exit_code'] == 0
            and receipt['launch_error'] is None and receipt['source_bytes_match_before_and_after'] is True,
            'Actual successful unchanged-source command required')
    for name in ['stdout', 'stderr', 'source_snapshot']:
        bound(receipt[name])
    return receipt

def main():
    require(Path.cwd().resolve() == ROOT, 'Repository cwd required')
    require(len(sys.argv) == 3 and sys.argv[1] == '--output-dir', 'Explicit output directory required')
    output = path(sys.argv[2])
    require(output.is_relative_to(path(BASE)) and not output.exists(), 'Fresh ignored output required')
    started = now()
    proposal = read(PROPOSAL)
    receipt = command(EXECUTION)
    require(read(receipt['stdout']['path']) == proposal, 'Actual proposal stdout differs')
    require(proposal['actual_writer_command'] == receipt['command'], 'Actual proposal argv differs')
    require(receipt['finished_utc'] <= started, 'Proposal must precede application')
    require(proposal['schema'] == 'clk03-proposed-card-closure.v1'
            and proposal['status'] == 'ready-for-reviewed-root-apply'
            and proposal['only_CLK03_plan_task_changed'] is True
            and proposal['proposed_four_CLK03_gates_pass'] is True
            and proposal['canonical_files_written'] is False
            and proposal['native_retirement_admitted'] is False, 'Reviewed proposal protocol differs')
    before = [bound(row) for row in proposal['before']]
    require({row['path'] for row in before} == set(TARGETS[1:]), 'Exact prior canonical files required')
    require(proposal['previously_absent'] == [TARGETS[0]] and not path(TARGETS[0]).exists(),
            'Canonical evidence must be absent')
    require(len(proposal['after']) == 3 and [row['historical_path'] for row in proposal['after']] == TARGETS,
            'Exact three proposed destinations required')
    proposed = {row['historical_path']: path(bound(row['archive'])['path']).read_bytes()
                for row in proposal['after']}
    for row in proposal['executed_writer_sources']:
        require(ref(row['historical_path'])['sha256'] == bound(row['archive'])['sha256'],
                'Executed writer source changed')
    for row in proposal['preserved_historical_evidence']:
        bound(row)
    old = read(TARGETS[1])
    plan = json.loads(proposed[TARGETS[1]])
    require(len(old['tasks']) == len(plan['tasks'])
            and all(a == b for a, b in zip(old['tasks'], plan['tasks']) if a['id'] != 'CLK-03')
            and all(old[key] == plan[key] for key in old if key != 'tasks'), 'Another task/root changed')
    old_task = next(row for row in old['tasks'] if row['id'] == 'CLK-03')
    task = next(row for row in plan['tasks'] if row['id'] == 'CLK-03')
    require(all(old_task[key] == '미확인' and task[key] == '통과' for key in GATES), 'Four gate transition differs')
    card = proposed[TARGETS[2]].decode('utf8')
    require(card.count('- [x] ') == 4 and '- [ ] ' not in card, 'Actual card gates differ')
    evidence = json.loads(proposed[TARGETS[0]])
    require(evidence['unit_counts']['comparison_count'] == 239178
            and evidence['unit_counts']['mismatch_count'] == 0
            and evidence['production_counts']['comparison_count'] == 1742058
            and evidence['production_counts']['mismatch_count'] == 0
            and evidence['preserved_prior_scientific_failure']['mismatch_count'] == 1,
            'Actual selected results/failure history differ')
    lock = tomllib.loads(path('config/default.toml').read_text(encoding='utf-8-sig'))['oracle']
    def proposed_read(p):
        name = p.resolve().relative_to(ROOT).as_posix()
        return proposed[name] if name in proposed else p.read_bytes()
    errors = validate_plan(plan, lock, ROOT, path(PLAN), proposed_read)
    require(not errors, 'Proposed plan invalid: ' + '; '.join(errors))
    source_bytes = Path(__file__).read_bytes()
    output.mkdir()
    (output / 'apply_closure.py').write_bytes(source_bytes)
    views = [PLAN + 'CHECKLIST.md', PLAN + 'energyplus_porting_checklist.html']
    views_before = [ref(value) for value in views]
    for target in TARGETS:
        path(target).write_bytes(proposed[target])
        require(path(target).read_bytes() == proposed[target], 'Canonical write differs')
    sync_output = output / 'view-sync-command'
    argv = [sys.executable, '-X', 'utf8', '-B', 'tools/porting/record_command.py',
            '--output-dir', sync_output.relative_to(ROOT).as_posix(), '--',
            sys.executable, '-X', 'utf8', '-B', 'tools/porting/sync_plan_views.py', '--write']
    sync = subprocess.run(argv, cwd=ROOT, capture_output=True)
    (output / 'view-sync-launcher.stdout.log').write_bytes(sync.stdout)
    (output / 'view-sync-launcher.stderr.log').write_bytes(sync.stderr)
    require(sync.returncode == 0, 'Actual view sync failed; applied canonical bytes remain recorded')
    sync_receipt = sync_output.relative_to(ROOT).as_posix() + '/receipt.json'
    command(sync_receipt)
    require(read(TARGETS[1]) == plan, 'Canonical plan changed after apply')
    for row in proposal['preserved_historical_evidence']:
        bound(row)
    result = {'schema': 'clk03-metadata-card-closure.v1', 'status': 'applied-reviewed-canonical-card-closure',
              'started_utc': started, 'completed_utc': now(), 'actual_command': list(sys.orig_argv),
              'actual_cwd': str(ROOT), 'before': before, 'previously_absent': [TARGETS[0]],
              'after': [ref(value) for value in TARGETS], 'view_before': views_before,
              'view_after': [ref(value) for value in views], 'view_sync_execution': ref(sync_receipt),
              'proposal': ref(PROPOSAL), 'proposal_execution': ref(EXECUTION),
              'only_CLK03_gates_updated': True, 'all_four_CLK03_gates_pass': True,
              'preserved_historical_evidence': proposal['preserved_historical_evidence'],
              'unit_comparison': bound(proposal['unit_comparison']),
              'production_comparison': bound(proposal['production_comparison']),
              'independent_unit_result_review': bound(proposal['independent_unit_result_review']),
              'independent_production_result_review': bound(proposal['independent_production_result_review']),
              'executed_apply_source': ref((output / 'apply_closure.py').relative_to(ROOT).as_posix()),
              'scientific_or_native_execution_performed': False, 'native_retirement_admitted': False,
              'prior_actual_failure_relabelled_as_PASS': False}
    (output / 'closure.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n', encoding='utf8')
    print(json.dumps(result, ensure_ascii=False, allow_nan=False))

if __name__ == '__main__':
    main()
