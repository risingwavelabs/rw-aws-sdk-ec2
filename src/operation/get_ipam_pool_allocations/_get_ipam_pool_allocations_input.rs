// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetIpamPoolAllocationsInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the IPAM pool you want to see the allocations for.</p>
    pub ipam_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the allocation.</p>
    pub ipam_pool_allocation_id: ::std::option::Option<::std::string::String>,
    /// <p>One or more filters for the request. For more information about filtering, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-filter.html">Filtering CLI output</a>.</p>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of results you would like returned per page.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token for the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl GetIpamPoolAllocationsInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the IPAM pool you want to see the allocations for.</p>
    pub fn ipam_pool_id(&self) -> ::std::option::Option<&str> {
        self.ipam_pool_id.as_deref()
    }
    /// <p>The ID of the allocation.</p>
    pub fn ipam_pool_allocation_id(&self) -> ::std::option::Option<&str> {
        self.ipam_pool_allocation_id.as_deref()
    }
    /// <p>One or more filters for the request. For more information about filtering, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-filter.html">Filtering CLI output</a>.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>The maximum number of results you would like returned per page.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl GetIpamPoolAllocationsInput {
    /// Creates a new builder-style object to manufacture [`GetIpamPoolAllocationsInput`](crate::operation::get_ipam_pool_allocations::GetIpamPoolAllocationsInput).
    pub fn builder() -> crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsInputBuilder {
        crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsInputBuilder::default()
    }
}

/// A builder for [`GetIpamPoolAllocationsInput`](crate::operation::get_ipam_pool_allocations::GetIpamPoolAllocationsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetIpamPoolAllocationsInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) ipam_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) ipam_pool_allocation_id: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl GetIpamPoolAllocationsInputBuilder {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// <p>The ID of the IPAM pool you want to see the allocations for.</p>
    /// This field is required.
    pub fn ipam_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipam_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the IPAM pool you want to see the allocations for.</p>
    pub fn set_ipam_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipam_pool_id = input;
        self
    }
    /// <p>The ID of the IPAM pool you want to see the allocations for.</p>
    pub fn get_ipam_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipam_pool_id
    }
    /// <p>The ID of the allocation.</p>
    pub fn ipam_pool_allocation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipam_pool_allocation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the allocation.</p>
    pub fn set_ipam_pool_allocation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipam_pool_allocation_id = input;
        self
    }
    /// <p>The ID of the allocation.</p>
    pub fn get_ipam_pool_allocation_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipam_pool_allocation_id
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters for the request. For more information about filtering, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-filter.html">Filtering CLI output</a>.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more filters for the request. For more information about filtering, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-filter.html">Filtering CLI output</a>.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>One or more filters for the request. For more information about filtering, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-filter.html">Filtering CLI output</a>.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
    }
    /// <p>The maximum number of results you would like returned per page.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results you would like returned per page.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of results you would like returned per page.</p>
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
    /// Consumes the builder and constructs a [`GetIpamPoolAllocationsInput`](crate::operation::get_ipam_pool_allocations::GetIpamPoolAllocationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_ipam_pool_allocations::GetIpamPoolAllocationsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_ipam_pool_allocations::GetIpamPoolAllocationsInput {
            dry_run: self.dry_run,
            ipam_pool_id: self.ipam_pool_id,
            ipam_pool_allocation_id: self.ipam_pool_allocation_id,
            filters: self.filters,
            max_results: self.max_results,
            next_token: self.next_token,
        })
    }
}