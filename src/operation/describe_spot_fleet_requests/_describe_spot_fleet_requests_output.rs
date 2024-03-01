// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of DescribeSpotFleetRequests.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSpotFleetRequestsOutput {
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Information about the configuration of your Spot Fleet.</p>
    pub spot_fleet_request_configs: ::std::option::Option<::std::vec::Vec<crate::types::SpotFleetRequestConfig>>,
    _request_id: Option<String>,
}
impl DescribeSpotFleetRequestsOutput {
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Information about the configuration of your Spot Fleet.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.spot_fleet_request_configs.is_none()`.
    pub fn spot_fleet_request_configs(&self) -> &[crate::types::SpotFleetRequestConfig] {
        self.spot_fleet_request_configs.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DescribeSpotFleetRequestsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeSpotFleetRequestsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSpotFleetRequestsOutput`](crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsOutput).
    pub fn builder() -> crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsOutputBuilder {
        crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsOutputBuilder::default()
    }
}

/// A builder for [`DescribeSpotFleetRequestsOutput`](crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeSpotFleetRequestsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) spot_fleet_request_configs: ::std::option::Option<::std::vec::Vec<crate::types::SpotFleetRequestConfig>>,
    _request_id: Option<String>,
}
impl DescribeSpotFleetRequestsOutputBuilder {
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Appends an item to `spot_fleet_request_configs`.
    ///
    /// To override the contents of this collection use [`set_spot_fleet_request_configs`](Self::set_spot_fleet_request_configs).
    ///
    /// <p>Information about the configuration of your Spot Fleet.</p>
    pub fn spot_fleet_request_configs(mut self, input: crate::types::SpotFleetRequestConfig) -> Self {
        let mut v = self.spot_fleet_request_configs.unwrap_or_default();
        v.push(input);
        self.spot_fleet_request_configs = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the configuration of your Spot Fleet.</p>
    pub fn set_spot_fleet_request_configs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SpotFleetRequestConfig>>) -> Self {
        self.spot_fleet_request_configs = input;
        self
    }
    /// <p>Information about the configuration of your Spot Fleet.</p>
    pub fn get_spot_fleet_request_configs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SpotFleetRequestConfig>> {
        &self.spot_fleet_request_configs
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeSpotFleetRequestsOutput`](crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsOutput).
    pub fn build(self) -> crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsOutput {
        crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsOutput {
            next_token: self.next_token,
            spot_fleet_request_configs: self.spot_fleet_request_configs,
            _request_id: self._request_id,
        }
    }
}
