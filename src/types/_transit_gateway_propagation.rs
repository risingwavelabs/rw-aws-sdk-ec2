// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes route propagation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransitGatewayPropagation {
    /// <p>The ID of the attachment.</p>
    pub transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the resource.</p>
    pub resource_id: ::std::option::Option<::std::string::String>,
    /// <p>The resource type. Note that the <code>tgw-peering</code> resource type has been deprecated.</p>
    pub resource_type: ::std::option::Option<crate::types::TransitGatewayAttachmentResourceType>,
    /// <p>The ID of the transit gateway route table.</p>
    pub transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>The state.</p>
    pub state: ::std::option::Option<crate::types::TransitGatewayPropagationState>,
    /// <p>The ID of the transit gateway route table announcement.</p>
    pub transit_gateway_route_table_announcement_id: ::std::option::Option<::std::string::String>,
}
impl TransitGatewayPropagation {
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>The ID of the resource.</p>
    pub fn resource_id(&self) -> ::std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// <p>The resource type. Note that the <code>tgw-peering</code> resource type has been deprecated.</p>
    pub fn resource_type(&self) -> ::std::option::Option<&crate::types::TransitGatewayAttachmentResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn transit_gateway_route_table_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_route_table_id.as_deref()
    }
    /// <p>The state.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::TransitGatewayPropagationState> {
        self.state.as_ref()
    }
    /// <p>The ID of the transit gateway route table announcement.</p>
    pub fn transit_gateway_route_table_announcement_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_route_table_announcement_id.as_deref()
    }
}
impl TransitGatewayPropagation {
    /// Creates a new builder-style object to manufacture [`TransitGatewayPropagation`](crate::types::TransitGatewayPropagation).
    pub fn builder() -> crate::types::builders::TransitGatewayPropagationBuilder {
        crate::types::builders::TransitGatewayPropagationBuilder::default()
    }
}

/// A builder for [`TransitGatewayPropagation`](crate::types::TransitGatewayPropagation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TransitGatewayPropagationBuilder {
    pub(crate) transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_type: ::std::option::Option<crate::types::TransitGatewayAttachmentResourceType>,
    pub(crate) transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::TransitGatewayPropagationState>,
    pub(crate) transit_gateway_route_table_announcement_id: ::std::option::Option<::std::string::String>,
}
impl TransitGatewayPropagationBuilder {
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_attachment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn set_transit_gateway_attachment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn get_transit_gateway_attachment_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_attachment_id
    }
    /// <p>The ID of the resource.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn get_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_id
    }
    /// <p>The resource type. Note that the <code>tgw-peering</code> resource type has been deprecated.</p>
    pub fn resource_type(mut self, input: crate::types::TransitGatewayAttachmentResourceType) -> Self {
        self.resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The resource type. Note that the <code>tgw-peering</code> resource type has been deprecated.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<crate::types::TransitGatewayAttachmentResourceType>) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The resource type. Note that the <code>tgw-peering</code> resource type has been deprecated.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<crate::types::TransitGatewayAttachmentResourceType> {
        &self.resource_type
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn transit_gateway_route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_route_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn set_transit_gateway_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_route_table_id = input;
        self
    }
    /// <p>The ID of the transit gateway route table.</p>
    pub fn get_transit_gateway_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_route_table_id
    }
    /// <p>The state.</p>
    pub fn state(mut self, input: crate::types::TransitGatewayPropagationState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::TransitGatewayPropagationState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::TransitGatewayPropagationState> {
        &self.state
    }
    /// <p>The ID of the transit gateway route table announcement.</p>
    pub fn transit_gateway_route_table_announcement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_route_table_announcement_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway route table announcement.</p>
    pub fn set_transit_gateway_route_table_announcement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_route_table_announcement_id = input;
        self
    }
    /// <p>The ID of the transit gateway route table announcement.</p>
    pub fn get_transit_gateway_route_table_announcement_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_route_table_announcement_id
    }
    /// Consumes the builder and constructs a [`TransitGatewayPropagation`](crate::types::TransitGatewayPropagation).
    pub fn build(self) -> crate::types::TransitGatewayPropagation {
        crate::types::TransitGatewayPropagation {
            transit_gateway_attachment_id: self.transit_gateway_attachment_id,
            resource_id: self.resource_id,
            resource_type: self.resource_type,
            transit_gateway_route_table_id: self.transit_gateway_route_table_id,
            state: self.state,
            transit_gateway_route_table_announcement_id: self.transit_gateway_route_table_announcement_id,
        }
    }
}
