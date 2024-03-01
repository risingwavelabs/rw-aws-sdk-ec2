// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::cancel_reserved_instances_listing::_cancel_reserved_instances_listing_output::CancelReservedInstancesListingOutputBuilder;

pub use crate::operation::cancel_reserved_instances_listing::_cancel_reserved_instances_listing_input::CancelReservedInstancesListingInputBuilder;

impl CancelReservedInstancesListingInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.cancel_reserved_instances_listing();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CancelReservedInstancesListing`.
///
/// <p>Cancels the specified Reserved Instance listing in the Reserved Instance Marketplace.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ri-market-general.html">Reserved Instance Marketplace</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CancelReservedInstancesListingFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::cancel_reserved_instances_listing::builders::CancelReservedInstancesListingInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingOutput,
        crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingError,
    > for CancelReservedInstancesListingFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingOutput,
            crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CancelReservedInstancesListingFluentBuilder {
    /// Creates a new `CancelReservedInstancesListing`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CancelReservedInstancesListing as a reference.
    pub fn as_input(&self) -> &crate::operation::cancel_reserved_instances_listing::builders::CancelReservedInstancesListingInputBuilder {
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
        crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListing::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListing::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingOutput,
        crate::operation::cancel_reserved_instances_listing::CancelReservedInstancesListingError,
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
    /// <p>The ID of the Reserved Instance listing.</p>
    pub fn reserved_instances_listing_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reserved_instances_listing_id(input.into());
        self
    }
    /// <p>The ID of the Reserved Instance listing.</p>
    pub fn set_reserved_instances_listing_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_reserved_instances_listing_id(input);
        self
    }
    /// <p>The ID of the Reserved Instance listing.</p>
    pub fn get_reserved_instances_listing_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_reserved_instances_listing_id()
    }
}