// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVerifiedAccessInstancesInput {
    /// <p>The IDs of the Verified Access instances.</p>
    pub verified_access_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token for the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DescribeVerifiedAccessInstancesInput {
    /// <p>The IDs of the Verified Access instances.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.verified_access_instance_ids.is_none()`.
    pub fn verified_access_instance_ids(&self) -> &[::std::string::String] {
        self.verified_access_instance_ids.as_deref().unwrap_or_default()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeVerifiedAccessInstancesInput {
    /// Creates a new builder-style object to manufacture [`DescribeVerifiedAccessInstancesInput`](crate::operation::describe_verified_access_instances::DescribeVerifiedAccessInstancesInput).
    pub fn builder() -> crate::operation::describe_verified_access_instances::builders::DescribeVerifiedAccessInstancesInputBuilder {
        crate::operation::describe_verified_access_instances::builders::DescribeVerifiedAccessInstancesInputBuilder::default()
    }
}

/// A builder for [`DescribeVerifiedAccessInstancesInput`](crate::operation::describe_verified_access_instances::DescribeVerifiedAccessInstancesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeVerifiedAccessInstancesInputBuilder {
    pub(crate) verified_access_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DescribeVerifiedAccessInstancesInputBuilder {
    /// Appends an item to `verified_access_instance_ids`.
    ///
    /// To override the contents of this collection use [`set_verified_access_instance_ids`](Self::set_verified_access_instance_ids).
    ///
    /// <p>The IDs of the Verified Access instances.</p>
    pub fn verified_access_instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.verified_access_instance_ids.unwrap_or_default();
        v.push(input.into());
        self.verified_access_instance_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the Verified Access instances.</p>
    pub fn set_verified_access_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.verified_access_instance_ids = input;
        self
    }
    /// <p>The IDs of the Verified Access instances.</p>
    pub fn get_verified_access_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.verified_access_instance_ids
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>One or more filters. Filter names and values are case-sensitive.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
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
    /// Consumes the builder and constructs a [`DescribeVerifiedAccessInstancesInput`](crate::operation::describe_verified_access_instances::DescribeVerifiedAccessInstancesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_verified_access_instances::DescribeVerifiedAccessInstancesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_verified_access_instances::DescribeVerifiedAccessInstancesInput {
                verified_access_instance_ids: self.verified_access_instance_ids,
                max_results: self.max_results,
                next_token: self.next_token,
                filters: self.filters,
                dry_run: self.dry_run,
            },
        )
    }
}
