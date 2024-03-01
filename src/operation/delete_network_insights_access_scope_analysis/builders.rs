// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_network_insights_access_scope_analysis::_delete_network_insights_access_scope_analysis_output::DeleteNetworkInsightsAccessScopeAnalysisOutputBuilder;

pub use crate::operation::delete_network_insights_access_scope_analysis::_delete_network_insights_access_scope_analysis_input::DeleteNetworkInsightsAccessScopeAnalysisInputBuilder;

impl DeleteNetworkInsightsAccessScopeAnalysisInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_network_insights_access_scope_analysis();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteNetworkInsightsAccessScopeAnalysis`.
///
/// <p>Deletes the specified Network Access Scope analysis.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteNetworkInsightsAccessScopeAnalysisFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_network_insights_access_scope_analysis::builders::DeleteNetworkInsightsAccessScopeAnalysisInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisOutput,
        crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisError,
    > for DeleteNetworkInsightsAccessScopeAnalysisFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisOutput,
            crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteNetworkInsightsAccessScopeAnalysisFluentBuilder {
    /// Creates a new `DeleteNetworkInsightsAccessScopeAnalysis`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteNetworkInsightsAccessScopeAnalysis as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::delete_network_insights_access_scope_analysis::builders::DeleteNetworkInsightsAccessScopeAnalysisInputBuilder {
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
        crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysis::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysis::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisOutput,
        crate::operation::delete_network_insights_access_scope_analysis::DeleteNetworkInsightsAccessScopeAnalysisError,
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
    /// <p>The ID of the Network Access Scope analysis.</p>
    pub fn network_insights_access_scope_analysis_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.network_insights_access_scope_analysis_id(input.into());
        self
    }
    /// <p>The ID of the Network Access Scope analysis.</p>
    pub fn set_network_insights_access_scope_analysis_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_network_insights_access_scope_analysis_id(input);
        self
    }
    /// <p>The ID of the Network Access Scope analysis.</p>
    pub fn get_network_insights_access_scope_analysis_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_network_insights_access_scope_analysis_id()
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
