// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reset_address_attribute::_reset_address_attribute_output::ResetAddressAttributeOutputBuilder;

pub use crate::operation::reset_address_attribute::_reset_address_attribute_input::ResetAddressAttributeInputBuilder;

impl ResetAddressAttributeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::reset_address_attribute::ResetAddressAttributeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::reset_address_attribute::ResetAddressAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.reset_address_attribute();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ResetAddressAttribute`.
///
/// <p>Resets the attribute of the specified IP address. For requirements, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/elastic-ip-addresses-eip.html#Using_Elastic_Addressing_Reverse_DNS">Using reverse DNS for email applications</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ResetAddressAttributeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::reset_address_attribute::builders::ResetAddressAttributeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::reset_address_attribute::ResetAddressAttributeOutput,
        crate::operation::reset_address_attribute::ResetAddressAttributeError,
    > for ResetAddressAttributeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::reset_address_attribute::ResetAddressAttributeOutput,
            crate::operation::reset_address_attribute::ResetAddressAttributeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ResetAddressAttributeFluentBuilder {
    /// Creates a new `ResetAddressAttribute`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ResetAddressAttribute as a reference.
    pub fn as_input(&self) -> &crate::operation::reset_address_attribute::builders::ResetAddressAttributeInputBuilder {
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
        crate::operation::reset_address_attribute::ResetAddressAttributeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::reset_address_attribute::ResetAddressAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::reset_address_attribute::ResetAddressAttribute::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::reset_address_attribute::ResetAddressAttribute::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::reset_address_attribute::ResetAddressAttributeOutput,
        crate::operation::reset_address_attribute::ResetAddressAttributeError,
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
    /// <p>[EC2-VPC] The allocation ID.</p>
    pub fn allocation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.allocation_id(input.into());
        self
    }
    /// <p>[EC2-VPC] The allocation ID.</p>
    pub fn set_allocation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_allocation_id(input);
        self
    }
    /// <p>[EC2-VPC] The allocation ID.</p>
    pub fn get_allocation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_allocation_id()
    }
    /// <p>The attribute of the IP address.</p>
    pub fn attribute(mut self, input: crate::types::AddressAttributeName) -> Self {
        self.inner = self.inner.attribute(input);
        self
    }
    /// <p>The attribute of the IP address.</p>
    pub fn set_attribute(mut self, input: ::std::option::Option<crate::types::AddressAttributeName>) -> Self {
        self.inner = self.inner.set_attribute(input);
        self
    }
    /// <p>The attribute of the IP address.</p>
    pub fn get_attribute(&self) -> &::std::option::Option<crate::types::AddressAttributeName> {
        self.inner.get_attribute()
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
