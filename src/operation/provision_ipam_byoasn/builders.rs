// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::provision_ipam_byoasn::_provision_ipam_byoasn_output::ProvisionIpamByoasnOutputBuilder;

pub use crate::operation::provision_ipam_byoasn::_provision_ipam_byoasn_input::ProvisionIpamByoasnInputBuilder;

impl ProvisionIpamByoasnInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.provision_ipam_byoasn();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ProvisionIpamByoasn`.
///
/// <p>Provisions your Autonomous System Number (ASN) for use in your Amazon Web Services account. This action requires authorization context for Amazon to bring the ASN to an Amazon Web Services account. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/tutorials-byoasn.html">Tutorial: Bring your ASN to IPAM</a> in the <i>Amazon VPC IPAM guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ProvisionIpamByoasnFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::provision_ipam_byoasn::builders::ProvisionIpamByoasnInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput,
        crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnError,
    > for ProvisionIpamByoasnFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput,
            crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ProvisionIpamByoasnFluentBuilder {
    /// Creates a new `ProvisionIpamByoasn`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ProvisionIpamByoasn as a reference.
    pub fn as_input(&self) -> &crate::operation::provision_ipam_byoasn::builders::ProvisionIpamByoasnInputBuilder {
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
        crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::provision_ipam_byoasn::ProvisionIpamByoasn::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::provision_ipam_byoasn::ProvisionIpamByoasn::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput,
        crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnError,
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
    /// <p>An IPAM ID.</p>
    pub fn ipam_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ipam_id(input.into());
        self
    }
    /// <p>An IPAM ID.</p>
    pub fn set_ipam_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ipam_id(input);
        self
    }
    /// <p>An IPAM ID.</p>
    pub fn get_ipam_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ipam_id()
    }
    /// <p>A public 2-byte or 4-byte ASN.</p>
    pub fn asn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.asn(input.into());
        self
    }
    /// <p>A public 2-byte or 4-byte ASN.</p>
    pub fn set_asn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_asn(input);
        self
    }
    /// <p>A public 2-byte or 4-byte ASN.</p>
    pub fn get_asn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_asn()
    }
    /// <p>An ASN authorization context.</p>
    pub fn asn_authorization_context(mut self, input: crate::types::AsnAuthorizationContext) -> Self {
        self.inner = self.inner.asn_authorization_context(input);
        self
    }
    /// <p>An ASN authorization context.</p>
    pub fn set_asn_authorization_context(mut self, input: ::std::option::Option<crate::types::AsnAuthorizationContext>) -> Self {
        self.inner = self.inner.set_asn_authorization_context(input);
        self
    }
    /// <p>An ASN authorization context.</p>
    pub fn get_asn_authorization_context(&self) -> &::std::option::Option<crate::types::AsnAuthorizationContext> {
        self.inner.get_asn_authorization_context()
    }
}