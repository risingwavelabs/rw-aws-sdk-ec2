// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_launch_template::_modify_launch_template_output::ModifyLaunchTemplateOutputBuilder;

pub use crate::operation::modify_launch_template::_modify_launch_template_input::ModifyLaunchTemplateInputBuilder;

impl ModifyLaunchTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_launch_template::ModifyLaunchTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_launch_template::ModifyLaunchTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_launch_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyLaunchTemplate`.
///
/// <p>Modifies a launch template. You can specify which version of the launch template to set as the default version. When launching an instance, the default version applies when a launch template version is not specified.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyLaunchTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_launch_template::builders::ModifyLaunchTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_launch_template::ModifyLaunchTemplateOutput,
        crate::operation::modify_launch_template::ModifyLaunchTemplateError,
    > for ModifyLaunchTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_launch_template::ModifyLaunchTemplateOutput,
            crate::operation::modify_launch_template::ModifyLaunchTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyLaunchTemplateFluentBuilder {
    /// Creates a new `ModifyLaunchTemplate`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyLaunchTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_launch_template::builders::ModifyLaunchTemplateInputBuilder {
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
        crate::operation::modify_launch_template::ModifyLaunchTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_launch_template::ModifyLaunchTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_launch_template::ModifyLaunchTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_launch_template::ModifyLaunchTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_launch_template::ModifyLaunchTemplateOutput,
        crate::operation::modify_launch_template::ModifyLaunchTemplateError,
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
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    /// <p>Constraint: Maximum 128 ASCII characters.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    /// <p>Constraint: Maximum 128 ASCII characters.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    /// <p>Constraint: Maximum 128 ASCII characters.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify either the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn launch_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.launch_template_id(input.into());
        self
    }
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify either the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn set_launch_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_launch_template_id(input);
        self
    }
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify either the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn get_launch_template_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_launch_template_id()
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify either the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn launch_template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.launch_template_name(input.into());
        self
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify either the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn set_launch_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_launch_template_name(input);
        self
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify either the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn get_launch_template_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_launch_template_name()
    }
    /// <p>The version number of the launch template to set as the default version.</p>
    pub fn default_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.default_version(input.into());
        self
    }
    /// <p>The version number of the launch template to set as the default version.</p>
    pub fn set_default_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_default_version(input);
        self
    }
    /// <p>The version number of the launch template to set as the default version.</p>
    pub fn get_default_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_default_version()
    }
}
