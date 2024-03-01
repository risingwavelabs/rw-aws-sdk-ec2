// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_client_vpn_target_network::_disassociate_client_vpn_target_network_output::DisassociateClientVpnTargetNetworkOutputBuilder;

pub use crate::operation::disassociate_client_vpn_target_network::_disassociate_client_vpn_target_network_input::DisassociateClientVpnTargetNetworkInputBuilder;

impl DisassociateClientVpnTargetNetworkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_client_vpn_target_network();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateClientVpnTargetNetwork`.
///
/// <p>Disassociates a target network from the specified Client VPN endpoint. When you disassociate the last target network from a Client VPN, the following happens:</p>
/// <ul>
/// <li> <p>The route that was automatically added for the VPC is deleted</p> </li>
/// <li> <p>All active client connections are terminated</p> </li>
/// <li> <p>New client connections are disallowed</p> </li>
/// <li> <p>The Client VPN endpoint's status changes to <code>pending-associate</code> </p> </li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateClientVpnTargetNetworkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_client_vpn_target_network::builders::DisassociateClientVpnTargetNetworkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkOutput,
        crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkError,
    > for DisassociateClientVpnTargetNetworkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkOutput,
            crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateClientVpnTargetNetworkFluentBuilder {
    /// Creates a new `DisassociateClientVpnTargetNetwork`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateClientVpnTargetNetwork as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_client_vpn_target_network::builders::DisassociateClientVpnTargetNetworkInputBuilder {
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
        crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetwork::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetwork::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkOutput,
        crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkError,
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
    /// <p>The ID of the Client VPN endpoint from which to disassociate the target network.</p>
    pub fn client_vpn_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_vpn_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint from which to disassociate the target network.</p>
    pub fn set_client_vpn_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_vpn_endpoint_id(input);
        self
    }
    /// <p>The ID of the Client VPN endpoint from which to disassociate the target network.</p>
    pub fn get_client_vpn_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_vpn_endpoint_id()
    }
    /// <p>The ID of the target network association.</p>
    pub fn association_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.association_id(input.into());
        self
    }
    /// <p>The ID of the target network association.</p>
    pub fn set_association_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_association_id(input);
        self
    }
    /// <p>The ID of the target network association.</p>
    pub fn get_association_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_association_id()
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
