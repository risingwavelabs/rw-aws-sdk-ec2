// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_fast_launch::_enable_fast_launch_output::EnableFastLaunchOutputBuilder;

pub use crate::operation::enable_fast_launch::_enable_fast_launch_input::EnableFastLaunchInputBuilder;

impl EnableFastLaunchInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::enable_fast_launch::EnableFastLaunchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_fast_launch::EnableFastLaunchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.enable_fast_launch();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EnableFastLaunch`.
///
/// <p>When you enable Windows fast launch for a Windows AMI, images are pre-provisioned, using snapshots to launch instances up to 65% faster. To create the optimized Windows image, Amazon EC2 launches an instance and runs through Sysprep steps, rebooting as required. Then it creates a set of reserved snapshots that are used for subsequent launches. The reserved snapshots are automatically replenished as they are used, depending on your settings for launch frequency.</p> <note>
/// <p>You can only change these settings for Windows AMIs that you own or that have been shared with you.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableFastLaunchFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable_fast_launch::builders::EnableFastLaunchInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::enable_fast_launch::EnableFastLaunchOutput,
        crate::operation::enable_fast_launch::EnableFastLaunchError,
    > for EnableFastLaunchFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::enable_fast_launch::EnableFastLaunchOutput,
            crate::operation::enable_fast_launch::EnableFastLaunchError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl EnableFastLaunchFluentBuilder {
    /// Creates a new `EnableFastLaunch`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the EnableFastLaunch as a reference.
    pub fn as_input(&self) -> &crate::operation::enable_fast_launch::builders::EnableFastLaunchInputBuilder {
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
        crate::operation::enable_fast_launch::EnableFastLaunchOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_fast_launch::EnableFastLaunchError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::enable_fast_launch::EnableFastLaunch::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::enable_fast_launch::EnableFastLaunch::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::enable_fast_launch::EnableFastLaunchOutput,
        crate::operation::enable_fast_launch::EnableFastLaunchError,
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
    /// <p>Specify the ID of the image for which to enable Windows fast launch.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.image_id(input.into());
        self
    }
    /// <p>Specify the ID of the image for which to enable Windows fast launch.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_image_id(input);
        self
    }
    /// <p>Specify the ID of the image for which to enable Windows fast launch.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_image_id()
    }
    /// <p>The type of resource to use for pre-provisioning the AMI for Windows fast launch. Supported values include: <code>snapshot</code>, which is the default value.</p>
    pub fn resource_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_type(input.into());
        self
    }
    /// <p>The type of resource to use for pre-provisioning the AMI for Windows fast launch. Supported values include: <code>snapshot</code>, which is the default value.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>The type of resource to use for pre-provisioning the AMI for Windows fast launch. Supported values include: <code>snapshot</code>, which is the default value.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_type()
    }
    /// <p>Configuration settings for creating and managing the snapshots that are used for pre-provisioning the AMI for Windows fast launch. The associated <code>ResourceType</code> must be <code>snapshot</code>.</p>
    pub fn snapshot_configuration(mut self, input: crate::types::FastLaunchSnapshotConfigurationRequest) -> Self {
        self.inner = self.inner.snapshot_configuration(input);
        self
    }
    /// <p>Configuration settings for creating and managing the snapshots that are used for pre-provisioning the AMI for Windows fast launch. The associated <code>ResourceType</code> must be <code>snapshot</code>.</p>
    pub fn set_snapshot_configuration(mut self, input: ::std::option::Option<crate::types::FastLaunchSnapshotConfigurationRequest>) -> Self {
        self.inner = self.inner.set_snapshot_configuration(input);
        self
    }
    /// <p>Configuration settings for creating and managing the snapshots that are used for pre-provisioning the AMI for Windows fast launch. The associated <code>ResourceType</code> must be <code>snapshot</code>.</p>
    pub fn get_snapshot_configuration(&self) -> &::std::option::Option<crate::types::FastLaunchSnapshotConfigurationRequest> {
        self.inner.get_snapshot_configuration()
    }
    /// <p>The launch template to use when launching Windows instances from pre-provisioned snapshots. Launch template parameters can include either the name or ID of the launch template, but not both.</p>
    pub fn launch_template(mut self, input: crate::types::FastLaunchLaunchTemplateSpecificationRequest) -> Self {
        self.inner = self.inner.launch_template(input);
        self
    }
    /// <p>The launch template to use when launching Windows instances from pre-provisioned snapshots. Launch template parameters can include either the name or ID of the launch template, but not both.</p>
    pub fn set_launch_template(mut self, input: ::std::option::Option<crate::types::FastLaunchLaunchTemplateSpecificationRequest>) -> Self {
        self.inner = self.inner.set_launch_template(input);
        self
    }
    /// <p>The launch template to use when launching Windows instances from pre-provisioned snapshots. Launch template parameters can include either the name or ID of the launch template, but not both.</p>
    pub fn get_launch_template(&self) -> &::std::option::Option<crate::types::FastLaunchLaunchTemplateSpecificationRequest> {
        self.inner.get_launch_template()
    }
    /// <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows fast launch. Value must be <code>6</code> or greater.</p>
    pub fn max_parallel_launches(mut self, input: i32) -> Self {
        self.inner = self.inner.max_parallel_launches(input);
        self
    }
    /// <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows fast launch. Value must be <code>6</code> or greater.</p>
    pub fn set_max_parallel_launches(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_parallel_launches(input);
        self
    }
    /// <p>The maximum number of instances that Amazon EC2 can launch at the same time to create pre-provisioned snapshots for Windows fast launch. Value must be <code>6</code> or greater.</p>
    pub fn get_max_parallel_launches(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_parallel_launches()
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