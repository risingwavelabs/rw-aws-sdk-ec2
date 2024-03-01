// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableAwsNetworkPerformanceMetricSubscriptionInput {
    /// <p>The source Region or Availability Zone that the metric subscription is enabled for. For example, <code>us-east-1</code>.</p>
    pub source: ::std::option::Option<::std::string::String>,
    /// <p>The target Region or Availability Zone that the metric subscription is enabled for. For example, <code>eu-west-1</code>.</p>
    pub destination: ::std::option::Option<::std::string::String>,
    /// <p>The metric used for the enabled subscription.</p>
    pub metric: ::std::option::Option<crate::types::MetricType>,
    /// <p>The statistic used for the enabled subscription.</p>
    pub statistic: ::std::option::Option<crate::types::StatisticType>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl EnableAwsNetworkPerformanceMetricSubscriptionInput {
    /// <p>The source Region or Availability Zone that the metric subscription is enabled for. For example, <code>us-east-1</code>.</p>
    pub fn source(&self) -> ::std::option::Option<&str> {
        self.source.as_deref()
    }
    /// <p>The target Region or Availability Zone that the metric subscription is enabled for. For example, <code>eu-west-1</code>.</p>
    pub fn destination(&self) -> ::std::option::Option<&str> {
        self.destination.as_deref()
    }
    /// <p>The metric used for the enabled subscription.</p>
    pub fn metric(&self) -> ::std::option::Option<&crate::types::MetricType> {
        self.metric.as_ref()
    }
    /// <p>The statistic used for the enabled subscription.</p>
    pub fn statistic(&self) -> ::std::option::Option<&crate::types::StatisticType> {
        self.statistic.as_ref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl EnableAwsNetworkPerformanceMetricSubscriptionInput {
    /// Creates a new builder-style object to manufacture [`EnableAwsNetworkPerformanceMetricSubscriptionInput`](crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionInput).
    pub fn builder(
    ) -> crate::operation::enable_aws_network_performance_metric_subscription::builders::EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder
    {
        crate::operation::enable_aws_network_performance_metric_subscription::builders::EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder::default()
    }
}

/// A builder for [`EnableAwsNetworkPerformanceMetricSubscriptionInput`](crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder {
    pub(crate) source: ::std::option::Option<::std::string::String>,
    pub(crate) destination: ::std::option::Option<::std::string::String>,
    pub(crate) metric: ::std::option::Option<crate::types::MetricType>,
    pub(crate) statistic: ::std::option::Option<crate::types::StatisticType>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl EnableAwsNetworkPerformanceMetricSubscriptionInputBuilder {
    /// <p>The source Region or Availability Zone that the metric subscription is enabled for. For example, <code>us-east-1</code>.</p>
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source Region or Availability Zone that the metric subscription is enabled for. For example, <code>us-east-1</code>.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source = input;
        self
    }
    /// <p>The source Region or Availability Zone that the metric subscription is enabled for. For example, <code>us-east-1</code>.</p>
    pub fn get_source(&self) -> &::std::option::Option<::std::string::String> {
        &self.source
    }
    /// <p>The target Region or Availability Zone that the metric subscription is enabled for. For example, <code>eu-west-1</code>.</p>
    pub fn destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The target Region or Availability Zone that the metric subscription is enabled for. For example, <code>eu-west-1</code>.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination = input;
        self
    }
    /// <p>The target Region or Availability Zone that the metric subscription is enabled for. For example, <code>eu-west-1</code>.</p>
    pub fn get_destination(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination
    }
    /// <p>The metric used for the enabled subscription.</p>
    pub fn metric(mut self, input: crate::types::MetricType) -> Self {
        self.metric = ::std::option::Option::Some(input);
        self
    }
    /// <p>The metric used for the enabled subscription.</p>
    pub fn set_metric(mut self, input: ::std::option::Option<crate::types::MetricType>) -> Self {
        self.metric = input;
        self
    }
    /// <p>The metric used for the enabled subscription.</p>
    pub fn get_metric(&self) -> &::std::option::Option<crate::types::MetricType> {
        &self.metric
    }
    /// <p>The statistic used for the enabled subscription.</p>
    pub fn statistic(mut self, input: crate::types::StatisticType) -> Self {
        self.statistic = ::std::option::Option::Some(input);
        self
    }
    /// <p>The statistic used for the enabled subscription.</p>
    pub fn set_statistic(mut self, input: ::std::option::Option<crate::types::StatisticType>) -> Self {
        self.statistic = input;
        self
    }
    /// <p>The statistic used for the enabled subscription.</p>
    pub fn get_statistic(&self) -> &::std::option::Option<crate::types::StatisticType> {
        &self.statistic
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`EnableAwsNetworkPerformanceMetricSubscriptionInput`](crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::enable_aws_network_performance_metric_subscription::EnableAwsNetworkPerformanceMetricSubscriptionInput {
                source: self.source,
                destination: self.destination,
                metric: self.metric,
                statistic: self.statistic,
                dry_run: self.dry_run,
            },
        )
    }
}
