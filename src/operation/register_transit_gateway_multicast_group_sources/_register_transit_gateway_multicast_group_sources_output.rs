// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterTransitGatewayMulticastGroupSourcesOutput {
    /// <p>Information about the transit gateway multicast group sources.</p>
    pub registered_multicast_group_sources: ::std::option::Option<crate::types::TransitGatewayMulticastRegisteredGroupSources>,
    _request_id: Option<String>,
}
impl RegisterTransitGatewayMulticastGroupSourcesOutput {
    /// <p>Information about the transit gateway multicast group sources.</p>
    pub fn registered_multicast_group_sources(&self) -> ::std::option::Option<&crate::types::TransitGatewayMulticastRegisteredGroupSources> {
        self.registered_multicast_group_sources.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for RegisterTransitGatewayMulticastGroupSourcesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl RegisterTransitGatewayMulticastGroupSourcesOutput {
    /// Creates a new builder-style object to manufacture [`RegisterTransitGatewayMulticastGroupSourcesOutput`](crate::operation::register_transit_gateway_multicast_group_sources::RegisterTransitGatewayMulticastGroupSourcesOutput).
    pub fn builder(
    ) -> crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesOutputBuilder {
        crate::operation::register_transit_gateway_multicast_group_sources::builders::RegisterTransitGatewayMulticastGroupSourcesOutputBuilder::default()
    }
}

/// A builder for [`RegisterTransitGatewayMulticastGroupSourcesOutput`](crate::operation::register_transit_gateway_multicast_group_sources::RegisterTransitGatewayMulticastGroupSourcesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RegisterTransitGatewayMulticastGroupSourcesOutputBuilder {
    pub(crate) registered_multicast_group_sources: ::std::option::Option<crate::types::TransitGatewayMulticastRegisteredGroupSources>,
    _request_id: Option<String>,
}
impl RegisterTransitGatewayMulticastGroupSourcesOutputBuilder {
    /// <p>Information about the transit gateway multicast group sources.</p>
    pub fn registered_multicast_group_sources(mut self, input: crate::types::TransitGatewayMulticastRegisteredGroupSources) -> Self {
        self.registered_multicast_group_sources = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the transit gateway multicast group sources.</p>
    pub fn set_registered_multicast_group_sources(
        mut self,
        input: ::std::option::Option<crate::types::TransitGatewayMulticastRegisteredGroupSources>,
    ) -> Self {
        self.registered_multicast_group_sources = input;
        self
    }
    /// <p>Information about the transit gateway multicast group sources.</p>
    pub fn get_registered_multicast_group_sources(&self) -> &::std::option::Option<crate::types::TransitGatewayMulticastRegisteredGroupSources> {
        &self.registered_multicast_group_sources
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`RegisterTransitGatewayMulticastGroupSourcesOutput`](crate::operation::register_transit_gateway_multicast_group_sources::RegisterTransitGatewayMulticastGroupSourcesOutput).
    pub fn build(self) -> crate::operation::register_transit_gateway_multicast_group_sources::RegisterTransitGatewayMulticastGroupSourcesOutput {
        crate::operation::register_transit_gateway_multicast_group_sources::RegisterTransitGatewayMulticastGroupSourcesOutput {
            registered_multicast_group_sources: self.registered_multicast_group_sources,
            _request_id: self._request_id,
        }
    }
}
