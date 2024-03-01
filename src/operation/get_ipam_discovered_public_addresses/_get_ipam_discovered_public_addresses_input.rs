// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetIpamDiscoveredPublicAddressesInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>An IPAM resource discovery ID.</p>
    pub ipam_resource_discovery_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Web Services Region for the IP address.</p>
    pub address_region: ::std::option::Option<::std::string::String>,
    /// <p>Filters.</p>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The token for the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of IPAM discovered public addresses to return in one page of results.</p>
    pub max_results: ::std::option::Option<i32>,
}
impl GetIpamDiscoveredPublicAddressesInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>An IPAM resource discovery ID.</p>
    pub fn ipam_resource_discovery_id(&self) -> ::std::option::Option<&str> {
        self.ipam_resource_discovery_id.as_deref()
    }
    /// <p>The Amazon Web Services Region for the IP address.</p>
    pub fn address_region(&self) -> ::std::option::Option<&str> {
        self.address_region.as_deref()
    }
    /// <p>Filters.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of IPAM discovered public addresses to return in one page of results.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl GetIpamDiscoveredPublicAddressesInput {
    /// Creates a new builder-style object to manufacture [`GetIpamDiscoveredPublicAddressesInput`](crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesInput).
    pub fn builder() -> crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesInputBuilder {
        crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesInputBuilder::default()
    }
}

/// A builder for [`GetIpamDiscoveredPublicAddressesInput`](crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetIpamDiscoveredPublicAddressesInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) ipam_resource_discovery_id: ::std::option::Option<::std::string::String>,
    pub(crate) address_region: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl GetIpamDiscoveredPublicAddressesInputBuilder {
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
    /// <p>An IPAM resource discovery ID.</p>
    /// This field is required.
    pub fn ipam_resource_discovery_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipam_resource_discovery_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An IPAM resource discovery ID.</p>
    pub fn set_ipam_resource_discovery_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipam_resource_discovery_id = input;
        self
    }
    /// <p>An IPAM resource discovery ID.</p>
    pub fn get_ipam_resource_discovery_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipam_resource_discovery_id
    }
    /// <p>The Amazon Web Services Region for the IP address.</p>
    /// This field is required.
    pub fn address_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.address_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services Region for the IP address.</p>
    pub fn set_address_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.address_region = input;
        self
    }
    /// <p>The Amazon Web Services Region for the IP address.</p>
    pub fn get_address_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.address_region
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>Filters.</p>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>Filters.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>Filters.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
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
    /// <p>The maximum number of IPAM discovered public addresses to return in one page of results.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of IPAM discovered public addresses to return in one page of results.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of IPAM discovered public addresses to return in one page of results.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// Consumes the builder and constructs a [`GetIpamDiscoveredPublicAddressesInput`](crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesInput {
                dry_run: self.dry_run,
                ipam_resource_discovery_id: self.ipam_resource_discovery_id,
                address_region: self.address_region,
                filters: self.filters,
                next_token: self.next_token,
                max_results: self.max_results,
            },
        )
    }
}
