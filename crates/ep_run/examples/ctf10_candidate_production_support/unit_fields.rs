//! Copies actual selected retry owners; no source arithmetic or Native event reconstruction.
use ep_runtime::heat_balance::ctf_first_assembly_owner::{
    ConstructionCtfFirstAssembly, CtfFirstAssemblyUnavailable,
};
use ep_runtime::heat_balance::ctf_initial_owner::{
    ConstructionCtfInitialDiscretization, CtfInitialUnavailable,
};
use ep_runtime::heat_balance::ctf_retry_driver::{
    ConstructionCtfRetryDriver, CtfRetryAssemblySnapshot, CtfRetryAttempt,
};
use ep_runtime::heat_balance::ctf_retry_initialization::ConstructionCtfRetryInitialization;
use ep_runtime::heat_balance::surface_manager::ctf_caller_stability::{
    CtfCallerSharedFlags, CtfCallerStabilityDecision, CtfCallerStabilityScopeError,
};
use ep_runtime::heat_balance::surface_manager::ctf_exponential_matrix::{
    CtfExponentialObservation, CtfExponentialScopeError,
};
use ep_runtime::heat_balance::surface_manager::ctf_final_coefficients::{
    CtfFinalCoefficientObservation, CtfFinalCoefficientScopeError,
};
use ep_runtime::heat_balance::surface_manager::ctf_gamma_matrix::{
    CtfGammaObservation, CtfGammaScopeError,
};
use ep_runtime::heat_balance::surface_manager::ctf_inverse_matrix::{
    CtfInverseMatrix, CtfInverseScopeError,
};
use ep_runtime::heat_balance::surface_manager::ctf_layer_preprocessing::{
    ConstructionCtfLayerPreprocessing, CtfLayerCheckpoint, CtfLayerInput, CtfLayerPreprocessing,
    CtfLayerScopeError,
};
use ep_runtime::heat_balance::surface_manager::ctf_state_space_assembly::{
    CtfAssemblyContext, CtfAssemblyObservation,
};
use serde_json::{Value, json};

pub(super) fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value.is_infinite() {
        if value.is_sign_negative() {
            "negative_infinity"
        } else {
            "positive_infinity"
        }
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value":if value.is_finite() { Some(value) } else { None },
        "value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}

pub(super) fn input(source: &CtfLayerInput) -> Value {
    json!({"material_id":source.material_id.0,"Thickness":scalar(source.thickness),
        "Conductivity":scalar(source.conductivity),"Density":scalar(source.density),
        "SpecHeat":scalar(source.specific_heat),"Resistance":scalar(source.resistance),"ROnly":source.resistance_only})
}
fn checkpoint(source: Option<&CtfLayerCheckpoint>, errors: bool, reason: &str) -> Value {
    let Some(actual) = source else {
        return json!({"reached":false,"available":false,"snapshot":null,
            "unavailable_reason":reason,"counted_as_PASS":false});
    };
    json!({"reached":true,"available":true,"unavailable_reason":null,"snapshot":{
        "LayersInConstruct":actual.layers.len(),"NumResLayers":actual.num_res_layers,
        "ErrorsFound":errors,"DoCTFErrorReport":null,"DoCTFErrorReport_paired":false,
        "active_prefix_available":true,"copied_layer_count":actual.layers.len(),
        "active_prefix":actual.layers.iter().map(|v|json!({"dl":scalar(v.dl),"rk":scalar(v.rk),
            "rho":scalar(v.rho),"cp":scalar(v.cp),"lr":scalar(v.lr),"ResLayer":v.res_layer})).collect::<Vec<_>>(),
        "dyn":null,"rs":null,"cnd":null}})
}
pub(super) fn result(
    source: &std::result::Result<CtfLayerPreprocessing, CtfLayerScopeError>,
) -> Value {
    match source {
        Err(error) => {
            json!({"status":"scope-error-returned","actual_scope_error":format!("{error:?}"),
            "returned_preprocessing_owner":null,"observations":null,"counted_as_PASS":false,
            "selected_ErrorsFound_mutation_available":false,"Native_fatal_or_exception_inferred":false})
        }
        Ok(actual) => {
            let reason = if actual.skipped_unused {
                "actual API returned unused without a checkpoint"
            } else {
                "actual API returned before this checkpoint"
            };
            let mut converted = checkpoint(
                actual.after_conversion.as_ref().map(|v| &v.active),
                actual.errors_found,
                reason,
            );
            if let Some(value) = &actual.after_conversion {
                converted["snapshot"]["dyn"] = scalar(value.dyn_spacing);
                converted["snapshot"]["rs"] = scalar(value.total_resistance);
                converted["snapshot"]["cnd"] = scalar(value.conductance);
            }
            json!({"status":"source-returned","skipped_unused":actual.skipped_unused,
                "selected_ErrorsFound":actual.errors_found,"issues_in_actual_order":format!("{:?}",actual.issues),
                "observations":{"PostLoad":checkpoint(actual.after_load.as_ref(),actual.errors_found,reason),
                    "PostMerge":checkpoint(actual.after_merge.as_ref(),actual.errors_found,reason),
                    "PostConversion":converted,"unused_fixed_array_tail_emitted":false,
                    "normalized_layer_material_IDs_fabricated":false},
                "Native_whole_method_or_coefficients_observed":false,"numerical_PASS_claimed":false})
        }
    }
}

pub(super) fn initial(
    source: &ConstructionCtfInitialDiscretization,
    preprocessing: Option<&ConstructionCtfLayerPreprocessing>,
) -> Value {
    let phases = preprocessing.and_then(|owner| owner.result.as_ref().ok());
    let converted = phases.and_then(|owner| owner.after_conversion.as_ref());
    let context = json!({"is_used_ctf":source.context.is_used_ctf,
        "errors_found":source.context.errors_found,
        "source_sink_present":source.context.source_sink_present,
        "solution_dimensions":source.context.solution_dimensions});
    let (available, route, reverse_id, reason, snapshot) = match &source.result {
        Err(reason) => {
            let (route, reverse_id) = match reason {
                CtfInitialUnavailable::PreprocessingScope(_) => ("PreprocessingScope", None),
                CtfInitialUnavailable::UnusedConstruction => ("UnusedConstruction", None),
                CtfInitialUnavailable::PreprocessingErrorReturn => {
                    ("PreprocessingErrorReturn", None)
                }
                CtfInitialUnavailable::PostConversionUnavailable => {
                    ("PostConversionUnavailable", None)
                }
                CtfInitialUnavailable::AllResistiveBranch => ("AllResistiveBranch", None),
                CtfInitialUnavailable::ReverseConstruction { construction_id } => {
                    ("ReverseConstruction", Some(construction_id.0))
                }
                CtfInitialUnavailable::InitialScope(_) => ("InitialScope", None),
            };
            (
                false,
                route,
                reverse_id,
                Some(format!("{reason:?}")),
                Value::Null,
            )
        }
        Ok(actual) => (
            true,
            "InitialOwnerReturned",
            None,
            None,
            json!({
            "LayersInConstruct":converted.map(|owner|owner.active.layers.len()),
            "NumResLayers":converted.map(|owner|owner.active.num_res_layers),
            "ErrorsFound":phases.map(|owner|owner.errors_found),
            "DoCTFErrorReport":null,"DoCTFErrorReport_paired":false,
            "active_prefix_available":true,"copied_layer_count":actual.active.len(),
            "active_prefix":actual.active.iter().map(|layer|json!({"Nodes":layer.nodes,"dx":scalar(layer.dx)})).collect::<Vec<_>>(),
            "rcmax":actual.rcmax,"NodeSource":actual.node_source,"NodeUserTemp":actual.node_user_temp,
            "CTFTimeStep":scalar(actual.initial_ctf_time_step_hours),
            "NumHistories":actual.initial_num_histories,"TimeStepZone":scalar(actual.time_step_zone_hours)}),
        ),
    };
    json!({"protocol":"ctf03-initial-discretization.v1","owner_available":available,
        "selected_initial_operands":available,"actual_source_route":route,
        "actual_reverse_construction_id":reverse_id,"unavailable_reason":reason,"snapshot":snapshot,
        "construction_id":source.construction_id.0,"actual_context":context,
        "actual_wrapper_TimeStepZone":scalar(source.time_step_zone_hours),
        "same_preprocessing_owner_available":preprocessing.is_some(),
        "same_post_conversion_handoff_available":converted.is_some(),
        "same_preprocessing_construction_id":preprocessing.map(|owner|owner.construction_id.0),
        "initial_state_distinct_from_retry_and_whole_return":true,
        "observer_nodal_or_route_arithmetic_performed":false,
        "Native_whole_method_flags_or_callback_reachability_inferred":false,"counted_as_PASS":false})
}

pub(super) fn first_assembly(
    source: &ConstructionCtfFirstAssembly,
    initial: Option<&ConstructionCtfInitialDiscretization>,
    preprocessing: Option<&ConstructionCtfLayerPreprocessing>,
) -> Value {
    let phases = preprocessing.and_then(|owner| owner.result.as_ref().ok());
    let converted = phases.and_then(|owner| owner.after_conversion.as_ref());
    let (available, route, reason, snapshot) = match &source.result {
        Err(reason) => {
            let route = match reason {
                CtfFirstAssemblyUnavailable::InitialUnavailable(_) => "InitialUnavailable",
                CtfFirstAssemblyUnavailable::ConstructionIdentityMismatch => {
                    "ConstructionIdentityMismatch"
                }
                CtfFirstAssemblyUnavailable::PreprocessingScope(_) => "PreprocessingScope",
                CtfFirstAssemblyUnavailable::PreprocessingRouteMismatch => {
                    "PreprocessingRouteMismatch"
                }
                CtfFirstAssemblyUnavailable::PostConversionUnavailable => {
                    "PostConversionUnavailable"
                }
                CtfFirstAssemblyUnavailable::CallerTimeStepMismatch => "CallerTimeStepMismatch",
                CtfFirstAssemblyUnavailable::AssemblyScope(_) => "AssemblyScope",
            };
            (false, route, Some(format!("{reason:?}")), Value::Null)
        }
        Ok(CtfAssemblyObservation::Unavailable(reason)) => (
            false,
            "AssemblyUnavailable",
            Some(format!("{reason:?}")),
            Value::Null,
        ),
        Ok(CtfAssemblyObservation::Assembled(actual)) => {
            let values = |source: &[f64]| source.iter().copied().map(scalar).collect::<Vec<_>>();
            (
                true,
                "FirstAssemblyReturned",
                None,
                json!({
                "LayersInConstruct":actual.layers.len(),
                "NumResLayers":converted.map(|owner|owner.active.num_res_layers),
                "ErrorsFound":phases.map(|owner|owner.errors_found),
                "DoCTFErrorReport":null,"DoCTFErrorReport_paired":false,
                "IsUsedCTF":initial.map(|owner|owner.context.is_used_ctf),
                "SourceSinkPresent":actual.context.source_sink_present,
                "SolutionDimensions":actual.context.solution_dimensions,
                "active_prefix_available":true,"copied_layer_count":actual.layers.len(),
                "active_prefix":actual.layers.iter().enumerate().map(|(index,layer)|json!({
                    "dl":scalar(layer.dl),"rk":scalar(layer.rk),"rho":scalar(layer.rho),
                    "cp":scalar(layer.cp),"lr":scalar(layer.lr),"ResLayer":layer.res_layer,
                    "Nodes":actual.nodes.get(index).copied(),
                    "dx":actual.dx.get(index).copied().map(scalar)})).collect::<Vec<_>>(),
                "actual_nodes_count":actual.nodes.len(),"actual_dx_count":actual.dx.len(),
                "rcmax":actual.rcmax,"NodeSource":actual.context.node_source,
                "NodeUserTemp":actual.context.node_user_temp,
                "CTFTimeStep":scalar(actual.context.ctf_time_step),
                "NumHistories":actual.context.num_histories,
                "TimeStepZone":scalar(actual.context.time_step_zone),
                "attempt_ordinal":actual.context.attempt_ordinal,
                "actual_context_construction_id":actual.context.construction_id.0,
                "actual_route":format!("{:?}",actual.context.route),
                "matrices_available":true,"matrices":{
                    "AMat":values(&actual.a_mat),"IdenMatrix":values(&actual.iden_matrix),
                    "BMat":values(&actual.b_mat),"CMat":values(&actual.c_mat),"DMat":values(&actual.d_mat)},
                "storage_order":"source_i1_outer_i2_inner_row_first",
                "Native_Objexx_bounds_not_Rust_owner":true}),
            )
        }
    };
    json!({"protocol":"ctf04-matrix-assembly.v1","owner_available":available,
        "selected_first_assembly_operands":available,"actual_source_route":route,
        "unavailable_reason":reason,"snapshot":snapshot,"construction_id":source.construction_id.0,
        "same_initial_owner_available":initial.is_some(),
        "same_initial_construction_id":initial.map(|owner|owner.construction_id.0),
        "same_preprocessing_owner_available":preprocessing.is_some(),
        "same_preprocessing_construction_id":preprocessing.map(|owner|owner.construction_id.0),
        "same_post_conversion_handoff_available":converted.is_some(),
        "only_first_Rust_assembly_invocation_owned":true,
        "later_Native_attempts_available_in_Rust":false,"later_Native_attempts_counted_as_PASS":false,
        "retry_driver_available":false,"Native_retry_state_or_counts_supplied_to_Rust":false,
        "observer_matrix_identity_or_route_arithmetic_performed":false,
        "Native_whole_method_outcome_or_flags_inferred":false,"counted_as_PASS":false})
}

fn matrix_values(source: &[f64]) -> Vec<Value> {
    source.iter().copied().map(scalar).collect()
}
fn attempt_context(source: &CtfAssemblyContext) -> Value {
    json!({"construction_id":source.construction_id.0,"route":format!("{:?}",source.route),
        "SourceSinkPresent":source.source_sink_present,"SolutionDimensions":source.solution_dimensions,
        "NodeSource":source.node_source,"NodeUserTemp":source.node_user_temp,
        "attempt_ordinal":source.attempt_ordinal,"TimeStepZone":scalar(source.time_step_zone),
        "CTFTimeStep":scalar(source.ctf_time_step),"NumHistories":source.num_histories})
}
pub(super) fn first_wrapper_uninvoked(api: &str, construction_id: u32) -> Value {
    json!({"protocol":"ctf09-uninvoked-first-wrapper-context.v1",
        "construction_id":construction_id,"actual_public_API":api,
        "actual_public_API_invoked":false,"owner_available":false,"snapshot":null,
        "unavailable_reason":"retry driver invokes the actual pure methods; this first-only public wrapper was not invoked",
        "actual_first_method_handoff_location":"CTF09_retry_initialization.driver.first_method_handoff",
        "historical_first_only_policy_reinterpreted":false,"counted_as_PASS":false})
}
pub(super) fn retry_initialization(source: &ConstructionCtfRetryInitialization) -> Value {
    json!({"protocol":"ctf09-actual-rust-retry-initialization.v1",
        "construction_id":source.construction_id.0,"construction_name":source.construction_name,
        "original_tot_layers":source.original_tot_layers,
        "original_layer_material_names":source.original_layer_material_names,
        "flags_on_entry":flags(source.flags_on_entry),"flags_on_return":flags(source.flags_on_return),
        "actual_initialization_API":"initialize_construction_ctf_retry",
        "actual_initialization_API_invoked":source.result.is_ok(),
        "actual_skip_API":"skip_construction_ctf_retry","actual_skip_API_invoked":source.result.is_err(),
        "actual_retry_driver_invoked":source.result.is_ok(),
        "actual_unavailable_reason":source.result.as_ref().err().map(|reason|format!("{reason:?}")),
        "selected_chain_stop":source.selected_chain_stop.as_ref().map(|stop|format!("{stop:?}")),
        "driver":source.result.as_ref().ok().map(retry_driver),
        "Native_global_fatal_or_OS_status_inferred":false,"counted_as_PASS":false})
}
fn method(api: &str, invoked: bool, route: &str, reason: Option<String>, snapshot: Value) -> Value {
    json!({"actual_Rust_API":api,"actual_source_invoked":invoked,
        "status":if !invoked { "not-invoked" } else if route == "ScopeError" { "scope-error-returned" } else { "source-returned" },
        "owner_available":!snapshot.is_null(),"actual_source_route":route,
        "unavailable_reason":reason,"snapshot":snapshot,
        "Native_stage_or_return_marker_inferred":false,"counted_as_PASS":false})
}
fn uninvoked(api: &str) -> Value {
    method(
        api,
        false,
        "NotInvoked",
        Some("actual preceding Rust stop; no method Result owner".to_owned()),
        Value::Null,
    )
}
fn exponential(source: &Result<CtfExponentialObservation, CtfExponentialScopeError>) -> Value {
    let api = "calculate_selected_ctf_matrix_exponential";
    match source {
        Err(error) => method(
            api,
            true,
            "ScopeError",
            Some(format!("{error:?}")),
            Value::Null,
        ),
        Ok(CtfExponentialObservation::Unavailable(reason)) => method(
            api,
            true,
            "Unavailable",
            Some(format!("{reason:?}")),
            Value::Null,
        ),
        Ok(CtfExponentialObservation::Exponential(actual)) => method(
            api,
            true,
            "Exponential",
            None,
            json!({"context":attempt_context(&actual.context),"rcmax":actual.rcmax,
                "AMatRowNormMax":scalar(actual.a_mat_row_norm_max),"k":actual.k,
                "fact":scalar(actual.fact),"CheckVal":scalar(actual.check_val),"l":actual.l,
                "final_i":actual.final_power_i,
                "PowerControl":actual.power_steps.iter().map(|event|json!({"i":event.i,"SigFigLimit":event.sig_fig_limit})).collect::<Vec<_>>(),
                "SquareControl":actual.square_steps.iter().map(|event|json!({"isq":event.isq,"Backup":event.backup})).collect::<Vec<_>>(),
                "actual_power_control_count":actual.power_steps.len(),"actual_square_control_count":actual.square_steps.len(),
                "matrices":{"AExp":matrix_values(&actual.a_exp),"AMat1":matrix_values(&actual.a_mat1),
                    "AMato":matrix_values(&actual.a_mato),"AMatN":matrix_values(&actual.a_mat_n)},
                "actual_matrix_value_counts":{"AExp":actual.a_exp.len(),"AMat1":actual.a_mat1.len(),
                    "AMato":actual.a_mato.len(),"AMatN":actual.a_mat_n.len()},
                "storage_order":"source_i1_outer_i2_inner_row_first"}),
        ),
    }
}
fn inverse(
    source: Option<&Result<CtfInverseMatrix, CtfInverseScopeError>>,
    context: &CtfAssemblyContext,
) -> Value {
    let api = "invert_selected_ctf_matrix";
    match source {
        None => uninvoked(api),
        Some(Err(error)) => method(
            api,
            true,
            "ScopeError",
            Some(format!("{error:?}")),
            Value::Null,
        ),
        Some(Ok(actual)) => method(
            api,
            true,
            "Inverse",
            None,
            json!({"actual_call_context":attempt_context(context),"rcmax":actual.rcmax,
                "matrices":{"AInv":matrix_values(&actual.a_inv),"AMat1":matrix_values(&actual.a_mat1)},
                "actual_matrix_value_counts":{"AInv":actual.a_inv.len(),"AMat1":actual.a_mat1.len()},
                "inverse_input_is_original_AMat_and_IdenMatrix":true,
                "storage_order":"source_i1_outer_i2_inner_row_first"}),
        ),
    }
}
fn gammas(source: Option<&Result<CtfGammaObservation, CtfGammaScopeError>>) -> Value {
    let api = "calculate_selected_ctf_gammas";
    match source {
        None => uninvoked(api),
        Some(Err(error)) => method(
            api,
            true,
            "ScopeError",
            Some(format!("{error:?}")),
            Value::Null,
        ),
        Some(Ok(CtfGammaObservation::Unavailable(reason))) => method(
            api,
            true,
            "Unavailable",
            Some(format!("{reason:?}")),
            Value::Null,
        ),
        Some(Ok(CtfGammaObservation::Gamma(actual))) => method(
            api,
            true,
            "Gamma",
            None,
            json!({"context":attempt_context(&actual.context),"rcmax":actual.rcmax,
                "matrices":{"ATemp":matrix_values(&actual.a_temp),
                    "Gamma1":matrix_values(&actual.gamma1),"Gamma2":matrix_values(&actual.gamma2)},
                "actual_matrix_value_counts":{"ATemp":actual.a_temp.len(),
                    "Gamma1":actual.gamma1.len(),"Gamma2":actual.gamma2.len()},
                "actual_matrix_shapes":{"ATemp":[actual.rcmax,actual.rcmax],
                    "Gamma1":[3,actual.rcmax],"Gamma2":[3,actual.rcmax]},
                "ATemp_storage_order":"source_i1_outer_i2_inner_row_first",
                "Gamma_storage_order":"source_j_outer_i_inner_row_first"}),
        ),
    }
}
fn coefficients(
    source: Option<&Result<CtfFinalCoefficientObservation, CtfFinalCoefficientScopeError>>,
) -> Value {
    let api = "calculate_selected_ctf_final_coefficients";
    match source {
        None => uninvoked(api),
        Some(Err(error)) => method(
            api,
            true,
            "ScopeError",
            Some(format!("{error:?}")),
            Value::Null,
        ),
        Some(Ok(CtfFinalCoefficientObservation::Unavailable(reason))) => method(
            api,
            true,
            "Unavailable",
            Some(format!("{reason:?}")),
            Value::Null,
        ),
        Some(Ok(CtfFinalCoefficientObservation::Coefficients(actual))) => method(
            api,
            true,
            "Coefficients",
            None,
            json!({"context":attempt_context(&actual.context),"rcmax":actual.rcmax,
                "NumCTFTerms":actual.num_ctf_terms,
                "controls":{"inum":actual.next_history_term,"CTFConvrg":actual.history_loop_converged},
                "matrices":{"s0":matrix_values(&actual.s0),"s":matrix_values(&actual.s),
                    "e":matrix_values(&actual.e),"Gamma1":matrix_values(&actual.gamma1_minus_gamma2),
                    "PhiR0":matrix_values(&actual.phi_r0),"Rnew":matrix_values(&actual.rnew),
                    "Rold":matrix_values(&actual.rold)},
                "actual_matrix_value_counts":{"s0":actual.s0.len(),"s":actual.s.len(),"e":actual.e.len(),
                    "Gamma1":actual.gamma1_minus_gamma2.len(),"PhiR0":actual.phi_r0.len(),
                    "Rnew":actual.rnew.len(),"Rold":actual.rold.len()},
                "actual_matrix_shapes":{"s0":[3,4],"s":[3,4,actual.rcmax],"e":[actual.rcmax],
                    "Gamma1":[3,actual.rcmax],"PhiR0":[actual.rcmax,actual.rcmax],
                    "Rnew":[actual.rcmax,actual.rcmax],"Rold":[actual.rcmax,actual.rcmax]},
                "square_storage_order":"source_i1_outer_i2_inner_row_first",
                "Gamma_storage_order":"source_j_outer_i_inner_row_first",
                "s0_storage_order":"source_j_outer_k_inner",
                "s_storage_order":"source_j_outer_k_middle_term_inner",
                "private_method_final_owner_only":true,"Native_SourceReturn_workspace_inferred":false}),
        ),
    }
}
pub(super) fn flags(source: CtfCallerSharedFlags) -> Value {
    json!({"ErrorsFound":source.errors_found,"DoCTFErrorReport":source.do_ctf_error_report})
}
fn caller(
    source: Option<&Result<CtfCallerStabilityDecision, CtfCallerStabilityScopeError>>,
) -> Value {
    let api = "decide_selected_ctf_caller_stability";
    match source {
        None => uninvoked(api),
        Some(Err(error)) => method(
            api,
            true,
            "ScopeError",
            Some(format!("{error:?}")),
            Value::Null,
        ),
        Some(Ok(actual)) => method(
            api,
            true,
            "CallerDecision",
            None,
            json!({"input_context":attempt_context(&actual.input_context),
                "postcheck_context":attempt_context(&actual.postcheck_context),
                "flags_before":flags(actual.flags_before),"flags_after":flags(actual.flags_after),
                "NumCTFTerms":actual.num_ctf_terms,"caller_CTFConvrg":actual.caller_ctf_converged,
                "retry_reason":actual.retry_reason.map(|reason|format!("{reason:?}")),
                "outcome":format!("{:?}",actual.outcome),
                "assigned_sums":actual.sums.map(|sums|json!({"assignedAbsSumXi":scalar(sums.sum_xi),
                    "assignedAbsSumYi":scalar(sums.sum_yi),"assignedAbsSumZi":scalar(sums.sum_zi),
                    "BiggestSum":scalar(sums.biggest_sum)})),
                "unpaired_Rust_expression_bookkeeping":{"first_relative_error":actual.first_relative_error.map(scalar),
                    "second_relative_error":actual.second_relative_error.map(scalar),
                    "Native_ratio_local_owner_available":false,"counted_as_PASS":false},
                "diagnostic_call_intents":actual.diagnostics.iter().map(|record|json!({
                    "level":format!("{:?}",record.level),"message":record.message})).collect::<Vec<_>>(),
                "Native_global_diagnostics_or_fatal_backend_invoked":false,
                "diagnostic_intents_equated_to_Native_global_messages":false,
                "terminal_decision_is_whole_method_return_or_SI_store_safety":false}),
        ),
    }
}
fn assembly_values(
    context: &CtfAssemblyContext,
    rcmax: i32,
    a_mat: &[f64],
    iden_matrix: &[f64],
    b_mat: &[f64],
    c_mat: &[f64],
    d_mat: &[f64],
) -> Value {
    json!({"context":attempt_context(context),"rcmax":rcmax,
        "matrices":{"AMat":matrix_values(a_mat),"IdenMatrix":matrix_values(iden_matrix),
            "BMat":matrix_values(b_mat),"CMat":matrix_values(c_mat),"DMat":matrix_values(d_mat)},
        "actual_matrix_value_counts":{"AMat":a_mat.len(),"IdenMatrix":iden_matrix.len(),
            "BMat":b_mat.len(),"CMat":c_mat.len(),"DMat":d_mat.len()},
        "storage_order":"source_i1_outer_i2_inner_row_first"})
}
fn assembly_snapshot(source: &CtfRetryAssemblySnapshot) -> Value {
    assembly_values(
        &source.context,
        source.rcmax,
        &source.a_mat,
        &source.iden_matrix,
        &source.b_mat,
        &source.c_mat,
        &source.d_mat,
    )
}
fn attempt(source: &CtfRetryAttempt) -> Value {
    json!({"attempt_ordinal":source.assembly.context.attempt_ordinal,
        "assembly":assembly_snapshot(&source.assembly),
        "Exponential":exponential(&source.exponential),
        "Inverse":inverse(source.inverse.as_ref(),&source.assembly.context),
        "Gammas":gammas(source.gammas.as_ref()),
        "FinalCoefficients":coefficients(source.final_coefficients.as_ref()),
        "caller":caller(source.caller.as_ref()),
        "Native_callback_events_or_return_markers_reconstructed":false,"counted_as_PASS":false})
}
pub(super) fn retry_driver(source: &ConstructionCtfRetryDriver) -> Value {
    let initial_checkpoint = source.initial_pre_assignment().map(|actual|json!({
        "context":attempt_context(&actual.context),"rcmax":actual.rcmax,
        "pre_AMat":matrix_values(&actual.a_mat),"pre_IdenMatrix":matrix_values(&actual.iden_matrix),
        "BMat3_after_actual_reset":scalar(actual.b_mat3_after_reset),
        "actual_matrix_value_counts":{"pre_AMat":actual.a_mat.len(),"pre_IdenMatrix":actual.iden_matrix.len()},
        "captured_by_actual_initial_assembly_producer":true,
        "NumCTFTerms_and_shared_flags_not_owned_by_checkpoint":true}));
    let terminal = source.retained_assembly.as_ref().map(|actual|json!({
        "assembly":assembly_values(&actual.context,actual.rcmax,&actual.a_mat,&actual.iden_matrix,
            &actual.b_mat,&actual.c_mat,&actual.d_mat),
        "active_prefix":actual.layers.iter().enumerate().map(|(index,layer)|json!({
            "dl":scalar(layer.dl),"rk":scalar(layer.rk),"rho":scalar(layer.rho),"cp":scalar(layer.cp),
            "lr":scalar(layer.lr),"ResLayer":layer.res_layer,"Nodes":actual.nodes.get(index).copied(),
            "dx":actual.dx.get(index).copied().map(scalar)})).collect::<Vec<_>>(),
        "actual_nodes_count":actual.nodes.len(),"actual_dx_count":actual.dx.len(),
        "last_method_stamp_distinct_from_postcheck_stamp":true}));
    json!({"protocol":"ctf09-actual-rust-retry-owner.v1","construction_id":source.construction_id.0,
        "actual_Rust_API":"run_selected_construction_ctf_retry_driver","actual_source_invoked":true,
        "flags_on_entry":flags(source.flags_on_entry),"flags_on_return":flags(source.flags_on_return),
        "initial_context":source.initial_context.as_ref().map(attempt_context),
        "postcheck_context":source.postcheck_context.as_ref().map(attempt_context),
        "outcome":format!("{:?}",source.outcome),
        "initial_pre_assignment_owner_available":initial_checkpoint.is_some(),
        "initial_pre_assignment":initial_checkpoint,
        "genuine_initial_checkpoint_required_for_selected_first_preband_pairing":true,
        "first_method_handoff":source.attempts.first().map(attempt),
        "attempts":source.attempts.iter().map(attempt).collect::<Vec<_>>(),
        "reassignments":source.reassignments.iter().map(|record|json!({
            "actual_Rust_API":"reassign_1d_ctf_state_space","actual_source_invoked":true,
            "before_reassignment_call":assembly_snapshot(&record.before),
            "preceding_NumCTFTerms":record.preceding_num_ctf_terms,
            "supplied_context":attempt_context(&record.supplied_context),
            "actual_result":format!("{:?}",record.result),
            "actual_returned_ok":record.result.is_ok(),"context_after":attempt_context(&record.context_after),
            "before_call_BMat3_not_claimed_as_after_reset_checkpoint":true,
            "Native_pre_assignment_event_or_return_marker_inferred":false})).collect::<Vec<_>>(),
        "actual_attempt_count":source.attempts.len(),"actual_reassignment_count":source.reassignments.len(),
        "actual_method_invocation_counts":{"Exponential":source.attempts.len(),
            "Inverse":source.attempts.iter().filter(|attempt|attempt.inverse.is_some()).count(),
            "Gammas":source.attempts.iter().filter(|attempt|attempt.gammas.is_some()).count(),
            "FinalCoefficients":source.attempts.iter().filter(|attempt|attempt.final_coefficients.is_some()).count(),
            "CallerStability":source.attempts.iter().filter(|attempt|attempt.caller.is_some()).count()},
        "retained_original_assembly":terminal,
        "original_first_assembly_consumed_once_without_clone_or_replay":true,
        "old_first_public_wrappers_invoked":false,
        "all_actual_Rust_visits_retained":true,"record_cap":null,
        "Native_counts_operands_flags_or_callback_events_supplied_to_Rust":false,
        "observer_matrix_or_control_arithmetic_performed":false,
        "global_diagnostics_whole_Native_return_or_SI19_storage_certified":false,
        "scientific_comparison_performed":false,"counted_as_PASS":false})
}

pub(super) const OBSERVER_PATHS: &[&str] = &[
    "crates/ep_run/examples/ctf10_candidate_unit_observer.rs",
    "crates/ep_run/examples/ctf10_candidate_unit_support/fields.rs",
];
// Exact path inventory; approved formatted SHAs are runtime operands and must
// also match Root's actual Cargo source capture before results are admitted.
pub(super) const API_PATHS: &[&str] = &[
    "crates/ep_run/Cargo.toml",
    "crates/ep_model/src/lib.rs",
    "crates/ep_model/src/ids.rs",
    "crates/ep_runtime/Cargo.toml",
    "crates/ep_runtime/src/lib.rs",
    "crates/ep_runtime/src/heat_balance/mod.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_layer_preprocessing.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_initial_discretization.rs",
    "crates/ep_runtime/src/heat_balance/ctf_initial_owner.rs",
    "crates/ep_model/src/objects/construction.rs",
    "crates/ep_runtime/examples/clk02_probe_support/digest.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_state_space_assembly.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_assembly_owner.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_exponential_matrix.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_exponential_owner.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_inverse_matrix.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_inverse_owner.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_matrix_owner.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_gamma_matrix.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_gamma_owner.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_gamma_ingress.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_final_coefficients.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_final_coefficients_owner.rs",
    "crates/ep_runtime/src/heat_balance/ctf_first_final_coefficients_ingress.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_caller_stability.rs",
    "crates/ep_runtime/src/heat_balance/ctf_retry_driver.rs",
    "crates/ep_runtime/src/heat_balance/ctf_retry_initialization.rs",
    "crates/ep_runtime/src/heat_balance/ctf_public_initialization.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_public_storage.rs",
    "crates/ep_runtime/src/heat_balance/surface_manager/ctf_all_resistive.rs",
];

// Copy accepted CTF10 owners only. These are not Native phase/return events.
pub(super) fn public_storage(
    source: &ep_runtime::heat_balance::surface_manager::ctf_public_storage::CtfPublicCoefficientStorage,
) -> Value {
    let values = |source: &[f64]| source.iter().copied().map(scalar).collect::<Vec<_>>();
    json!({"CTFOutside":values(&source.outside),"CTFCross":values(&source.cross),
        "CTFInside":values(&source.inside),"CTFFlux":values(&source.flux),
        "CTFSourceIn":values(&source.source_in),"CTFSourceOut":values(&source.source_out),
        "CTFTSourceOut":values(&source.temperature_source_out),"CTFTSourceIn":values(&source.temperature_source_in),
        "CTFTSourceQ":values(&source.temperature_source_q),"CTFTUserOut":values(&source.temperature_user_out),
        "CTFTUserIn":values(&source.temperature_user_in),"CTFTUserSource":values(&source.temperature_user_source),
        "CTFTimeStep":scalar(source.ctf_time_step),"UValue":scalar(source.u_value),
        "NumHistories":source.num_histories,"NumCTFTerms":source.num_ctf_terms,
        "fixed_array_count":12,"actual_slots_per_array":source.outside.len(),
        "coefficients_recomputed_by_observer":false,"counted_as_PASS":false})
}
pub(super) fn public_unavailable(reason: &str) -> Value {
    json!({"owner_available":false,"actual_public_initializer_invoked":false,"snapshot":null,
        "unavailable_reason":reason,"Native_return_or_reset_inferred":false,"counted_as_PASS":false})
}
pub(super) fn public_construction(
    source: &ep_runtime::heat_balance::ctf_public_initialization::ConstructionCtfPublicInitialization,
) -> Value {
    let (available, returned, error) = match &source.result {
        Ok(actual) => (
            true,
            json!({"actual_Rust_route":format!("{:?}",actual.route),"public_storage":public_storage(&actual.storage)}),
            Value::Null,
        ),
        Err(reason) => (
            false,
            Value::Null,
            json!({"actual_Rust_unavailable":format!("{reason:?}")}),
        ),
    };
    json!({"construction_id":source.construction_id.0,"declared_index":source.declared_index,
        "original_material_ids":source.original_material_ids.iter().map(|id|id.0).collect::<Vec<_>>(),
        "actual_Rust_is_used_ctf":source.is_used_ctf,"actual_public_initializer_invoked":source.public_initializer_invoked,
        "actual_retry_flags_on_entry":source.retry_flags_on_entry.map(flags),
        "actual_retry_flags_on_return":source.retry_flags_on_return.map(flags),
        "owner_available":available,"snapshot":returned,"unavailable":error,
        "Native_whole_call_outcome_or_public_checkpoint_inferred":false,"counted_as_PASS":false})
}
pub(super) fn public_initialization(
    source: &ep_runtime::heat_balance::ctf_public_initialization::OrderedCtfPublicInitialization,
) -> Value {
    json!({"protocol":"ctf10-selected-Rust-public-initialization.v1","owner_available":true,
        "constructions":source.constructions.iter().map(public_construction).collect::<Vec<_>>(),
        "actual_owner_count":source.constructions.len(),
        "actual_public_initializer_invocations":source.constructions.iter().filter(|row|row.public_initializer_invoked).count(),
        "aggregate_after_returned_calls":{"SimpleCTFOnly":source.aggregate_after_returned_calls.simple_ctf_only,
            "MaxCTFTerms":source.aggregate_after_returned_calls.max_ctf_terms,
            "returned_constructions":source.aggregate_after_returned_calls.returned_constructions},
        "selected_stop":source.selected_stop.as_ref().map(|reason|format!("{reason:?}")),
        "selected_declared_loop_completed":source.selected_declared_loop_completed,
        "actual_AnyInternalHeatSource_postloop_invoked":false,"actual_ScanForReports_invoked":false,
        "actual_reportLayers_or_reportTransferFunction_invoked":false,"actual_final_fatal_backend_invoked":false,
        "Native_true_whole_Init_outcome_or_event_ledger_inferred":false,
        "surface_public_coefficient_activation_certified":false,"counted_as_PASS":false})
}
