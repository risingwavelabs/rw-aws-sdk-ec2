// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCoipPoolUsageInput {
    /// <p>The ID of the address pool.</p>
    pub pool_id: ::std::option::Option<::std::string::String>,
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>coip-address-usage.allocation-id</code> - The allocation ID of the address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-account-id</code> - The ID of the Amazon Web Services account that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-service</code> - The Amazon Web Services service that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.co-ip</code> - The customer-owned IP address.</p> </li>
    /// </ul>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token for the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl GetCoipPoolUsageInput {
    /// <p>The ID of the address pool.</p>
    pub fn pool_id(&self) -> ::std::option::Option<&str> {
        self.pool_id.as_deref()
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>coip-address-usage.allocation-id</code> - The allocation ID of the address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-account-id</code> - The ID of the Amazon Web Services account that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-service</code> - The Amazon Web Services service that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.co-ip</code> - The customer-owned IP address.</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl GetCoipPoolUsageInput {
    /// Creates a new builder-style object to manufacture [`GetCoipPoolUsageInput`](crate::operation::get_coip_pool_usage::GetCoipPoolUsageInput).
    pub fn builder() -> crate::operation::get_coip_pool_usage::builders::GetCoipPoolUsageInputBuilder {
        crate::operation::get_coip_pool_usage::builders::GetCoipPoolUsageInputBuilder::default()
    }
}

/// A builder for [`GetCoipPoolUsageInput`](crate::operation::get_coip_pool_usage::GetCoipPoolUsageInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetCoipPoolUsageInputBuilder {
    pub(crate) pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl GetCoipPoolUsageInputBuilder {
    /// <p>The ID of the address pool.</p>
    /// This field is required.
    pub fn pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the address pool.</p>
    pub fn set_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pool_id = input;
        self
    }
    /// <p>The ID of the address pool.</p>
    pub fn get_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.pool_id
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>coip-address-usage.allocation-id</code> - The allocation ID of the address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-account-id</code> - The ID of the Amazon Web Services account that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-service</code> - The Amazon Web Services service that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.co-ip</code> - The customer-owned IP address.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>coip-address-usage.allocation-id</code> - The allocation ID of the address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-account-id</code> - The ID of the Amazon Web Services account that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-service</code> - The Amazon Web Services service that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.co-ip</code> - The customer-owned IP address.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>coip-address-usage.allocation-id</code> - The allocation ID of the address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-account-id</code> - The ID of the Amazon Web Services account that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.aws-service</code> - The Amazon Web Services service that is using the customer-owned IP address.</p> </li>
    /// <li> <p> <code>coip-address-usage.co-ip</code> - The customer-owned IP address.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
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
    /// Consumes the builder and constructs a [`GetCoipPoolUsageInput`](crate::operation::get_coip_pool_usage::GetCoipPoolUsageInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_coip_pool_usage::GetCoipPoolUsageInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_coip_pool_usage::GetCoipPoolUsageInput {
            pool_id: self.pool_id,
            filters: self.filters,
            max_results: self.max_results,
            next_token: self.next_token,
            dry_run: self.dry_run,
        })
    }
}
