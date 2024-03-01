// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTransitGatewayPeeringAttachmentOutput {
    /// <p>The transit gateway peering attachment.</p>
    pub transit_gateway_peering_attachment: ::std::option::Option<crate::types::TransitGatewayPeeringAttachment>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayPeeringAttachmentOutput {
    /// <p>The transit gateway peering attachment.</p>
    pub fn transit_gateway_peering_attachment(&self) -> ::std::option::Option<&crate::types::TransitGatewayPeeringAttachment> {
        self.transit_gateway_peering_attachment.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for CreateTransitGatewayPeeringAttachmentOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateTransitGatewayPeeringAttachmentOutput {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayPeeringAttachmentOutput`](crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentOutput).
    pub fn builder() -> crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentOutputBuilder {
        crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentOutputBuilder::default()
    }
}

/// A builder for [`CreateTransitGatewayPeeringAttachmentOutput`](crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateTransitGatewayPeeringAttachmentOutputBuilder {
    pub(crate) transit_gateway_peering_attachment: ::std::option::Option<crate::types::TransitGatewayPeeringAttachment>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayPeeringAttachmentOutputBuilder {
    /// <p>The transit gateway peering attachment.</p>
    pub fn transit_gateway_peering_attachment(mut self, input: crate::types::TransitGatewayPeeringAttachment) -> Self {
        self.transit_gateway_peering_attachment = ::std::option::Option::Some(input);
        self
    }
    /// <p>The transit gateway peering attachment.</p>
    pub fn set_transit_gateway_peering_attachment(mut self, input: ::std::option::Option<crate::types::TransitGatewayPeeringAttachment>) -> Self {
        self.transit_gateway_peering_attachment = input;
        self
    }
    /// <p>The transit gateway peering attachment.</p>
    pub fn get_transit_gateway_peering_attachment(&self) -> &::std::option::Option<crate::types::TransitGatewayPeeringAttachment> {
        &self.transit_gateway_peering_attachment
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateTransitGatewayPeeringAttachmentOutput`](crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentOutput).
    pub fn build(self) -> crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentOutput {
        crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentOutput {
            transit_gateway_peering_attachment: self.transit_gateway_peering_attachment,
            _request_id: self._request_id,
        }
    }
}
