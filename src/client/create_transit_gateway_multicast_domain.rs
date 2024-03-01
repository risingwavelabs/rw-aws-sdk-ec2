// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateTransitGatewayMulticastDomain`](crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_id(impl Into<String>)`](crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder::transit_gateway_id) / [`set_transit_gateway_id(Option<String>)`](crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder::set_transit_gateway_id):<br>required: **true**<br><p>The ID of the transit gateway.</p><br>
    ///   - [`options(CreateTransitGatewayMulticastDomainRequestOptions)`](crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder::options) / [`set_options(Option<CreateTransitGatewayMulticastDomainRequestOptions>)`](crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder::set_options):<br>required: **false**<br><p>The options for the transit gateway multicast domain.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags for the transit gateway multicast domain.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`CreateTransitGatewayMulticastDomainOutput`](crate::operation::create_transit_gateway_multicast_domain::CreateTransitGatewayMulticastDomainOutput) with field(s):
    ///   - [`transit_gateway_multicast_domain(Option<TransitGatewayMulticastDomain>)`](crate::operation::create_transit_gateway_multicast_domain::CreateTransitGatewayMulticastDomainOutput::transit_gateway_multicast_domain): <p>Information about the transit gateway multicast domain.</p>
    /// - On failure, responds with [`SdkError<CreateTransitGatewayMulticastDomainError>`](crate::operation::create_transit_gateway_multicast_domain::CreateTransitGatewayMulticastDomainError)
    pub fn create_transit_gateway_multicast_domain(
        &self,
    ) -> crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder {
        crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainFluentBuilder::new(
            self.handle.clone(),
        )
    }
}