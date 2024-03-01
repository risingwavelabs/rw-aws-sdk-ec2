// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVpnGateway`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`availability_zone(impl Into<String>)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::availability_zone) / [`set_availability_zone(Option<String>)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::set_availability_zone):<br>required: **false**<br><p>The Availability Zone for the virtual private gateway.</p><br>
    ///   - [`r#type(GatewayType)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::type) / [`set_type(Option<GatewayType>)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::set_type):<br>required: **true**<br><p>The type of VPN connection this virtual private gateway supports.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to apply to the virtual private gateway.</p><br>
    ///   - [`amazon_side_asn(i64)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::amazon_side_asn) / [`set_amazon_side_asn(Option<i64>)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::set_amazon_side_asn):<br>required: **false**<br><p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. If you're using a 16-bit ASN, it must be in the 64512 to 65534 range. If you're using a 32-bit ASN, it must be in the 4200000000 to 4294967294 range.</p>  <p>Default: 64512</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`CreateVpnGatewayOutput`](crate::operation::create_vpn_gateway::CreateVpnGatewayOutput) with field(s):
    ///   - [`vpn_gateway(Option<VpnGateway>)`](crate::operation::create_vpn_gateway::CreateVpnGatewayOutput::vpn_gateway): <p>Information about the virtual private gateway.</p>
    /// - On failure, responds with [`SdkError<CreateVpnGatewayError>`](crate::operation::create_vpn_gateway::CreateVpnGatewayError)
    pub fn create_vpn_gateway(&self) -> crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder {
        crate::operation::create_vpn_gateway::builders::CreateVpnGatewayFluentBuilder::new(self.handle.clone())
    }
}
