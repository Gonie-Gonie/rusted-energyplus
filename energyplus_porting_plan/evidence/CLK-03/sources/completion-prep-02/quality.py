"""Read actual quality logs and distinct historical source-lineage receipts."""
import re
from common import *

QUALITY={
    'workspace':('final-workspace-unit-command-01/receipt.json','f41f8d482379f979fc346091882600d03dd56bc89d01cf2a5ed84421b8905dc7'),
    'clippy':('final-clippy-command-01/receipt.json','ffc047375fc3b0950c3057db600a72b2b8c825c80f06377f35101eb18bced932'),
    'source_quality':('final-source-quality-command-03/receipt.json','9a591af959f70dbaab5356b0f8139760daec38aacf893e0d4d41e97a509abb03'),
    'structure':('final-structure-command-06/receipt.json','a97eb3d5910528e664c392b31ba628e04494ef2dc8ceba887719e07ff4f30e10'),
    'format':('final-scoped-format-command-03/receipt.json','d9f3ca3ae163c6d078105ea45fe1b2262b702124beb7c936c250818dcaf2693c'),
    'final_git_metadata':('final-quality-git-metadata-command-01/receipt.json','88bb66767cf6437822810f68fb2a82935d0873d0402f54a19eba1bbc12e5a9f3'),
}

def actual_quality():
    records={};refs={}
    for key,(name,sha) in QUALITY.items():
        read(BASE+name,sha);refs[key]=ref(BASE+name);records[key]=same_source(recorded(BASE+name))
    require(records['workspace']['command']==['cargo','test','--workspace','-j','2'],'Actual workspace command differs')
    require(records['clippy']['command']==['cargo','clippy','--workspace','--all-targets','--','-D','warnings'],'Actual Clippy command differs')
    for key,name in [('source_quality','source-quality-gate.ps1'),('structure','heat-balance-structure-audit.ps1')]:
        require(records[key]['command']==['powershell','-NoProfile','-ExecutionPolicy','Bypass','-File','scripts/quality/'+name],
            'Actual quality command differs: '+key)
    require(records['format']['command'][:4]==['rustfmt','--edition','2024','--check']
        and records['format']['command'][4:6]==['--config','skip_children=true'],'Actual scoped rustfmt check required')
    log=path_of(records['workspace']['stdout']['path']).read_text(encoding='utf8')
    groups=re.findall(r'^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;',log,re.M)
    require(groups and all(row[0]=='ok' for row in groups),'Actual workspace suite outcome differs')
    counts={'passed':sum(int(row[1]) for row in groups),'failed':sum(int(row[2]) for row in groups),
        'ignored':sum(int(row[3]) for row in groups),'reported_suite_groups':len(groups)}
    require(counts['passed']==4580 and counts['failed']==counts['ignored']==0,'Actual workspace counts differ')
    quality_log=path_of(records['source_quality']['stdout']['path']).read_text(encoding='utf8')
    require('checked_rust_files: 2533' in quality_log and 'skipped_test_files: 1172' in quality_log,'Actual source-quality counts differ')
    git=records['final_git_metadata'];require(git['command']==['git','rev-parse','HEAD','HEAD:crates']
        and git['repository_before']['head']==COMMIT and git['repository_before']['worktree_clean'] is True
        and git['repository_after']['worktree_clean'] is True,'Actual clean final metadata identity differs')
    require(path_of(git['stdout']['path']).read_text(encoding='utf8').splitlines()==[COMMIT,TREE],'Final metadata stdout differs')
    lineage_path=BASE+'quality-source-lineage-review-02/review.json'
    lineage=read(lineage_path,'81a5e84017e7afce3fb43ac6f66922795aee5d96bd33618b11bdeb16bd404b52')
    lineage_command=recorded(BASE+'quality-source-lineage-review-command-02/receipt.json')
    require(exact(read(lineage_command['stdout']['path']),{'review':ref(lineage_path),'status':lineage['status']}),
        'Actual compact lineage metadata stdout differs')
    require(lineage['schema']=='clk03-independent-quality-source-lineage-review.v1'
        and lineage['status']=='pass-exact-function-move-and-test-format-only-lineage'
        and lineage['literal_function_signatures_and_bodies_unchanged'] is True
        and lineage['existing_quality_line_limits_and_calculation_isolation_not_relaxed'] is True
        and lineage['full_final_source_snapshot_equal_to_scientific_snapshot'] is False
        and lineage['final_source_is_already_scientifically_executed_claimed'] is False,'Historical source-lineage scope differs')
    require(timestamp(lineage_command['started_utc'])<=timestamp(lineage['review_started_utc'])
        <=timestamp(lineage['review_completed_utc'])<=timestamp(lineage_command['finished_utc']),'Lineage chronology differs')
    artifacts(lineage)
    refs.update({'historical_lineage':ref(lineage_path),'historical_lineage_execution':ref(BASE+'quality-source-lineage-review-command-02/receipt.json')})
    return {'records':records,'refs':refs,'counts':counts,'lineage':lineage,
        'actual_command_heads':{key:value['repository_before']['head'] for key,value in records.items()},
        'actual_repository_states':{key:{'before':value['repository_before'],'after':value['repository_after']} for key,value in records.items()},
        'scope':'All final QA commands bound final snapshot; precommit QA HEADs remain recorded. Current final unit and production require their own executions.'}
