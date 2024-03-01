// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_associated_ipv6_pool_cidrs::_get_associated_ipv6_pool_cidrs_output::GetAssociatedIpv6PoolCidrsOutputBuilder;

pub use crate::operation::get_associated_ipv6_pool_cidrs::_get_associated_ipv6_pool_cidrs_input::GetAssociatedIpv6PoolCidrsInputBuilder;

impl GetAssociatedIpv6PoolCidrsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_associated_ipv6_pool_cidrs();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetAssociatedIpv6PoolCidrs`.
///
/// <p>Gets information about the IPv6 CIDR block associations for a specified IPv6 address pool.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetAssociatedIpv6PoolCidrsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_associated_ipv6_pool_cidrs::builders::GetAssociatedIpv6PoolCidrsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput,
        crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsError,
    > for GetAssociatedIpv6PoolCidrsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput,
            crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetAssociatedIpv6PoolCidrsFluentBuilder {
    /// Creates a new `GetAssociatedIpv6PoolCidrs`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetAssociatedIpv6PoolCidrs as a reference.
    pub fn as_input(&self) -> &crate::operation::get_associated_ipv6_pool_cidrs::builders::GetAssociatedIpv6PoolCidrsInputBuilder {
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
        crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrs::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrs::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsOutput,
        crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::get_associated_ipv6_pool_cidrs::paginator::GetAssociatedIpv6PoolCidrsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::get_associated_ipv6_pool_cidrs::paginator::GetAssociatedIpv6PoolCidrsPaginator {
        crate::operation::get_associated_ipv6_pool_cidrs::paginator::GetAssociatedIpv6PoolCidrsPaginator::new(self.handle, self.inner)
    }
    /// <p>The ID of the IPv6 address pool.</p>
    pub fn pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.pool_id(input.into());
        self
    }
    /// <p>The ID of the IPv6 address pool.</p>
    pub fn set_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_pool_id(input);
        self
    }
    /// <p>The ID of the IPv6 address pool.</p>
    pub fn get_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_pool_id()
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
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
}