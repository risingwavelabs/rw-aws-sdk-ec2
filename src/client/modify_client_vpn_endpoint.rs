// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyClientVpnEndpoint`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_vpn_endpoint_id(impl Into<String>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::client_vpn_endpoint_id) / [`set_client_vpn_endpoint_id(Option<String>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_client_vpn_endpoint_id):<br>required: **true**<br><p>The ID of the Client VPN endpoint to modify.</p><br>
    ///   - [`server_certificate_arn(impl Into<String>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::server_certificate_arn) / [`set_server_certificate_arn(Option<String>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_server_certificate_arn):<br>required: **false**<br><p>The ARN of the server certificate to be used. The server certificate must be provisioned in Certificate Manager (ACM).</p><br>
    ///   - [`connection_log_options(ConnectionLogOptions)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::connection_log_options) / [`set_connection_log_options(Option<ConnectionLogOptions>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_connection_log_options):<br>required: **false**<br><p>Information about the client connection logging options.</p>  <p>If you enable client connection logging, data about client connections is sent to a Cloudwatch Logs log stream. The following information is logged:</p>  <ul>   <li> <p>Client connection requests</p> </li>   <li> <p>Client connection results (successful and unsuccessful)</p> </li>   <li> <p>Reasons for unsuccessful client connection requests</p> </li>   <li> <p>Client connection termination time</p> </li>  </ul><br>
    ///   - [`dns_servers(DnsServersOptionsModifyStructure)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::dns_servers) / [`set_dns_servers(Option<DnsServersOptionsModifyStructure>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_dns_servers):<br>required: **false**<br><p>Information about the DNS servers to be used by Client VPN connections. A Client VPN endpoint can have up to two DNS servers.</p><br>
    ///   - [`vpn_port(i32)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::vpn_port) / [`set_vpn_port(Option<i32>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_vpn_port):<br>required: **false**<br><p>The port number to assign to the Client VPN endpoint for TCP and UDP traffic.</p>  <p>Valid Values: <code>443</code> | <code>1194</code> </p>  <p>Default Value: <code>443</code> </p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_description):<br>required: **false**<br><p>A brief description of the Client VPN endpoint.</p><br>
    ///   - [`split_tunnel(bool)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::split_tunnel) / [`set_split_tunnel(Option<bool>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_split_tunnel):<br>required: **false**<br><p>Indicates whether the VPN is split-tunnel.</p>  <p>For information about split-tunnel VPN endpoints, see <a href="https://docs.aws.amazon.com/vpn/latest/clientvpn-admin/split-tunnel-vpn.html">Split-tunnel Client VPN endpoint</a> in the <i>Client VPN Administrator Guide</i>.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`security_group_ids(impl Into<String>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::security_group_ids) / [`set_security_group_ids(Option<Vec::<String>>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_security_group_ids):<br>required: **false**<br><p>The IDs of one or more security groups to apply to the target network.</p><br>
    ///   - [`vpc_id(impl Into<String>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::vpc_id) / [`set_vpc_id(Option<String>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_vpc_id):<br>required: **false**<br><p>The ID of the VPC to associate with the Client VPN endpoint.</p><br>
    ///   - [`self_service_portal(SelfServicePortal)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::self_service_portal) / [`set_self_service_portal(Option<SelfServicePortal>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_self_service_portal):<br>required: **false**<br><p>Specify whether to enable the self-service portal for the Client VPN endpoint.</p><br>
    ///   - [`client_connect_options(ClientConnectOptions)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::client_connect_options) / [`set_client_connect_options(Option<ClientConnectOptions>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_client_connect_options):<br>required: **false**<br><p>The options for managing connection authorization for new client connections.</p><br>
    ///   - [`session_timeout_hours(i32)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::session_timeout_hours) / [`set_session_timeout_hours(Option<i32>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_session_timeout_hours):<br>required: **false**<br><p>The maximum VPN session duration time in hours.</p>  <p>Valid values: <code>8 | 10 | 12 | 24</code> </p>  <p>Default value: <code>24</code> </p><br>
    ///   - [`client_login_banner_options(ClientLoginBannerOptions)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::client_login_banner_options) / [`set_client_login_banner_options(Option<ClientLoginBannerOptions>)`](crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::set_client_login_banner_options):<br>required: **false**<br><p>Options for enabling a customizable text banner that will be displayed on Amazon Web Services provided clients when a VPN session is established.</p><br>
    /// - On success, responds with [`ModifyClientVpnEndpointOutput`](crate::operation::modify_client_vpn_endpoint::ModifyClientVpnEndpointOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::modify_client_vpn_endpoint::ModifyClientVpnEndpointOutput::return): <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    /// - On failure, responds with [`SdkError<ModifyClientVpnEndpointError>`](crate::operation::modify_client_vpn_endpoint::ModifyClientVpnEndpointError)
    pub fn modify_client_vpn_endpoint(&self) -> crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder {
        crate::operation::modify_client_vpn_endpoint::builders::ModifyClientVpnEndpointFluentBuilder::new(self.handle.clone())
    }
}
