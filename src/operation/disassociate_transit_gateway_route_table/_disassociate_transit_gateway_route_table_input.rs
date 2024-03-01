// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateTransitGatewayRouteTableInput {
    /// <p>The ID of the transit gateway route table.</p>
    pub transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the attachment.</p>
    pub transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DisassociateTransitGatewayRouteTableInput {
    /// <p>The ID of the transit gateway route table.</p>
    pub fn transit_gateway_route_table_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_route_table_id.as_deref()
    }
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DisassociateTransitGatewayRouteTableInput {
    /// Creates a new builder-style object to manufacture [`DisassociateTransitGatewayRouteTableInput`](crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableInput).
    pub fn builder() -> crate::operation::disassociate_transit_gateway_route_table::builders::DisassociateTransitGatewayRouteTableInputBuilder {
        crate::operation::disassociate_transit_gateway_route_table::builders::DisassociateTransitGatewayRouteTableInputBuilder::default()
    }
}

/// A builder for [`DisassociateTransitGatewayRouteTableInput`](crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisassociateTransitGatewayRouteTableInputBuilder {
    pub(crate) transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DisassociateTransitGatewayRouteTableInputBuilder {
    /// <p>The ID of the transit gateway route table.</p>
    /// This field is required.
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
    /// <p>The ID of the attachment.</p>
    /// This field is required.
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
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`DisassociateTransitGatewayRouteTableInput`](crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::disassociate_transit_gateway_route_table::DisassociateTransitGatewayRouteTableInput {
                transit_gateway_route_table_id: self.transit_gateway_route_table_id,
                transit_gateway_attachment_id: self.transit_gateway_attachment_id,
                dry_run: self.dry_run,
            },
        )
    }
}
