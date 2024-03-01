// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSecurityGroupsOutput {
    /// <p>Information about the security groups.</p>
    pub security_groups: ::std::option::Option<::std::vec::Vec<crate::types::SecurityGroup>>,
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeSecurityGroupsOutput {
    /// <p>Information about the security groups.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.security_groups.is_none()`.
    pub fn security_groups(&self) -> &[crate::types::SecurityGroup] {
        self.security_groups.as_deref().unwrap_or_default()
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeSecurityGroupsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeSecurityGroupsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeSecurityGroupsOutput`](crate::operation::describe_security_groups::DescribeSecurityGroupsOutput).
    pub fn builder() -> crate::operation::describe_security_groups::builders::DescribeSecurityGroupsOutputBuilder {
        crate::operation::describe_security_groups::builders::DescribeSecurityGroupsOutputBuilder::default()
    }
}

/// A builder for [`DescribeSecurityGroupsOutput`](crate::operation::describe_security_groups::DescribeSecurityGroupsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeSecurityGroupsOutputBuilder {
    pub(crate) security_groups: ::std::option::Option<::std::vec::Vec<crate::types::SecurityGroup>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeSecurityGroupsOutputBuilder {
    /// Appends an item to `security_groups`.
    ///
    /// To override the contents of this collection use [`set_security_groups`](Self::set_security_groups).
    ///
    /// <p>Information about the security groups.</p>
    pub fn security_groups(mut self, input: crate::types::SecurityGroup) -> Self {
        let mut v = self.security_groups.unwrap_or_default();
        v.push(input);
        self.security_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the security groups.</p>
    pub fn set_security_groups(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SecurityGroup>>) -> Self {
        self.security_groups = input;
        self
    }
    /// <p>Information about the security groups.</p>
    pub fn get_security_groups(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SecurityGroup>> {
        &self.security_groups
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
    /// Consumes the builder and constructs a [`DescribeSecurityGroupsOutput`](crate::operation::describe_security_groups::DescribeSecurityGroupsOutput).
    pub fn build(self) -> crate::operation::describe_security_groups::DescribeSecurityGroupsOutput {
        crate::operation::describe_security_groups::DescribeSecurityGroupsOutput {
            security_groups: self.security_groups,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
