// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_snapshot::_delete_snapshot_output::DeleteSnapshotOutputBuilder;

pub use crate::operation::delete_snapshot::_delete_snapshot_input::DeleteSnapshotInputBuilder;

impl DeleteSnapshotInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_snapshot::DeleteSnapshotOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_snapshot::DeleteSnapshotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_snapshot();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteSnapshot`.
///
/// <p>Deletes the specified snapshot.</p>
/// <p>When you make periodic snapshots of a volume, the snapshots are incremental, and only the blocks on the device that have changed since your last snapshot are saved in the new snapshot. When you delete a snapshot, only the data not needed for any other snapshot is removed. So regardless of which prior snapshots have been deleted, all active snapshots will have access to all the information needed to restore the volume.</p>
/// <p>You cannot delete a snapshot of the root device of an EBS volume used by a registered AMI. You must first de-register the AMI before you can delete the snapshot.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-deleting-snapshot.html">Delete an Amazon EBS snapshot</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteSnapshotFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_snapshot::builders::DeleteSnapshotInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_snapshot::DeleteSnapshotOutput,
        crate::operation::delete_snapshot::DeleteSnapshotError,
    > for DeleteSnapshotFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_snapshot::DeleteSnapshotOutput,
            crate::operation::delete_snapshot::DeleteSnapshotError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteSnapshotFluentBuilder {
    /// Creates a new `DeleteSnapshot`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteSnapshot as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_snapshot::builders::DeleteSnapshotInputBuilder {
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
        crate::operation::delete_snapshot::DeleteSnapshotOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_snapshot::DeleteSnapshotError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_snapshot::DeleteSnapshot::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_snapshot::DeleteSnapshot::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_snapshot::DeleteSnapshotOutput,
        crate::operation::delete_snapshot::DeleteSnapshotError,
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
    /// <p>The ID of the EBS snapshot.</p>
    pub fn snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.snapshot_id(input.into());
        self
    }
    /// <p>The ID of the EBS snapshot.</p>
    pub fn set_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_id(input);
        self
    }
    /// <p>The ID of the EBS snapshot.</p>
    pub fn get_snapshot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_snapshot_id()
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
