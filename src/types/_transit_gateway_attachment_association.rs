// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an association.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransitGatewayAttachmentAssociation {
    /// <p>The ID of the route table for the transit gateway.</p>
    pub transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>The state of the association.</p>
    pub state: ::std::option::Option<crate::types::TransitGatewayAssociationState>,
}
impl TransitGatewayAttachmentAssociation {
    /// <p>The ID of the route table for the transit gateway.</p>
    pub fn transit_gateway_route_table_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_route_table_id.as_deref()
    }
    /// <p>The state of the association.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::TransitGatewayAssociationState> {
        self.state.as_ref()
    }
}
impl TransitGatewayAttachmentAssociation {
    /// Creates a new builder-style object to manufacture [`TransitGatewayAttachmentAssociation`](crate::types::TransitGatewayAttachmentAssociation).
    pub fn builder() -> crate::types::builders::TransitGatewayAttachmentAssociationBuilder {
        crate::types::builders::TransitGatewayAttachmentAssociationBuilder::default()
    }
}

/// A builder for [`TransitGatewayAttachmentAssociation`](crate::types::TransitGatewayAttachmentAssociation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TransitGatewayAttachmentAssociationBuilder {
    pub(crate) transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::TransitGatewayAssociationState>,
}
impl TransitGatewayAttachmentAssociationBuilder {
    /// <p>The ID of the route table for the transit gateway.</p>
    pub fn transit_gateway_route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_route_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the route table for the transit gateway.</p>
    pub fn set_transit_gateway_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_route_table_id = input;
        self
    }
    /// <p>The ID of the route table for the transit gateway.</p>
    pub fn get_transit_gateway_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_route_table_id
    }
    /// <p>The state of the association.</p>
    pub fn state(mut self, input: crate::types::TransitGatewayAssociationState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the association.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::TransitGatewayAssociationState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the association.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::TransitGatewayAssociationState> {
        &self.state
    }
    /// Consumes the builder and constructs a [`TransitGatewayAttachmentAssociation`](crate::types::TransitGatewayAttachmentAssociation).
    pub fn build(self) -> crate::types::TransitGatewayAttachmentAssociation {
        crate::types::TransitGatewayAttachmentAssociation {
            transit_gateway_route_table_id: self.transit_gateway_route_table_id,
            state: self.state,
        }
    }
}