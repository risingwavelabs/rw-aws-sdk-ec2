// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_scheduled_instances::_describe_scheduled_instances_output::DescribeScheduledInstancesOutputBuilder;

pub use crate::operation::describe_scheduled_instances::_describe_scheduled_instances_input::DescribeScheduledInstancesInputBuilder;

impl DescribeScheduledInstancesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_scheduled_instances::DescribeScheduledInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_scheduled_instances::DescribeScheduledInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_scheduled_instances();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeScheduledInstances`.
///
/// <p>Describes the specified Scheduled Instances or all your Scheduled Instances.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeScheduledInstancesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_scheduled_instances::builders::DescribeScheduledInstancesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_scheduled_instances::DescribeScheduledInstancesOutput,
        crate::operation::describe_scheduled_instances::DescribeScheduledInstancesError,
    > for DescribeScheduledInstancesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_scheduled_instances::DescribeScheduledInstancesOutput,
            crate::operation::describe_scheduled_instances::DescribeScheduledInstancesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeScheduledInstancesFluentBuilder {
    /// Creates a new `DescribeScheduledInstances`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeScheduledInstances as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_scheduled_instances::builders::DescribeScheduledInstancesInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_scheduled_instances::DescribeScheduledInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_scheduled_instances::DescribeScheduledInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_scheduled_instances::DescribeScheduledInstances::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_scheduled_instances::DescribeScheduledInstances::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_scheduled_instances::DescribeScheduledInstancesOutput,
        crate::operation::describe_scheduled_instances::DescribeScheduledInstancesError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::describe_scheduled_instances::paginator::DescribeScheduledInstancesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_scheduled_instances::paginator::DescribeScheduledInstancesPaginator {
        crate::operation::describe_scheduled_instances::paginator::DescribeScheduledInstancesPaginator::new(self.handle, self.inner)
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone (for example, <code>us-west-2a</code>).</p> </li>
    /// <li> <p> <code>instance-type</code> - The instance type (for example, <code>c4.large</code>).</p> </li>
    /// <li> <p> <code>platform</code> - The platform (<code>Linux/UNIX</code> or <code>Windows</code>).</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone (for example, <code>us-west-2a</code>).</p> </li>
    /// <li> <p> <code>instance-type</code> - The instance type (for example, <code>c4.large</code>).</p> </li>
    /// <li> <p> <code>platform</code> - The platform (<code>Linux/UNIX</code> or <code>Windows</code>).</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone (for example, <code>us-west-2a</code>).</p> </li>
    /// <li> <p> <code>instance-type</code> - The instance type (for example, <code>c4.large</code>).</p> </li>
    /// <li> <p> <code>platform</code> - The platform (<code>Linux/UNIX</code> or <code>Windows</code>).</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>The maximum number of results to return in a single call. This value can be between 5 and 300. The default value is 100. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. This value can be between 5 and 300. The default value is 100. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. This value can be between 5 and 300. The default value is 100. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token for the next set of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// Appends an item to `ScheduledInstanceIds`.
    ///
    /// To override the contents of this collection use [`set_scheduled_instance_ids`](Self::set_scheduled_instance_ids).
    ///
    /// <p>The Scheduled Instance IDs.</p>
    pub fn scheduled_instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.scheduled_instance_ids(input.into());
        self
    }
    /// <p>The Scheduled Instance IDs.</p>
    pub fn set_scheduled_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_scheduled_instance_ids(input);
        self
    }
    /// <p>The Scheduled Instance IDs.</p>
    pub fn get_scheduled_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_scheduled_instance_ids()
    }
    /// <p>The time period for the first schedule to start.</p>
    pub fn slot_start_time_range(mut self, input: crate::types::SlotStartTimeRangeRequest) -> Self {
        self.inner = self.inner.slot_start_time_range(input);
        self
    }
    /// <p>The time period for the first schedule to start.</p>
    pub fn set_slot_start_time_range(mut self, input: ::std::option::Option<crate::types::SlotStartTimeRangeRequest>) -> Self {
        self.inner = self.inner.set_slot_start_time_range(input);
        self
    }
    /// <p>The time period for the first schedule to start.</p>
    pub fn get_slot_start_time_range(&self) -> &::std::option::Option<crate::types::SlotStartTimeRangeRequest> {
        self.inner.get_slot_start_time_range()
    }
}
