// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::register_instance_event_notification_attributes::_register_instance_event_notification_attributes_output::RegisterInstanceEventNotificationAttributesOutputBuilder;

pub use crate::operation::register_instance_event_notification_attributes::_register_instance_event_notification_attributes_input::RegisterInstanceEventNotificationAttributesInputBuilder;

impl RegisterInstanceEventNotificationAttributesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.register_instance_event_notification_attributes();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RegisterInstanceEventNotificationAttributes`.
///
/// <p>Registers a set of tag keys to include in scheduled event notifications for your resources. </p>
/// <p>To remove tags, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_DeregisterInstanceEventNotificationAttributes.html">DeregisterInstanceEventNotificationAttributes</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RegisterInstanceEventNotificationAttributesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::register_instance_event_notification_attributes::builders::RegisterInstanceEventNotificationAttributesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesOutput,
        crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesError,
    > for RegisterInstanceEventNotificationAttributesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesOutput,
            crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RegisterInstanceEventNotificationAttributesFluentBuilder {
    /// Creates a new `RegisterInstanceEventNotificationAttributes`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RegisterInstanceEventNotificationAttributes as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::register_instance_event_notification_attributes::builders::RegisterInstanceEventNotificationAttributesInputBuilder {
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
        crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributes::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributes::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesOutput,
        crate::operation::register_instance_event_notification_attributes::RegisterInstanceEventNotificationAttributesError,
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
    /// <p>Information about the tag keys to register.</p>
    pub fn instance_tag_attribute(mut self, input: crate::types::RegisterInstanceTagAttributeRequest) -> Self {
        self.inner = self.inner.instance_tag_attribute(input);
        self
    }
    /// <p>Information about the tag keys to register.</p>
    pub fn set_instance_tag_attribute(mut self, input: ::std::option::Option<crate::types::RegisterInstanceTagAttributeRequest>) -> Self {
        self.inner = self.inner.set_instance_tag_attribute(input);
        self
    }
    /// <p>Information about the tag keys to register.</p>
    pub fn get_instance_tag_attribute(&self) -> &::std::option::Option<crate::types::RegisterInstanceTagAttributeRequest> {
        self.inner.get_instance_tag_attribute()
    }
}