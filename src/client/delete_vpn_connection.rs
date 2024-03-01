// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVpnConnection`](crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`vpn_connection_id(impl Into<String>)`](crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionFluentBuilder::vpn_connection_id) / [`set_vpn_connection_id(Option<String>)`](crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionFluentBuilder::set_vpn_connection_id):<br>required: **true**<br><p>The ID of the VPN connection.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DeleteVpnConnectionOutput`](crate::operation::delete_vpn_connection::DeleteVpnConnectionOutput)
    /// - On failure, responds with [`SdkError<DeleteVpnConnectionError>`](crate::operation::delete_vpn_connection::DeleteVpnConnectionError)
    pub fn delete_vpn_connection(&self) -> crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionFluentBuilder {
        crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionFluentBuilder::new(self.handle.clone())
    }
}
