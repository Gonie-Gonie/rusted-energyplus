"""Fresh custody and retained readback for one closed positive-OA profile."""
from pathlib import Path
import argparse,json,os,runpy,subprocess,sys,time,traceback
from . import common116 as c
from . import contracts as d
from . import catalogue as g

def source_projection(source,ctx):
 q=ctx.recipe;summary=c.read(source/'run-summary.json');assert summary['input']['kind']=='epjson'
 staged=source/'input/original.epJSON';converted=source/'input/converted.epJSON';expected=c.auth(q['input']).read_bytes()
 assert staged.read_bytes()==converted.read_bytes()==expected and g.typed_dom(c.read(converted))==g.typed_dom(ctx.document)
 assert summary['input']['original']==str(staged)and summary['input']['converted_epjson']==str(converted)
 projection=c.read(source/'compiled-geometry.json');assert projection['physics_executed']is False and projection['observer_supplies_inputs']is False and projection['inputs']['source_assisted_lexical_conversion']is False
 actual=projection['parsed_input'];assert actual['raw_source_is_epjson']is True and actual['scheduled_idf_declaration_overlay']is actual['nodelist_idf_declaration_overlay']is False
 for family,count in [('NodeList',5),('SetpointManager:Scheduled',9)]:assert len(actual['objects'][family])==count and g.typed_dom(actual['objects'][family])==g.typed_dom(ctx.document[family])
 assert actual['nodelist_source_order_unavailable']is None and actual['nodelist_source_order']==q['nodelist_order']
 typed=c.read(source/'model/typed-model-summary.json');graph=c.read(source/'model/graph-summary.json');assert typed['schema_version']==graph['schema_version']==1 and typed['status']=='compiled'
 for field in ['zones','zone_equipment_lists','ideal_loads_air_systems']:assert typed['counts'][field]==1
 for field in ['zone_ideal_loads_edges','ideal_loads_supply_node_edges','ideal_loads_outdoor_air_spec_edges']:assert graph[field]==1
 # The complete input inverse/lexical proof was checked by contracts.load_context;
 # this view claims only real staged input and registered NodeList/SPM exports.
 value=dict(artifact=c.ref(source/'compiled-geometry.json'),staged_original=c.ref(staged),staged_complete_input=c.ref(converted),actual_NodeList_inputs=actual['objects']['NodeList'],actual_Scheduled_inputs=actual['objects']['SetpointManager:Scheduled'],actual_NodeList_source_order=actual['nodelist_source_order'],actual_IdealLoads_objects_from_staged_input=ctx.document['ZoneHVAC:IdealLoadsAirSystem'],actual_DSOA_objects_from_staged_input=ctx.document['DesignSpecification:OutdoorAir'],actual_parent_EquipmentConnections=ctx.document['ZoneHVAC:EquipmentConnections'],actual_DesignDay_inputs=ctx.document['SizingPeriod:DesignDay'],whole_reviewed_input_inverse=True,edit_ledger=ctx.case['edit_ledger'],private_getter_enum_observed=False,typed_model_Invalid_observed=False,typed_model_summary=c.ref(source/'model/typed-model-summary.json'),graph_summary=c.ref(source/'model/graph-summary.json'),raw_source_is_epjson=True,scheduled_idf_declaration_overlay=False,nodelist_idf_declaration_overlay=False,private_OperatingMode_or_Psy_invocations_observed=False)
 ledger=d.load(ctx.case['edit_ledger'])
 if not ledger['changed_numeric_records']and not ledger['added_numeric_records']and not ledger['removed_numeric_records']:
  count=len(ledger['changed_alpha_records'])
  if count in [1,2]:value['whole_'+('one'if count==1 else'two')+'_alpha_inverse_to_144_SUM']=True
 return value

def _proof(ctx,pin,receipt_pin,native_launch=None):
 proof=d.load(pin);assert proof['schema']=='paired.NativePrecheck.v1'and proof['accepted']is True and proof['structural_precheck_accepted']is proof['Source_first_launch_authorized']is True
 assert proof['qualifying_science_accepted']is proof['qualifying_pair_credit']is proof['scientific_acceptance']is False
 assert proof['release']==ctx.release_ref and proof['bindings']==ctx.binding_ref and proof['case_config']==ctx.case_ref and proof['source_authority']==ctx.authority_ref and proof['case_id']==ctx.case['id']
 if native_launch is not None:assert proof['native_launch']==native_launch
 command=[sys.executable,'-X','utf8','-B',str(Path(__file__).with_name('__main__.py')),'precheck','--bindings',ctx.binding_ref['path']+'#'+ctx.binding_ref['sha256'],'--case',ctx.case['id'],'--mode','fresh','--native-launch',proof['native_launch']['path']+'#'+proof['native_launch']['sha256'],'--output',str(d.auth(pin))]
 reader_receipt(ctx,receipt_pin,command,pin,proof,[proof['native_launch']])
 return proof

def actor_plan(ctx,engine,home,native_proof,native_precheck_receipt):
 assert ctx.mode=='fresh'and engine in ['native','source'];environment,weather=d.clean_environment();build=d.load(ctx.authority['build'])
 assert not home.exists();output=home/'output';proof=None
 if engine=='source':
  assert native_proof is not None and native_precheck_receipt is not None;proof=_proof(ctx,native_proof,native_precheck_receipt);check_launch(ctx,proof['native_launch'],'native')
 else:assert native_proof is native_precheck_receipt is None
 pins=list(ctx.pins)+([native_proof,native_precheck_receipt]if native_proof else [])
 return dict(schema='paired.EnginePlan.v1',case_id=ctx.case['id'],release=ctx.release_ref,bindings=ctx.binding_ref,case_config=ctx.case_ref,source_authority=ctx.authority_ref,engine=engine,build=ctx.authority['build'],output=str(output),command=d.command(engine,build,ctx.native_bundle,d.auth(ctx.case['input']),output),environment=environment,default_weather=weather,explicit_pins=pins,native_proof=native_proof,native_precheck_receipt=native_precheck_receipt,scientific_acceptance=False,automatic_retry=False)

def record(ctx,plan_pin,command_dir):
 """Fixed trusted recorder adapter avoids Windows' command-line length limit."""
 assert ctx.mode=='fresh';plan=d.load(plan_pin);home=command_dir.parent
 assert plan==actor_plan_check(ctx,plan,home)and not command_dir.exists()
 pins=plan['explicit_pins']+[plan_pin]
 argv=[str(d.auth(ctx.release['recorder'])),'--output-dir',str(command_dir)]
 for pin in pins:argv+=['--input',str(d.auth(pin))]
 argv+=['--',*plan['command']];before=sys.argv[:]
 try:
  sys.argv=argv
  try:runpy.run_path(str(d.auth(ctx.release['recorder'])),run_name='__main__')
  except SystemExit as error:return error.code if type(error.code)is int else 1
 finally:sys.argv=before
 raise RuntimeError('Trusted recorder returned without its CLI exit; no-op is not execution')

def actor_plan_check(ctx,plan,home):
 assert plan['schema']=='paired.EnginePlan.v1'and plan['release']==ctx.release_ref and plan['bindings']==ctx.binding_ref and plan['case_config']==ctx.case_ref and plan['source_authority']==ctx.authority_ref and plan['case_id']==ctx.case['id']
 assert plan['scientific_acceptance']is False and plan['automatic_retry']is False and plan['build']==ctx.authority['build']
 assert Path(plan['output'])==home/'output'and home.is_relative_to(d.ROOT/'.runtime/porting')
 build=d.load(ctx.authority['build']);assert plan['command']==d.command(plan['engine'],build,ctx.native_bundle,d.auth(ctx.case['input']),home/'output')
 expected=list(ctx.pins)
 if plan['engine']=='source':
  proof=_proof(ctx,plan['native_proof'],plan['native_precheck_receipt']);check_launch(ctx,proof['native_launch'],'native');expected.extend([plan['native_proof'],plan['native_precheck_receipt']])
 else:assert plan['engine']=='native'and plan['native_proof']is plan['native_precheck_receipt']is None
 assert plan['explicit_pins']==expected
 environment,weather=d.clean_environment();assert environment==plan['environment']and weather==plan['default_weather']
 return plan

def act(ctx,args):
 home=d.inside(args.run_dir,d.ROOT/'.runtime/porting');proof=d.parse_ref(args.native_proof)if args.native_proof else None;precheck=d.parse_ref(args.native_precheck_receipt)if args.native_precheck_receipt else None;plan=actor_plan(ctx,args.engine,home,proof,precheck)
 home.mkdir(parents=True);pin=d.output(home/'plan.json',plan)
 argv=record_argv(ctx,pin,home)
 started=time.time();error=None;rc=None
 with(home/'recorder.stdout.log').open('xb')as out,(home/'recorder.stderr.log').open('xb')as err:
  try:rc=subprocess.run(argv,cwd=d.ROOT,stdin=subprocess.DEVNULL,stdout=out,stderr=err).returncode
  except OSError as exc:error=repr(exc)
 try:
  for p in plan['explicit_pins']+[pin]:d.auth(p)
  d.snapshot(ctx.authority['snapshot'],True);environment,weather=d.clean_environment();assert environment==plan['environment']and weather==plan['default_weather']
  assert rc==0;validate_receipt(ctx,plan,pin,d.ref(home/'command/receipt.json'))
 except Exception:error=traceback.format_exc();environment=c.selected_environment(dict(os.environ));weather=c.default_weather_observation()
 output=home/'output';receipt=home/'command/receipt.json'
 launch=dict(schema='paired.EngineLaunch.v1',case_id=ctx.case['id'],release=ctx.release_ref,engine=args.engine,plan=pin,command=plan['command'],argv=argv,cwd=str(d.ROOT),started=started,finished=time.time(),recorder_exit_code=rc,receipt=d.ref(receipt)if receipt.exists()else None,environment_before=plan['environment'],environment_after=environment,weather_before=plan['default_weather'],weather_after=weather,custody_error=error,artifacts=[d.ref(p)for p in sorted(output.rglob('*'))if p.is_file()]if output.exists()else [],scientific_acceptance=False,automatic_retry=False)
 result=d.output(home/'launch.json',launch);print(json.dumps(dict(launch=result,recorder_exit_code=rc,custody_error=error)));return 0 if rc==0 and error is None else 1

def record_argv(ctx,plan_pin,home):
 return [sys.executable,'-X','utf8','-B',str(Path(__file__).with_name('__main__.py')),'record','--bindings',ctx.binding_ref['path']+'#'+ctx.binding_ref['sha256'],'--case',ctx.case['id'],'--mode','fresh','--plan',plan_pin['path']+'#'+plan_pin['sha256'],'--output-dir',str(home/'command')]

def validate_receipt(ctx,plan,plan_pin,receipt_pin):
 receipt=d.load(receipt_pin);assert receipt['exit_code']==0 and receipt['launch_error']is None and receipt['command']==plan['command']and receipt['source_snapshot']==ctx.authority['snapshot']and receipt['source_bytes_match_before_and_after']and receipt['repository_before']==receipt['repository_after']and Path(receipt['cwd']).resolve()==d.ROOT
 custody=receipt['explicit_input_custody'];expected=plan['explicit_pins']+[plan_pin]
 assert custody['command_process_started']and custody['all_explicit_inputs_unchanged']and not custody['read_errors_before']and not custody['read_errors_after']and custody['before']==custody['after']==expected
 request=d.load(custody['request']);assert request['command']==plan['command']and request['inputs_before']==expected and not request['input_read_errors_before']
 return receipt

def check_launch(ctx,pin,engine):
 assert ctx.mode=='fresh';launch=d.load(pin);assert launch['schema']=='paired.EngineLaunch.v1'and launch['engine']==engine and launch['case_id']==ctx.case['id']and launch['release']==ctx.release_ref
 assert launch['scientific_acceptance']is launch['automatic_retry']is False and launch['custody_error']is None and launch['recorder_exit_code']==0
 plan=d.load(launch['plan']);home=d.auth(pin).parent;actor_plan_check(ctx,plan,home);assert plan['engine']==engine and launch['command']==plan['command']
 assert launch['argv']==record_argv(ctx,launch['plan'],home)and launch['receipt']==d.ref(home/'command/receipt.json')
 receipt=validate_receipt(ctx,plan,launch['plan'],launch['receipt'])
 assert Path(receipt['cwd']).resolve()==d.ROOT==Path(launch['cwd']).resolve();custody=receipt['explicit_input_custody'];expected=plan['explicit_pins']+[launch['plan']]
 assert custody['command_process_started']and custody['all_explicit_inputs_unchanged']and not custody['read_errors_before']and not custody['read_errors_after']and custody['before']==custody['after']==expected
 request=d.load(custody['request']);assert request['command']==plan['command']and request['inputs_before']==expected and not request['input_read_errors_before']
 assert launch['environment_before']==launch['environment_after']==plan['environment']and launch['weather_before']==launch['weather_after']==plan['default_weather']and launch['started']<=launch['finished']
 output=Path(plan['output']);assert launch['artifacts']==[d.ref(p)for p in sorted(output.rglob('*'))if p.is_file()]
 return launch,plan,receipt,output,d.files(output,engine,ctx.recipe),d.load(ctx.authority['build'])

def retained_original_pin(ctx,pin,members):
 """Authenticate a past Source input against that exact build, keep its old path."""
 assert ctx.mode=='retained'
 if pin['path']not in members:return d.auth(pin)
 row=members[pin['path']]
 assert pin['sha256']==row['archive']['sha256']and pin['bytes']==row['size_bytes'],'Historical Source pin differs from authority snapshot'
 archive=d.auth(row['archive']);assert archive.stat().st_size==row['size_bytes']
 original=(d.ROOT/pin['path']).resolve();assert original.is_relative_to(d.ROOT)and original!=d.ROOT
 return original

def retained_launch(ctx,pin,engine):
 """Explicit original159/158 custody adapter, never a current Source waiver."""
 assert ctx.mode=='retained';r=d.load(pin);assert r['schema']=='paired.RetainedRun.v1'and r['case_config']==ctx.case_ref and r['source_authority']==ctx.authority_ref and r['engine']==engine
 assert r['unit']in [158,159]and r['release_profile']==d.PROFILE and r['full_migration']is True
 launch=d.load(r['launch']);plan=d.load(launch['plan']);receipt=d.load(launch['receipt']);q=ctx.recipe
 assert launch['schema']==f"unit{r['unit']}-observed-engine-launch.v1"and launch['engine']==engine and launch['scientific_acceptance']is launch['automatic_retry']is False and launch['recorder_exit_code']==0 and launch['custody_error']is None
 original=d.load(r['recipe']);binding=d.load(r['bindings']);assert plan['recipe']==r['recipe']and plan['bindings']==r['bindings']and plan['case_kind']==r['original_case_kind']and plan['batch_plan']==r['batch_plan']and launch['batch_plan']==r['batch_plan']
 assert binding['ready_for_actors']is True and binding['actual_build']==ctx.authority['build']and plan['build_row']==ctx.authority['build']and launch['case_kind']==r['original_case_kind']
 build=d.load(ctx.authority['build']);output=d.inside(plan['output'],d.ROOT/'.runtime/porting');assert plan['command']==launch['command']==d.command(engine,build,ctx.native_bundle,d.auth(q['input']),output)
 assert receipt['exit_code']==0 and receipt['launch_error']is None and receipt['command']==plan['command']and receipt['source_snapshot']==ctx.authority['snapshot']and receipt['source_bytes_match_before_and_after']and receipt['repository_before']==receipt['repository_after']
 custody=receipt['explicit_input_custody'];pins=plan['explicit_pins']+[launch['plan']]
 members={row['historical_path']:row for row in d.load(ctx.authority['snapshot'])['files']}
 for p in pins:retained_original_pin(ctx,p,members)
 assert custody['command_process_started']and custody['all_explicit_inputs_unchanged']and not custody['read_errors_before']and not custody['read_errors_after']and custody['before']==custody['after']==pins
 assert plan['reader']in pins and plan['common']in pins and plan['protocol']in pins and r['recipe']in pins and r['bindings']in pins
 request=d.load(custody['request']);assert request['command']==plan['command']and request['inputs_before']==pins and not request['input_read_errors_before']
 assert Path(launch['cwd']).resolve()==Path(receipt['cwd']).resolve()==d.ROOT
 recorder=d.ROOT/'tools/porting/record_command.py';assert ctx.release['recorder']in plan['explicit_pins']
 argv=[launch['argv'][0],'-X','utf8','-B',str(recorder),'--output-dir',str(d.auth(r['launch']).parent/'command')]
 for pin in pins:argv+=['--input',str(retained_original_pin(ctx,pin,members))]
 argv+=['--',*plan['command']];assert launch['argv']==argv
 assert launch['environment_before']==launch['environment_after']==plan['environment']and all(v is None for v in plan['environment'].values())and len(plan['environment'])==39
 assert launch['weather_before']==launch['weather_after']==plan['default_weather']and not plan['default_weather']['exists']and not plan['default_weather']['lexical_entry_exists']
 assert launch['started']<=launch['finished']and launch['artifacts']==[d.ref(p)for p in sorted(output.rglob('*'))if p.is_file()]
 if engine=='source':
  proof=d.load(plan['native_proof']);assert proof['schema']==f"unit{r['unit']}-native-structural-precheck.v1"and proof['accepted']is proof['Source_first_launch_authorized']is True and proof['qualifying_pair_credit']is proof['qualifying_science_accepted']is proof['scientific_acceptance']is False
  assert proof['native_launch']==r['original_native_launch']and proof['bindings']==r['bindings']and proof['recipe']==r['recipe']and proof['verifier']==plan['reader']
 assert original['input']==q['input']and original['scientific_dom']==q['scientific_dom']and original['derived_policy']==q['derived_policy']
 for key in ctx.release['retained_scientific_fields']:assert original[key]==q[key]
 return launch,plan,receipt,output,d.files(output,engine,q),build

def compare(ctx,args,result):
 c.FAILURES.clear();c.AUTHENTICATED.clear();q=ctx.recipe;fn=check_launch if ctx.mode=='fresh'else retained_launch
 nr=fn(ctx,d.parse_ref(args.native_launch),'native');nl,np,nreceipt,native,nf,build=nr;n,no=g.native_outputs(nf,q,ctx.policy,ctx.environments)
 result.update(case_id=ctx.case['id'],case_kind=ctx.case['case_kind'],release=ctx.release_ref,bindings=ctx.binding_ref,case_config=ctx.case_ref,source_authority=ctx.authority_ref,native_launch=d.parse_ref(args.native_launch),Native=no)
 if args.command=='precheck':
  assert ctx.mode=='fresh'and not args.source_launch;result.update(accepted=True,structural_precheck_accepted=True,Source_first_launch_authorized=True,qualifying_science_accepted=False,qualifying_pair_credit=False,scientific_acceptance=False);return
 assert args.source_launch;sl,sp,sreceipt,source,sf,sbuild=fn(ctx,d.parse_ref(args.source_launch),'source');assert sbuild==build and nl['finished']<=sl['started']and nreceipt['source_snapshot']==sreceipt['source_snapshot']and nl['environment_before']==sl['environment_before']
 if ctx.mode=='fresh':_proof(ctx,sp['native_proof'],sp['native_precheck_receipt'],d.parse_ref(args.native_launch))
 else:assert d.load(d.parse_ref(args.source_launch))['original_native_launch']==d.load(d.parse_ref(args.native_launch))['launch']
 s,so=g.native_outputs(sf,q,ctx.policy,ctx.environments);metrics=[];result['Source']=so
 for ext in ['eso','mtr']:
  ao,ap,ab=n[ext];bo,bp,bb=s[ext];assert ab==bb and ao['dictionary']==bo['dictionary']and ao['shape']==bo['shape']and ao['footer']==bo['footer'];metrics+=c.compare_values(ap,bp,ab,ext)
 counts=ctx.case['counts'];assert len(metrics)==counts['total_series']and sum(x['samples']for x in metrics)==counts['total_values']
 result.update(series=metrics,total_series=len(metrics),total_values=sum(x['samples']for x in metrics),all_uncapped_failures=c.FAILURES,numeric_parity_accepted=not c.FAILURES,engineering_comparison_completed=True)
 summary=c.read(source/'run-summary.json');exports={}
 for ext in ['eso','mtr']:
  old,data,bounds=s[ext]
  def exported():
   joined=c.inspect_json_export(summary,source,data,bounds,ext);csv=g.inspect_csv_export131(source,data,bounds,ext);assert joined==data['count']==csv['operand_bits_line_owner_clock_joins'];return dict(json_joins=joined,csv=csv)
  exports[ext]=g.record_gate(ext+'_full_exports',exported)
 ng=g.science_gates158(nf,n,q);sg=g.science_gates158(sf,s,q);assert len(ng)==len(sg)==12
 pair=g.paired_science(n,s,no,so,ng,sg,nf,sf);pair['actual_Source_epJSON_input_projection']=g.record_gate('actual_Source_epJSON_input_projection',source_projection,source,ctx);pair['Source_scope_and_run_summary']=g.record_gate('Source_scope_and_run_summary',g.source_scope_checks,source,ctx.environments);assert len(pair)==8
 failures=[dict(engine=engine,gate=name,error=value.get('error'),traceback=value.get('traceback'))for engine,gates in [('Native',ng),('Source',sg),('paired',pair),('Source_exports',exports)]for name,value in gates.items()if value['passed']is not True]
 accepted=not c.FAILURES and not failures;result.update(source_launch=d.parse_ref(args.source_launch),Native_science_gates=ng,Source_science_gates=sg,paired_science_gates=pair,Source_exports=exports,science_gate_failures=failures,accepted=accepted,scientific_acceptance=accepted,qualifying_science_accepted=accepted,qualifying_pair_credit=accepted if ctx.mode=='fresh'else False,new_physical_credit=0 if ctx.mode=='retained'else int(accepted),engine_calls=0)

def verify_command(ctx,native_launch,source_launch,output):
 return [sys.executable,'-X','utf8','-B',str(Path(__file__).with_name('__main__.py')),'verify','--bindings',ctx.binding_ref['path']+'#'+ctx.binding_ref['sha256'],'--case',ctx.case['id'],'--mode',ctx.mode,'--native-launch',native_launch['path']+'#'+native_launch['sha256'],'--source-launch',source_launch['path']+'#'+source_launch['sha256'],'--output',str(output)]

def reader_receipt(ctx,receipt_pin,command,output_pin,result,actor_pins):
 receipt=d.load(receipt_pin)
 assert receipt['schema']=='recorded-porting-command.v1'and receipt['command']==command and Path(receipt['cwd']).resolve()==d.ROOT and receipt['launch_error']is None and receipt['exit_code']==(0 if result['accepted']is True else 1)
 assert receipt['source_bytes_match_before_and_after']is True and receipt['repository_before']==receipt['repository_after']
 if ctx.mode=='fresh':assert receipt['source_snapshot']==ctx.authority['snapshot']
 else:
  # Verification ran now; the two model actors remain on their historical Source.
  # Authenticate the actual current verifier snapshot independently of that epoch.
  rows=d.snapshot(receipt['source_snapshot'],True);paths=set((d.ROOT/'crates').rglob('*.rs'))|set((d.ROOT/'crates').rglob('Cargo.toml'))
  paths|={d.ROOT/name for name in ['Cargo.toml','Cargo.lock','rust-toolchain.toml','.cargo/config.toml']if(d.ROOT/name).is_file()};assert set(rows)=={p.relative_to(d.ROOT).as_posix()for p in paths}
 launcher=receipt['executed_launcher'];assert launcher['historical_path']==ctx.release['recorder']['path']and launcher['archive']['sha256']==ctx.release['recorder']['sha256'];d.auth(launcher['archive'])
 pins=list(ctx.pins)+actor_pins;custody=receipt['explicit_input_custody']
 assert custody['command_process_started']is True and custody['all_explicit_inputs_unchanged']is True and not custody['read_errors_before']and not custody['read_errors_after']and custody['before']==custody['after']==pins
 for pin in pins:d.auth(pin)
 request=d.load(custody['request']);assert request['command']==command and request['inputs_before']==pins and not request['input_read_errors_before']
 lines=d.auth(receipt['stdout']).read_text('utf8').splitlines();assert len(lines)==1;reported=json.loads(lines[0]);assert reported==dict(output=output_pin,accepted=result['accepted'],error=result.get('error'));d.auth(receipt['stderr'])
 return receipt_pin

def completion_provenance(ctx,args,pair):
 paired=d.parse_ref(args.paired);receipt_pin=d.parse_ref(args.verification_receipt)
 # Check the actual original actors as well as the recorded read-only verifier.
 fn=check_launch if ctx.mode=='fresh'else retained_launch
 nl,np,nr,native,nf,nb=fn(ctx,pair['native_launch'],'native');sl,sp,sr,source,sf,sb=fn(ctx,pair['source_launch'],'source')
 assert nb==sb and nl['finished']<=sl['started']and nr['source_snapshot']==sr['source_snapshot']and nl['environment_before']==sl['environment_before']
 if ctx.mode=='fresh':_proof(ctx,sp['native_proof'],sp['native_precheck_receipt'],pair['native_launch'])
 else:assert d.load(pair['source_launch'])['original_native_launch']==d.load(pair['native_launch'])['launch']
 return reader_receipt(ctx,receipt_pin,verify_command(ctx,pair['native_launch'],pair['source_launch'],d.auth(paired)),paired,pair,[pair['native_launch'],pair['source_launch']])

def complete(ctx,args):
 pair=d.load(d.parse_ref(args.paired));assert pair['schema']=='paired.Readback.v1'and pair['case_config']==ctx.case_ref and pair['source_authority']==ctx.authority_ref and pair['release']==ctx.release_ref and pair['engineering_comparison_completed']is True
 verification_receipt=completion_provenance(ctx,args,pair)
 assert pair['bindings']==ctx.binding_ref and pair['mode']==ctx.mode and pair['total_series']==ctx.case['counts']['total_series']and pair['total_values']==ctx.case['counts']['total_values']
 assert len(pair['series'])==pair['total_series']and sum(x['samples']for x in pair['series'])==pair['total_values']
 for key,category in [('Native_science_gates','engine'),('Source_science_gates','engine'),('paired_science_gates','paired'),('Source_exports','exports')]:
  assert list(pair[key])==ctx.release['gate_ids'][category]
  for name,value in pair[key].items():
   assert {'name','passed','observation','error'}<=value.keys()and set(value)<={'name','passed','observation','error','traceback'}and type(value['passed'])is bool
   assert value['name']==(name+'_full_exports'if category=='exports'else name)
   assert value['error']is None if value['passed']else type(value['error'])is str
 assert pair['numeric_parity_accepted']is (not pair['all_uncapped_failures'])
 assert all(type(x['tolerance_failures'])is int and x['tolerance_failures']>=0 and type(x['rmse_limit_met'])is bool for x in pair['series'])
 assert sum(x['tolerance_failures']for x in pair['series'])==sum(x['kind']=='point'for x in pair['all_uncapped_failures'])and sum(not x['rmse_limit_met']for x in pair['series'])==sum(x['kind']=='rmse'for x in pair['all_uncapped_failures'])
 accepted=not pair['all_uncapped_failures']and not pair['science_gate_failures']and all(v['passed']is True for key in ['Native_science_gates','Source_science_gates','paired_science_gates','Source_exports']for v in pair[key].values())
 retained=ctx.mode=='retained';assert pair['accepted']is pair['qualifying_science_accepted']is pair['scientific_acceptance']is accepted and pair['qualifying_pair_credit']is (False if retained else accepted)
 result=dict(schema='paired.Completed.v1',case_id=ctx.case['id'],mode=ctx.mode,release=ctx.release_ref,bindings=ctx.binding_ref,case_config=ctx.case_ref,source_authority=ctx.authority_ref,paired=d.parse_ref(args.paired),verification_receipt=verification_receipt,native_launch=pair['native_launch'],source_launch=pair['source_launch'],accepted_for_retention=True,numeric_parity_accepted=pair['numeric_parity_accepted'],engineering_comparison_completed=True,scientific_acceptance=accepted,qualifying_cases=int(accepted)if not retained else 0,qualifying_values=pair['total_values']if accepted and not retained else 0,engines_executed_by_completion=0,actual_Native_engine_calls=0 if retained else 1,actual_Source_engine_calls=0 if retained else 1,source_launch_authorized=False,no_engine_reruns=retained,automatic_retry=False,new_physical_credit=0 if retained else int(accepted),original_actor_identities_retained=retained)
 return d.output(d.inside(args.output),result)

def main(argv=None):
 if not __debug__:raise RuntimeError('Optimized Python disables required assertions')
 parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('command',choices=['validate','authority','act','record','precheck','verify','complete']);parser.add_argument('--bindings');parser.add_argument('--case');parser.add_argument('--release');parser.add_argument('--evidence');parser.add_argument('--mode',choices=['fresh','retained'],required=True)
 for name in ['engine','run-dir','native-proof','native-precheck-receipt','plan','output-dir','native-launch','source-launch','paired','verification-receipt','output']:parser.add_argument('--'+name)
 args=parser.parse_args(argv);assert __debug__ and Path.cwd().resolve()==d.ROOT
 if args.command=='authority':
  assert args.evidence and args.release and args.output and not args.bindings and not args.case
  print(json.dumps(d.output(d.inside(args.output),d.authority_from_evidence(d.parse_ref(args.evidence),args.mode))));return 0
 assert args.bindings and args.case and not args.release and not args.evidence;ctx=d.load_context(d.parse_ref(args.bindings),args.case,args.mode)
 if args.command=='validate':print(json.dumps(dict(validated=True,case_config=ctx.case_ref,source_authority=ctx.authority_ref,profile=d.PROFILE,engine_calls=0)));return 0
 if args.command=='act':return act(ctx,args)
 if args.command=='record':return record(ctx,d.parse_ref(args.plan),d.inside(args.output_dir,d.ROOT/'.runtime/porting'))
 if args.command=='complete':print(json.dumps(complete(ctx,args)));return 0
 result=dict(schema='paired.NativePrecheck.v1'if args.command=='precheck'else'paired.Readback.v1',accepted=False,mode=ctx.mode,engine_calls=0,policy_changes=0,new_physical_credit=0)
 try:compare(ctx,args,result)
 except Exception as error:result.update(accepted=False,error=repr(error),traceback=traceback.format_exc(),all_uncapped_failures=c.FAILURES)
 print(json.dumps(dict(output=d.output(d.inside(args.output),result),accepted=result['accepted'],error=result.get('error'))));return 0 if result['accepted']else 1
