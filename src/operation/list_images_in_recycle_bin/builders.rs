// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_images_in_recycle_bin::_list_images_in_recycle_bin_output::ListImagesInRecycleBinOutputBuilder;

pub use crate::operation::list_images_in_recycle_bin::_list_images_in_recycle_bin_input::ListImagesInRecycleBinInputBuilder;

impl ListImagesInRecycleBinInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_images_in_recycle_bin();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListImagesInRecycleBin`.
///
/// <p>Lists one or more AMIs that are currently in the Recycle Bin. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/recycle-bin.html">Recycle Bin</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListImagesInRecycleBinFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinOutput,
        crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinError,
    > for ListImagesInRecycleBinFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinOutput,
            crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListImagesInRecycleBinFluentBuilder {
    /// Creates a new `ListImagesInRecycleBin`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListImagesInRecycleBin as a reference.
    pub fn as_input(&self) -> &crate::operation::list_images_in_recycle_bin::builders::ListImagesInRecycleBinInputBuilder {
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
        crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBin::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBin::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinOutput,
        crate::operation::list_images_in_recycle_bin::ListImagesInRecycleBinError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_images_in_recycle_bin::paginator::ListImagesInRecycleBinPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_images_in_recycle_bin::paginator::ListImagesInRecycleBinPaginator {
        crate::operation::list_images_in_recycle_bin::paginator::ListImagesInRecycleBinPaginator::new(self.handle, self.inner)
    }
    /// Appends an item to `ImageIds`.
    ///
    /// To override the contents of this collection use [`set_image_ids`](Self::set_image_ids).
    ///
    /// <p>The IDs of the AMIs to list. Omit this parameter to list all of the AMIs that are in the Recycle Bin. You can specify up to 20 IDs in a single request.</p>
    pub fn image_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.image_ids(input.into());
        self
    }
    /// <p>The IDs of the AMIs to list. Omit this parameter to list all of the AMIs that are in the Recycle Bin. You can specify up to 20 IDs in a single request.</p>
    pub fn set_image_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_image_ids(input);
        self
    }
    /// <p>The IDs of the AMIs to list. Omit this parameter to list all of the AMIs that are in the Recycle Bin. You can specify up to 20 IDs in a single request.</p>
    pub fn get_image_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_image_ids()
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
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