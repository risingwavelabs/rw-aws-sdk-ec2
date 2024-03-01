// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_instance_maintenance_options::_modify_instance_maintenance_options_output::ModifyInstanceMaintenanceOptionsOutputBuilder;

pub use crate::operation::modify_instance_maintenance_options::_modify_instance_maintenance_options_input::ModifyInstanceMaintenanceOptionsInputBuilder;

impl ModifyInstanceMaintenanceOptionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_instance_maintenance_options();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyInstanceMaintenanceOptions`.
///
/// <p>Modifies the recovery behavior of your instance to disable simplified automatic recovery or set the recovery behavior to default. The default configuration will not enable simplified automatic recovery for an unsupported instance type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-recover.html#instance-configuration-recovery">Simplified automatic recovery</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyInstanceMaintenanceOptionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_instance_maintenance_options::builders::ModifyInstanceMaintenanceOptionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsOutput,
        crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError,
    > for ModifyInstanceMaintenanceOptionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsOutput,
            crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyInstanceMaintenanceOptionsFluentBuilder {
    /// Creates a new `ModifyInstanceMaintenanceOptions`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyInstanceMaintenanceOptions as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_instance_maintenance_options::builders::ModifyInstanceMaintenanceOptionsInputBuilder {
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
        crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsOutput,
        crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError,
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
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>Disables the automatic recovery behavior of your instance or sets it to default.</p>
    pub fn auto_recovery(mut self, input: crate::types::InstanceAutoRecoveryState) -> Self {
        self.inner = self.inner.auto_recovery(input);
        self
    }
    /// <p>Disables the automatic recovery behavior of your instance or sets it to default.</p>
    pub fn set_auto_recovery(mut self, input: ::std::option::Option<crate::types::InstanceAutoRecoveryState>) -> Self {
        self.inner = self.inner.set_auto_recovery(input);
        self
    }
    /// <p>Disables the automatic recovery behavior of your instance or sets it to default.</p>
    pub fn get_auto_recovery(&self) -> &::std::option::Option<crate::types::InstanceAutoRecoveryState> {
        self.inner.get_auto_recovery()
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