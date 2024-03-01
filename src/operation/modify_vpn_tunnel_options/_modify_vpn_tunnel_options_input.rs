// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct ModifyVpnTunnelOptionsInput {
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub vpn_connection_id: ::std::option::Option<::std::string::String>,
    /// <p>The external IP address of the VPN tunnel.</p>
    pub vpn_tunnel_outside_ip_address: ::std::option::Option<::std::string::String>,
    /// <p>The tunnel options to modify.</p>
    pub tunnel_options: ::std::option::Option<crate::types::ModifyVpnTunnelOptionsSpecification>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Choose whether or not to trigger immediate tunnel replacement. This is only applicable when turning on or off <code>EnableTunnelLifecycleControl</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub skip_tunnel_replacement: ::std::option::Option<bool>,
}
impl ModifyVpnTunnelOptionsInput {
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn vpn_connection_id(&self) -> ::std::option::Option<&str> {
        self.vpn_connection_id.as_deref()
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    pub fn vpn_tunnel_outside_ip_address(&self) -> ::std::option::Option<&str> {
        self.vpn_tunnel_outside_ip_address.as_deref()
    }
    /// <p>The tunnel options to modify.</p>
    pub fn tunnel_options(&self) -> ::std::option::Option<&crate::types::ModifyVpnTunnelOptionsSpecification> {
        self.tunnel_options.as_ref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Choose whether or not to trigger immediate tunnel replacement. This is only applicable when turning on or off <code>EnableTunnelLifecycleControl</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn skip_tunnel_replacement(&self) -> ::std::option::Option<bool> {
        self.skip_tunnel_replacement
    }
}
impl ::std::fmt::Debug for ModifyVpnTunnelOptionsInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ModifyVpnTunnelOptionsInput");
        formatter.field("vpn_connection_id", &self.vpn_connection_id);
        formatter.field("vpn_tunnel_outside_ip_address", &self.vpn_tunnel_outside_ip_address);
        formatter.field("tunnel_options", &"*** Sensitive Data Redacted ***");
        formatter.field("dry_run", &self.dry_run);
        formatter.field("skip_tunnel_replacement", &self.skip_tunnel_replacement);
        formatter.finish()
    }
}
impl ModifyVpnTunnelOptionsInput {
    /// Creates a new builder-style object to manufacture [`ModifyVpnTunnelOptionsInput`](crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsInput).
    pub fn builder() -> crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsInputBuilder {
        crate::operation::modify_vpn_tunnel_options::builders::ModifyVpnTunnelOptionsInputBuilder::default()
    }
}

/// A builder for [`ModifyVpnTunnelOptionsInput`](crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct ModifyVpnTunnelOptionsInputBuilder {
    pub(crate) vpn_connection_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpn_tunnel_outside_ip_address: ::std::option::Option<::std::string::String>,
    pub(crate) tunnel_options: ::std::option::Option<crate::types::ModifyVpnTunnelOptionsSpecification>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) skip_tunnel_replacement: ::std::option::Option<bool>,
}
impl ModifyVpnTunnelOptionsInputBuilder {
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    /// This field is required.
    pub fn vpn_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpn_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn set_vpn_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpn_connection_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p>
    pub fn get_vpn_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpn_connection_id
    }
    /// <p>The external IP address of the VPN tunnel.</p>
    /// This field is required.
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
    /// <p>The tunnel options to modify.</p>
    /// This field is required.
    pub fn tunnel_options(mut self, input: crate::types::ModifyVpnTunnelOptionsSpecification) -> Self {
        self.tunnel_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The tunnel options to modify.</p>
    pub fn set_tunnel_options(mut self, input: ::std::option::Option<crate::types::ModifyVpnTunnelOptionsSpecification>) -> Self {
        self.tunnel_options = input;
        self
    }
    /// <p>The tunnel options to modify.</p>
    pub fn get_tunnel_options(&self) -> &::std::option::Option<crate::types::ModifyVpnTunnelOptionsSpecification> {
        &self.tunnel_options
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
    /// <p>Choose whether or not to trigger immediate tunnel replacement. This is only applicable when turning on or off <code>EnableTunnelLifecycleControl</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn skip_tunnel_replacement(mut self, input: bool) -> Self {
        self.skip_tunnel_replacement = ::std::option::Option::Some(input);
        self
    }
    /// <p>Choose whether or not to trigger immediate tunnel replacement. This is only applicable when turning on or off <code>EnableTunnelLifecycleControl</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn set_skip_tunnel_replacement(mut self, input: ::std::option::Option<bool>) -> Self {
        self.skip_tunnel_replacement = input;
        self
    }
    /// <p>Choose whether or not to trigger immediate tunnel replacement. This is only applicable when turning on or off <code>EnableTunnelLifecycleControl</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn get_skip_tunnel_replacement(&self) -> &::std::option::Option<bool> {
        &self.skip_tunnel_replacement
    }
    /// Consumes the builder and constructs a [`ModifyVpnTunnelOptionsInput`](crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::modify_vpn_tunnel_options::ModifyVpnTunnelOptionsInput {
            vpn_connection_id: self.vpn_connection_id,
            vpn_tunnel_outside_ip_address: self.vpn_tunnel_outside_ip_address,
            tunnel_options: self.tunnel_options,
            dry_run: self.dry_run,
            skip_tunnel_replacement: self.skip_tunnel_replacement,
        })
    }
}
impl ::std::fmt::Debug for ModifyVpnTunnelOptionsInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("ModifyVpnTunnelOptionsInputBuilder");
        formatter.field("vpn_connection_id", &self.vpn_connection_id);
        formatter.field("vpn_tunnel_outside_ip_address", &self.vpn_tunnel_outside_ip_address);
        formatter.field("tunnel_options", &"*** Sensitive Data Redacted ***");
        formatter.field("dry_run", &self.dry_run);
        formatter.field("skip_tunnel_replacement", &self.skip_tunnel_replacement);
        formatter.finish()
    }
}
