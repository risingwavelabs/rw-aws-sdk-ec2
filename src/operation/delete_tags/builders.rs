// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_tags::_delete_tags_output::DeleteTagsOutputBuilder;

pub use crate::operation::delete_tags::_delete_tags_input::DeleteTagsInputBuilder;

impl DeleteTagsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_tags::DeleteTagsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_tags::DeleteTagsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_tags();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteTags`.
///
/// <p>Deletes the specified set of tags from the specified set of resources.</p>
/// <p>To list the current tags, use <code>DescribeTags</code>. For more information about tags, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html">Tag your Amazon EC2 resources</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteTagsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_tags::builders::DeleteTagsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_tags::DeleteTagsOutput,
        crate::operation::delete_tags::DeleteTagsError,
    > for DeleteTagsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_tags::DeleteTagsOutput,
            crate::operation::delete_tags::DeleteTagsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteTagsFluentBuilder {
    /// Creates a new `DeleteTags`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteTags as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_tags::builders::DeleteTagsInputBuilder {
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
        crate::operation::delete_tags::DeleteTagsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_tags::DeleteTagsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_tags::DeleteTags::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_tags::DeleteTags::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_tags::DeleteTagsOutput,
        crate::operation::delete_tags::DeleteTagsError,
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
    /// Appends an item to `Resources`.
    ///
    /// To override the contents of this collection use [`set_resources`](Self::set_resources).
    ///
    /// <p>The IDs of the resources, separated by spaces.</p>
    /// <p>Constraints: Up to 1000 resource IDs. We recommend breaking up this request into smaller batches.</p>
    pub fn resources(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resources(input.into());
        self
    }
    /// <p>The IDs of the resources, separated by spaces.</p>
    /// <p>Constraints: Up to 1000 resource IDs. We recommend breaking up this request into smaller batches.</p>
    pub fn set_resources(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_resources(input);
        self
    }
    /// <p>The IDs of the resources, separated by spaces.</p>
    /// <p>Constraints: Up to 1000 resource IDs. We recommend breaking up this request into smaller batches.</p>
    pub fn get_resources(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_resources()
    }
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags to delete. Specify a tag key and an optional tag value to delete specific tags. If you specify a tag key without a tag value, we delete any tag with this key regardless of its value. If you specify a tag key with an empty string as the tag value, we delete the tag only if its value is an empty string.</p>
    /// <p>If you omit this parameter, we delete all user-defined tags for the specified resources. We do not delete Amazon Web Services-generated tags (tags that have the <code>aws:</code> prefix).</p>
    /// <p>Constraints: Up to 1000 tags.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>The tags to delete. Specify a tag key and an optional tag value to delete specific tags. If you specify a tag key without a tag value, we delete any tag with this key regardless of its value. If you specify a tag key with an empty string as the tag value, we delete the tag only if its value is an empty string.</p>
    /// <p>If you omit this parameter, we delete all user-defined tags for the specified resources. We do not delete Amazon Web Services-generated tags (tags that have the <code>aws:</code> prefix).</p>
    /// <p>Constraints: Up to 1000 tags.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags to delete. Specify a tag key and an optional tag value to delete specific tags. If you specify a tag key without a tag value, we delete any tag with this key regardless of its value. If you specify a tag key with an empty string as the tag value, we delete the tag only if its value is an empty string.</p>
    /// <p>If you omit this parameter, we delete all user-defined tags for the specified resources. We do not delete Amazon Web Services-generated tags (tags that have the <code>aws:</code> prefix).</p>
    /// <p>Constraints: Up to 1000 tags.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
