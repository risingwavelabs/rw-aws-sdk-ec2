// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_vpn_connection_options::_modify_vpn_connection_options_output::ModifyVpnConnectionOptionsOutputBuilder;

pub use crate::operation::modify_vpn_connection_options::_modify_vpn_connection_options_input::ModifyVpnConnectionOptionsInputBuilder;

impl ModifyVpnConnectionOptionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_vpn_connection_options();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyVpnConnectionOptions`.
///
/// <p>Modifies the connection options for your Site-to-Site VPN connection.</p>
/// <p>When you modify the VPN connection options, the VPN endpoint IP addresses on the Amazon Web Services side do not change, and the tunnel options do not change. Your VPN connection will be temporarily unavailable for a brief period while the VPN connection is updated.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyVpnConnectionOptionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_vpn_connection_options::builders::ModifyVpnConnectionOptionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput,
        crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsError,
    > for ModifyVpnConnectionOptionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput,
            crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyVpnConnectionOptionsFluentBuilder {
    /// Creates a new `ModifyVpnConnectionOptions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyVpnConnectionOptions as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_vpn_connection_options::builders::ModifyVpnConnectionOptionsInputBuilder {
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
        crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsOutput,
        crate::operation::modify_vpn_connection_options::ModifyVpnConnectionOptionsError,
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
    /// <p>The ID of the Site-to-Site VPN connection. </p>
    pub fn vpn_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpn_connection_id(input.into());
        self
    }
    /// <p>The ID of the Site-to-Site VPN connection. </p>
    pub fn set_vpn_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpn_connection_id(input);
        self
    }
    /// <p>The ID of the Site-to-Site VPN connection. </p>
    pub fn get_vpn_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpn_connection_id()
    }
    /// <p>The IPv4 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    /// <p>Default: <code>0.0.0.0/0</code> </p>
    pub fn local_ipv4_network_cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.local_ipv4_network_cidr(input.into());
        self
    }
    /// <p>The IPv4 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    /// <p>Default: <code>0.0.0.0/0</code> </p>
    pub fn set_local_ipv4_network_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_local_ipv4_network_cidr(input);
        self
    }
    /// <p>The IPv4 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    /// <p>Default: <code>0.0.0.0/0</code> </p>
    pub fn get_local_ipv4_network_cidr(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_local_ipv4_network_cidr()
    }
    /// <p>The IPv4 CIDR on the Amazon Web Services side of the VPN connection.</p>
    /// <p>Default: <code>0.0.0.0/0</code> </p>
    pub fn remote_ipv4_network_cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.remote_ipv4_network_cidr(input.into());
        self
    }
    /// <p>The IPv4 CIDR on the Amazon Web Services side of the VPN connection.</p>
    /// <p>Default: <code>0.0.0.0/0</code> </p>
    pub fn set_remote_ipv4_network_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_remote_ipv4_network_cidr(input);
        self
    }
    /// <p>The IPv4 CIDR on the Amazon Web Services side of the VPN connection.</p>
    /// <p>Default: <code>0.0.0.0/0</code> </p>
    pub fn get_remote_ipv4_network_cidr(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_remote_ipv4_network_cidr()
    }
    /// <p>The IPv6 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    /// <p>Default: <code>::/0</code> </p>
    pub fn local_ipv6_network_cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.local_ipv6_network_cidr(input.into());
        self
    }
    /// <p>The IPv6 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    /// <p>Default: <code>::/0</code> </p>
    pub fn set_local_ipv6_network_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_local_ipv6_network_cidr(input);
        self
    }
    /// <p>The IPv6 CIDR on the customer gateway (on-premises) side of the VPN connection.</p>
    /// <p>Default: <code>::/0</code> </p>
    pub fn get_local_ipv6_network_cidr(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_local_ipv6_network_cidr()
    }
    /// <p>The IPv6 CIDR on the Amazon Web Services side of the VPN connection.</p>
    /// <p>Default: <code>::/0</code> </p>
    pub fn remote_ipv6_network_cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.remote_ipv6_network_cidr(input.into());
        self
    }
    /// <p>The IPv6 CIDR on the Amazon Web Services side of the VPN connection.</p>
    /// <p>Default: <code>::/0</code> </p>
    pub fn set_remote_ipv6_network_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_remote_ipv6_network_cidr(input);
        self
    }
    /// <p>The IPv6 CIDR on the Amazon Web Services side of the VPN connection.</p>
    /// <p>Default: <code>::/0</code> </p>
    pub fn get_remote_ipv6_network_cidr(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_remote_ipv6_network_cidr()
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
