// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_managed_prefix_list::_delete_managed_prefix_list_output::DeleteManagedPrefixListOutputBuilder;

pub use crate::operation::delete_managed_prefix_list::_delete_managed_prefix_list_input::DeleteManagedPrefixListInputBuilder;

impl DeleteManagedPrefixListInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_managed_prefix_list();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteManagedPrefixList`.
///
/// <p>Deletes the specified managed prefix list. You must first remove all references to the prefix list in your resources.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteManagedPrefixListFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_managed_prefix_list::builders::DeleteManagedPrefixListInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListOutput,
        crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError,
    > for DeleteManagedPrefixListFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListOutput,
            crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteManagedPrefixListFluentBuilder {
    /// Creates a new `DeleteManagedPrefixList`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteManagedPrefixList as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_managed_prefix_list::builders::DeleteManagedPrefixListInputBuilder {
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
        crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_managed_prefix_list::DeleteManagedPrefixList::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_managed_prefix_list::DeleteManagedPrefixList::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListOutput,
        crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError,
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
    /// <p>The ID of the prefix list.</p>
    pub fn prefix_list_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.prefix_list_id(input.into());
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn set_prefix_list_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_prefix_list_id(input);
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn get_prefix_list_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_prefix_list_id()
    }
}
