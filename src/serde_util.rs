// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn spot_fleet_request_config_data_correct_errors(
    mut builder: crate::types::builders::SpotFleetRequestConfigDataBuilder,
) -> crate::types::builders::SpotFleetRequestConfigDataBuilder {
    if builder.iam_fleet_role.is_none() {
        builder.iam_fleet_role = Some(Default::default())
    }
    if builder.target_capacity.is_none() {
        builder.target_capacity = Some(Default::default())
    }
    builder
}

pub(crate) fn run_instances_monitoring_enabled_correct_errors(
    mut builder: crate::types::builders::RunInstancesMonitoringEnabledBuilder,
) -> crate::types::builders::RunInstancesMonitoringEnabledBuilder {
    if builder.enabled.is_none() {
        builder.enabled = Some(Default::default())
    }
    builder
}
