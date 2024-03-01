// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteClientVpnRouteInput {
    /// <p>The ID of the Client VPN endpoint from which the route is to be deleted.</p>
    pub client_vpn_endpoint_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the target subnet used by the route.</p>
    pub target_vpc_subnet_id: ::std::option::Option<::std::string::String>,
    /// <p>The IPv4 address range, in CIDR notation, of the route to be deleted.</p>
    pub destination_cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DeleteClientVpnRouteInput {
    /// <p>The ID of the Client VPN endpoint from which the route is to be deleted.</p>
    pub fn client_vpn_endpoint_id(&self) -> ::std::option::Option<&str> {
        self.client_vpn_endpoint_id.as_deref()
    }
    /// <p>The ID of the target subnet used by the route.</p>
    pub fn target_vpc_subnet_id(&self) -> ::std::option::Option<&str> {
        self.target_vpc_subnet_id.as_deref()
    }
    /// <p>The IPv4 address range, in CIDR notation, of the route to be deleted.</p>
    pub fn destination_cidr_block(&self) -> ::std::option::Option<&str> {
        self.destination_cidr_block.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteClientVpnRouteInput {
    /// Creates a new builder-style object to manufacture [`DeleteClientVpnRouteInput`](crate::operation::delete_client_vpn_route::DeleteClientVpnRouteInput).
    pub fn builder() -> crate::operation::delete_client_vpn_route::builders::DeleteClientVpnRouteInputBuilder {
        crate::operation::delete_client_vpn_route::builders::DeleteClientVpnRouteInputBuilder::default()
    }
}

/// A builder for [`DeleteClientVpnRouteInput`](crate::operation::delete_client_vpn_route::DeleteClientVpnRouteInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteClientVpnRouteInputBuilder {
    pub(crate) client_vpn_endpoint_id: ::std::option::Option<::std::string::String>,
    pub(crate) target_vpc_subnet_id: ::std::option::Option<::std::string::String>,
    pub(crate) destination_cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DeleteClientVpnRouteInputBuilder {
    /// <p>The ID of the Client VPN endpoint from which the route is to be deleted.</p>
    /// This field is required.
    pub fn client_vpn_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_vpn_endpoint_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint from which the route is to be deleted.</p>
    pub fn set_client_vpn_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_vpn_endpoint_id = input;
        self
    }
    /// <p>The ID of the Client VPN endpoint from which the route is to be deleted.</p>
    pub fn get_client_vpn_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_vpn_endpoint_id
    }
    /// <p>The ID of the target subnet used by the route.</p>
    pub fn target_vpc_subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.target_vpc_subnet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the target subnet used by the route.</p>
    pub fn set_target_vpc_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.target_vpc_subnet_id = input;
        self
    }
    /// <p>The ID of the target subnet used by the route.</p>
    pub fn get_target_vpc_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.target_vpc_subnet_id
    }
    /// <p>The IPv4 address range, in CIDR notation, of the route to be deleted.</p>
    /// This field is required.
    pub fn destination_cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination_cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, of the route to be deleted.</p>
    pub fn set_destination_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination_cidr_block = input;
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, of the route to be deleted.</p>
    pub fn get_destination_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination_cidr_block
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
    /// Consumes the builder and constructs a [`DeleteClientVpnRouteInput`](crate::operation::delete_client_vpn_route::DeleteClientVpnRouteInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_client_vpn_route::DeleteClientVpnRouteInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::delete_client_vpn_route::DeleteClientVpnRouteInput {
            client_vpn_endpoint_id: self.client_vpn_endpoint_id,
            target_vpc_subnet_id: self.target_vpc_subnet_id,
            destination_cidr_block: self.destination_cidr_block,
            dry_run: self.dry_run,
        })
    }
}
