// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::disassociate_instance_event_window::_disassociate_instance_event_window_output::DisassociateInstanceEventWindowOutputBuilder;

pub use crate::operation::disassociate_instance_event_window::_disassociate_instance_event_window_input::DisassociateInstanceEventWindowInputBuilder;

impl DisassociateInstanceEventWindowInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.disassociate_instance_event_window();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DisassociateInstanceEventWindow`.
///
/// <p>Disassociates one or more targets from an event window.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/event-windows.html">Define event windows for scheduled events</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DisassociateInstanceEventWindowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::disassociate_instance_event_window::builders::DisassociateInstanceEventWindowInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowOutput,
        crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError,
    > for DisassociateInstanceEventWindowFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowOutput,
            crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DisassociateInstanceEventWindowFluentBuilder {
    /// Creates a new `DisassociateInstanceEventWindow`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DisassociateInstanceEventWindow as a reference.
    pub fn as_input(&self) -> &crate::operation::disassociate_instance_event_window::builders::DisassociateInstanceEventWindowInputBuilder {
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
        crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindow::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindow::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowOutput,
        crate::operation::disassociate_instance_event_window::DisassociateInstanceEventWindowError,
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
    /// <p>The ID of the event window.</p>
    pub fn instance_event_window_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_event_window_id(input.into());
        self
    }
    /// <p>The ID of the event window.</p>
    pub fn set_instance_event_window_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_event_window_id(input);
        self
    }
    /// <p>The ID of the event window.</p>
    pub fn get_instance_event_window_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_event_window_id()
    }
    /// <p>One or more targets to disassociate from the specified event window.</p>
    pub fn association_target(mut self, input: crate::types::InstanceEventWindowDisassociationRequest) -> Self {
        self.inner = self.inner.association_target(input);
        self
    }
    /// <p>One or more targets to disassociate from the specified event window.</p>
    pub fn set_association_target(mut self, input: ::std::option::Option<crate::types::InstanceEventWindowDisassociationRequest>) -> Self {
        self.inner = self.inner.set_association_target(input);
        self
    }
    /// <p>One or more targets to disassociate from the specified event window.</p>
    pub fn get_association_target(&self) -> &::std::option::Option<crate::types::InstanceEventWindowDisassociationRequest> {
        self.inner.get_association_target()
    }
}