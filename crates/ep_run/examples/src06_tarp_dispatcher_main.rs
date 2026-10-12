//! Input-only selected dispatcher harness; no Native result is an operand.
#![recursion_limit = "256"]
#[path = "../../ep_runtime/examples/clk02_probe_support/digest.rs"]
mod digest;
#[path = "src06_tarp_dispatcher_support/fields.rs"]
mod fields;

use ep_compiler::compile_raw_model;
use ep_model::{SpaceId, SurfaceId, SurfaceType, ZoneId};
use ep_raw_model::parse_epjson_str_with_idf_order;
use ep_runtime::heat_balance::inside_convection_dispatcher::*;
use ep_runtime::heat_balance::inside_convection_layout::{
    InsideConvectionAssociation, InsideConvectionGroup, InsideConvectionLayout,
};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn require(value: bool, message: &str) -> Result<()> {
    if value {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}
fn keys(value: &Value, expected: &[&str]) -> Result<()> {
    require(
        value.as_object().is_some_and(|o| {
            o.len() == expected.len() && expected.iter().all(|k| o.contains_key(*k))
        }),
        "complete exact input object required",
    )
}
fn text(value: &Value) -> Result<&str> {
    value.as_str().ok_or_else(|| "string input required".into())
}
fn rows(value: &Value) -> Result<&[Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "array input required".into())
}
fn boolean(value: &Value) -> Result<bool> {
    value
        .as_bool()
        .ok_or_else(|| "Boolean input required".into())
}
fn integer(value: &Value) -> Result<i32> {
    Ok(i32::try_from(
        value.as_i64().ok_or("signed integer input required")?,
    )?)
}
fn id(value: &Value) -> Result<u32> {
    Ok(u32::try_from(
        value.as_u64().ok_or("nonnegative owner index required")?,
    )?)
}
fn real(value: &Value) -> Result<f64> {
    keys(value, &["bits"])?;
    let bits = text(&value["bits"])?;
    require(
        bits.len() == 16
            && bits
                .bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)),
        "sixteen lowercase binary64 hex digits required",
    )?;
    let result = f64::from_bits(u64::from_str_radix(bits, 16)?);
    require(result.is_finite(), "finite literal input required")?;
    Ok(result)
}
fn identifier(value: &Value) -> Result<&str> {
    let value = text(value)?;
    require(
        !value.is_empty()
            && value.len() <= 80
            && value
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_'),
        "safe literal case/operation ID required",
    )?;
    Ok(value)
}
fn checked(root: &Path, binding: &Value) -> Result<Vec<u8>> {
    let path = root.join(text(&binding["path"])?).canonicalize()?;
    require(
        path.starts_with(root) && path.is_file(),
        "contained regular bound input required",
    )?;
    let bytes = std::fs::read(&path)?;
    require(
        bytes.len() <= 16 * 1024 * 1024 && digest::sha256(&bytes) == text(&binding["sha256"])?,
        "bound input SHA differs",
    )?;
    if let Some(size) = binding.get("size_bytes") {
        require(
            u64::try_from(bytes.len())? == size.as_u64().ok_or("integer input size required")?,
            "bound input size differs",
        )?;
    }
    Ok(bytes)
}
fn read(root: &Path, binding: &Value) -> Result<Value> {
    Ok(serde_json::from_slice(&checked(root, binding)?)?)
}
fn file(path: &Path) -> Result<Value> {
    let bytes = std::fs::read(path)?;
    Ok(
        json!({"path":path.to_string_lossy(),"sha256":digest::sha256(&bytes),"size_bytes":bytes.len()}),
    )
}
fn source_bindings(root: &Path, binding: &Value) -> Result<Value> {
    let value = read(root, binding)?;
    keys(
        &value,
        &["schema", "observer_sources", "actual_public_API_sources"],
    )?;
    require(
        value["schema"] == "src06-tarp-dispatcher-runtime-source-bindings.v1",
        "actual runtime Source binding schema required",
    )?;
    for (key, expected) in [
        ("observer_sources", fields::OBSERVER_PATHS.as_slice()),
        ("actual_public_API_sources", fields::API_PATHS.as_slice()),
    ] {
        let bindings = rows(&value[key])?;
        require(
            bindings.len() == expected.len(),
            "complete ordered Source closure required",
        )?;
        for (row, name) in bindings.iter().zip(expected) {
            require(text(&row["path"])? == *name, "Source path order differs")?;
            checked(root, row)?;
        }
    }
    Ok(value)
}
fn conversion(root: &Path, input: &Value, native: &Value) -> Result<(String, String, Value)> {
    let ledger = read(root, &input["conversion_ledger"])?;
    let execution = read(root, &input["conversion_execution"])?;
    let receipt = read(root, &input["conversion_command_receipt"])?;
    require(
        ledger["schema"] == "sch01-converted-inputs.v1"
            && ledger["scientific_execution_performed"] == false
            && ledger["Native_scientific_outputs_used_as_inputs"] == false
            && ledger["expected_values_supplied"] == false
            && ledger["case_count"] == 1
            && ledger["available_converted_input_count"] == 1
            && ledger["unavailable_converted_input_count"] == 0,
        "genuine complete input-only conversion required",
    )?;
    require(
        execution["schema"] == "sch01-input-conversion-command.v1"
            && execution["case_id"] == "SRC06-DISPATCHER-NO-USER"
            && execution["exit_code"] == 0
            && execution["launch_error"].is_null()
            && execution["epjson_available"] == true
            && execution["original_and_staged_IDF_preserved"] == true
            && execution["numerical_engine_executed"] == false
            && execution["Native_scientific_outputs_used_as_inputs"] == false
            && execution["expected_answers_supplied"] == false,
        "successful actual converter execution required",
    )?;
    require(
        receipt["schema"] == "recorded-porting-command.v1"
            && receipt["exit_code"] == 0
            && receipt["launch_error"].is_null()
            && receipt["source_bytes_match_before_and_after"] == true,
        "closed unchanged-Source conversion receipt required",
    )?;
    require(
        receipt["executed_launcher"]["archive"]["sha256"]
            == "5745ad9809e196af00b858fcdc0d3d783531628fa2abf1e086f75c7d293514ba",
        "actual approved recorder Source required",
    )?;
    checked(root, &receipt["executed_launcher"]["archive"])?;
    checked(root, &receipt["source_snapshot"])?;
    let recorded_ledger: Value = serde_json::from_slice(&checked(root, &receipt["stdout"])?)?;
    require(
        recorded_ledger == ledger,
        "converter recorded stdout/ledger differs",
    )?;
    checked(root, &receipt["stderr"])?;
    let command = rows(&receipt["command"])?;
    require(
        command.len() == 13
            && command[..5]
                == json!([
                    "python",
                    "-X",
                    "utf8",
                    "-B",
                    ".runtime/porting/SCH-01/input-prep-02/convert_inputs.py"
                ])
                .as_array()
                .ok_or("literal command required")?[..]
            && command[5] == "--approved-source-sha256"
            && command[6] == "82f27c8327d2409bb3bcba727044ebe6a36641ed8d7c404e08dd4bc92c990dbb"
            && command[7] == "--input-cases"
            && command[8]
                == format!(
                    "{}#{}",
                    text(&ledger["input_cases"]["path"])?,
                    text(&ledger["input_cases"]["sha256"])?
                )
            && command[9] == "--converter"
            && command[10]
                == format!(
                    "{}#{}",
                    text(&ledger["converter"]["path"])?,
                    text(&ledger["converter"]["sha256"])?
                )
            && command[11] == "--output-dir"
            && command[12] == ".runtime/porting/SCH-01/src06-dispatcher-converted-inputs-01",
        "actual existing converter argv differs",
    )?;
    let source_hashes = [
        "82f27c8327d2409bb3bcba727044ebe6a36641ed8d7c404e08dd4bc92c990dbb",
        "b38466a0dd8851935d0ef28c079220d7c48f2c05506926d1ad4e7fb463f6de77",
    ];
    let runner_sources = rows(&ledger["executed_runner_sources"])?;
    require(
        runner_sources.len() == source_hashes.len(),
        "complete converter Sources required",
    )?;
    for (source, hash) in runner_sources.iter().zip(source_hashes) {
        require(
            text(&source["archive"]["sha256"])? == hash,
            "converter Source differs",
        )?;
        checked(root, &source["archive"])?;
    }
    let times = [
        &receipt["started_utc"],
        &ledger["started_utc"],
        &execution["started_utc"],
        &execution["finished_utc"],
        &ledger["finished_utc"],
        &receipt["finished_utc"],
    ]
    .into_iter()
    .map(text)
    .collect::<Result<Vec<_>>>()?;
    require(
        times.iter().all(|v| v.ends_with("+00:00")) && times.windows(2).all(|v| v[0] <= v[1]),
        "actual converter receipt/ledger/child chronology differs",
    )?;
    let entry = &rows(&ledger["conversions"])?[0];
    require(
        entry["case_id"] == execution["case_id"]
            && entry["available"] == true
            && checked(root, &entry["actual_conversion_execution"])?
                == checked(root, &input["conversion_execution"])?
            && checked(root, &entry["input"])? == checked(root, &execution["source_input"])?
            && checked(root, &entry["epjson_input"])?
                == checked(root, &execution["available_epjson_input"])?
            && ledger["converter"] == execution["converter"],
        "conversion ledger/command/source/output join differs",
    )?;
    require(
        ledger["converter"]["path"] == ".runtime/energyplus/26.1.0/ConvertInputFormat.exe"
            && ledger["converter"]["sha256"]
                == "04f1327fef7148118ea1d79d9e990fb2447b9731e9c1b28a9a8276b840d7cbfe",
        "actual existing production converter required",
    )?;
    let declared = read(root, &ledger["input_cases"])?;
    require(
        declared["schema"] == "sch01-input-cases.v1"
            && declared["scientific_execution_performed"] == false
            && declared["expected_values_supplied"] == false
            && declared["expected_exits_supplied"] == false
            && declared["final_scientific_packet_frozen"] == false
            && rows(&declared["model_cases"])?.len() == 1
            && declared["model_cases"][0]["id"] == execution["case_id"]
            && checked(root, &declared["model_cases"][0]["input"])?
                == checked(root, &execution["source_input"])?,
        "actual input-only declared case differs",
    )?;
    let child_command = rows(&execution["command"])?;
    require(
        child_command.len() == 2
            && Path::new(text(&child_command[0])?).canonicalize()?
                == root
                    .join(text(&ledger["converter"]["path"])?)
                    .canonicalize()?
            && child_command[1] == "input.idf"
            && root
                .join(text(&execution["cwd"])?)
                .join("input.idf")
                .canonicalize()?
                == root
                    .join(text(&execution["staged_input"]["path"])?)
                    .canonicalize()?,
        "actual converter child argv/cwd differs",
    )?;
    for binding in rows(&execution["actual_output_inventory"])? {
        checked(root, binding)?;
    }
    let idf = checked(root, &execution["source_input"])?;
    require(
        idf == checked(root, &execution["staged_input"])?
            && idf == text(&native["literal_IDF"])?.as_bytes()
            && digest::sha256(&idf)
                == "f3a89522e94a03bff136879aa64545aaad434437142023dac886bf51d723003a"
            && idf.len() == 158,
        "same exact frozen literal IDF required",
    )?;
    let epjson = checked(root, &execution["available_epjson_input"])?;
    checked(root, &execution["stdout"])?;
    checked(root, &execution["stderr"])?;
    Ok((
        String::from_utf8(idf)?,
        String::from_utf8(epjson)?,
        json!({"ledger":input["conversion_ledger"],"execution":input["conversion_execution"],"recorded_command":input["conversion_command_receipt"]}),
    ))
}

struct Fixture {
    layout: InsideConvectionLayout,
    zones: Vec<InsideConvectionZone>,
    surfaces: Vec<InsideConvectionSurface>,
    state: InsideConvectionState,
    active_elements: Vec<(ZoneId, [bool; 3])>,
    space_air: SpaceAirTemperatures,
    zone_mat: f64,
    low_limit: f64,
    temp_in: Vec<(SurfaceId, f64)>,
    temp_tmp: Vec<(SurfaceId, f64)>,
    nodes_allocated: bool,
    readiness: InsideConvectionReadiness,
    caller: fields::Caller,
}
impl Fixture {
    fn new(input: &Value, backend: &Value) -> Result<Self> {
        keys(
            input,
            &[
                "zone",
                "spaces",
                "surfaces",
                "node_allocated",
                "zone_equipment_active_elements",
            ],
        )?;
        let zone = &input["zone"];
        keys(
            zone,
            &[
                "Name",
                "spaceIndexes",
                "MAT",
                "CeilingHeight",
                "Volume",
                "FloorArea",
                "MinimumX",
                "MaximumX",
                "MinimumY",
                "MaximumY",
                "ExtGrossWallArea",
                "ExtWindowArea",
                "Multiplier",
                "ListMultiplier",
            ],
        )?;
        let space_rows = rows(&input["spaces"])?;
        let surface_rows = rows(&input["surfaces"])?;
        require(
            !space_rows.is_empty(),
            "actual complete space owner required",
        )?;
        let mut spaces = Vec::new();
        for (index, row) in space_rows.iter().enumerate() {
            keys(
                row,
                &[
                    "actual_array_index",
                    "Name",
                    "HTSurfaceFirst",
                    "HTSurfaceLast",
                    "MAT",
                ],
            )?;
            require(
                usize::try_from(id(&row["actual_array_index"])?)? == index + 1,
                "space index differs",
            )?;
            text(&row["Name"])?;
            spaces.push((SpaceId(id(&row["actual_array_index"])?), ZoneId(1)));
        }
        let mut associations = Vec::new();
        let mut surfaces = Vec::new();
        let mut states = Vec::new();
        let mut ti = Vec::new();
        let mut tmp = Vec::new();
        let mut names = BTreeSet::new();
        for (index, row) in surface_rows.iter().enumerate() {
            keys(
                row,
                &[
                    "actual_array_index",
                    "Name",
                    "spaceNum",
                    "Class",
                    "model",
                    "CosTilt",
                    "Tilt",
                    "Azimuth",
                    "Area",
                    "GrossArea",
                    "Width",
                    "Height",
                    "Perimeter",
                    "Vertex",
                    "SurfTempIn",
                    "SurfTempInTmp",
                    "SurfHConvInt",
                    "SurfTAirRef",
                    "SurfTAirRefRpt",
                ],
            )?;
            let surface_id = SurfaceId(id(&row["actual_array_index"])?);
            require(
                usize::try_from(surface_id.0)? == index + 1
                    && !text(&row["Name"])?.is_empty()
                    && names.insert(text(&row["Name"])?.to_owned()),
                "surface identity differs",
            )?;
            let space_id = SpaceId(id(&row["spaceNum"])?);
            associations.push(InsideConvectionAssociation {
                surface_id,
                zone_id: ZoneId(1),
                space_id,
            });
            let class = match text(&row["Class"])? {
                "Wall" => SurfaceType::Wall,
                "Roof" => SurfaceType::Roof,
                "Floor" => SurfaceType::Floor,
                _ => return Err("opaque literal class required".into()),
            };
            let mut vertices = Vec::new();
            require(
                rows(&row["Vertex"])?.len() == 4,
                "four actual literal vertices required",
            )?;
            for point in rows(&row["Vertex"])? {
                keys(point, &["x", "y", "z"])?;
                vertices.push([real(&point["x"])?, real(&point["y"])?, real(&point["z"])?]);
            }
            require(
                real(&row["Height"])? > 0.0,
                "positive actual Height required",
            )?;
            surfaces.push(InsideConvectionSurface {
                surface_id,
                class,
                exterior: true,
                heat_transfer: true,
                construction_is_window: false,
                kiva: false,
                representative_calculations: false,
                ems_override: false,
                outward_cos_tilt: real(&row["CosTilt"])?,
                tilt: real(&row["Tilt"])?,
                azimuth: real(&row["Azimuth"])?,
                area: real(&row["Area"])?,
                gross_area: real(&row["GrossArea"])?,
                width: real(&row["Width"])?,
                perimeter: real(&row["Perimeter"])?,
                vertices,
            });
            let mut state = InsideConvectionSurfaceState::new(
                surface_id,
                real(&row["SurfHConvInt"])?,
                [
                    integer(&row["SurfTAirRef"])?,
                    integer(&row["SurfTAirRefRpt"])?,
                ],
            );
            state.model = match text(&row["model"])? {
                "SetByZone" => InsideConvectionModel::SetByZone,
                "ASHRAETARP" => InsideConvectionModel::Tarp,
                _ => return Err("actual default/explicit TARP required".into()),
            };
            states.push(state);
            ti.push((surface_id, real(&row["SurfTempIn"])?));
            tmp.push((surface_id, real(&row["SurfTempInTmp"])?));
        }
        let mut groups = Vec::new();
        for entry in rows(&zone["spaceIndexes"])? {
            let space_id = SpaceId(id(entry)?);
            let row = space_rows
                .iter()
                .find(|r| r["actual_array_index"].as_u64() == Some(u64::from(space_id.0)))
                .ok_or("space traversal ID missing")?;
            let first = integer(&row["HTSurfaceFirst"])?;
            let last = integer(&row["HTSurfaceLast"])?;
            require(
                (first == 1 && last == 0)
                    || (first >= 1
                        && last >= first
                        && usize::try_from(last)? <= surface_rows.len()),
                "full actual inclusive HT range required",
            )?;
            groups.push(InsideConvectionGroup {
                zone_id: ZoneId(1),
                space_id,
                surfaces: (first..=last)
                    .map(|v| Ok(SurfaceId(u32::try_from(v)?)))
                    .collect::<Result<Vec<_>>>()?,
            });
        }
        let layout = InsideConvectionLayout::new(vec![ZoneId(1)], spaces, associations, groups)
            .map_err(|_| "actual owner layout unavailable")?;
        let elements = &input["zone_equipment_active_elements"];
        keys(elements, &["wall", "ceiling", "floor"])?;
        let active_elements = vec![(
            ZoneId(1),
            [
                boolean(&elements["wall"])?,
                boolean(&elements["ceiling"])?,
                boolean(&elements["floor"])?,
            ],
        )];
        let state = InsideConvectionState::new(&layout, states)?;
        let mut space_air = SpaceAirTemperatures::allocate(&layout);
        let mut air = space_air.rows().to_vec();
        for (owner, row) in air.iter_mut().zip(space_rows) {
            owner.mat = real(&row["MAT"])?;
        }
        space_air.write_all(&layout, &air)?;
        let zones = vec![InsideConvectionZone {
            zone_id: ZoneId(1),
            tarp: true,
            ceiling_height: real(&zone["CeilingHeight"])?,
            volume: real(&zone["Volume"])?,
            floor_area: real(&zone["FloorArea"])?,
            xy_bounds: [
                real(&zone["MinimumX"])?,
                real(&zone["MaximumX"])?,
                real(&zone["MinimumY"])?,
                real(&zone["MaximumY"])?,
            ],
            exterior_gross_wall_area: real(&zone["ExtGrossWallArea"])?,
            exterior_window_area: real(&zone["ExtWindowArea"])?,
            multiplier: integer(&zone["Multiplier"])?,
            list_multiplier: integer(&zone["ListMultiplier"])?,
        }];
        Ok(Self {
            layout,
            zones,
            surfaces,
            state,
            active_elements,
            space_air,
            zone_mat: real(&zone["MAT"])?,
            low_limit: 0.1,
            temp_in: ti,
            temp_tmp: tmp,
            nodes_allocated: boolean(&input["node_allocated"])?,
            readiness: InsideConvectionReadiness {
                begin_environment: false,
                system_sizing: false,
                zone_sizing: false,
                equipment_inputs_filled: false,
                nodes_allocated: boolean(&input["node_allocated"])?,
                equipment_simulated_once: false,
            },
            caller: fields::Caller::new(backend)?,
        })
    }
    fn snapshot(&self) -> Value {
        fields::snapshot(
            &self.layout,
            &self.zones,
            &self.surfaces,
            &self.state,
            &self.active_elements,
            &self.space_air,
            self.zone_mat,
            self.low_limit,
            &self.temp_in,
            &self.temp_tmp,
            self.readiness,
            &self.caller,
        )
    }
    fn write(&mut self, op: &Value) -> Result<()> {
        keys(
            op,
            &[
                "operation_id",
                "temperature_owner",
                "BeginEnvrnFlag",
                "SysSizingCalc",
                "ZoneSizingCalc",
                "ZoneEquipInputsFilled",
                "ZoneEquipSimulatedOnce",
                "zone_MAT",
                "space_MAT",
                "SurfTempIn",
                "SurfTempInTmp",
            ],
        )?;
        self.readiness = InsideConvectionReadiness {
            begin_environment: boolean(&op["BeginEnvrnFlag"])?,
            system_sizing: boolean(&op["SysSizingCalc"])?,
            zone_sizing: boolean(&op["ZoneSizingCalc"])?,
            equipment_inputs_filled: boolean(&op["ZoneEquipInputsFilled"])?,
            nodes_allocated: self.nodes_allocated,
            equipment_simulated_once: boolean(&op["ZoneEquipSimulatedOnce"])?,
        };
        self.zone_mat = real(&op["zone_MAT"])?;
        let mut air = self.space_air.rows().to_vec();
        let mat = rows(&op["space_MAT"])?;
        require(
            mat.len() == air.len()
                && rows(&op["SurfTempIn"])?.len() == self.temp_in.len()
                && rows(&op["SurfTempInTmp"])?.len() == self.temp_tmp.len(),
            "full literal owner write widths required",
        )?;
        for (row, value) in air.iter_mut().zip(mat) {
            row.mat = real(value)?;
        }
        self.space_air.write_all(&self.layout, &air)?;
        for (row, value) in self.temp_in.iter_mut().zip(rows(&op["SurfTempIn"])?) {
            row.1 = real(value)?;
        }
        for (row, value) in self.temp_tmp.iter_mut().zip(rows(&op["SurfTempInTmp"])?) {
            row.1 = real(value)?;
        }
        Ok(())
    }
}

fn case(
    root: &Path,
    input: &Value,
    idf: &str,
    epjson: &str,
    backend: &Value,
    directory: &Path,
    stopped: &mut bool,
) -> Result<Value> {
    keys(input, &["case_id", "purpose", "initial", "operations"])?;
    let case_id = identifier(&input["case_id"])?;
    text(&input["purpose"])?;
    require(
        !rows(&input["operations"])?.is_empty(),
        "nonempty declared operation list required",
    )?;
    std::fs::create_dir(directory)?;
    let physical = directory.join("literal-input.idf");
    std::fs::write(&physical, idf)?;
    let physical_before = file(&physical)?;
    let raw = parse_epjson_str_with_idf_order(epjson, idf)?;
    let counts: Vec<Value> = NoUserConvectionInput::FAMILIES.iter().enumerate().map(|(i, family)| json!({"actual_validation_getter_ordinal":i+1,"family":family,"actual_count":raw.objects.iter().filter(|(kind, _)|kind.0 == *family).map(|(_, objects)|objects.len()).sum::<usize>()})).collect();
    let mut count_values = [0; 6];
    for (value, row) in count_values.iter_mut().zip(&counts) {
        *value = usize::try_from(
            row["actual_count"]
                .as_u64()
                .ok_or("actual RawModel count required")?,
        )?;
    }
    let no_user = NoUserConvectionInput::new(count_values)?;
    let compiled = compile_raw_model(&raw);
    let diagnostics = format!("{:?}", compiled.report.diagnostics);
    let compiler_report = format!("{:?}", compiled.report);
    let Some(model) = compiled.model else {
        require(
            std::fs::read(&physical)? == idf.as_bytes(),
            "physical literal IDF changed",
        )?;
        *stopped = true;
        return Ok(
            json!({"case_id":case_id,"literal_input":input,"status":"actual-typed-input-unavailable","physical_IDF_before":physical_before,"physical_IDF_after":file(&physical)?,
            "physical_IDF_bytes_unchanged":true,"actual_raw_object_counts":counts,"compiler_diagnostics":diagnostics,"compiler_report":compiler_report,"actual_raw_parser_calls":1,"actual_compiler_calls":1,
            "actual_report_inputs":null,"prepared_literal_owners":null,"actual_operation_input_writes":[],"actual_calls":[],"unreached_operations":input["operations"],"final":null,
            "actual_recorded_selected_call_count":0,"actual_source_stopped":true,"inner_surface_visit_trace_available":false,"Native_InputProcessor_semantics_verified":false,
            "unavailable_owner_families":fields::UNAVAILABLE,"ordinary_production_handoff_verified":false,"numerical_PASS_claimed":false}),
        );
    };
    let reports = model.report_inputs;
    let report_inputs = fields::reports(&reports);
    let mut fixture = Fixture::new(&input["initial"], backend)?;
    let prepared = fixture.snapshot();
    let mut calls = Vec::new();
    let mut writes = Vec::new();
    let mut unreached = Vec::new();
    let mut ids = BTreeSet::new();
    let mut source_stopped = false;
    for op in rows(&input["operations"])? {
        let operation_id = identifier(&op["operation_id"])?;
        require(ids.insert(operation_id), "unique operation ID required")?;
        if source_stopped {
            unreached.push(op.clone());
            continue;
        }
        let before_write = fixture.snapshot();
        fixture.write(op)?;
        let after_write = fixture.snapshot();
        writes.push(json!({"operation_id":operation_id,"literal_input":op,"before_declared_writes":before_write,"after_declared_writes":after_write}));
        let temperature_owner = match text(&op["temperature_owner"])? {
            "SurfTempInTmp" => InsideSurfaceTemperatureOwner::SurfTempInTmp,
            "SurfTempIn" => InsideSurfaceTemperatureOwner::SurfTempIn,
            _ => return Err("actual temperature owner selector required".into()),
        };
        let before = fixture.snapshot();
        let temperatures = if temperature_owner == InsideSurfaceTemperatureOwner::SurfTempInTmp {
            &fixture.temp_tmp
        } else {
            &fixture.temp_in
        };
        let result = fixture.caller.invoke(
            &fixture.layout,
            &fixture.zones,
            &fixture.surfaces,
            no_user,
            &mut fixture.state,
            &mut fixture.active_elements[..],
            &fixture.space_air,
            InsideSurfaceTemperatures {
                owner: temperature_owner,
                rows: temperatures,
            },
            fixture.readiness,
            fixture.low_limit,
            &reports,
        );
        source_stopped = result.is_err();
        let source_error = result.err().map(|error| format!("{error:?}"));
        calls.push(json!({"operation_id":operation_id,"actual_call_ordinal":calls.len()+1,"api":"initialize_inside_convection","actual_source_invoked":true,
            "actual_source_return_observed":!source_stopped,"source_error":source_error,"before":before,"after":fixture.snapshot()}));
    }
    require(
        std::fs::read(&physical)? == idf.as_bytes(),
        "physical literal IDF changed",
    )?;
    *stopped |= source_stopped;
    Ok(
        json!({"case_id":case_id,"literal_input":input,"physical_IDF_before":physical_before,"physical_IDF_after":file(&physical)?,"physical_IDF_bytes_unchanged":true,
        "actual_raw_object_counts":counts,"actual_report_inputs":report_inputs,"compiler_diagnostics":diagnostics,"compiler_report":compiler_report,"actual_raw_parser_calls":1,"actual_compiler_calls":1,
        "prepared_literal_owners":prepared,"actual_operation_input_writes":writes,"actual_calls":calls,"unreached_operations":unreached,"final":fixture.snapshot(),
        "actual_recorded_selected_call_count":calls.len(),"actual_source_stopped":source_stopped,"inner_surface_visit_trace_available":false,
        "Native_InputProcessor_semantics_verified":false,"ordinary_production_handoff_verified":false,"numerical_PASS_claimed":false,
        "unavailable_owner_families":fields::UNAVAILABLE,"request_root":root.to_string_lossy()}),
    )
}

fn run() -> Result<bool> {
    let args: Vec<String> = std::env::args().collect();
    require(
        args.len() == 4,
        "usage: src06_tarp_dispatcher_main <binding.json> <binding-sha256> <fresh-run-directory>",
    )?;
    let root = std::env::current_dir()?.canonicalize()?;
    let packet_bytes = std::fs::read(&args[1])?;
    require(
        digest::sha256(&packet_bytes) == args[2],
        "actual invocation binding SHA differs",
    )?;
    let input: Value = serde_json::from_slice(&packet_bytes)?;
    keys(
        &input,
        &[
            "schema",
            "native_source_preparation",
            "dispatcher_source_preparation",
            "native_request",
            "conversion_ledger",
            "conversion_execution",
            "conversion_command_receipt",
            "runtime_sources",
            "backend_context",
            "pair_policy",
            "expected_values_supplied",
            "Native_results_supplied",
        ],
    )?;
    require(
        input["schema"] == "src06-tarp-dispatcher-Rust-observer-inputs.v1"
            && input["expected_values_supplied"] == false
            && input["Native_results_supplied"] == false,
        "input-only observer binding required",
    )?;
    require(
        text(&input["native_source_preparation"]["sha256"])?
            == "954a7779868d3ac65d88c553aab324f5e5e0a88d5d455a3126aecc5012ca3d50"
            && text(&input["dispatcher_source_preparation"]["sha256"])?
                == "0888b88cd65dff863461bac16b271d967b90c5af6ef43df6e247a025aa325889",
        "reviewed immutable Source authorities required",
    )?;
    checked(&root, &input["native_source_preparation"])?;
    checked(&root, &input["dispatcher_source_preparation"])?;
    let native_bytes = checked(&root, &input["native_request"])?;
    require(
        digest::sha256(&native_bytes)
            == "cb52b8d846b280dcae02b9261eca84e90a742e01051180bca0fac8318487e23d",
        "unchanged complete eight-case literal request required",
    )?;
    let native: Value = serde_json::from_slice(&native_bytes)?;
    keys(
        &native,
        &[
            "schema",
            "expected_values_supplied",
            "ordinary_production_prestate_supplied",
            "literal_IDF",
            "cases",
        ],
    )?;
    require(
        native["schema"] == "src06-tarp-dispatcher-direct-request.v1"
            && native["expected_values_supplied"] == false
            && native["ordinary_production_prestate_supplied"] == false,
        "literal scientific input-only request required",
    )?;
    let runtime_sources = source_bindings(&root, &input["runtime_sources"])?;
    let policy_before = checked(&root, &input["pair_policy"])?;
    let (idf, epjson, conversion_proof) = conversion(&root, &input, &native)?;
    let output = root.join(PathBuf::from(&args[3]));
    require(
        !output.exists()
            && output
                .parent()
                .ok_or("output parent required")?
                .canonicalize()?
                .starts_with(root.join(".runtime/porting/SRC-06")),
        "fresh contained ignored output root required",
    )?;
    std::fs::create_dir(&output)?;
    let mut cases = Vec::new();
    let mut ids = BTreeSet::new();
    let mut stopped = false;
    for item in rows(&native["cases"])? {
        let id = identifier(&item["case_id"])?;
        require(ids.insert(id), "unique case ID required")?;
        cases.push(case(
            &root,
            item,
            &idf,
            &epjson,
            &input["backend_context"],
            &output.join(id),
            &mut stopped,
        )?);
    }
    require(
        std::fs::read(&args[1])? == packet_bytes
            && checked(&root, &input["native_request"])? == native_bytes
            && checked(&root, &input["pair_policy"])? == policy_before,
        "frozen input/policy changed",
    )?;
    source_bindings(&root, &input["runtime_sources"])?;
    conversion(&root, &input, &native)?;
    println!(
        "{}",
        serde_json::to_string(
            &json!({"schema":"src06-tarp-dispatcher-Rust-observer-results.v1","input_binding_sha256":args[2],"literal_request":input["native_request"],
        "conversion_proof":conversion_proof,"actual_runtime_sources":runtime_sources,"prospective_pair_policy":input["pair_policy"],"requested_case_count":rows(&native["cases"])?.len(),
        "recorded_case_count":cases.len(),"cases":cases,"inner_surface_visit_trace_available":false,"scientific_PASS_claimed":false,"ordinary_production_handoff_verified":false,"gates_updated":false})
        )?
    );
    Ok(stopped)
}
fn main() {
    match run() {
        Ok(false) => {}
        Ok(true) => std::process::exit(2),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(2);
        }
    }
}
