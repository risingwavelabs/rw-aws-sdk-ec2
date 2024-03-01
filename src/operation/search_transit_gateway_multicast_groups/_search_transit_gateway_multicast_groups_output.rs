// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SearchTransitGatewayMulticastGroupsOutput {
    /// <p>Information about the transit gateway multicast group.</p>
    pub multicast_groups: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayMulticastGroup>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl SearchTransitGatewayMulticastGroupsOutput {
    /// <p>Information about the transit gateway multicast group.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.multicast_groups.is_none()`.
    pub fn multicast_groups(&self) -> &[crate::types::TransitGatewayMulticastGroup] {
        self.multicast_groups.as_deref().unwrap_or_default()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for SearchTransitGatewayMulticastGroupsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SearchTransitGatewayMulticastGroupsOutput {
    /// Creates a new builder-style object to manufacture [`SearchTransitGatewayMulticastGroupsOutput`](crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsOutput).
    pub fn builder() -> crate::operation::search_transit_gateway_multicast_groups::builders::SearchTransitGatewayMulticastGroupsOutputBuilder {
        crate::operation::search_transit_gateway_multicast_groups::builders::SearchTransitGatewayMulticastGroupsOutputBuilder::default()
    }
}

/// A builder for [`SearchTransitGatewayMulticastGroupsOutput`](crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SearchTransitGatewayMulticastGroupsOutputBuilder {
    pub(crate) multicast_groups: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayMulticastGroup>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl SearchTransitGatewayMulticastGroupsOutputBuilder {
    /// Appends an item to `multicast_groups`.
    ///
    /// To override the contents of this collection use [`set_multicast_groups`](Self::set_multicast_groups).
    ///
    /// <p>Information about the transit gateway multicast group.</p>
    pub fn multicast_groups(mut self, input: crate::types::TransitGatewayMulticastGroup) -> Self {
        let mut v = self.multicast_groups.unwrap_or_default();
        v.push(input);
        self.multicast_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the transit gateway multicast group.</p>
    pub fn set_multicast_groups(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayMulticastGroup>>) -> Self {
        self.multicast_groups = input;
        self
    }
    /// <p>Information about the transit gateway multicast group.</p>
    pub fn get_multicast_groups(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayMulticastGroup>> {
        &self.multicast_groups
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`SearchTransitGatewayMulticastGroupsOutput`](crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsOutput).
    pub fn build(self) -> crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsOutput {
        crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsOutput {
            multicast_groups: self.multicast_groups,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
