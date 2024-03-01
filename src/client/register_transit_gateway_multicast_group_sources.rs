// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterTransitGatewayMulticastGroupSources`](crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_multicast_domain_id(impl Into<String>)`](crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder::transit_gateway_multicast_domain_id) / [`set_transit_gateway_multicast_domain_id(Option<String>)`](crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder::set_transit_gateway_multicast_domain_id):<br>required: **true**<br><p>The ID of the transit gateway multicast domain.</p><br>
    ///   - [`group_ip_address(impl Into<String>)`](crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder::group_ip_address) / [`set_group_ip_address(Option<String>)`](crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder::set_group_ip_address):<br>required: **false**<br><p>The IP address assigned to the transit gateway multicast group.</p><br>
    ///   - [`network_interface_ids(impl Into<String>)`](crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder::network_interface_ids) / [`set_network_interface_ids(Option<Vec::<String>>)`](crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder::set_network_interface_ids):<br>required: **true**<br><p>The group sources' network interface IDs to register with the transit gateway multicast group.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`RegisterTransitGatewayMulticastGroupSourcesOutput`](crate::operation::register_transit_gateway_multicast_group_sources::RegisterTransitGatewayMulticastGroupSourcesOutput) with field(s):
    ///   - [`registered_multicast_group_sources(Option<TransitGatewayMulticastRegisteredGroupSources>)`](crate::operation::register_transit_gateway_multicast_group_sources::RegisterTransitGatewayMulticastGroupSourcesOutput::registered_multicast_group_sources): <p>Information about the transit gateway multicast group sources.</p>
    /// - On failure, responds with [`SdkError<RegisterTransitGatewayMulticastGroupSourcesError>`](crate::operation::register_transit_gateway_multicast_group_sources::RegisterTransitGatewayMulticastGroupSourcesError)
    pub fn register_transit_gateway_multicast_group_sources(
        &self,
    ) -> crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder {
        crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
