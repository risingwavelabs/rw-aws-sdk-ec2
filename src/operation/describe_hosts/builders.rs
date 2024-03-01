// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_hosts::_describe_hosts_output::DescribeHostsOutputBuilder;

pub use crate::operation::describe_hosts::_describe_hosts_input::DescribeHostsInputBuilder;

impl DescribeHostsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_hosts::DescribeHostsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_hosts::DescribeHostsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_hosts();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeHosts`.
///
/// <p>Describes the specified Dedicated Hosts or all your Dedicated Hosts.</p>
/// <p>The results describe only the Dedicated Hosts in the Region you're currently using. All listed instances consume capacity on your Dedicated Host. Dedicated Hosts that have recently been released are listed with the state <code>released</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeHostsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_hosts::builders::DescribeHostsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_hosts::DescribeHostsOutput,
        crate::operation::describe_hosts::DescribeHostsError,
    > for DescribeHostsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_hosts::DescribeHostsOutput,
            crate::operation::describe_hosts::DescribeHostsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeHostsFluentBuilder {
    /// Creates a new `DescribeHosts`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeHosts as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_hosts::builders::DescribeHostsInputBuilder {
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
        crate::operation::describe_hosts::DescribeHostsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_hosts::DescribeHostsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_hosts::DescribeHosts::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_hosts::DescribeHosts::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_hosts::DescribeHostsOutput,
        crate::operation::describe_hosts::DescribeHostsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_hosts::paginator::DescribeHostsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_hosts::paginator::DescribeHostsPaginator {
        crate::operation::describe_hosts::paginator::DescribeHostsPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `Filter`.
    ///
    /// To override the contents of this collection use [`set_filter`](Self::set_filter).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>auto-placement</code> - Whether auto-placement is enabled or disabled (<code>on</code> | <code>off</code>).</p> </li>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone of the host.</p> </li>
    /// <li> <p> <code>client-token</code> - The idempotency token that you provided when you allocated the host.</p> </li>
    /// <li> <p> <code>host-reservation-id</code> - The ID of the reservation assigned to this host.</p> </li>
    /// <li> <p> <code>instance-type</code> - The instance type size that the Dedicated Host is configured to support.</p> </li>
    /// <li> <p> <code>state</code> - The allocation state of the Dedicated Host (<code>available</code> | <code>under-assessment</code> | <code>permanent-failure</code> | <code>released</code> | <code>released-permanent-failure</code>).</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn filter(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filter(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>auto-placement</code> - Whether auto-placement is enabled or disabled (<code>on</code> | <code>off</code>).</p> </li>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone of the host.</p> </li>
    /// <li> <p> <code>client-token</code> - The idempotency token that you provided when you allocated the host.</p> </li>
    /// <li> <p> <code>host-reservation-id</code> - The ID of the reservation assigned to this host.</p> </li>
    /// <li> <p> <code>instance-type</code> - The instance type size that the Dedicated Host is configured to support.</p> </li>
    /// <li> <p> <code>state</code> - The allocation state of the Dedicated Host (<code>available</code> | <code>under-assessment</code> | <code>permanent-failure</code> | <code>released</code> | <code>released-permanent-failure</code>).</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn set_filter(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filter(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>auto-placement</code> - Whether auto-placement is enabled or disabled (<code>on</code> | <code>off</code>).</p> </li>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone of the host.</p> </li>
    /// <li> <p> <code>client-token</code> - The idempotency token that you provided when you allocated the host.</p> </li>
    /// <li> <p> <code>host-reservation-id</code> - The ID of the reservation assigned to this host.</p> </li>
    /// <li> <p> <code>instance-type</code> - The instance type size that the Dedicated Host is configured to support.</p> </li>
    /// <li> <p> <code>state</code> - The allocation state of the Dedicated Host (<code>available</code> | <code>under-assessment</code> | <code>permanent-failure</code> | <code>released</code> | <code>released-permanent-failure</code>).</p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// </ul>
    pub fn get_filter(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filter()
    }
    /// Appends an item to `HostIds`.
    ///
    /// To override the contents of this collection use [`set_host_ids`](Self::set_host_ids).
    ///
    /// <p>The IDs of the Dedicated Hosts. The IDs are used for targeted instance launches.</p>
    pub fn host_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.host_ids(input.into());
        self
    }
    /// <p>The IDs of the Dedicated Hosts. The IDs are used for targeted instance launches.</p>
    pub fn set_host_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_host_ids(input);
        self
    }
    /// <p>The IDs of the Dedicated Hosts. The IDs are used for targeted instance launches.</p>
    pub fn get_host_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_host_ids()
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    /// <p>You cannot specify this parameter and the host IDs parameter in the same request.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    /// <p>You cannot specify this parameter and the host IDs parameter in the same request.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return for the request in a single page. The remaining results can be seen by sending another request with the returned <code>nextToken</code> value. This value can be between 5 and 500. If <code>maxResults</code> is given a larger value than 500, you receive an error.</p>
    /// <p>You cannot specify this parameter and the host IDs parameter in the same request.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token to use to retrieve the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}