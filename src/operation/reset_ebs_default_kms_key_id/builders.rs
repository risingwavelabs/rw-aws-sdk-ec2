// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::reset_ebs_default_kms_key_id::_reset_ebs_default_kms_key_id_output::ResetEbsDefaultKmsKeyIdOutputBuilder;

pub use crate::operation::reset_ebs_default_kms_key_id::_reset_ebs_default_kms_key_id_input::ResetEbsDefaultKmsKeyIdInputBuilder;

impl ResetEbsDefaultKmsKeyIdInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.reset_ebs_default_kms_key_id();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ResetEbsDefaultKmsKeyId`.
///
/// <p>Resets the default KMS key for EBS encryption for your account in this Region to the Amazon Web Services managed KMS key for EBS.</p>
/// <p>After resetting the default KMS key to the Amazon Web Services managed KMS key, you can continue to encrypt by a customer managed KMS key by specifying it when you create the volume. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ResetEbsDefaultKmsKeyIdFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::reset_ebs_default_kms_key_id::builders::ResetEbsDefaultKmsKeyIdInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdOutput,
        crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdError,
    > for ResetEbsDefaultKmsKeyIdFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdOutput,
            crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ResetEbsDefaultKmsKeyIdFluentBuilder {
    /// Creates a new `ResetEbsDefaultKmsKeyId`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ResetEbsDefaultKmsKeyId as a reference.
    pub fn as_input(&self) -> &crate::operation::reset_ebs_default_kms_key_id::builders::ResetEbsDefaultKmsKeyIdInputBuilder {
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
        crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyId::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyId::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdOutput,
        crate::operation::reset_ebs_default_kms_key_id::ResetEbsDefaultKmsKeyIdError,
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
}
