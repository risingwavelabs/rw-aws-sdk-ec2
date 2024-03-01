// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVpnTunnelCertificate`](crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vpn_connection_id(impl Into<String>)`](crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateFluentBuilder::vpn_connection_id) / [`set_vpn_connection_id(Option<String>)`](crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateFluentBuilder::set_vpn_connection_id):<br>required: **true**<br><p>The ID of the Amazon Web Services Site-to-Site VPN connection.</p><br>
    ///   - [`vpn_tunnel_outside_ip_address(impl Into<String>)`](crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateFluentBuilder::vpn_tunnel_outside_ip_address) / [`set_vpn_tunnel_outside_ip_address(Option<String>)`](crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateFluentBuilder::set_vpn_tunnel_outside_ip_address):<br>required: **true**<br><p>The external IP address of the VPN tunnel.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`ModifyVpnTunnelCertificateOutput`](crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateOutput) with field(s):
    ///   - [`vpn_connection(Option<VpnConnection>)`](crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateOutput::vpn_connection): <p>Information about the VPN connection.</p>
    /// - On failure, responds with [`SdkError<ModifyVpnTunnelCertificateError>`](crate::operation::modify_vpn_tunnel_certificate::ModifyVpnTunnelCertificateError)
    pub fn modify_vpn_tunnel_certificate(
        &self,
    ) -> crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateFluentBuilder {
        crate::operation::modify_vpn_tunnel_certificate::builders::ModifyVpnTunnelCertificateFluentBuilder::new(self.handle.clone())
    }
}