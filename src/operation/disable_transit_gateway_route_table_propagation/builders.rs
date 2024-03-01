// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disable_transit_gateway_route_table_propagation::_disable_transit_gateway_route_table_propagation_output::DisableTransitGatewayRouteTablePropagationOutputBuilder;

pub use crate::operation::disable_transit_gateway_route_table_propagation::_disable_transit_gateway_route_table_propagation_input::DisableTransitGatewayRouteTablePropagationInputBuilder;

impl DisableTransitGatewayRouteTablePropagationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disable_transit_gateway_route_table_propagation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisableTransitGatewayRouteTablePropagation`.
///
/// <p>Disables the specified resource attachment from propagating routes to the specified propagation route table.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisableTransitGatewayRouteTablePropagationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disable_transit_gateway_route_table_propagation::builders::DisableTransitGatewayRouteTablePropagationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationOutput,
        crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationError,
    > for DisableTransitGatewayRouteTablePropagationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationOutput,
            crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisableTransitGatewayRouteTablePropagationFluentBuilder {
    /// Creates a new `DisableTransitGatewayRouteTablePropagation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisableTransitGatewayRouteTablePropagation as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::disable_transit_gateway_route_table_propagation::builders::DisableTransitGatewayRouteTablePropagationInputBuilder {
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
        crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagation::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagation::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationOutput,
        crate::operation::disable_transit_gateway_route_table_propagation::DisableTransitGatewayRouteTablePropagationError,
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
    /// <p>The ID of the propagation route table.</p>
    pub fn transit_gateway_route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_route_table_id(input.into());
        self
    }
    /// <p>The ID of the propagation route table.</p>
    pub fn set_transit_gateway_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transit_gateway_route_table_id(input);
        self
    }
    /// <p>The ID of the propagation route table.</p>
    pub fn get_transit_gateway_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transit_gateway_route_table_id()
    }
    /// <p>The ID of the attachment.</p>
    pub fn transit_gateway_attachment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_attachment_id(input.into());
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn set_transit_gateway_attachment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transit_gateway_attachment_id(input);
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn get_transit_gateway_attachment_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transit_gateway_attachment_id()
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
    /// <p>The ID of the route table announcement.</p>
    pub fn transit_gateway_route_table_announcement_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transit_gateway_route_table_announcement_id(input.into());
        self
    }
    /// <p>The ID of the route table announcement.</p>
    pub fn set_transit_gateway_route_table_announcement_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transit_gateway_route_table_announcement_id(input);
        self
    }
    /// <p>The ID of the route table announcement.</p>
    pub fn get_transit_gateway_route_table_announcement_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transit_gateway_route_table_announcement_id()
    }
}