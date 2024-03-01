// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeIpamPoolsOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Information about the IPAM pools.</p>
    pub ipam_pools: ::std::option::Option<::std::vec::Vec<crate::types::IpamPool>>,
    _request_id: Option<String>,
}
impl DescribeIpamPoolsOutput {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Information about the IPAM pools.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ipam_pools.is_none()`.
    pub fn ipam_pools(&self) -> &[crate::types::IpamPool] {
        self.ipam_pools.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DescribeIpamPoolsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeIpamPoolsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeIpamPoolsOutput`](crate::operation::describe_ipam_pools::DescribeIpamPoolsOutput).
    pub fn builder() -> crate::operation::describe_ipam_pools::builders::DescribeIpamPoolsOutputBuilder {
        crate::operation::describe_ipam_pools::builders::DescribeIpamPoolsOutputBuilder::default()
    }
}

/// A builder for [`DescribeIpamPoolsOutput`](crate::operation::describe_ipam_pools::DescribeIpamPoolsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeIpamPoolsOutputBuilder {
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) ipam_pools: ::std::option::Option<::std::vec::Vec<crate::types::IpamPool>>,
    _request_id: Option<String>,
}
impl DescribeIpamPoolsOutputBuilder {
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Appends an item to `ipam_pools`.
    ///
    /// To override the contents of this collection use [`set_ipam_pools`](Self::set_ipam_pools).
    ///
    /// <p>Information about the IPAM pools.</p>
    pub fn ipam_pools(mut self, input: crate::types::IpamPool) -> Self {
        let mut v = self.ipam_pools.unwrap_or_default();
        v.push(input);
        self.ipam_pools = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the IPAM pools.</p>
    pub fn set_ipam_pools(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::IpamPool>>) -> Self {
        self.ipam_pools = input;
        self
    }
    /// <p>Information about the IPAM pools.</p>
    pub fn get_ipam_pools(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::IpamPool>> {
        &self.ipam_pools
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeIpamPoolsOutput`](crate::operation::describe_ipam_pools::DescribeIpamPoolsOutput).
    pub fn build(self) -> crate::operation::describe_ipam_pools::DescribeIpamPoolsOutput {
        crate::operation::describe_ipam_pools::DescribeIpamPoolsOutput {
            next_token: self.next_token,
            ipam_pools: self.ipam_pools,
            _request_id: self._request_id,
        }
    }
}