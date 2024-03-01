// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTransitGatewayConnect`](crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transport_transit_gateway_attachment_id(impl Into<String>)`](crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder::transport_transit_gateway_attachment_id) / [`set_transport_transit_gateway_attachment_id(Option<String>)`](crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder::set_transport_transit_gateway_attachment_id):<br>required: **true**<br><p>The ID of the transit gateway attachment. You can specify a VPC attachment or Amazon Web Services Direct Connect attachment.</p><br>
    ///   - [`options(CreateTransitGatewayConnectRequestOptions)`](crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder::options) / [`set_options(Option<CreateTransitGatewayConnectRequestOptions>)`](crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder::set_options):<br>required: **true**<br><p>The Connect attachment options.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to apply to the Connect attachment.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`CreateTransitGatewayConnectOutput`](crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectOutput) with field(s):
    ///   - [`transit_gateway_connect(Option<TransitGatewayConnect>)`](crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectOutput::transit_gateway_connect): <p>Information about the Connect attachment.</p>
    /// - On failure, responds with [`SdkError<CreateTransitGatewayConnectError>`](crate::operation::create_transit_gateway_connect::CreateTransitGatewayConnectError)
    pub fn create_transit_gateway_connect(
        &self,
    ) -> crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder {
        crate::operation::create_transit_gateway_connect::builders::CreateTransitGatewayConnectFluentBuilder::new(self.handle.clone())
    }
}
