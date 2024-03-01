// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateTransitGatewayMulticastDomain`](crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_multicast_domain_id(impl Into<String>)`](crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder::transit_gateway_multicast_domain_id) / [`set_transit_gateway_multicast_domain_id(Option<String>)`](crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder::set_transit_gateway_multicast_domain_id):<br>required: **true**<br><p>The ID of the transit gateway multicast domain.</p><br>
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder::set_transit_gateway_attachment_id):<br>required: **true**<br><p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p><br>
    ///   - [`subnet_ids(impl Into<String>)`](crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder::subnet_ids) / [`set_subnet_ids(Option<Vec::<String>>)`](crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder::set_subnet_ids):<br>required: **true**<br><p>The IDs of the subnets to associate with the transit gateway multicast domain.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`AssociateTransitGatewayMulticastDomainOutput`](crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput) with field(s):
    ///   - [`associations(Option<TransitGatewayMulticastDomainAssociations>)`](crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainOutput::associations): <p>Information about the transit gateway multicast domain associations.</p>
    /// - On failure, responds with [`SdkError<AssociateTransitGatewayMulticastDomainError>`](crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainError)
    pub fn associate_transit_gateway_multicast_domain(
        &self,
    ) -> crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder {
        crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
