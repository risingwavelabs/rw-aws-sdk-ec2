// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_transit_gateway_route_table_announcements::_describe_transit_gateway_route_table_announcements_output::DescribeTransitGatewayRouteTableAnnouncementsOutputBuilder;

pub use crate::operation::describe_transit_gateway_route_table_announcements::_describe_transit_gateway_route_table_announcements_input::DescribeTransitGatewayRouteTableAnnouncementsInputBuilder;

impl DescribeTransitGatewayRouteTableAnnouncementsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_transit_gateway_route_table_announcements();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeTransitGatewayRouteTableAnnouncements`.
///
/// <p>Describes one or more transit gateway route table advertisements.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeTransitGatewayRouteTableAnnouncementsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_transit_gateway_route_table_announcements::builders::DescribeTransitGatewayRouteTableAnnouncementsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsOutput,
        crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsError,
    > for DescribeTransitGatewayRouteTableAnnouncementsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsOutput,
            crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeTransitGatewayRouteTableAnnouncementsFluentBuilder {
    /// Creates a new `DescribeTransitGatewayRouteTableAnnouncements`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeTransitGatewayRouteTableAnnouncements as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::describe_transit_gateway_route_table_announcements::builders::DescribeTransitGatewayRouteTableAnnouncementsInputBuilder
    {
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
        crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncements::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncements::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsOutput,
        crate::operation::describe_transit_gateway_route_table_announcements::DescribeTransitGatewayRouteTableAnnouncementsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_transit_gateway_route_table_announcements::paginator::DescribeTransitGatewayRouteTableAnnouncementsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(
        self,
    ) -> crate::operation::describe_transit_gateway_route_table_announcements::paginator::DescribeTransitGatewayRouteTableAnnouncementsPaginator {
        crate::operation::describe_transit_gateway_route_table_announcements::paginator::DescribeTransitGatewayRouteTableAnnouncementsPaginator::new(
            self.handle,
            self.inner,
        )
    }
    /// Appends an item to `TransitGatewayRouteTableAnnouncementIds`.
    ///
    /// To override the contents of this collection use [`set_transit_gateway_route_table_announcement_ids`](Self::set_transit_gateway_route_table_announcement_ids).
    ///
    /// <p>The IDs of the transit gateway route tables that are being advertised.</p>
    pub fn transit_gateway_route_table_announcement_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_route_table_announcement_ids(input.into());
        self
    }
    /// <p>The IDs of the transit gateway route tables that are being advertised.</p>
    pub fn set_transit_gateway_route_table_announcement_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_transit_gateway_route_table_announcement_ids(input);
        self
    }
    /// <p>The IDs of the transit gateway route tables that are being advertised.</p>
    pub fn get_transit_gateway_route_table_announcement_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_transit_gateway_route_table_announcement_ids()
    }
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters associated with the transit gateway policy table.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters associated with the transit gateway policy table.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filters associated with the transit gateway policy table.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
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
