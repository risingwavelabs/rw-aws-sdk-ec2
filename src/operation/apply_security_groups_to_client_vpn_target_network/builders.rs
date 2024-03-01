// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::apply_security_groups_to_client_vpn_target_network::_apply_security_groups_to_client_vpn_target_network_output::ApplySecurityGroupsToClientVpnTargetNetworkOutputBuilder;

pub use crate::operation::apply_security_groups_to_client_vpn_target_network::_apply_security_groups_to_client_vpn_target_network_input::ApplySecurityGroupsToClientVpnTargetNetworkInputBuilder;

impl ApplySecurityGroupsToClientVpnTargetNetworkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.apply_security_groups_to_client_vpn_target_network();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ApplySecurityGroupsToClientVpnTargetNetwork`.
///
/// <p>Applies a security group to the association between the target network and the Client VPN endpoint. This action replaces the existing security groups with the specified security groups.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ApplySecurityGroupsToClientVpnTargetNetworkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::apply_security_groups_to_client_vpn_target_network::builders::ApplySecurityGroupsToClientVpnTargetNetworkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkOutput,
        crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkError,
    > for ApplySecurityGroupsToClientVpnTargetNetworkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkOutput,
            crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ApplySecurityGroupsToClientVpnTargetNetworkFluentBuilder {
    /// Creates a new `ApplySecurityGroupsToClientVpnTargetNetwork`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ApplySecurityGroupsToClientVpnTargetNetwork as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::apply_security_groups_to_client_vpn_target_network::builders::ApplySecurityGroupsToClientVpnTargetNetworkInputBuilder
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
        crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetwork::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetwork::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkOutput,
        crate::operation::apply_security_groups_to_client_vpn_target_network::ApplySecurityGroupsToClientVpnTargetNetworkError,
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
    /// <p>The ID of the Client VPN endpoint.</p>
    pub fn client_vpn_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_vpn_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint.</p>
    pub fn set_client_vpn_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_vpn_endpoint_id(input);
        self
    }
    /// <p>The ID of the Client VPN endpoint.</p>
    pub fn get_client_vpn_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_vpn_endpoint_id()
    }
    /// <p>The ID of the VPC in which the associated target network is located.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>The ID of the VPC in which the associated target network is located.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
    /// <p>The ID of the VPC in which the associated target network is located.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_id()
    }
    /// Appends an item to `SecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>The IDs of the security groups to apply to the associated target network. Up to 5 security groups can be applied to an associated target network.</p>
    pub fn security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.security_group_ids(input.into());
        self
    }
    /// <p>The IDs of the security groups to apply to the associated target network. Up to 5 security groups can be applied to an associated target network.</p>
    pub fn set_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_security_group_ids(input);
        self
    }
    /// <p>The IDs of the security groups to apply to the associated target network. Up to 5 security groups can be applied to an associated target network.</p>
    pub fn get_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_security_group_ids()
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