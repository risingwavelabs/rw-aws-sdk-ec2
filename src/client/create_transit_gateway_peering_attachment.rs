// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTransitGatewayPeeringAttachment`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_id(impl Into<String>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::transit_gateway_id) / [`set_transit_gateway_id(Option<String>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::set_transit_gateway_id):<br>required: **true**<br><p>The ID of the transit gateway.</p><br>
    ///   - [`peer_transit_gateway_id(impl Into<String>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::peer_transit_gateway_id) / [`set_peer_transit_gateway_id(Option<String>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::set_peer_transit_gateway_id):<br>required: **true**<br><p>The ID of the peer transit gateway with which to create the peering attachment.</p><br>
    ///   - [`peer_account_id(impl Into<String>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::peer_account_id) / [`set_peer_account_id(Option<String>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::set_peer_account_id):<br>required: **true**<br><p>The ID of the Amazon Web Services account that owns the peer transit gateway.</p><br>
    ///   - [`peer_region(impl Into<String>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::peer_region) / [`set_peer_region(Option<String>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::set_peer_region):<br>required: **true**<br><p>The Region where the peer transit gateway is located.</p><br>
    ///   - [`options(CreateTransitGatewayPeeringAttachmentRequestOptions)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::options) / [`set_options(Option<CreateTransitGatewayPeeringAttachmentRequestOptions>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::set_options):<br>required: **false**<br><p>Requests a transit gateway peering attachment.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to apply to the transit gateway peering attachment.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`CreateTransitGatewayPeeringAttachmentOutput`](crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentOutput) with field(s):
    ///   - [`transit_gateway_peering_attachment(Option<TransitGatewayPeeringAttachment>)`](crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentOutput::transit_gateway_peering_attachment): <p>The transit gateway peering attachment.</p>
    /// - On failure, responds with [`SdkError<CreateTransitGatewayPeeringAttachmentError>`](crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentError)
    pub fn create_transit_gateway_peering_attachment(
        &self,
    ) -> crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder {
        crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
