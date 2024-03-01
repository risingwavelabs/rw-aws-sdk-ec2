// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifySpotFleetRequest`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`excess_capacity_termination_policy(ExcessCapacityTerminationPolicy)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::excess_capacity_termination_policy) / [`set_excess_capacity_termination_policy(Option<ExcessCapacityTerminationPolicy>)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::set_excess_capacity_termination_policy):<br>required: **false**<br><p>Indicates whether running instances should be terminated if the target capacity of the Spot Fleet request is decreased below the current size of the Spot Fleet.</p>  <p>Supported only for fleets of type <code>maintain</code>.</p><br>
    ///   - [`launch_template_configs(LaunchTemplateConfig)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::launch_template_configs) / [`set_launch_template_configs(Option<Vec::<LaunchTemplateConfig>>)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::set_launch_template_configs):<br>required: **false**<br><p>The launch template and overrides. You can only use this parameter if you specified a launch template (<code>LaunchTemplateConfigs</code>) in your Spot Fleet request. If you specified <code>LaunchSpecifications</code> in your Spot Fleet request, then omit this parameter.</p><br>
    ///   - [`spot_fleet_request_id(impl Into<String>)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::spot_fleet_request_id) / [`set_spot_fleet_request_id(Option<String>)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::set_spot_fleet_request_id):<br>required: **true**<br><p>The ID of the Spot Fleet request.</p><br>
    ///   - [`target_capacity(i32)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::target_capacity) / [`set_target_capacity(Option<i32>)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::set_target_capacity):<br>required: **false**<br><p>The size of the fleet.</p><br>
    ///   - [`on_demand_target_capacity(i32)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::on_demand_target_capacity) / [`set_on_demand_target_capacity(Option<i32>)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::set_on_demand_target_capacity):<br>required: **false**<br><p>The number of On-Demand Instances in the fleet.</p><br>
    ///   - [`context(impl Into<String>)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::context) / [`set_context(Option<String>)`](crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::set_context):<br>required: **false**<br><p>Reserved.</p><br>
    /// - On success, responds with [`ModifySpotFleetRequestOutput`](crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestOutput::return): <p>If the request succeeds, the response returns <code>true</code>. If the request fails, no response is returned, and instead an error message is returned.</p>
    /// - On failure, responds with [`SdkError<ModifySpotFleetRequestError>`](crate::operation::modify_spot_fleet_request::ModifySpotFleetRequestError)
    pub fn modify_spot_fleet_request(&self) -> crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder {
        crate::operation::modify_spot_fleet_request::builders::ModifySpotFleetRequestFluentBuilder::new(self.handle.clone())
    }
}