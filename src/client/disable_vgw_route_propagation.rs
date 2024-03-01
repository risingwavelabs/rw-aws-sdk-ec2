// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableVgwRoutePropagation`](crate::operation::disable_vgw_route_propagation::builders::DisableVgwRoutePropagationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`gateway_id(impl Into<String>)`](crate::operation::disable_vgw_route_propagation::builders::DisableVgwRoutePropagationFluentBuilder::gateway_id) / [`set_gateway_id(Option<String>)`](crate::operation::disable_vgw_route_propagation::builders::DisableVgwRoutePropagationFluentBuilder::set_gateway_id):<br>required: **true**<br><p>The ID of the virtual private gateway.</p><br>
    ///   - [`route_table_id(impl Into<String>)`](crate::operation::disable_vgw_route_propagation::builders::DisableVgwRoutePropagationFluentBuilder::route_table_id) / [`set_route_table_id(Option<String>)`](crate::operation::disable_vgw_route_propagation::builders::DisableVgwRoutePropagationFluentBuilder::set_route_table_id):<br>required: **true**<br><p>The ID of the route table.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::disable_vgw_route_propagation::builders::DisableVgwRoutePropagationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::disable_vgw_route_propagation::builders::DisableVgwRoutePropagationFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DisableVgwRoutePropagationOutput`](crate::operation::disable_vgw_route_propagation::DisableVgwRoutePropagationOutput)
    /// - On failure, responds with [`SdkError<DisableVgwRoutePropagationError>`](crate::operation::disable_vgw_route_propagation::DisableVgwRoutePropagationError)
    pub fn disable_vgw_route_propagation(
        &self,
    ) -> crate::operation::disable_vgw_route_propagation::builders::DisableVgwRoutePropagationFluentBuilder {
        crate::operation::disable_vgw_route_propagation::builders::DisableVgwRoutePropagationFluentBuilder::new(self.handle.clone())
    }
}