// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of CreateVpnConnection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVpnConnectionOutput {
    /// <p>Information about the VPN connection.</p>
    pub vpn_connection: ::std::option::Option<crate::types::VpnConnection>,
    _request_id: Option<String>,
}
impl CreateVpnConnectionOutput {
    /// <p>Information about the VPN connection.</p>
    pub fn vpn_connection(&self) -> ::std::option::Option<&crate::types::VpnConnection> {
        self.vpn_connection.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for CreateVpnConnectionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateVpnConnectionOutput {
    /// Creates a new builder-style object to manufacture [`CreateVpnConnectionOutput`](crate::operation::create_vpn_connection::CreateVpnConnectionOutput).
    pub fn builder() -> crate::operation::create_vpn_connection::builders::CreateVpnConnectionOutputBuilder {
        crate::operation::create_vpn_connection::builders::CreateVpnConnectionOutputBuilder::default()
    }
}

/// A builder for [`CreateVpnConnectionOutput`](crate::operation::create_vpn_connection::CreateVpnConnectionOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateVpnConnectionOutputBuilder {
    pub(crate) vpn_connection: ::std::option::Option<crate::types::VpnConnection>,
    _request_id: Option<String>,
}
impl CreateVpnConnectionOutputBuilder {
    /// <p>Information about the VPN connection.</p>
    pub fn vpn_connection(mut self, input: crate::types::VpnConnection) -> Self {
        self.vpn_connection = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the VPN connection.</p>
    pub fn set_vpn_connection(mut self, input: ::std::option::Option<crate::types::VpnConnection>) -> Self {
        self.vpn_connection = input;
        self
    }
    /// <p>Information about the VPN connection.</p>
    pub fn get_vpn_connection(&self) -> &::std::option::Option<crate::types::VpnConnection> {
        &self.vpn_connection
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateVpnConnectionOutput`](crate::operation::create_vpn_connection::CreateVpnConnectionOutput).
    pub fn build(self) -> crate::operation::create_vpn_connection::CreateVpnConnectionOutput {
        crate::operation::create_vpn_connection::CreateVpnConnectionOutput {
            vpn_connection: self.vpn_connection,
            _request_id: self._request_id,
        }
    }
}
