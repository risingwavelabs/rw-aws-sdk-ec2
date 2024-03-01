// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTransitGatewayConnectsOutput {
    /// <p>Information about the Connect attachments.</p>
    pub transit_gateway_connects: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayConnect>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTransitGatewayConnectsOutput {
    /// <p>Information about the Connect attachments.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.transit_gateway_connects.is_none()`.
    pub fn transit_gateway_connects(&self) -> &[crate::types::TransitGatewayConnect] {
        self.transit_gateway_connects.as_deref().unwrap_or_default()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeTransitGatewayConnectsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeTransitGatewayConnectsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTransitGatewayConnectsOutput`](crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput).
    pub fn builder() -> crate::operation::describe_transit_gateway_connects::builders::DescribeTransitGatewayConnectsOutputBuilder {
        crate::operation::describe_transit_gateway_connects::builders::DescribeTransitGatewayConnectsOutputBuilder::default()
    }
}

/// A builder for [`DescribeTransitGatewayConnectsOutput`](crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeTransitGatewayConnectsOutputBuilder {
    pub(crate) transit_gateway_connects: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayConnect>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTransitGatewayConnectsOutputBuilder {
    /// Appends an item to `transit_gateway_connects`.
    ///
    /// To override the contents of this collection use [`set_transit_gateway_connects`](Self::set_transit_gateway_connects).
    ///
    /// <p>Information about the Connect attachments.</p>
    pub fn transit_gateway_connects(mut self, input: crate::types::TransitGatewayConnect) -> Self {
        let mut v = self.transit_gateway_connects.unwrap_or_default();
        v.push(input);
        self.transit_gateway_connects = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the Connect attachments.</p>
    pub fn set_transit_gateway_connects(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayConnect>>) -> Self {
        self.transit_gateway_connects = input;
        self
    }
    /// <p>Information about the Connect attachments.</p>
    pub fn get_transit_gateway_connects(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayConnect>> {
        &self.transit_gateway_connects
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
    /// Consumes the builder and constructs a [`DescribeTransitGatewayConnectsOutput`](crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput).
    pub fn build(self) -> crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput {
        crate::operation::describe_transit_gateway_connects::DescribeTransitGatewayConnectsOutput {
            transit_gateway_connects: self.transit_gateway_connects,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
