// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeregisterTransitGatewayMulticastGroupSourcesOutput {
    /// <p>Information about the deregistered group sources.</p>
    pub deregistered_multicast_group_sources: ::std::option::Option<crate::types::TransitGatewayMulticastDeregisteredGroupSources>,
    _request_id: Option<String>,
}
impl DeregisterTransitGatewayMulticastGroupSourcesOutput {
    /// <p>Information about the deregistered group sources.</p>
    pub fn deregistered_multicast_group_sources(&self) -> ::std::option::Option<&crate::types::TransitGatewayMulticastDeregisteredGroupSources> {
        self.deregistered_multicast_group_sources.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for DeregisterTransitGatewayMulticastGroupSourcesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeregisterTransitGatewayMulticastGroupSourcesOutput {
    /// Creates a new builder-style object to manufacture [`DeregisterTransitGatewayMulticastGroupSourcesOutput`](crate::operation::deregister_transit_gateway_multicast_group_sources::DeregisterTransitGatewayMulticastGroupSourcesOutput).
    pub fn builder(
    ) -> crate::operation::deregister_transit_gateway_multicast_group_sources::builders::DeregisterTransitGatewayMulticastGroupSourcesOutputBuilder
    {
        crate::operation::deregister_transit_gateway_multicast_group_sources::builders::DeregisterTransitGatewayMulticastGroupSourcesOutputBuilder::default()
    }
}

/// A builder for [`DeregisterTransitGatewayMulticastGroupSourcesOutput`](crate::operation::deregister_transit_gateway_multicast_group_sources::DeregisterTransitGatewayMulticastGroupSourcesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeregisterTransitGatewayMulticastGroupSourcesOutputBuilder {
    pub(crate) deregistered_multicast_group_sources: ::std::option::Option<crate::types::TransitGatewayMulticastDeregisteredGroupSources>,
    _request_id: Option<String>,
}
impl DeregisterTransitGatewayMulticastGroupSourcesOutputBuilder {
    /// <p>Information about the deregistered group sources.</p>
    pub fn deregistered_multicast_group_sources(mut self, input: crate::types::TransitGatewayMulticastDeregisteredGroupSources) -> Self {
        self.deregistered_multicast_group_sources = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the deregistered group sources.</p>
    pub fn set_deregistered_multicast_group_sources(
        mut self,
        input: ::std::option::Option<crate::types::TransitGatewayMulticastDeregisteredGroupSources>,
    ) -> Self {
        self.deregistered_multicast_group_sources = input;
        self
    }
    /// <p>Information about the deregistered group sources.</p>
    pub fn get_deregistered_multicast_group_sources(&self) -> &::std::option::Option<crate::types::TransitGatewayMulticastDeregisteredGroupSources> {
        &self.deregistered_multicast_group_sources
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeregisterTransitGatewayMulticastGroupSourcesOutput`](crate::operation::deregister_transit_gateway_multicast_group_sources::DeregisterTransitGatewayMulticastGroupSourcesOutput).
    pub fn build(self) -> crate::operation::deregister_transit_gateway_multicast_group_sources::DeregisterTransitGatewayMulticastGroupSourcesOutput {
        crate::operation::deregister_transit_gateway_multicast_group_sources::DeregisterTransitGatewayMulticastGroupSourcesOutput {
            deregistered_multicast_group_sources: self.deregistered_multicast_group_sources,
            _request_id: self._request_id,
        }
    }
}
