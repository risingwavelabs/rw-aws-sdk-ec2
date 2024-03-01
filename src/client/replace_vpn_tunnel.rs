// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ReplaceVpnTunnel`](crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vpn_connection_id(impl Into<String>)`](crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder::vpn_connection_id) / [`set_vpn_connection_id(Option<String>)`](crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder::set_vpn_connection_id):<br>required: **true**<br><p>The ID of the Site-to-Site VPN connection. </p><br>
    ///   - [`vpn_tunnel_outside_ip_address(impl Into<String>)`](crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder::vpn_tunnel_outside_ip_address) / [`set_vpn_tunnel_outside_ip_address(Option<String>)`](crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder::set_vpn_tunnel_outside_ip_address):<br>required: **true**<br><p>The external IP address of the VPN tunnel.</p><br>
    ///   - [`apply_pending_maintenance(bool)`](crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder::apply_pending_maintenance) / [`set_apply_pending_maintenance(Option<bool>)`](crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder::set_apply_pending_maintenance):<br>required: **false**<br><p>Trigger pending tunnel endpoint maintenance.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`ReplaceVpnTunnelOutput`](crate::operation::replace_vpn_tunnel::ReplaceVpnTunnelOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::replace_vpn_tunnel::ReplaceVpnTunnelOutput::return): <p>Confirmation of replace tunnel operation.</p>
    /// - On failure, responds with [`SdkError<ReplaceVpnTunnelError>`](crate::operation::replace_vpn_tunnel::ReplaceVpnTunnelError)
    pub fn replace_vpn_tunnel(&self) -> crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder {
        crate::operation::replace_vpn_tunnel::builders::ReplaceVpnTunnelFluentBuilder::new(self.handle.clone())
    }
}