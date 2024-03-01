// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetAssociatedIpv6PoolCidrsInput {
    /// <p>The ID of the IPv6 address pool.</p>
    pub pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The token for the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl GetAssociatedIpv6PoolCidrsInput {
    /// <p>The ID of the IPv6 address pool.</p>
    pub fn pool_id(&self) -> ::std::option::Option<&str> {
        self.pool_id.as_deref()
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl GetAssociatedIpv6PoolCidrsInput {
    /// Creates a new builder-style object to manufacture [`GetAssociatedIpv6PoolCidrsInput`](crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsInput).
    pub fn builder() -> crate::operation::get_associated_ipv6_pool_cidrs::builders::GetAssociatedIpv6PoolCidrsInputBuilder {
        crate::operation::get_associated_ipv6_pool_cidrs::builders::GetAssociatedIpv6PoolCidrsInputBuilder::default()
    }
}

/// A builder for [`GetAssociatedIpv6PoolCidrsInput`](crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetAssociatedIpv6PoolCidrsInputBuilder {
    pub(crate) pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl GetAssociatedIpv6PoolCidrsInputBuilder {
    /// <p>The ID of the IPv6 address pool.</p>
    /// This field is required.
    pub fn pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the IPv6 address pool.</p>
    pub fn set_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pool_id = input;
        self
    }
    /// <p>The ID of the IPv6 address pool.</p>
    pub fn get_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.pool_id
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
    /// Consumes the builder and constructs a [`GetAssociatedIpv6PoolCidrsInput`](crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_associated_ipv6_pool_cidrs::GetAssociatedIpv6PoolCidrsInput {
            pool_id: self.pool_id,
            next_token: self.next_token,
            max_results: self.max_results,
            dry_run: self.dry_run,
        })
    }
}
