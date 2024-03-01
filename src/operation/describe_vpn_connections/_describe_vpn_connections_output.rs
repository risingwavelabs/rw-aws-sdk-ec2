// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of DescribeVpnConnections.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVpnConnectionsOutput {
    /// <p>Information about one or more VPN connections.</p>
    pub vpn_connections: ::std::option::Option<::std::vec::Vec<crate::types::VpnConnection>>,
    _request_id: Option<String>,
}
impl DescribeVpnConnectionsOutput {
    /// <p>Information about one or more VPN connections.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.vpn_connections.is_none()`.
    pub fn vpn_connections(&self) -> &[crate::types::VpnConnection] {
        self.vpn_connections.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DescribeVpnConnectionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeVpnConnectionsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeVpnConnectionsOutput`](crate::operation::describe_vpn_connections::DescribeVpnConnectionsOutput).
    pub fn builder() -> crate::operation::describe_vpn_connections::builders::DescribeVpnConnectionsOutputBuilder {
        crate::operation::describe_vpn_connections::builders::DescribeVpnConnectionsOutputBuilder::default()
    }
}

/// A builder for [`DescribeVpnConnectionsOutput`](crate::operation::describe_vpn_connections::DescribeVpnConnectionsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeVpnConnectionsOutputBuilder {
    pub(crate) vpn_connections: ::std::option::Option<::std::vec::Vec<crate::types::VpnConnection>>,
    _request_id: Option<String>,
}
impl DescribeVpnConnectionsOutputBuilder {
    /// Appends an item to `vpn_connections`.
    ///
    /// To override the contents of this collection use [`set_vpn_connections`](Self::set_vpn_connections).
    ///
    /// <p>Information about one or more VPN connections.</p>
    pub fn vpn_connections(mut self, input: crate::types::VpnConnection) -> Self {
        let mut v = self.vpn_connections.unwrap_or_default();
        v.push(input);
        self.vpn_connections = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about one or more VPN connections.</p>
    pub fn set_vpn_connections(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::VpnConnection>>) -> Self {
        self.vpn_connections = input;
        self
    }
    /// <p>Information about one or more VPN connections.</p>
    pub fn get_vpn_connections(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::VpnConnection>> {
        &self.vpn_connections
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeVpnConnectionsOutput`](crate::operation::describe_vpn_connections::DescribeVpnConnectionsOutput).
    pub fn build(self) -> crate::operation::describe_vpn_connections::DescribeVpnConnectionsOutput {
        crate::operation::describe_vpn_connections::DescribeVpnConnectionsOutput {
            vpn_connections: self.vpn_connections,
            _request_id: self._request_id,
        }
    }
}