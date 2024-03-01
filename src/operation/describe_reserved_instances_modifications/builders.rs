// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_reserved_instances_modifications::_describe_reserved_instances_modifications_output::DescribeReservedInstancesModificationsOutputBuilder;

pub use crate::operation::describe_reserved_instances_modifications::_describe_reserved_instances_modifications_input::DescribeReservedInstancesModificationsInputBuilder;

impl DescribeReservedInstancesModificationsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_reserved_instances_modifications();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeReservedInstancesModifications`.
///
/// <p>Describes the modifications made to your Reserved Instances. If no parameter is specified, information about all your Reserved Instances modification requests is returned. If a modification ID is specified, only information about the specific modification is returned.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ri-modifying.html">Modifying Reserved Instances</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeReservedInstancesModificationsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_reserved_instances_modifications::builders::DescribeReservedInstancesModificationsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsOutput,
        crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsError,
    > for DescribeReservedInstancesModificationsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsOutput,
            crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeReservedInstancesModificationsFluentBuilder {
    /// Creates a new `DescribeReservedInstancesModifications`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeReservedInstancesModifications as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_reserved_instances_modifications::builders::DescribeReservedInstancesModificationsInputBuilder {
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
        crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModifications::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModifications::orchestrate(&runtime_plugins, input)
            .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsOutput,
        crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_reserved_instances_modifications::paginator::DescribeReservedInstancesModificationsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_reserved_instances_modifications::paginator::DescribeReservedInstancesModificationsPaginator {
        crate::operation::describe_reserved_instances_modifications::paginator::DescribeReservedInstancesModificationsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>client-token</code> - The idempotency token for the modification request.</p> </li>
    /// <li> <p> <code>create-date</code> - The time when the modification request was created.</p> </li>
    /// <li> <p> <code>effective-date</code> - The time when the modification becomes effective.</p> </li>
    /// <li> <p> <code>modification-result.reserved-instances-id</code> - The ID for the Reserved Instances created as part of the modification request. This ID is only available when the status of the modification is <code>fulfilled</code>.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.availability-zone</code> - The Availability Zone for the new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-count </code> - The number of new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-type</code> - The instance type of the new Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances modified.</p> </li>
    /// <li> <p> <code>reserved-instances-modification-id</code> - The ID of the modification request.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instances modification request (<code>processing</code> | <code>fulfilled</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// <li> <p> <code>update-date</code> - The time when the modification request was last updated.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>client-token</code> - The idempotency token for the modification request.</p> </li>
    /// <li> <p> <code>create-date</code> - The time when the modification request was created.</p> </li>
    /// <li> <p> <code>effective-date</code> - The time when the modification becomes effective.</p> </li>
    /// <li> <p> <code>modification-result.reserved-instances-id</code> - The ID for the Reserved Instances created as part of the modification request. This ID is only available when the status of the modification is <code>fulfilled</code>.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.availability-zone</code> - The Availability Zone for the new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-count </code> - The number of new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-type</code> - The instance type of the new Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances modified.</p> </li>
    /// <li> <p> <code>reserved-instances-modification-id</code> - The ID of the modification request.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instances modification request (<code>processing</code> | <code>fulfilled</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// <li> <p> <code>update-date</code> - The time when the modification request was last updated.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>client-token</code> - The idempotency token for the modification request.</p> </li>
    /// <li> <p> <code>create-date</code> - The time when the modification request was created.</p> </li>
    /// <li> <p> <code>effective-date</code> - The time when the modification becomes effective.</p> </li>
    /// <li> <p> <code>modification-result.reserved-instances-id</code> - The ID for the Reserved Instances created as part of the modification request. This ID is only available when the status of the modification is <code>fulfilled</code>.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.availability-zone</code> - The Availability Zone for the new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-count </code> - The number of new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-type</code> - The instance type of the new Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances modified.</p> </li>
    /// <li> <p> <code>reserved-instances-modification-id</code> - The ID of the modification request.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instances modification request (<code>processing</code> | <code>fulfilled</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// <li> <p> <code>update-date</code> - The time when the modification request was last updated.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// Appends an item to `ReservedInstancesModificationIds`.
    ///
    /// To override the contents of this collection use [`set_reserved_instances_modification_ids`](Self::set_reserved_instances_modification_ids).
    ///
    /// <p>IDs for the submitted modification request.</p>
    pub fn reserved_instances_modification_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reserved_instances_modification_ids(input.into());
        self
    }
    /// <p>IDs for the submitted modification request.</p>
    pub fn set_reserved_instances_modification_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_reserved_instances_modification_ids(input);
        self
    }
    /// <p>IDs for the submitted modification request.</p>
    pub fn get_reserved_instances_modification_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_reserved_instances_modification_ids()
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}