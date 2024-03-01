// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_ipam_resource_cidr::_modify_ipam_resource_cidr_output::ModifyIpamResourceCidrOutputBuilder;

pub use crate::operation::modify_ipam_resource_cidr::_modify_ipam_resource_cidr_input::ModifyIpamResourceCidrInputBuilder;

impl ModifyIpamResourceCidrInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_ipam_resource_cidr();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyIpamResourceCidr`.
///
/// <p>Modify a resource CIDR. You can use this action to transfer resource CIDRs between scopes and ignore resource CIDRs that you do not want to manage. If set to false, the resource will not be tracked for overlap, it cannot be auto-imported into a pool, and it will be removed from any pool it has an allocation in.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/move-resource-ipam.html">Move resource CIDRs between scopes</a> and <a href="https://docs.aws.amazon.com/vpc/latest/ipam/change-monitoring-state-ipam.html">Change the monitoring state of resource CIDRs</a> in the <i>Amazon VPC IPAM User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyIpamResourceCidrFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput,
        crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrError,
    > for ModifyIpamResourceCidrFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput,
            crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyIpamResourceCidrFluentBuilder {
    /// Creates a new `ModifyIpamResourceCidr`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyIpamResourceCidr as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrInputBuilder {
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
        crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidr::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidr::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput,
        crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrError,
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
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>The ID of the resource you want to modify.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_id(input.into());
        self
    }
    /// <p>The ID of the resource you want to modify.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_id(input);
        self
    }
    /// <p>The ID of the resource you want to modify.</p>
    pub fn get_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_id()
    }
    /// <p>The CIDR of the resource you want to modify.</p>
    pub fn resource_cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_cidr(input.into());
        self
    }
    /// <p>The CIDR of the resource you want to modify.</p>
    pub fn set_resource_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_cidr(input);
        self
    }
    /// <p>The CIDR of the resource you want to modify.</p>
    pub fn get_resource_cidr(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_cidr()
    }
    /// <p>The Amazon Web Services Region of the resource you want to modify.</p>
    pub fn resource_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_region(input.into());
        self
    }
    /// <p>The Amazon Web Services Region of the resource you want to modify.</p>
    pub fn set_resource_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_region(input);
        self
    }
    /// <p>The Amazon Web Services Region of the resource you want to modify.</p>
    pub fn get_resource_region(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_region()
    }
    /// <p>The ID of the current scope that the resource CIDR is in.</p>
    pub fn current_ipam_scope_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.current_ipam_scope_id(input.into());
        self
    }
    /// <p>The ID of the current scope that the resource CIDR is in.</p>
    pub fn set_current_ipam_scope_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_current_ipam_scope_id(input);
        self
    }
    /// <p>The ID of the current scope that the resource CIDR is in.</p>
    pub fn get_current_ipam_scope_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_current_ipam_scope_id()
    }
    /// <p>The ID of the scope you want to transfer the resource CIDR to.</p>
    pub fn destination_ipam_scope_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_ipam_scope_id(input.into());
        self
    }
    /// <p>The ID of the scope you want to transfer the resource CIDR to.</p>
    pub fn set_destination_ipam_scope_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_ipam_scope_id(input);
        self
    }
    /// <p>The ID of the scope you want to transfer the resource CIDR to.</p>
    pub fn get_destination_ipam_scope_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_ipam_scope_id()
    }
    /// <p>Determines if the resource is monitored by IPAM. If a resource is monitored, the resource is discovered by IPAM and you can view details about the resource’s CIDR.</p>
    pub fn monitored(mut self, input: bool) -> Self {
        self.inner = self.inner.monitored(input);
        self
    }
    /// <p>Determines if the resource is monitored by IPAM. If a resource is monitored, the resource is discovered by IPAM and you can view details about the resource’s CIDR.</p>
    pub fn set_monitored(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_monitored(input);
        self
    }
    /// <p>Determines if the resource is monitored by IPAM. If a resource is monitored, the resource is discovered by IPAM and you can view details about the resource’s CIDR.</p>
    pub fn get_monitored(&self) -> &::std::option::Option<bool> {
        self.inner.get_monitored()
    }
}