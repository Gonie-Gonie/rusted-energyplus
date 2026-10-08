"""Authenticate a post-execution transport correction without rewriting a plan."""
from __future__ import annotations

from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import shutil
import sys

from clk03_production_provenance import (ROOT, exact, execution_interval, read, ref, require,
    same_binding, timestamp, verify_contract_bindings)
from geo03_provenance import path_of

FIXED = {
    'prior_plan':{'path':'.runtime/porting/CLK-03/production-plan-01/plan.json','sha256':'3148fbbdcb5215a437eb03f0778039adf73deaa75b67a9f22c84161e84585a7b'},
    'prior_source_review':{'path':'.runtime/porting/CLK-03/independent-production-sources-review-01/review.json','sha256':'8cb335447542ca92a747e11219e8fcb1d4af393e656be70672beaa9fcabf6dda'},
    'retained_matrix':{'path':'.runtime/porting/CLK-03/production-matrix-01/matrix.json','sha256':'2299a4ecb25d3669c2f7ae7f5939dade0a1f974c5c845cb6f4bb2bb69ec5203c'},
    'retained_matrix_execution':{'path':'.runtime/porting/CLK-03/production-matrix-command-01/receipt.json','sha256':'03de533d1b3e67b5d1c9c59e8c3ee22317c1977cf41f93275659f485779f8481'},
    'preserved_failed_reader_execution':{'path':'.runtime/porting/CLK-03/production-comparison-command-01/receipt.json','sha256':'5fdcca184e8c8a29c96f07705abddfa4e7c8dee3d3bda1a8b5b168a0ed56b701'},
}
AUTHORITY = [
    ({'path':'.reference/energyplus-src/26.1.0/src/EnergyPlus/IOFiles.cc','sha256':'cfe62739c6327e692567d6e7fcd6abb5760548dd80d4195fe3c7eade5d33316a'},89,105),
    ({'path':'tools/porting/clk02_reference_helper.cpp','sha256':'1579511f4aee58b823b57cce6795950393331e8318a8f0b6ac527f42d5c63966'},353,369),
    ({'path':'.runtime/porting/CLK-03/packet-prep-01/freeze.py','sha256':'5b957b3e01b6cb7e7b40bba5e7c4e8cfa2697cbbd89dda40fcd7152366319816'},63,76),
]
WRITER = ROOT/'.runtime/porting/CLK-03/production-reader-amendment-prep-01/amend.py'
TRANSPORT_POLICY = {'map_hash':'SHA256 of every full mapped input byte range including its actual line ending',
    'native_hash':'SHA256 of exact UTF8 bytes of actual native delivered readLine string',
    'delivered_line':'remove at most one terminal LF delimiter, then at most one terminal CR, as genuine InputFile::readLine',
    'other_bytes':'unchanged; no rstrip, whitespace normalization, numeric parsing or rewritten input maps'}


def historical(bindings):
    values={key:read(bindings.check(value)) for key,value in FIXED.items()}
    matrix=values['retained_matrix'];record=values['retained_matrix_execution'];failed=values['preserved_failed_reader_execution']
    require(matrix['complete'] is True and matrix['actual_command_count']==6
            and same_binding(matrix['declared_plan'],FIXED['prior_plan'])
            and same_binding(matrix['independent_sources_review'],FIXED['prior_source_review']), 'Retained historical six-run matrix differs')
    for value in [record,failed]:
        require(value['schema']=='recorded-porting-command.v1' and value['launch_error'] is None
                and value['source_bytes_match_before_and_after'] is True and value['recorder_updates_gates'] is False
                and value['recorder_supplies_reference_answers'] is False,'Actual guarded historical commands required')
        execution_interval(value,bindings,exit_code=None);bindings.check(value['source_snapshot']);bindings.check(value['executed_launcher']['archive'])
    require(record['exit_code']==0 and failed['exit_code']==1
            and exact(json.loads(bindings.check(record['stdout']).read_text().splitlines()[-1]),FIXED['retained_matrix']),
            'Actual matrix completion and failed reader outcome differ')
    require(timestamp(record['finished_utc'])<=timestamp(failed['started_utc'])
            and 'ValueError: Actual native raw input identity differs' in bindings.check(failed['stderr']).read_text(),
            'Preserved reader failure is not the authenticated pre-report line-transport rejection')
    require(not (ROOT/'.runtime/porting/CLK-03/production-comparison-01/production-comparison.json').exists(),
            'Historical first reader unexpectedly has a scientific report')
    return values


def reviewed(binding,refs,sources,bindings):
    value=read(bindings.check(binding))
    require(value['schema']=='clk03-independent-production-reader-amendment-source-review.v1'
            and value['status']=='pass-source-line-transport-fix-before-preserved-data-reread'
            and value['scientific_execution_performed'] is False,'Independent post-execution amended-reader source review required')
    verify_contract_bindings(value['reviewed_contracts'],refs,bindings)
    for key in ['prior_plan','prior_source_review','retained_matrix','preserved_failed_reader_execution']:
        require(same_binding(value['reviewed_'+key],FIXED[key]),'Independent reader review historical binding differs: '+key)
    rows={row['historical_path']:row['archive'] for row in value['reviewed_sources']}
    require(len(rows)==len(value['reviewed_sources'])==len(sources)+1,'Exact amended-reader plus writer source inventory required')
    for path in [*sources,WRITER]:
        key=path.relative_to(ROOT).as_posix();require(key in rows and rows[key]['sha256']==ref(path)['sha256'],'Reviewed amended source differs: '+key)
        bindings.check(rows[key])
    return value


def authenticate_amendment(binding,execution_binding,matrix_ref,plan_ref,refs,policy,sources,bindings):
    value=read(bindings.check(binding));old=historical(bindings)
    require(value['schema']=='clk03-production-reader-transport-amendment.v1'
            and value['status']=='source-transport-fix-frozen-before-reread-of-preserved-production'
            and value['scientific_execution_performed'] is False and value['numerical_PASS_claimed'] is False
            and value['old_plan_or_proofs_rewritten'] is False and value['engine_rerun_requested'] is False
            and value['gates_updated'] is False and exact(value['source_transport_policy'],TRANSPORT_POLICY),
            'Explicit input-line transport amendment required')
    verify_contract_bindings(value['contracts'],refs,bindings)
    require(same_binding(matrix_ref,FIXED['retained_matrix']) and same_binding(plan_ref,FIXED['prior_plan'])
            and exact(value['selected_policy'],policy) and exact(policy,old['prior_plan']['selected_policy']),
            'Amendment changed frozen observation/numerical policy or retained inputs')
    for key,bound in FIXED.items():require(same_binding(value[key],bound),'Amendment historical reference differs: '+key)
    peer=reviewed(value['independent_source_review'],refs,sources,bindings)
    require(timestamp(old['preserved_failed_reader_execution']['finished_utc'])<=timestamp(peer['review_completed_utc'])
            <=timestamp(value['created_utc'])<=datetime.now(timezone.utc),'Amendment/review chronology differs')
    archives={row['historical_path']:row['archive'] for row in value['reader_source_archives']}
    require(len(archives)==len(value['reader_source_archives'])==len(sources)+1,'Full supplemental source projection required')
    for path in [*sources,WRITER]:
        key=path.relative_to(ROOT).as_posix();require(key in archives and archives[key]['sha256']==ref(path)['sha256'],'Frozen supplemental source differs')
        bindings.check(archives[key])
    for actual,(authority,start,end) in zip(value['source_transport_authorities'],AUTHORITY,strict=True):
        path=bindings.check(authority);data=b''.join(path.read_bytes().splitlines(keepends=True)[start-1:end])
        require(same_binding(actual['source'],authority) and actual['start_line']==start and actual['end_line']==end
                and actual['range_sha256']==hashlib.sha256(data).hexdigest()
                and actual['archive']['sha256']==authority['sha256'],'Exact readLine/map/native-wrapper authority differs')
        bindings.check(actual['archive'])
    record=read(bindings.check(execution_binding));start,finish=execution_interval(record,bindings)
    require(record['schema']=='recorded-porting-command.v1' and record['launch_error'] is None
            and record['source_bytes_match_before_and_after'] is True and record['recorder_updates_gates'] is False
            and record['recorder_supplies_reference_answers'] is False
            and timestamp(peer['review_completed_utc'])<=start<=timestamp(value['created_utc'])<=finish,
            'Actual zero metadata amendment execution required after independent source review')
    expected={'amendment':binding,'status':value['status'],'scientific_execution_performed':False}
    require(exact(read(bindings.check(record['stdout'])),expected)
            and len(record['command'])==9 and record['command'][1:4]==['-X','utf8','-B']
            and path_of(record['command'][4])==WRITER,'Actual amendment metadata writer stdout/owner differs')
    require(record['command'][5]=='--source-review' and same_binding(ref(record['command'][6]),value['independent_source_review'])
            and record['command'][7]=='--output-dir' and Path(record['command'][8]).resolve()==bindings.check(binding).parent,
            'Actual amendment writer argument protocol differs')
    bindings.check(record['source_snapshot']);bindings.check(record['executed_launcher']['archive'])
    require(record['repository_before']['head']==record['repository_after']['head']
            and record['repository_before']['worktree_clean'] is True and record['repository_after']['worktree_clean'] is True,
            'Actual metadata writer changed its own source/head identity')
    # This post-execution metadata receipt has its own current source identity.
    # Historical unit/CLI science remains bound to its archived compiled tree.
    return value
