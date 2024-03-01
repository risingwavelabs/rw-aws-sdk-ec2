// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_reserved_instances::_modify_reserved_instances_output::ModifyReservedInstancesOutputBuilder;

pub use crate::operation::modify_reserved_instances::_modify_reserved_instances_input::ModifyReservedInstancesInputBuilder;

impl ModifyReservedInstancesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_reserved_instances::ModifyReservedInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_reserved_instances::ModifyReservedInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_reserved_instances();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyReservedInstances`.
///
/// <p>Modifies the configuration of your Reserved Instances, such as the Availability Zone, instance count, or instance type. The Reserved Instances to be modified must be identical, except for Availability Zone, network platform, and instance type.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ri-modifying.html">Modifying Reserved Instances</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyReservedInstancesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_reserved_instances::builders::ModifyReservedInstancesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_reserved_instances::ModifyReservedInstancesOutput,
        crate::operation::modify_reserved_instances::ModifyReservedInstancesError,
    > for ModifyReservedInstancesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_reserved_instances::ModifyReservedInstancesOutput,
            crate::operation::modify_reserved_instances::ModifyReservedInstancesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyReservedInstancesFluentBuilder {
    /// Creates a new `ModifyReservedInstances`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyReservedInstances as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_reserved_instances::builders::ModifyReservedInstancesInputBuilder {
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
        crate::operation::modify_reserved_instances::ModifyReservedInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_reserved_instances::ModifyReservedInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_reserved_instances::ModifyReservedInstances::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_reserved_instances::ModifyReservedInstances::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_reserved_instances::ModifyReservedInstancesOutput,
        crate::operation::modify_reserved_instances::ModifyReservedInstancesError,
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
    /// Appends an item to `ReservedInstancesIds`.
    ///
    /// To override the contents of this collection use [`set_reserved_instances_ids`](Self::set_reserved_instances_ids).
    ///
    /// <p>The IDs of the Reserved Instances to modify.</p>
    pub fn reserved_instances_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reserved_instances_ids(input.into());
        self
    }
    /// <p>The IDs of the Reserved Instances to modify.</p>
    pub fn set_reserved_instances_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_reserved_instances_ids(input);
        self
    }
    /// <p>The IDs of the Reserved Instances to modify.</p>
    pub fn get_reserved_instances_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_reserved_instances_ids()
    }
    /// <p>A unique, case-sensitive token you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive token you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive token you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Appends an item to `TargetConfigurations`.
    ///
    /// To override the contents of this collection use [`set_target_configurations`](Self::set_target_configurations).
    ///
    /// <p>The configuration settings for the Reserved Instances to modify.</p>
    pub fn target_configurations(mut self, input: crate::types::ReservedInstancesConfiguration) -> Self {
        self.inner = self.inner.target_configurations(input);
        self
    }
    /// <p>The configuration settings for the Reserved Instances to modify.</p>
    pub fn set_target_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ReservedInstancesConfiguration>>) -> Self {
        self.inner = self.inner.set_target_configurations(input);
        self
    }
    /// <p>The configuration settings for the Reserved Instances to modify.</p>
    pub fn get_target_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ReservedInstancesConfiguration>> {
        self.inner.get_target_configurations()
    }
}