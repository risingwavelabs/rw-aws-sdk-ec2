// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetVpnTunnelReplacementStatusOutput {
    /// <p>The ID of the Site-to-Site VPN connection. </p>
    pub vpn_connection_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub transit_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the customer gateway.</p>
    pub customer_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the virtual private gateway.</p>
    pub vpn_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The external IP address of the VPN tunnel.</p>
    pub vpn_tunnel_outside_ip_address: ::std::option::Option<::std::string::String>,
    /// <p>Get details of pending tunnel endpoint maintenance.</p>
    pub maintenance_details: ::std::option::Option<crate::types::MaintenanceDetails>,
    _request_id: Option<String>,
}
impl GetVpnTunnelReplacementStatusOutput {
    /// <p>The ID of the Site-to-Site VPN connection. </p>
    pub fn vpn_connection_id(&self) -> ::std::option::Option<&str> {
        self.vpn_connection_id.as_deref()
    }
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub fn transit_gateway_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_id.as_deref()
    }
    /// <p>The ID of the customer gateway.</p>
    pub fn customer_gateway_id(&self) -> ::std::option::Option<&str> {
        self.customer_gateway_id.as_deref()
    }
    /// <p>The ID of the virtual private gateway.</p>
    pub fn vpn_gateway_id(&self) -> ::std::option::Option<&str> {
        self.vpn_gateway_id.as_deref()
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn vpn_tunnel_outside_ip_address(&self) -> ::std::option::Option<&str> {
        self.vpn_tunnel_outside_ip_address.as_deref()
    }
    /// <p>Get details of pending tunnel endpoint maintenance.</p>
    pub fn maintenance_details(&self) -> ::std::option::Option<&crate::types::MaintenanceDetails> {
        self.maintenance_details.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetVpnTunnelReplacementStatusOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetVpnTunnelReplacementStatusOutput {
    /// Creates a new builder-style object to manufacture [`GetVpnTunnelReplacementStatusOutput`](crate::operation::get_vpn_tunnel_replacement_status::GetVpnTunnelReplacementStatusOutput).
    pub fn builder() -> crate::operation::get_vpn_tunnel_replacement_status::builders::GetVpnTunnelReplacementStatusOutputBuilder {
        crate::operation::get_vpn_tunnel_replacement_status::builders::GetVpnTunnelReplacementStatusOutputBuilder::default()
    }
}

/// A builder for [`GetVpnTunnelReplacementStatusOutput`](crate::operation::get_vpn_tunnel_replacement_status::GetVpnTunnelReplacementStatusOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetVpnTunnelReplacementStatusOutputBuilder {
    pub(crate) vpn_connection_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) customer_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpn_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpn_tunnel_outside_ip_address: ::std::option::Option<::std::string::String>,
    pub(crate) maintenance_details: ::std::option::Option<crate::types::MaintenanceDetails>,
    _request_id: Option<String>,
}
impl GetVpnTunnelReplacementStatusOutputBuilder {
    /// <p>The ID of the Site-to-Site VPN connection. </p>
    pub fn vpn_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpn_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Site-to-Site VPN connection. </p>
    pub fn set_vpn_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpn_connection_id = input;
        self
    }
    /// <p>The ID of the Site-to-Site VPN connection. </p>
    pub fn get_vpn_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpn_connection_id
    }
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub fn transit_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub fn set_transit_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_id = input;
        self
    }
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub fn get_transit_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_id
    }
    /// <p>The ID of the customer gateway.</p>
    pub fn customer_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.customer_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the customer gateway.</p>
    pub fn set_customer_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.customer_gateway_id = input;
        self
    }
    /// <p>The ID of the customer gateway.</p>
    pub fn get_customer_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.customer_gateway_id
    }
    /// <p>The ID of the virtual private gateway.</p>
    pub fn vpn_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpn_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the virtual private gateway.</p>
    pub fn set_vpn_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpn_gateway_id = input;
        self
    }
    /// <p>The ID of the virtual private gateway.</p>
    pub fn get_vpn_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpn_gateway_id
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn vpn_tunnel_outside_ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpn_tunnel_outside_ip_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn set_vpn_tunnel_outside_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpn_tunnel_outside_ip_address = input;
        self
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn get_vpn_tunnel_outside_ip_address(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpn_tunnel_outside_ip_address
    }
    /// <p>Get details of pending tunnel endpoint maintenance.</p>
    pub fn maintenance_details(mut self, input: crate::types::MaintenanceDetails) -> Self {
        self.maintenance_details = ::std::option::Option::Some(input);
        self
    }
    /// <p>Get details of pending tunnel endpoint maintenance.</p>
    pub fn set_maintenance_details(mut self, input: ::std::option::Option<crate::types::MaintenanceDetails>) -> Self {
        self.maintenance_details = input;
        self
    }
    /// <p>Get details of pending tunnel endpoint maintenance.</p>
    pub fn get_maintenance_details(&self) -> &::std::option::Option<crate::types::MaintenanceDetails> {
        &self.maintenance_details
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetVpnTunnelReplacementStatusOutput`](crate::operation::get_vpn_tunnel_replacement_status::GetVpnTunnelReplacementStatusOutput).
    pub fn build(self) -> crate::operation::get_vpn_tunnel_replacement_status::GetVpnTunnelReplacementStatusOutput {
        crate::operation::get_vpn_tunnel_replacement_status::GetVpnTunnelReplacementStatusOutput {
            vpn_connection_id: self.vpn_connection_id,
            transit_gateway_id: self.transit_gateway_id,
            customer_gateway_id: self.customer_gateway_id,
            vpn_gateway_id: self.vpn_gateway_id,
            vpn_tunnel_outside_ip_address: self.vpn_tunnel_outside_ip_address,
            maintenance_details: self.maintenance_details,
            _request_id: self._request_id,
        }
    }
}