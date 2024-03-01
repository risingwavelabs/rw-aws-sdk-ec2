// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTransitGatewayMulticastDomainOutput {
    /// <p>Information about the transit gateway multicast domain.</p>
    pub transit_gateway_multicast_domain: ::std::option::Option<crate::types::TransitGatewayMulticastDomain>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayMulticastDomainOutput {
    /// <p>Information about the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain(&self) -> ::std::option::Option<&crate::types::TransitGatewayMulticastDomain> {
        self.transit_gateway_multicast_domain.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for CreateTransitGatewayMulticastDomainOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateTransitGatewayMulticastDomainOutput {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayMulticastDomainOutput`](crate::operation::create_transit_gateway_multicast_domain::CreateTransitGatewayMulticastDomainOutput).
    pub fn builder() -> crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainOutputBuilder {
        crate::operation::create_transit_gateway_multicast_domain::builders::CreateTransitGatewayMulticastDomainOutputBuilder::default()
    }
}

/// A builder for [`CreateTransitGatewayMulticastDomainOutput`](crate::operation::create_transit_gateway_multicast_domain::CreateTransitGatewayMulticastDomainOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateTransitGatewayMulticastDomainOutputBuilder {
    pub(crate) transit_gateway_multicast_domain: ::std::option::Option<crate::types::TransitGatewayMulticastDomain>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayMulticastDomainOutputBuilder {
    /// <p>Information about the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain(mut self, input: crate::types::TransitGatewayMulticastDomain) -> Self {
        self.transit_gateway_multicast_domain = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain(mut self, input: ::std::option::Option<crate::types::TransitGatewayMulticastDomain>) -> Self {
        self.transit_gateway_multicast_domain = input;
        self
    }
    /// <p>Information about the transit gateway multicast domain.</p>
    pub fn get_transit_gateway_multicast_domain(&self) -> &::std::option::Option<crate::types::TransitGatewayMulticastDomain> {
        &self.transit_gateway_multicast_domain
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateTransitGatewayMulticastDomainOutput`](crate::operation::create_transit_gateway_multicast_domain::CreateTransitGatewayMulticastDomainOutput).
    pub fn build(self) -> crate::operation::create_transit_gateway_multicast_domain::CreateTransitGatewayMulticastDomainOutput {
        crate::operation::create_transit_gateway_multicast_domain::CreateTransitGatewayMulticastDomainOutput {
            transit_gateway_multicast_domain: self.transit_gateway_multicast_domain,
            _request_id: self._request_id,
        }
    }
}
