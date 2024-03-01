// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::deprovision_byoip_cidr::_deprovision_byoip_cidr_output::DeprovisionByoipCidrOutputBuilder;

pub use crate::operation::deprovision_byoip_cidr::_deprovision_byoip_cidr_input::DeprovisionByoipCidrInputBuilder;

impl DeprovisionByoipCidrInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.deprovision_byoip_cidr();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeprovisionByoipCidr`.
///
/// <p>Releases the specified address range that you provisioned for use with your Amazon Web Services resources through bring your own IP addresses (BYOIP) and deletes the corresponding address pool.</p>
/// <p>Before you can release an address range, you must stop advertising it using <code>WithdrawByoipCidr</code> and you must not have any IP addresses allocated from its address range.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeprovisionByoipCidrFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrOutput,
        crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrError,
    > for DeprovisionByoipCidrFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrOutput,
            crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeprovisionByoipCidrFluentBuilder {
    /// Creates a new `DeprovisionByoipCidr`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeprovisionByoipCidr as a reference.
    pub fn as_input(&self) -> &crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrInputBuilder {
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
        crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidr::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidr::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrOutput,
        crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrError,
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
    /// <p>The address range, in CIDR notation. The prefix must be the same prefix that you specified when you provisioned the address range.</p>
    pub fn cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cidr(input.into());
        self
    }
    /// <p>The address range, in CIDR notation. The prefix must be the same prefix that you specified when you provisioned the address range.</p>
    pub fn set_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cidr(input);
        self
    }
    /// <p>The address range, in CIDR notation. The prefix must be the same prefix that you specified when you provisioned the address range.</p>
    pub fn get_cidr(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cidr()
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