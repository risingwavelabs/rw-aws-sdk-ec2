// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyTransitGateway`](crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_id(impl Into<String>)`](crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder::transit_gateway_id) / [`set_transit_gateway_id(Option<String>)`](crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder::set_transit_gateway_id):<br>required: **true**<br><p>The ID of the transit gateway.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder::set_description):<br>required: **false**<br><p>The description for the transit gateway.</p><br>
    ///   - [`options(ModifyTransitGatewayOptions)`](crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder::options) / [`set_options(Option<ModifyTransitGatewayOptions>)`](crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder::set_options):<br>required: **false**<br><p>The options to modify.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`ModifyTransitGatewayOutput`](crate::operation::modify_transit_gateway::ModifyTransitGatewayOutput) with field(s):
    ///   - [`transit_gateway(Option<TransitGateway>)`](crate::operation::modify_transit_gateway::ModifyTransitGatewayOutput::transit_gateway): <p>Information about the transit gateway.</p>
    /// - On failure, responds with [`SdkError<ModifyTransitGatewayError>`](crate::operation::modify_transit_gateway::ModifyTransitGatewayError)
    pub fn modify_transit_gateway(&self) -> crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder {
        crate::operation::modify_transit_gateway::builders::ModifyTransitGatewayFluentBuilder::new(self.handle.clone())
    }
}
