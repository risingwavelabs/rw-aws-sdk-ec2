// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::revoke_security_group_egress::_revoke_security_group_egress_output::RevokeSecurityGroupEgressOutputBuilder;

pub use crate::operation::revoke_security_group_egress::_revoke_security_group_egress_input::RevokeSecurityGroupEgressInputBuilder;

impl RevokeSecurityGroupEgressInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.revoke_security_group_egress();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RevokeSecurityGroupEgress`.
///
/// <p>Removes the specified outbound (egress) rules from the specified security group.</p>
/// <p>You can specify rules using either rule IDs or security group rule properties. If you use rule properties, the values that you specify (for example, ports) must match the existing rule's values exactly. Each rule has a protocol, from and to ports, and destination (CIDR range, security group, or prefix list). For the TCP and UDP protocols, you must also specify the destination port or range of ports. For the ICMP protocol, you must also specify the ICMP type and code. If the security group rule has a description, you do not need to specify the description to revoke the rule.</p>
/// <p>For a default VPC, if the values you specify do not match the existing rule's values, no error is returned, and the output describes the security group rules that were not revoked.</p>
/// <p>Amazon Web Services recommends that you describe the security group to verify that the rules were removed.</p>
/// <p>Rule changes are propagated to instances within the security group as quickly as possible. However, a small delay might occur.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RevokeSecurityGroupEgressFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput,
        crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
    > for RevokeSecurityGroupEgressFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput,
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RevokeSecurityGroupEgressFluentBuilder {
    /// Creates a new `RevokeSecurityGroupEgress`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RevokeSecurityGroupEgress as a reference.
    pub fn as_input(&self) -> &crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressInputBuilder {
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
        crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgress::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgress::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput,
        crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
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
    /// <p>The ID of the security group.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_id(input.into());
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_id(input);
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn get_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_id()
    }
    /// Appends an item to `IpPermissions`.
    ///
    /// To override the contents of this collection use [`set_ip_permissions`](Self::set_ip_permissions).
    ///
    /// <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn ip_permissions(mut self, input: crate::types::IpPermission) -> Self {
        self.inner = self.inner.ip_permissions(input);
        self
    }
    /// <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn set_ip_permissions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>) -> Self {
        self.inner = self.inner.set_ip_permissions(input);
        self
    }
    /// <p>The sets of IP permissions. You can't specify a destination security group and a CIDR IP address range in the same set of permissions.</p>
    pub fn get_ip_permissions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::IpPermission>> {
        self.inner.get_ip_permissions()
    }
    /// Appends an item to `SecurityGroupRuleIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_rule_ids`](Self::set_security_group_rule_ids).
    ///
    /// <p>The IDs of the security group rules.</p>
    pub fn security_group_rule_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.security_group_rule_ids(input.into());
        self
    }
    /// <p>The IDs of the security group rules.</p>
    pub fn set_security_group_rule_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_security_group_rule_ids(input);
        self
    }
    /// <p>The IDs of the security group rules.</p>
    pub fn get_security_group_rule_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_security_group_rule_ids()
    }
    /// <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    pub fn cidr_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cidr_ip(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    pub fn set_cidr_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cidr_ip(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the CIDR.</p>
    pub fn get_cidr_ip(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cidr_ip()
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.inner = self.inner.from_port(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn set_from_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_from_port(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn get_from_port(&self) -> &::std::option::Option<i32> {
        self.inner.get_from_port()
    }
    /// <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    pub fn ip_protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ip_protocol(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    pub fn set_ip_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ip_protocol(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the protocol name or number.</p>
    pub fn get_ip_protocol(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ip_protocol()
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.inner = self.inner.to_port(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn set_to_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_to_port(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify the port.</p>
    pub fn get_to_port(&self) -> &::std::option::Option<i32> {
        self.inner.get_to_port()
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn source_security_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_security_group_name(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn set_source_security_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_security_group_name(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn get_source_security_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_security_group_name()
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn source_security_group_owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_security_group_owner_id(input.into());
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn set_source_security_group_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_security_group_owner_id(input);
        self
    }
    /// <p>Not supported. Use a set of IP permissions to specify a destination security group.</p>
    pub fn get_source_security_group_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_security_group_owner_id()
    }
}