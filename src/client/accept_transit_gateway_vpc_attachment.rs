// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AcceptTransitGatewayVpcAttachment`](crate::operation::accept_transit_gateway_vpc_attachment::builders::AcceptTransitGatewayVpcAttachmentFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::operation::accept_transit_gateway_vpc_attachment::builders::AcceptTransitGatewayVpcAttachmentFluentBuilder::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::operation::accept_transit_gateway_vpc_attachment::builders::AcceptTransitGatewayVpcAttachmentFluentBuilder::set_transit_gateway_attachment_id):<br>required: **true**<br><p>The ID of the attachment.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::accept_transit_gateway_vpc_attachment::builders::AcceptTransitGatewayVpcAttachmentFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::accept_transit_gateway_vpc_attachment::builders::AcceptTransitGatewayVpcAttachmentFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`AcceptTransitGatewayVpcAttachmentOutput`](crate::operation::accept_transit_gateway_vpc_attachment::AcceptTransitGatewayVpcAttachmentOutput) with field(s):
    ///   - [`transit_gateway_vpc_attachment(Option<TransitGatewayVpcAttachment>)`](crate::operation::accept_transit_gateway_vpc_attachment::AcceptTransitGatewayVpcAttachmentOutput::transit_gateway_vpc_attachment): <p>The VPC attachment.</p>
    /// - On failure, responds with [`SdkError<AcceptTransitGatewayVpcAttachmentError>`](crate::operation::accept_transit_gateway_vpc_attachment::AcceptTransitGatewayVpcAttachmentError)
    pub fn accept_transit_gateway_vpc_attachment(
        &self,
    ) -> crate::operation::accept_transit_gateway_vpc_attachment::builders::AcceptTransitGatewayVpcAttachmentFluentBuilder {
        crate::operation::accept_transit_gateway_vpc_attachment::builders::AcceptTransitGatewayVpcAttachmentFluentBuilder::new(self.handle.clone())
    }
}
