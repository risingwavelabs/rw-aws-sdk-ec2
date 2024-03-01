// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetIpamDiscoveredAccountsInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>A resource discovery ID.</p>
    pub ipam_resource_discovery_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region that the account information is returned from.</p>
    pub discovery_region: ::std::option::Option<::std::string::String>,
    /// <p>Discovered account filters.</p>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of discovered accounts to return in one page of results.</p>
    pub max_results: ::std::option::Option<i32>,
}
impl GetIpamDiscoveredAccountsInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>A resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(&self) -> ::std::option::Option<&str> {
        self.ipam_resource_discovery_id.as_deref()
    }
    /// <p>The Amazon Web Services Region that the account information is returned from.</p>
    pub fn discovery_region(&self) -> ::std::option::Option<&str> {
        self.discovery_region.as_deref()
    }
    /// <p>Discovered account filters.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of discovered accounts to return in one page of results.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl GetIpamDiscoveredAccountsInput {
    /// Creates a new builder-style object to manufacture [`GetIpamDiscoveredAccountsInput`](crate::operation::get_ipam_discovered_accounts::GetIpamDiscoveredAccountsInput).
    pub fn builder() -> crate::operation::get_ipam_discovered_accounts::builders::GetIpamDiscoveredAccountsInputBuilder {
        crate::operation::get_ipam_discovered_accounts::builders::GetIpamDiscoveredAccountsInputBuilder::default()
    }
}

/// A builder for [`GetIpamDiscoveredAccountsInput`](crate::operation::get_ipam_discovered_accounts::GetIpamDiscoveredAccountsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetIpamDiscoveredAccountsInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) ipam_resource_discovery_id: ::std::option::Option<::std::string::String>,
    pub(crate) discovery_region: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl GetIpamDiscoveredAccountsInputBuilder {
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
    /// <p>A resource discovery ID.</p>
    /// This field is required.
    pub fn ipam_resource_discovery_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipam_resource_discovery_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A resource discovery ID.</p>
    pub fn set_ipam_resource_discovery_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipam_resource_discovery_id = input;
        self
    }
    /// <p>A resource discovery ID.</p>
    pub fn get_ipam_resource_discovery_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipam_resource_discovery_id
    }
    /// <p>The Amazon Web Services Region that the account information is returned from.</p>
    /// This field is required.
    pub fn discovery_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.discovery_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region that the account information is returned from.</p>
    pub fn set_discovery_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.discovery_region = input;
        self
    }
    /// <p>The Amazon Web Services Region that the account information is returned from.</p>
    pub fn get_discovery_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.discovery_region
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Discovered account filters.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>Discovered account filters.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>Discovered account filters.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The maximum number of discovered accounts to return in one page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of discovered accounts to return in one page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of discovered accounts to return in one page of results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// Consumes the builder and constructs a [`GetIpamDiscoveredAccountsInput`](crate::operation::get_ipam_discovered_accounts::GetIpamDiscoveredAccountsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_ipam_discovered_accounts::GetIpamDiscoveredAccountsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_ipam_discovered_accounts::GetIpamDiscoveredAccountsInput {
            dry_run: self.dry_run,
            ipam_resource_discovery_id: self.ipam_resource_discovery_id,
            discovery_region: self.discovery_region,
            filters: self.filters,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
