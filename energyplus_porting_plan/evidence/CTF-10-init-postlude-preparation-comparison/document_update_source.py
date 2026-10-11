"""Record actual selected Init comparison; preserve every card gate and old evidence."""
import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path.cwd()
DOCS = ROOT / 'energyplus_porting_plan'
BASE = ROOT / '.runtime/porting/CTF-10'
SUMMARY = DOCS / 'evidence/CTF-10-init-postlude-preparation-comparison/summary.json'
assert hashlib.sha256(SUMMARY.read_bytes()).hexdigest() == '2ab1b77789385dc7a650648b39d15b98e5bdb22d866b071bbdbe882a90124465'
summary = json.loads(SUMMARY.read_bytes())
assert summary['status'] == 'pass-selected-paired-Init-owner-fields' and summary['selected_paired_owner_fields_passed'] is True
paired = summary['comparison']
assert paired['mismatch_count'] == 0 and paired['complete_mismatch_ledger'] == [] and paired['mismatch_records_dropped'] == 0
assert all(r['required'] is False and r['counted_as_PASS'] is False for r in paired['unavailable_observations'])
assert all(summary[k] is False for k in ['whole_Init_equivalence_claimed', 'ordinary_persistent_integration_PASS_claimed', 'complete_CTF10_PASS_claimed', 'card_gates_promoted', 'A_and_B_gate_counts_changed', 'gates_updated'])
quality = summary['actual_quality']
assert all(r['actual_exit_code'] == r['failed'] == r['ignored'] == 0 for r in quality.values())
names = ['plan.json', 'cards/CTF-10.md', 'CHECKLIST.md', 'energyplus_porting_checklist.html']
before = {n: (DOCS / n).read_bytes() for n in names}
texts = {n: b.decode('utf8') for n, b in before.items()}
original = json.loads(texts['plan.json']); plan = copy.deepcopy(original)
task = next(t for t in plan['tasks'] if t['id'] == 'CTF-10')
prior = task['actual_selected_Init_postlude_owner_comparison']
assert prior['sha256'] == '167ea315b51dca7db6de633c5b1db4524d716e0f6318d70cf9797f91495e0857'
assert 'prior_failed_selected_Init_postlude_owner_comparison' not in task
task['prior_failed_selected_Init_postlude_owner_comparison'] = prior
task['actual_selected_Init_postlude_owner_comparison'] = dict(path=SUMMARY.relative_to(ROOT).as_posix(), sha256=hashlib.sha256(SUMMARY.read_bytes()).hexdigest(), size_bytes=SUMMARY.stat().st_size)
restored = copy.deepcopy(plan); t = next(t for t in restored['tasks'] if t['id'] == 'CTF-10')
t['actual_selected_Init_postlude_owner_comparison'] = t.pop('prior_failed_selected_Init_postlude_owner_comparison')
assert restored == original
new = {'plan.json': json.dumps(plan, ensure_ascii=False, indent=2, allow_nan=False) + '\n'}
commands = []
inventory = json.loads((ROOT / summary['actual_semantic_correction_inventory']['path']).read_bytes())
changed_files = '\n'.join('- `' + r['canonical_path'] + '`' for r in inventory['correction_application_sources'])
for role, row in quality.items():
    receipt = json.loads((ROOT / row['execution']['path']).read_bytes())
    assert receipt['exit_code'] == 0 and receipt['source_snapshot'] == summary['actual_Source_snapshot']
    commands.append(subprocess.list2cmdline(receipt['command']))
section = f'''## Actual Init postlude reference comparison

The [current selected Init comparison evidence](../evidence/CTF-10-init-postlude-preparation-comparison/summary.json) records {paired['numeric_comparisons']} numerical and {paired['discrete_comparisons']} discrete comparisons across the same frozen15 model and6 direct-state cases, with0 mismatches and no dropped records. All{len(paired['unavailable_observations'])} optional unpaired observations remain explicit and receive no PASS. Selected paired owner fields pass; whole-Init equivalence and persistent ordinary integration remain unconfirmed.

Corrections retain the genuine project Timestep/report preparation, early EMS Scan before registry allocation, ordered NominalR/NominalU film arithmetic, and real geometry predefined-table entries in Source order. Direct state preparation uses actual compiled inputs. The observer's historical card reference joins the authentic archived document. The [prior failed comparison](../evidence/CTF-10-init-postlude-comparison/summary.json) retains all829 original mismatches; the later production preflight and empty-diff validation failures remain in the new evidence lineage.

Pinned EP26.1 Source ranges: Data/EnergyPlusData.cc840–857; EMSManager.cc208–247; SimulationManager.cc622–705,801–944,1359–1507; Material.cc198–226,275–309; HeatBalanceManager.cc1390–1397,1497–1513,1777–1797; DataHeatBalance.cc893–982; SurfaceGeometry.cc641–669,1423–1530; OutputReportPredefined.cc1766–1835,1928–1973. The archived correction preparations retain the complete direct helpers, owner/default witnesses and exact deltas.

Changed Rust files:

{changed_files}

Original literal corrections and separate actual formatter deltas remain archived.

Actual fmt/Clippy0, focused{quality['focused']['passed']}, workspace{quality['workspace']['passed']} PASS/{quality['workspace']['test_groups']} groups, and direct-observer{quality['supplementary-direct-observer-tests']['passed']} PASS share SourceSnapshot `{summary['actual_Source_snapshot']['sha256']}`. Both candidate executions returned0. Actual quality commands:

```text
{chr(10).join(commands)}
```

The unchanged frozen arithmetic profiles and required-absence rules govern the comparison. Native04 and Clippy08 failures remain recorded. Selected scope exclusions, unavailable owners, different caller header inputs and optional backends receive no PASS. Complete CTF-10 gates, ordinary persistent history/warmup handoff and the predecessor integration chain remain unconfirmed; no card gate is promoted.

'''
card = texts['cards/CTF-10.md']; start = card.index('## Actual Init postlude reference comparison\n'); end = card.index('## Complete registry, producer diagnostics and Init postlude\n', start)
new['cards/CTF-10.md'] = card[:start] + section + card[end:]
notes = [l for l in texts['CHECKLIST.md'].splitlines() if l.startswith('CTF-10의 [현재 Init 비교 증거]')]
assert len(notes) == 1
note = f"CTF-10의 [현재 Init 비교 증거](evidence/CTF-10-init-postlude-preparation-comparison/summary.json)는 동일한 15개 모델·6개 직접 상태 사례에서 수치 {paired['numeric_comparisons']:,}개·상태/보고서 {paired['discrete_comparisons']:,}개를 비교해 불일치 0개를 기록했습니다. 전체 {quality['workspace']['passed']:,}개 테스트와 추가 관측기 {quality['supplementary-direct-observer-tests']['passed']}개도 통과했습니다. 선택 범위 밖 {len(paired['unavailable_observations'])}개 항목은 미확인으로 유지하며, 전체 초기화와 지속 실행 이력 인계는 아직 완료되지 않았습니다. 이전 829개 불일치와 수정 기록을 보존했고 A14/54·B14/64 완료 수는 변하지 않습니다."
new['CHECKLIST.md'] = texts['CHECKLIST.md'].replace(notes[0], note)
pattern = re.compile(r'(<script id="plan" type="application/json">)(.*?)(</script>)', re.S)
html = texts['energyplus_porting_checklist.html']; assert len(pattern.findall(html)) == 1 and json.loads(pattern.search(html)[2]) == original
payload = json.dumps(plan, ensure_ascii=False).replace('<', '\\u003c').replace('&', '\\u0026')
new['energyplus_porting_checklist.html'] = pattern.sub(lambda m: m[1] + payload + m[3], html)
table = lambda s: [l for l in s.splitlines() if l.startswith('|')]
assert len(table(texts['CHECKLIST.md'])) == 86 and table(texts['CHECKLIST.md']) == table(new['CHECKLIST.md'])
assert table(card) == table(new['cards/CTF-10.md']) and card.splitlines()[:8] == new['cards/CTF-10.md'].splitlines()[:8]
assert pattern.sub(r'\1\3', html) == pattern.sub(r'\1\3', new['energyplus_porting_checklist.html'])
archive = BASE / 'init-postlude-preparation-documents-01'; assert not archive.exists(); archive.mkdir()
for name in names:
    assert (DOCS / name).read_bytes() == before[name]
    destination = archive / 'before' / name; destination.parent.mkdir(parents=True, exist_ok=True); destination.write_bytes(before[name])
for name in names: (DOCS / name).write_bytes(new[name].encode('utf8'))
print(json.dumps(dict(status='updated-actual-selected-comparison-documents', selected_mismatches=0, optional_unpaired=len(paired['unavailable_observations']), gates_updated=False)), flush=True)
