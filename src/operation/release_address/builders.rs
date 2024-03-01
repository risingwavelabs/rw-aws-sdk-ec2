// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::release_address::_release_address_output::ReleaseAddressOutputBuilder;

pub use crate::operation::release_address::_release_address_input::ReleaseAddressInputBuilder;

impl ReleaseAddressInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::release_address::ReleaseAddressOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::release_address::ReleaseAddressError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.release_address();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ReleaseAddress`.
///
/// <p>Releases the specified Elastic IP address.</p>
/// <p>[Default VPC] Releasing an Elastic IP address automatically disassociates it from any instance that it's associated with. To disassociate an Elastic IP address without releasing it, use <code>DisassociateAddress</code>.</p>
/// <p>[Nondefault VPC] You must use <code>DisassociateAddress</code> to disassociate the Elastic IP address before you can release it. Otherwise, Amazon EC2 returns an error (<code>InvalidIPAddress.InUse</code>).</p>
/// <p>After releasing an Elastic IP address, it is released to the IP address pool. Be sure to update your DNS records and any servers or devices that communicate with the address. If you attempt to release an Elastic IP address that you already released, you'll get an <code>AuthFailure</code> error if the address is already allocated to another Amazon Web Services account.</p>
/// <p>After you release an Elastic IP address, you might be able to recover it. For more information, see <code>AllocateAddress</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ReleaseAddressFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::release_address::builders::ReleaseAddressInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::release_address::ReleaseAddressOutput,
        crate::operation::release_address::ReleaseAddressError,
    > for ReleaseAddressFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::release_address::ReleaseAddressOutput,
            crate::operation::release_address::ReleaseAddressError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ReleaseAddressFluentBuilder {
    /// Creates a new `ReleaseAddress`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ReleaseAddress as a reference.
    pub fn as_input(&self) -> &crate::operation::release_address::builders::ReleaseAddressInputBuilder {
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
        crate::operation::release_address::ReleaseAddressOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::release_address::ReleaseAddressError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::release_address::ReleaseAddress::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::release_address::ReleaseAddress::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::release_address::ReleaseAddressOutput,
        crate::operation::release_address::ReleaseAddressError,
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
    /// <p>The allocation ID. This parameter is required.</p>
    pub fn allocation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.allocation_id(input.into());
        self
    }
    /// <p>The allocation ID. This parameter is required.</p>
    pub fn set_allocation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_allocation_id(input);
        self
    }
    /// <p>The allocation ID. This parameter is required.</p>
    pub fn get_allocation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_allocation_id()
    }
    /// <p>Deprecated.</p>
    pub fn public_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.public_ip(input.into());
        self
    }
    /// <p>Deprecated.</p>
    pub fn set_public_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_public_ip(input);
        self
    }
    /// <p>Deprecated.</p>
    pub fn get_public_ip(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_public_ip()
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    pub fn network_border_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.network_border_group(input.into());
        self
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    pub fn set_network_border_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_network_border_group(input);
        self
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    pub fn get_network_border_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_network_border_group()
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
