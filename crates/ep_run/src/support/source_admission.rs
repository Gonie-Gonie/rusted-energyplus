//! Ancillary classification of raw inputs on the admitted ordinary Source lane.
//! This token never bypasses an input getter or provides Native runtime state.
use crate::porting_scope::{PortingScope, PortingScopeTrace};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use ep_runtime::heat_balance::selected_input_processor::SourceInputStatisticsValue;
use std::collections::{BTreeMap, BTreeSet};

// Custody comparison only; no projected values or runtime Curve state are created.
fn retained_curve_value_matches(raw: &RawValue, retained: &SourceInputStatisticsValue) -> bool {
    match (raw, retained) {
        (RawValue::Null, SourceInputStatisticsValue::Null) => true,
        (RawValue::Bool(a), SourceInputStatisticsValue::Bool(b)) => a == b,
        (RawValue::Number(a), SourceInputStatisticsValue::Number(b))
        | (RawValue::String(a), SourceInputStatisticsValue::String(b)) => a == b,
        (RawValue::Array(a), SourceInputStatisticsValue::Array(b)) => {
            a.len() == b.len()
                && a.iter()
                    .zip(b)
                    .all(|(a, b)| retained_curve_value_matches(a, b))
        }
        (RawValue::Object(a), SourceInputStatisticsValue::Object(b)) => {
            a.len() == b.len()
                && a.iter().all(|(name, value)| {
                    b.get(&name.0)
                        .is_some_and(|retained| retained_curve_value_matches(value, retained))
                })
        }
        _ => false,
    }
}

pub(crate) struct SourceSupportAdmission {
    covered_objects: BTreeSet<&'static str>,
    /// Validated unused ordinary Glass names; no advisory optical coefficients.
    glazing_bindings: BTreeSet<String>,
    /// Exact Spectral-vs-Average tag and original A3 lookup text;no optical seed.
    spectral_glazing_bindings: BTreeMap<String, Option<String>>,
    /// Exact Angle tag only; original A5-A7 Strings remain owned by Raw/IP.
    angle_glazing_bindings: BTreeSet<String>,
    /// Ordered effective dataset names,including duplicate uppercase names.
    spectral_data_bindings: Vec<String>,
    /// Validated unused Shade names only; runtime properties come from its getter.
    shade_bindings: BTreeSet<String>,
    /// Validated alternative Glass names only, never advisory optical values.
    refraction_bindings: BTreeSet<String>,
    /// Validated Raw uppercase loader names/types only, never material properties.
    window_gas_bindings: BTreeMap<String, ep_model::WindowGasType>,
    /// Raw active species/count only, never mixture fractions or properties.
    window_gas_mixture_bindings: BTreeMap<String, Vec<ep_model::WindowStandardGasType>>,
    /// Exact local alpha fields consumed by genuine GetZoneData and the live
    /// owning-Zone dispatchers. This is not a general Zone-feature exemption.
    zone_convection_overrides_consumed: bool,
    /// Source A DesignDay inputs, electric gains and bounded Zones/lists/whole-Zone Spaces use
    /// their own class without inheriting the frozen one-Zone capability.
    multizone_consumed: bool,
    /// Explicit validated Source B/raw equipment custody, not legacy release evidence.
    ideal_loads_consumed: bool,
}

impl SourceSupportAdmission {
    /// Only the fully validated finite scope/environment and existing raw
    /// admission may classify these actual Source-consumed input semantics.
    // Retain the existing nested input/trace guards and their error or diagnostic order.
    #[allow(clippy::collapsible_if)]
    pub(crate) fn from_validated_scope(raw: &RawModel, trace: &PortingScopeTrace) -> Option<Self> {
        if !trace.admissible || !trace.violations.is_empty() || trace.environment.is_none() {
            return None;
        }
        match trace.scope {
            PortingScope::A => crate::ordinary_equipment_input::resolve_source_a(raw),
            PortingScope::B => crate::ordinary_equipment_input::resolve_source_b(raw),
        }
        .ok()?;
        // The existing raw admission checks every field/frequency/schedule;
        // the same reporting pre-scan must retain every actual request object.
        let reporting =
            crate::ordinary_input_processor::resolve_reporting_prescan_source(raw).ok()?;
        let requested = raw.objects.get(&ObjectType("Output:Variable".into()));
        if reporting.variables.len() != requested.map_or(0, |objects| objects.len()) {
            return None;
        }
        if !raw
            .source_json_instances("Output:Variable")
            .ok()?
            .iter()
            .zip(&reporting.variables)
            .all(|((name, _), request)| name.0 == request.object_name)
        {
            return None;
        }
        // The same real IP/raw projection retains every normal meter request
        // (including unnamed blank keys) for UpdateMeterReporting. This proves
        // custody, not arbitrary meter-resource/SQLite or cumulative support.
        if ["Output:Meter", "Output:Meter:MeterFileOnly"]
            .iter()
            .any(|family| {
                raw.objects
                    .get(&ObjectType((*family).into()))
                    .is_some_and(|rows| !rows.is_empty())
            })
        {
            let input = match trace.scope {
                PortingScope::A => crate::ordinary_input_processor::resolve_source_a(raw),
                PortingScope::B => crate::ordinary_input_processor::resolve_source_b(raw),
            }
            .ok()?;
            let projected = crate::ordinary_heat_balance_input::resolve_source(raw, &input).ok()?;
            for family in ["Output:Meter", "Output:Meter:MeterFileOnly"] {
                let raw_names = raw
                    .objects
                    .get(&ObjectType(family.into()))
                    .map(|rows| {
                        rows.keys()
                            .map(|key| key.0.as_str())
                            .collect::<BTreeSet<_>>()
                    })
                    .unwrap_or_default();
                let names = projected
                    .objects
                    .get(family)
                    .map(|rows| {
                        rows.iter()
                            .map(|row| row.object_name.as_str())
                            .collect::<BTreeSet<_>>()
                    })
                    .unwrap_or_default();
                if raw_names != names {
                    return None;
                }
            }
        }
        let mut covered_objects: BTreeSet<_> = [
            // The same raw resolver and validated Source scope admit only the
            // implemented Scheduled control/reference route. This token lets
            // the typed guard distinguish it from legacy/broad HVAC semantics.
            "SetpointManager:Scheduled",
            "HeatBalanceAlgorithm",
            // Both explicit Source lanes use the same genuine raw DesignDay getter.
            "SizingPeriod:DesignDay",
            // Only the validated Source A resolver can reach this real family.
            "ElectricEquipment",
            // Real GetZoneData/GetSpaceData and list/group demand owners.
            "ZoneList",
            "ZoneGroup",
            "Space",
            "SpaceList",
            // Ordered IRT getter/ROnly CTF properties; the same raw resolver
            // checks safe Source uses before this consumed-family marker.
            "Material:InfraredTransparent",
            "SimulationControl",
            "SurfaceConvectionAlgorithm:Inside",
            "SurfaceConvectionAlgorithm:Outside",
            // This monthly weather input is consumed by the actual Source ground
            // model; admitted thermal boundary branches remain scope-validated.
            "Site:GroundTemperature:BuildingSurface",
            "Output:Variable",
            "Output:Meter",
            "Output:Meter:MeterFileOnly",
            // Actual Source schedule reporting consumes real key_field and EIO
            // owners; this token never substitutes a TypedModel schedule.
            "Output:Schedules",
            // Actual comfort initialization consumes all declared lists through
            // its shared IP getter; this remains compiler RawOnly.
            "ComfortViewFactorAngles",
            // Genuine authored single/list getter on the validated B lane.
            // The same Raw/Scope prerequisites still gate this classification.
            "OutdoorAir:Node",
            "OutdoorAir:NodeList",
            // Actual local getters populate the same retained Day/Week/Detailed
            // graph. Typed legacy schedules alone do not grant this token.
            "Schedule:Day:Hourly",
            "Schedule:Day:Interval",
            "Schedule:Day:List",
            "Schedule:Week:Compact",
            "Schedule:Week:Daily",
            "Schedule:Year",
            "Schedule:File",
        ]
        .into_iter()
        .filter(|family| {
            (!matches!(*family, "OutdoorAir:Node" | "OutdoorAir:NodeList")
                || trace.scope == PortingScope::B)
                && raw
                    .objects
                    .get(&ObjectType((*family).into()))
                    .is_some_and(|objects| !objects.is_empty())
        })
        .collect();
        // Scope and the same raw resolver already check every effective enum
        // and every supplied scalar. Keep authored Override provenance in the
        // typed model; only a nonempty, actually present local alpha is covered.
        let zone_convection_overrides_consumed = raw
            .objects
            .get(&ObjectType("Zone".into()))
            .is_some_and(|objects| {
                objects.values().any(|object| {
                    [
                        "zone_inside_convection_algorithm",
                        "zone_outside_convection_algorithm",
                    ]
                    .iter()
                    .any(|field| {
                        matches!(object.fields.get(&FieldName((*field).into())),
                        Some(RawValue::String(value)) if !value.is_empty())
                    })
                })
            });
        if trace.scope == PortingScope::B
            && raw
                .objects
                .get(&ObjectType("WindowMaterial:SimpleGlazingSystem".into()))
                .is_some_and(|rows| !rows.is_empty())
            && crate::ordinary_equipment_input::source_b_validate_unused_simple_glazing(raw).is_ok()
        {
            covered_objects.insert("WindowMaterial:SimpleGlazingSystem");
        }
        // Only validated SourceB custody can cover these genuinely read Curve families.
        // The actual Curve array/map lives in runtime SelectedCurves,never in this token.
        if trace.scope == PortingScope::B
            && raw.objects.iter().any(|(family, rows)| {
                crate::ordinary_equipment_input::SOURCE_B_ANALYTIC_CURVE_FAMILIES
                    .contains(&family.0.as_str())
                    && !rows.is_empty()
            })
            && crate::ordinary_equipment_input::source_b_validate_unused_analytic_curves(raw)
                .is_ok()
        {
            let input = crate::ordinary_input_processor::resolve_source_b(raw).ok()?;
            for family in crate::ordinary_equipment_input::SOURCE_B_ANALYTIC_CURVE_FAMILIES {
                let ordered = raw.source_ordered_instances(family).ok()?;
                if ordered.is_empty() {
                    continue;
                }
                let names = input.source_raw_object_names(family).ok()?;
                if names.len() != ordered.len()
                    || !names
                        .iter()
                        .zip(&ordered)
                        .all(|(name, (raw_name, _))| name == &raw_name.0)
                {
                    return None;
                }
                for (name, object) in &ordered {
                    let fields = input.source_raw_object_fields(family, &name.0).ok()?;
                    if fields.len() != object.fields.len()
                        || object.fields.iter().any(|(field, value)| {
                            fields.get(&field.0).is_none_or(|retained| {
                                !retained_curve_value_matches(value, retained)
                            })
                        })
                    {
                        return None;
                    }
                }
                covered_objects.insert(family);
            }
        }
        let mut glazing_bindings = BTreeSet::new();
        let mut spectral_glazing_bindings = BTreeMap::new();
        let mut angle_glazing_bindings = BTreeSet::new();
        let mut spectral_data_bindings = Vec::new();
        if trace.scope == PortingScope::B
            && raw
                .objects
                .get(&ObjectType("WindowMaterial:Glazing".into()))
                .is_some_and(|rows| !rows.is_empty())
            && crate::ordinary_equipment_input::source_b_validate_unused_spectral_average(raw)
                .is_ok()
        {
            for (name, object) in raw
                .objects
                .get(&ObjectType("WindowMaterial:Glazing".into()))?
            {
                if matches!(object.fields.get(&FieldName("optical_data_type".into())),Some(RawValue::String(mode)) if mode=="Spectral")
                {
                    let reference = match object
                        .fields
                        .get(&FieldName("window_glass_spectral_data_set_name".into()))
                    {
                        Some(RawValue::String(name)) => Some(name.to_ascii_uppercase()),
                        None => None,
                        _ => return None,
                    };
                    spectral_glazing_bindings.insert(name.0.to_ascii_uppercase(), reference);
                }
                if matches!(object.fields.get(&FieldName("optical_data_type".into())),Some(RawValue::String(mode)) if mode=="SpectralAndAngle")
                {
                    angle_glazing_bindings.insert(name.0.to_ascii_uppercase());
                }
                if !glazing_bindings.insert(name.0.to_ascii_uppercase()) {
                    return None;
                }
            }
            covered_objects.insert("WindowMaterial:Glazing");
        }
        if trace.scope == PortingScope::B
            && crate::ordinary_equipment_input::source_b_validate_spectral_data(raw).is_ok()
        {
            for (name, _) in raw
                .source_ordered_instances("MaterialProperty:GlazingSpectralData")
                .ok()?
            {
                spectral_data_bindings.push(name.0.to_ascii_uppercase());
            }
            if !spectral_data_bindings.is_empty() {
                covered_objects.insert("MaterialProperty:GlazingSpectralData");
            }
        }
        let mut shade_bindings = BTreeSet::new();
        if trace.scope == PortingScope::B
            && raw
                .objects
                .get(&ObjectType("WindowMaterial:Shade".into()))
                .is_some_and(|rows| !rows.is_empty())
            && crate::ordinary_equipment_input::source_b_validate_unused_shade(raw).is_ok()
        {
            for name in raw
                .objects
                .get(&ObjectType("WindowMaterial:Shade".into()))?
                .keys()
            {
                if !shade_bindings.insert(name.0.to_ascii_uppercase()) {
                    return None;
                }
            }
            covered_objects.insert("WindowMaterial:Shade");
        }
        let mut refraction_bindings = BTreeSet::new();
        if trace.scope == PortingScope::B
            && raw
                .objects
                .get(&ObjectType(
                    "WindowMaterial:Glazing:RefractionExtinctionMethod".into(),
                ))
                .is_some_and(|rows| !rows.is_empty())
            && crate::ordinary_equipment_input::source_b_validate_unused_refraction(raw).is_ok()
        {
            for name in raw
                .objects
                .get(&ObjectType(
                    "WindowMaterial:Glazing:RefractionExtinctionMethod".into(),
                ))?
                .keys()
            {
                if !refraction_bindings.insert(name.0.to_ascii_uppercase()) {
                    return None;
                }
            }
            covered_objects.insert("WindowMaterial:Glazing:RefractionExtinctionMethod");
        }
        let mut window_gas_bindings = BTreeMap::new();
        if trace.scope == PortingScope::B
            && raw
                .objects
                .get(&ObjectType("WindowMaterial:Gas".into()))
                .is_some_and(|rows| !rows.is_empty())
            && crate::ordinary_equipment_input::source_b_validate_unused_named_gas(raw).is_ok()
        {
            let rows = raw.objects.get(&ObjectType("WindowMaterial:Gas".into()))?;
            for (name, row) in rows {
                let RawValue::String(value) = row.fields.get(&FieldName("gas_type".into()))? else {
                    return None;
                };
                let gas_type = ep_model::WindowGasType::from_energyplus_name(value)?;
                if window_gas_bindings
                    .insert(name.0.to_ascii_uppercase(), gas_type)
                    .is_some()
                {
                    return None; // Existing global duplicate-name partition stays held.
                }
            }
            covered_objects.insert("WindowMaterial:Gas");
        }
        let mut window_gas_mixture_bindings = BTreeMap::new();
        if trace.scope == PortingScope::B
            && raw
                .objects
                .get(&ObjectType("WindowMaterial:GasMixture".into()))
                .is_some_and(|rows| !rows.is_empty())
            && crate::ordinary_equipment_input::source_b_validate_unused_gas_mixture(raw).is_ok()
        {
            let rows = raw
                .objects
                .get(&ObjectType("WindowMaterial:GasMixture".into()))?;
            for (name, row) in rows {
                let RawValue::Number(token) = row
                    .fields
                    .get(&FieldName("number_of_gases_in_mixture".into()))?
                else {
                    return None;
                };
                let count = ep_model::classify_source_json_number(token)
                    .ok()?
                    .as_i32()?;
                let mut active = Vec::new();
                for index in 1..=count {
                    let RawValue::String(value) =
                        row.fields.get(&FieldName(format!("gas_{index}_type")))?
                    else {
                        return None;
                    };
                    active.push(ep_model::WindowStandardGasType::from_energyplus_name(
                        value,
                    )?);
                }
                if window_gas_mixture_bindings
                    .insert(name.0.to_ascii_uppercase(), active)
                    .is_some()
                {
                    return None;
                }
            }
            covered_objects.insert("WindowMaterial:GasMixture");
        }
        Some(Self {
            covered_objects,
            glazing_bindings,
            spectral_glazing_bindings,
            angle_glazing_bindings,
            spectral_data_bindings,
            shade_bindings,
            refraction_bindings,
            window_gas_bindings,
            window_gas_mixture_bindings,
            zone_convection_overrides_consumed,
            ideal_loads_consumed: trace.scope == PortingScope::B
                && raw
                    .objects
                    .get(&ObjectType("ZoneHVAC:IdealLoadsAirSystem".into()))
                    .is_some_and(|rows| rows.len() == 1)
                && raw
                    .objects
                    .get(&ObjectType("ZoneHVAC:EquipmentConnections".into()))
                    .is_some_and(|rows| rows.len() == 1),
            multizone_consumed: trace.scope == PortingScope::A
                && (raw
                    .objects
                    .get(&ObjectType("SizingPeriod:DesignDay".into()))
                    .is_some_and(|rows| !rows.is_empty())
                    || raw
                        .objects
                        .get(&ObjectType("Zone".into()))
                        .is_some_and(|rows| rows.len() > 1)
                    || raw
                        .objects
                        .get(&ObjectType("ZoneList".into()))
                        .is_some_and(|rows| !rows.is_empty())
                    || raw
                        .objects
                        .get(&ObjectType("ZoneGroup".into()))
                        .is_some_and(|rows| !rows.is_empty())
                    || raw
                        .objects
                        .get(&ObjectType("Space".into()))
                        .is_some_and(|rows| !rows.is_empty())
                    || raw
                        .objects
                        .get(&ObjectType("SpaceList".into()))
                        .is_some_and(|rows| !rows.is_empty())
                    || raw
                        .objects
                        .get(&ObjectType("ElectricEquipment".into()))
                        .is_some_and(|rows| !rows.is_empty())),
        })
    }

    // Ordered per-instance custody,not a false unique-name bijection.
    pub(super) fn consumes_spectral_data(&self, model: &ep_model::TypedModel) -> bool {
        let expected = &self.spectral_data_bindings;
        if expected.is_empty() {
            return model.glazing_spectral_data.is_empty()
                && model.glazing_spectral_data_names.is_empty();
        }
        if !self.consumes("MaterialProperty:GlazingSpectralData")
            || !model
                .glazing_spectral_data_names
                .is_source_exact_namespace()
            || model.glazing_spectral_data.len() != expected.len()
            || model.glazing_spectral_data_names.len() != expected.len()
        {
            return false;
        }
        for (index, (row, name)) in model.glazing_spectral_data.iter().zip(expected).enumerate() {
            if usize::try_from(row.id.0).ok() != Some(index)
                || row.name.0 != *name
                || model.glazing_spectral_data_names.names()[index].0 != *name
            {
                return false;
            }
            let Some(first) = expected.iter().position(|candidate| candidate == name) else {
                return false;
            };
            if model.glazing_spectral_data_names.resolve(name)
                != Some(model.glazing_spectral_data[first].id)
            {
                return false;
            }
        }
        true
    }

    pub(super) fn consumes_unused_glazing(&self, model: &ep_model::TypedModel) -> bool {
        use ep_model::MaterialKind;
        if !self.consumes("WindowMaterial:Glazing")
            || !model.material_names.is_source_exact_namespace()
        {
            return false;
        }
        let all_ids: BTreeSet<_> = model.materials.iter().map(|row| row.id).collect();
        if all_ids.len() != model.materials.len() {
            return false;
        }
        let expected: BTreeSet<_> = self.glazing_bindings.iter().map(String::as_str).collect();
        let mut glass_rows = BTreeMap::new();
        for row in &model.materials {
            if matches!(row.kind(), MaterialKind::WindowGlazing)
                && glass_rows.insert(row.id, row).is_some()
            {
                return false;
            }
        }
        if glass_rows.len() != expected.len() {
            return false;
        }
        let mut material_map_names: BTreeMap<ep_model::MaterialId, &str> = BTreeMap::new();
        for name in model.material_names.names() {
            let id = model.material_names.resolve(&name.0);
            if id.is_some_and(|id| glass_rows.contains_key(&id))
                || expected.contains(name.0.as_str())
            {
                let Some(id) = id else {
                    return false;
                };
                if !glass_rows.contains_key(&id)
                    || !expected.contains(name.0.as_str())
                    || material_map_names.insert(id, name.0.as_str()).is_some()
                {
                    return false;
                }
            }
        }
        let mut by_name: BTreeMap<&str, Vec<ep_model::MaterialId>> = BTreeMap::new();
        let mut by_id: BTreeMap<ep_model::MaterialId, Vec<&str>> = BTreeMap::new();
        for (id, name) in &model.ctf_report_names.materials {
            if !all_ids.contains(id) {
                return false;
            }
            if glass_rows.contains_key(id) || expected.contains(name.as_str()) {
                by_name.entry(name.as_str()).or_default().push(*id);
                by_id.entry(*id).or_default().push(name.as_str());
            }
        }
        for name in expected {
            let Some(ids) = by_name.get(name) else {
                return false;
            };
            if ids.len() != 1 {
                return false;
            }
            let id = ids[0];
            let Some(row) = glass_rows.get(&id) else {
                return false;
            };
            if row.name.0 != name
                || model.material_names.resolve(name) != Some(id)
                || material_map_names.get(&id).copied() != Some(name)
                || by_id.get(&id).is_none_or(|names| names.len() != 1)
            {
                return false;
            }
            match (
                self.angle_glazing_bindings.contains(name),
                self.spectral_glazing_bindings.get(name),
                row.definition,
            ) {
                (false, None, ep_model::MaterialDefinition::WindowGlazingSpectralAverage(_)) => {}
                (true, None, ep_model::MaterialDefinition::WindowGlazingSpectralAndAngle(_)) => {}
                (
                    false,
                    Some(reference),
                    ep_model::MaterialDefinition::WindowGlazingSpectral(material),
                ) => {
                    if !self.consumes_spectral_data(model) {
                        return false;
                    }
                    let expected = reference
                        .as_deref()
                        .and_then(|name| model.glazing_spectral_data_names.resolve(name));
                    if material.spectral_data != expected {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }

    pub(super) fn consumes_unused_refraction(&self, model: &ep_model::TypedModel) -> bool {
        use ep_model::MaterialKind;
        if !self.consumes("WindowMaterial:Glazing:RefractionExtinctionMethod")
            || !model.material_names.is_source_exact_namespace()
        {
            return false;
        }
        let all_ids: BTreeSet<_> = model.materials.iter().map(|row| row.id).collect();
        if all_ids.len() != model.materials.len() {
            return false;
        }
        let expected: BTreeSet<_> = self
            .refraction_bindings
            .iter()
            .map(String::as_str)
            .collect();
        let mut glass_rows = BTreeMap::new();
        for row in &model.materials {
            if matches!(row.kind(), MaterialKind::WindowGlazingRefractionExtinction)
                && glass_rows.insert(row.id, row).is_some()
            {
                return false;
            }
        }
        if glass_rows.len() != expected.len() {
            return false;
        }
        let mut material_map_names: BTreeMap<ep_model::MaterialId, &str> = BTreeMap::new();
        for name in model.material_names.names() {
            let id = model.material_names.resolve(&name.0);
            if id.is_some_and(|id| glass_rows.contains_key(&id))
                || expected.contains(name.0.as_str())
            {
                let Some(id) = id else {
                    return false;
                };
                if !glass_rows.contains_key(&id)
                    || !expected.contains(name.0.as_str())
                    || material_map_names.insert(id, name.0.as_str()).is_some()
                {
                    return false;
                }
            }
        }
        let mut by_name: BTreeMap<&str, Vec<ep_model::MaterialId>> = BTreeMap::new();
        let mut by_id: BTreeMap<ep_model::MaterialId, Vec<&str>> = BTreeMap::new();
        for (id, name) in &model.ctf_report_names.materials {
            if !all_ids.contains(id) {
                return false;
            }
            if glass_rows.contains_key(id) || expected.contains(name.as_str()) {
                by_name.entry(name.as_str()).or_default().push(*id);
                by_id.entry(*id).or_default().push(name.as_str());
            }
        }
        for name in expected {
            let Some(ids) = by_name.get(name) else {
                return false;
            };
            if ids.len() != 1 {
                return false;
            }
            let id = ids[0];
            let Some(row) = glass_rows.get(&id) else {
                return false;
            };
            if row.name.0 != name
                || model.material_names.resolve(name) != Some(id)
                || material_map_names.get(&id).copied() != Some(name)
                || by_id.get(&id).is_none_or(|names| names.len() != 1)
            {
                return false;
            }
            if !matches!(
                row.definition,
                ep_model::MaterialDefinition::WindowGlazingRefractionExtinction(_)
            ) {
                return false;
            }
        }
        true
    }

    pub(super) fn consumes_unused_shade(&self, model: &ep_model::TypedModel) -> bool {
        use ep_model::MaterialKind;
        if !self.consumes("WindowMaterial:Shade")
            || !model.material_names.is_source_exact_namespace()
        {
            return false;
        }
        let all_ids: BTreeSet<_> = model.materials.iter().map(|row| row.id).collect();
        if all_ids.len() != model.materials.len() {
            return false;
        }
        let expected: BTreeSet<_> = self.shade_bindings.iter().map(String::as_str).collect();
        let mut shade_rows = BTreeMap::new();
        for row in &model.materials {
            if matches!(row.kind(), MaterialKind::WindowShade)
                && shade_rows.insert(row.id, row).is_some()
            {
                return false;
            }
        }
        if shade_rows.len() != expected.len() {
            return false;
        }
        let mut material_map_names: BTreeMap<ep_model::MaterialId, &str> = BTreeMap::new();
        for name in model.material_names.names() {
            let id = model.material_names.resolve(&name.0);
            if id.is_some_and(|id| shade_rows.contains_key(&id))
                || expected.contains(name.0.as_str())
            {
                let Some(id) = id else {
                    return false;
                };
                if !shade_rows.contains_key(&id)
                    || !expected.contains(name.0.as_str())
                    || material_map_names.insert(id, name.0.as_str()).is_some()
                {
                    return false;
                }
            }
        }
        let mut by_name: BTreeMap<&str, Vec<ep_model::MaterialId>> = BTreeMap::new();
        let mut by_id: BTreeMap<ep_model::MaterialId, Vec<&str>> = BTreeMap::new();
        for (id, name) in &model.ctf_report_names.materials {
            if !all_ids.contains(id) {
                return false;
            }
            if shade_rows.contains_key(id) || expected.contains(name.as_str()) {
                by_name.entry(name.as_str()).or_default().push(*id);
                by_id.entry(*id).or_default().push(name.as_str());
            }
        }
        for name in expected {
            let Some(ids) = by_name.get(name) else {
                return false;
            };
            if ids.len() != 1 {
                return false;
            }
            let id = ids[0];
            let Some(row) = shade_rows.get(&id) else {
                return false;
            };
            if row.name.0 != name
                || model.material_names.resolve(name) != Some(id)
                || material_map_names.get(&id).copied() != Some(name)
                || by_id.get(&id).is_none_or(|names| names.len() != 1)
            {
                return false;
            }
            if !matches!(row.definition, ep_model::MaterialDefinition::WindowShade(_)) {
                return false;
            }
        }
        true
    }

    // A total Raw/typed loader bijection for both owned MaterialGasMix kinds.
    pub(super) fn consumes_window_gas_materials(&self, model: &ep_model::TypedModel) -> bool {
        use ep_model::MaterialKind;
        if !(self.consumes("WindowMaterial:Gas") || self.consumes("WindowMaterial:GasMixture"))
            || !model.material_names.is_source_exact_namespace()
        {
            return false;
        }
        let all_ids: BTreeSet<_> = model.materials.iter().map(|row| row.id).collect();
        if all_ids.len() != model.materials.len() {
            return false;
        }
        let expected: BTreeSet<_> = self
            .window_gas_bindings
            .keys()
            .chain(self.window_gas_mixture_bindings.keys())
            .map(String::as_str)
            .collect();
        if expected.len() != self.window_gas_bindings.len() + self.window_gas_mixture_bindings.len()
        {
            return false;
        }
        let mut gas_rows = BTreeMap::new();
        for row in &model.materials {
            if matches!(
                row.kind(),
                MaterialKind::WindowGas | MaterialKind::WindowGasMixture
            ) && gas_rows.insert(row.id, row).is_some()
            {
                return false;
            }
        }
        if gas_rows.len() != expected.len() {
            return false;
        }
        let mut material_map_names: BTreeMap<ep_model::MaterialId, &str> = BTreeMap::new();
        for name in model.material_names.names() {
            let id = model.material_names.resolve(&name.0);
            if id.is_some_and(|id| gas_rows.contains_key(&id)) || expected.contains(name.0.as_str())
            {
                let Some(id) = id else {
                    return false;
                };
                if !gas_rows.contains_key(&id)
                    || !expected.contains(name.0.as_str())
                    || material_map_names.insert(id, name.0.as_str()).is_some()
                {
                    return false;
                }
            }
        }
        let mut by_name: BTreeMap<&str, Vec<ep_model::MaterialId>> = BTreeMap::new();
        let mut by_id: BTreeMap<ep_model::MaterialId, Vec<&str>> = BTreeMap::new();
        for (id, name) in &model.ctf_report_names.materials {
            if !all_ids.contains(id) {
                return false;
            }
            if gas_rows.contains_key(id) || expected.contains(name.as_str()) {
                by_name.entry(name.as_str()).or_default().push(*id);
                by_id.entry(*id).or_default().push(name.as_str());
            }
        }
        for name in expected {
            let Some(ids) = by_name.get(name) else {
                return false;
            };
            if ids.len() != 1 {
                return false;
            }
            let id = ids[0];
            let Some(row) = gas_rows.get(&id) else {
                return false;
            };
            if row.name.0 != name
                || model.material_names.resolve(name) != Some(id)
                || material_map_names.get(&id).copied() != Some(name)
                || by_id.get(&id).is_none_or(|names| names.len() != 1)
            {
                return false;
            }
            if let Some(expected_type) = self.window_gas_bindings.get(name) {
                if row
                    .as_window_gas()
                    .is_none_or(|gas| gas.gas_type != *expected_type)
                {
                    return false;
                }
            } else if let Some(expected_types) = self.window_gas_mixture_bindings.get(name) {
                let Some(mixture) = row.as_window_gas_mixture() else {
                    return false;
                };
                let active = mixture.gases.components();
                if active.len() != expected_types.len()
                    || active
                        .iter()
                        .zip(expected_types)
                        .any(|(gas, expected)| gas.gas_type != *expected)
                {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub(super) fn consumes_ideal_loads(&self) -> bool {
        self.ideal_loads_consumed
    }

    pub(super) fn consumes_multizone(&self) -> bool {
        self.multizone_consumed
    }

    pub(super) fn consumes_zone_convection_overrides(&self) -> bool {
        self.zone_convection_overrides_consumed
    }

    pub(super) fn consumes(&self, family: &str) -> bool {
        self.covered_objects.contains(family)
    }
}
