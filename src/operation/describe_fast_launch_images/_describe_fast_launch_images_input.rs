// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeFastLaunchImagesInput {
    /// <p>Specify one or more Windows AMI image IDs for the request.</p>
    pub image_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Use the following filters to streamline results.</p>
    /// <ul>
    /// <li> <p> <code>resource-type</code> - The resource type for pre-provisioning.</p> </li>
    /// <li> <p> <code>owner-id</code> - The owner ID for the pre-provisioning resource.</p> </li>
    /// <li> <p> <code>state</code> - The current state of fast launching for the Windows AMI.</p> </li>
    /// </ul>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DescribeFastLaunchImagesInput {
    /// <p>Specify one or more Windows AMI image IDs for the request.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.image_ids.is_none()`.
    pub fn image_ids(&self) -> &[::std::string::String] {
        self.image_ids.as_deref().unwrap_or_default()
    }
    /// <p>Use the following filters to streamline results.</p>
    /// <ul>
    /// <li> <p> <code>resource-type</code> - The resource type for pre-provisioning.</p> </li>
    /// <li> <p> <code>owner-id</code> - The owner ID for the pre-provisioning resource.</p> </li>
    /// <li> <p> <code>state</code> - The current state of fast launching for the Windows AMI.</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeFastLaunchImagesInput {
    /// Creates a new builder-style object to manufacture [`DescribeFastLaunchImagesInput`](crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesInput).
    pub fn builder() -> crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesInputBuilder {
        crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesInputBuilder::default()
    }
}

/// A builder for [`DescribeFastLaunchImagesInput`](crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeFastLaunchImagesInputBuilder {
    pub(crate) image_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DescribeFastLaunchImagesInputBuilder {
    /// Appends an item to `image_ids`.
    ///
    /// To override the contents of this collection use [`set_image_ids`](Self::set_image_ids).
    ///
    /// <p>Specify one or more Windows AMI image IDs for the request.</p>
    pub fn image_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.image_ids.unwrap_or_default();
        v.push(input.into());
        self.image_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specify one or more Windows AMI image IDs for the request.</p>
    pub fn set_image_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.image_ids = input;
        self
    }
    /// <p>Specify one or more Windows AMI image IDs for the request.</p>
    pub fn get_image_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.image_ids
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Use the following filters to streamline results.</p>
    /// <ul>
    /// <li> <p> <code>resource-type</code> - The resource type for pre-provisioning.</p> </li>
    /// <li> <p> <code>owner-id</code> - The owner ID for the pre-provisioning resource.</p> </li>
    /// <li> <p> <code>state</code> - The current state of fast launching for the Windows AMI.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>Use the following filters to streamline results.</p>
    /// <ul>
    /// <li> <p> <code>resource-type</code> - The resource type for pre-provisioning.</p> </li>
    /// <li> <p> <code>owner-id</code> - The owner ID for the pre-provisioning resource.</p> </li>
    /// <li> <p> <code>state</code> - The current state of fast launching for the Windows AMI.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>Use the following filters to streamline results.</p>
    /// <ul>
    /// <li> <p> <code>resource-type</code> - The resource type for pre-provisioning.</p> </li>
    /// <li> <p> <code>owner-id</code> - The owner ID for the pre-provisioning resource.</p> </li>
    /// <li> <p> <code>state</code> - The current state of fast launching for the Windows AMI.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`DescribeFastLaunchImagesInput`](crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesInput {
            image_ids: self.image_ids,
            filters: self.filters,
            max_results: self.max_results,
            next_token: self.next_token,
            dry_run: self.dry_run,
        })
    }
}
