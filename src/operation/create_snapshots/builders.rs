// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_snapshots::_create_snapshots_output::CreateSnapshotsOutputBuilder;

pub use crate::operation::create_snapshots::_create_snapshots_input::CreateSnapshotsInputBuilder;

impl CreateSnapshotsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_snapshots::CreateSnapshotsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_snapshots::CreateSnapshotsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_snapshots();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateSnapshots`.
///
/// <p>Creates crash-consistent snapshots of multiple EBS volumes and stores the data in S3. Volumes are chosen by specifying an instance. Any attached volumes will produce one snapshot each that is crash-consistent across the instance.</p>
/// <p>You can include all of the volumes currently attached to the instance, or you can exclude the root volume or specific data (non-root) volumes from the multi-volume snapshot set.</p>
/// <p>You can create multi-volume snapshots of instances in a Region and instances on an Outpost. If you create snapshots from an instance in a Region, the snapshots must be stored in the same Region as the instance. If you create snapshots from an instance on an Outpost, the snapshots can be stored on the same Outpost as the instance, or in the Region for that Outpost.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateSnapshotsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_snapshots::builders::CreateSnapshotsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_snapshots::CreateSnapshotsOutput,
        crate::operation::create_snapshots::CreateSnapshotsError,
    > for CreateSnapshotsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_snapshots::CreateSnapshotsOutput,
            crate::operation::create_snapshots::CreateSnapshotsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateSnapshotsFluentBuilder {
    /// Creates a new `CreateSnapshots`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateSnapshots as a reference.
    pub fn as_input(&self) -> &crate::operation::create_snapshots::builders::CreateSnapshotsInputBuilder {
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
        crate::operation::create_snapshots::CreateSnapshotsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_snapshots::CreateSnapshotsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_snapshots::CreateSnapshots::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_snapshots::CreateSnapshots::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_snapshots::CreateSnapshotsOutput,
        crate::operation::create_snapshots::CreateSnapshotsError,
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
    /// <p> A description propagated to every snapshot specified by the instance.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p> A description propagated to every snapshot specified by the instance.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p> A description propagated to every snapshot specified by the instance.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The instance to specify which volumes should be included in the snapshots.</p>
    pub fn instance_specification(mut self, input: crate::types::InstanceSpecification) -> Self {
        self.inner = self.inner.instance_specification(input);
        self
    }
    /// <p>The instance to specify which volumes should be included in the snapshots.</p>
    pub fn set_instance_specification(mut self, input: ::std::option::Option<crate::types::InstanceSpecification>) -> Self {
        self.inner = self.inner.set_instance_specification(input);
        self
    }
    /// <p>The instance to specify which volumes should be included in the snapshots.</p>
    pub fn get_instance_specification(&self) -> &::std::option::Option<crate::types::InstanceSpecification> {
        self.inner.get_instance_specification()
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create the local snapshots.</p>
    /// <ul>
    /// <li> <p>To create snapshots from an instance in a Region, omit this parameter. The snapshots are created in the same Region as the instance.</p> </li>
    /// <li> <p>To create snapshots from an instance on an Outpost and store the snapshots in the Region, omit this parameter. The snapshots are created in the Region for the Outpost.</p> </li>
    /// <li> <p>To create snapshots from an instance on an Outpost and store the snapshots on an Outpost, specify the ARN of the destination Outpost. The snapshots must be created on the same Outpost as the instance.</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-multivol-snapshot"> Create multi-volume local snapshots from instances on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn outpost_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.outpost_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create the local snapshots.</p>
    /// <ul>
    /// <li> <p>To create snapshots from an instance in a Region, omit this parameter. The snapshots are created in the same Region as the instance.</p> </li>
    /// <li> <p>To create snapshots from an instance on an Outpost and store the snapshots in the Region, omit this parameter. The snapshots are created in the Region for the Outpost.</p> </li>
    /// <li> <p>To create snapshots from an instance on an Outpost and store the snapshots on an Outpost, specify the ARN of the destination Outpost. The snapshots must be created on the same Outpost as the instance.</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-multivol-snapshot"> Create multi-volume local snapshots from instances on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn set_outpost_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_outpost_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost on which to create the local snapshots.</p>
    /// <ul>
    /// <li> <p>To create snapshots from an instance in a Region, omit this parameter. The snapshots are created in the same Region as the instance.</p> </li>
    /// <li> <p>To create snapshots from an instance on an Outpost and store the snapshots in the Region, omit this parameter. The snapshots are created in the Region for the Outpost.</p> </li>
    /// <li> <p>To create snapshots from an instance on an Outpost and store the snapshots on an Outpost, specify the ARN of the destination Outpost. The snapshots must be created on the same Outpost as the instance.</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#create-multivol-snapshot"> Create multi-volume local snapshots from instances on an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn get_outpost_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_outpost_arn()
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>Tags to apply to every snapshot specified by the instance.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>Tags to apply to every snapshot specified by the instance.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>Tags to apply to every snapshot specified by the instance.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
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
    /// <p>Copies the tags from the specified volume to corresponding snapshot.</p>
    pub fn copy_tags_from_source(mut self, input: crate::types::CopyTagsFromSource) -> Self {
        self.inner = self.inner.copy_tags_from_source(input);
        self
    }
    /// <p>Copies the tags from the specified volume to corresponding snapshot.</p>
    pub fn set_copy_tags_from_source(mut self, input: ::std::option::Option<crate::types::CopyTagsFromSource>) -> Self {
        self.inner = self.inner.set_copy_tags_from_source(input);
        self
    }
    /// <p>Copies the tags from the specified volume to corresponding snapshot.</p>
    pub fn get_copy_tags_from_source(&self) -> &::std::option::Option<crate::types::CopyTagsFromSource> {
        self.inner.get_copy_tags_from_source()
    }
}
