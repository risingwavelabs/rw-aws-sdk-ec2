// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_verified_access_endpoint_policy::_modify_verified_access_endpoint_policy_output::ModifyVerifiedAccessEndpointPolicyOutputBuilder;

pub use crate::operation::modify_verified_access_endpoint_policy::_modify_verified_access_endpoint_policy_input::ModifyVerifiedAccessEndpointPolicyInputBuilder;

impl ModifyVerifiedAccessEndpointPolicyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_verified_access_endpoint_policy();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyVerifiedAccessEndpointPolicy`.
///
/// <p>Modifies the specified Amazon Web Services Verified Access endpoint policy.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyVerifiedAccessEndpointPolicyFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyOutput,
        crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyError,
    > for ModifyVerifiedAccessEndpointPolicyFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyOutput,
            crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyVerifiedAccessEndpointPolicyFluentBuilder {
    /// Creates a new `ModifyVerifiedAccessEndpointPolicy`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyVerifiedAccessEndpointPolicy as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_verified_access_endpoint_policy::builders::ModifyVerifiedAccessEndpointPolicyInputBuilder {
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
        crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicy::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicy::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyOutput,
        crate::operation::modify_verified_access_endpoint_policy::ModifyVerifiedAccessEndpointPolicyError,
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
    /// <p>The ID of the Verified Access endpoint.</p>
    pub fn verified_access_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.verified_access_endpoint_id(input.into());
        self
    }
    /// <p>The ID of the Verified Access endpoint.</p>
    pub fn set_verified_access_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_verified_access_endpoint_id(input);
        self
    }
    /// <p>The ID of the Verified Access endpoint.</p>
    pub fn get_verified_access_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_verified_access_endpoint_id()
    }
    /// <p>The status of the Verified Access policy.</p>
    pub fn policy_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.policy_enabled(input);
        self
    }
    /// <p>The status of the Verified Access policy.</p>
    pub fn set_policy_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_policy_enabled(input);
        self
    }
    /// <p>The status of the Verified Access policy.</p>
    pub fn get_policy_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_policy_enabled()
    }
    /// <p>The Verified Access policy document.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.policy_document(input.into());
        self
    }
    /// <p>The Verified Access policy document.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_policy_document(input);
        self
    }
    /// <p>The Verified Access policy document.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_policy_document()
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
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
    /// <p>The options for server side encryption.</p>
    pub fn sse_specification(mut self, input: crate::types::VerifiedAccessSseSpecificationRequest) -> Self {
        self.inner = self.inner.sse_specification(input);
        self
    }
    /// <p>The options for server side encryption.</p>
    pub fn set_sse_specification(mut self, input: ::std::option::Option<crate::types::VerifiedAccessSseSpecificationRequest>) -> Self {
        self.inner = self.inner.set_sse_specification(input);
        self
    }
    /// <p>The options for server side encryption.</p>
    pub fn get_sse_specification(&self) -> &::std::option::Option<crate::types::VerifiedAccessSseSpecificationRequest> {
        self.inner.get_sse_specification()
    }
}