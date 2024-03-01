// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAwsNetworkPerformanceMetricSubscriptionsOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Describes the current Infrastructure Performance subscriptions.</p>
    pub subscriptions: ::std::option::Option<::std::vec::Vec<crate::types::Subscription>>,
    _request_id: Option<String>,
}
impl DescribeAwsNetworkPerformanceMetricSubscriptionsOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Describes the current Infrastructure Performance subscriptions.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subscriptions.is_none()`.
    pub fn subscriptions(&self) -> &[crate::types::Subscription] {
        self.subscriptions.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DescribeAwsNetworkPerformanceMetricSubscriptionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeAwsNetworkPerformanceMetricSubscriptionsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAwsNetworkPerformanceMetricSubscriptionsOutput`](crate::operation::describe_aws_network_performance_metric_subscriptions::DescribeAwsNetworkPerformanceMetricSubscriptionsOutput).
    pub fn builder() -> crate::operation::describe_aws_network_performance_metric_subscriptions::builders::DescribeAwsNetworkPerformanceMetricSubscriptionsOutputBuilder{
        crate::operation::describe_aws_network_performance_metric_subscriptions::builders::DescribeAwsNetworkPerformanceMetricSubscriptionsOutputBuilder::default()
    }
}

/// A builder for [`DescribeAwsNetworkPerformanceMetricSubscriptionsOutput`](crate::operation::describe_aws_network_performance_metric_subscriptions::DescribeAwsNetworkPerformanceMetricSubscriptionsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeAwsNetworkPerformanceMetricSubscriptionsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) subscriptions: ::std::option::Option<::std::vec::Vec<crate::types::Subscription>>,
    _request_id: Option<String>,
}
impl DescribeAwsNetworkPerformanceMetricSubscriptionsOutputBuilder {
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
    /// Appends an item to `subscriptions`.
    ///
    /// To override the contents of this collection use [`set_subscriptions`](Self::set_subscriptions).
    ///
    /// <p>Describes the current Infrastructure Performance subscriptions.</p>
    pub fn subscriptions(mut self, input: crate::types::Subscription) -> Self {
        let mut v = self.subscriptions.unwrap_or_default();
        v.push(input);
        self.subscriptions = ::std::option::Option::Some(v);
        self
    }
    /// <p>Describes the current Infrastructure Performance subscriptions.</p>
    pub fn set_subscriptions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Subscription>>) -> Self {
        self.subscriptions = input;
        self
    }
    /// <p>Describes the current Infrastructure Performance subscriptions.</p>
    pub fn get_subscriptions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Subscription>> {
        &self.subscriptions
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAwsNetworkPerformanceMetricSubscriptionsOutput`](crate::operation::describe_aws_network_performance_metric_subscriptions::DescribeAwsNetworkPerformanceMetricSubscriptionsOutput).
    pub fn build(
        self,
    ) -> crate::operation::describe_aws_network_performance_metric_subscriptions::DescribeAwsNetworkPerformanceMetricSubscriptionsOutput {
        crate::operation::describe_aws_network_performance_metric_subscriptions::DescribeAwsNetworkPerformanceMetricSubscriptionsOutput {
            next_token: self.next_token,
            subscriptions: self.subscriptions,
            _request_id: self._request_id,
        }
    }
}