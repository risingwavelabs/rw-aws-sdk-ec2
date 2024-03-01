// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableTransitGatewayRouteTablePropagationInput {
    /// <p>The ID of the propagation route table.</p>
    pub transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the attachment.</p>
    pub transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the route table announcement.</p>
    pub transit_gateway_route_table_announcement_id: ::std::option::Option<::std::string::String>,
}
impl DisableTransitGatewayRouteTablePropagationInput {
    /// <p>The ID of the propagation route table.</p>
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
    /// <p>The ID of the route table announcement.</p>
    pub fn transit_gateway_route_table_announcement_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_route_table_announcement_id.as_deref()
    }
}
impl DisableTransitGatewayRouteTablePropagationInput {
    /// Creates a new builder-style object to manufacture [`DisableTransitGatewayRouteTablePropagationInput`](crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationInput).
    pub fn builder(
    ) -> crate::operation::disable_transit_gateway_route_table_propagation::builders::DisableTransitGatewayRouteTablePropagationInputBuilder {
        crate::operation::disable_transit_gateway_route_table_propagation::builders::DisableTransitGatewayRouteTablePropagationInputBuilder::default()
    }
}

/// A builder for [`DisableTransitGatewayRouteTablePropagationInput`](crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisableTransitGatewayRouteTablePropagationInputBuilder {
    pub(crate) transit_gateway_route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) transit_gateway_route_table_announcement_id: ::std::option::Option<::std::string::String>,
}
impl DisableTransitGatewayRouteTablePropagationInputBuilder {
    /// <p>The ID of the propagation route table.</p>
    /// This field is required.
    pub fn transit_gateway_route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_route_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the propagation route table.</p>
    pub fn set_transit_gateway_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_route_table_id = input;
        self
    }
    /// <p>The ID of the propagation route table.</p>
    pub fn get_transit_gateway_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_route_table_id
    }
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
    /// <p>The ID of the route table announcement.</p>
    pub fn transit_gateway_route_table_announcement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_route_table_announcement_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the route table announcement.</p>
    pub fn set_transit_gateway_route_table_announcement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_route_table_announcement_id = input;
        self
    }
    /// <p>The ID of the route table announcement.</p>
    pub fn get_transit_gateway_route_table_announcement_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_route_table_announcement_id
    }
    /// Consumes the builder and constructs a [`DisableTransitGatewayRouteTablePropagationInput`](crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationInput {
                transit_gateway_route_table_id: self.transit_gateway_route_table_id,
                transit_gateway_attachment_id: self.transit_gateway_attachment_id,
                dry_run: self.dry_run,
                transit_gateway_route_table_announcement_id: self.transit_gateway_route_table_announcement_id,
            },
        )
    }
}
