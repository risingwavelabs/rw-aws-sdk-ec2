// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSubnetsOutput {
    /// <p>Information about one or more subnets.</p>
    pub subnets: ::std::option::Option<::std::vec::Vec<crate::types::Subnet>>,
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeSubnetsOutput {
    /// <p>Information about one or more subnets.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subnets.is_none()`.
    pub fn subnets(&self) -> &[crate::types::Subnet] {
        self.subnets.as_deref().unwrap_or_default()
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeSubnetsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeSubnetsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSubnetsOutput`](crate::operation::describe_subnets::DescribeSubnetsOutput).
    pub fn builder() -> crate::operation::describe_subnets::builders::DescribeSubnetsOutputBuilder {
        crate::operation::describe_subnets::builders::DescribeSubnetsOutputBuilder::default()
    }
}

/// A builder for [`DescribeSubnetsOutput`](crate::operation::describe_subnets::DescribeSubnetsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeSubnetsOutputBuilder {
    pub(crate) subnets: ::std::option::Option<::std::vec::Vec<crate::types::Subnet>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeSubnetsOutputBuilder {
    /// Appends an item to `subnets`.
    ///
    /// To override the contents of this collection use [`set_subnets`](Self::set_subnets).
    ///
    /// <p>Information about one or more subnets.</p>
    pub fn subnets(mut self, input: crate::types::Subnet) -> Self {
        let mut v = self.subnets.unwrap_or_default();
        v.push(input);
        self.subnets = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about one or more subnets.</p>
    pub fn set_subnets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Subnet>>) -> Self {
        self.subnets = input;
        self
    }
    /// <p>Information about one or more subnets.</p>
    pub fn get_subnets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Subnet>> {
        &self.subnets
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeSubnetsOutput`](crate::operation::describe_subnets::DescribeSubnetsOutput).
    pub fn build(self) -> crate::operation::describe_subnets::DescribeSubnetsOutput {
        crate::operation::describe_subnets::DescribeSubnetsOutput {
            subnets: self.subnets,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
