// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeNetworkInsightsAccessScopesOutput {
    /// <p>The Network Access Scopes.</p>
    pub network_insights_access_scopes: ::std::option::Option<::std::vec::Vec<crate::types::NetworkInsightsAccessScope>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeNetworkInsightsAccessScopesOutput {
    /// <p>The Network Access Scopes.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.network_insights_access_scopes.is_none()`.
    pub fn network_insights_access_scopes(&self) -> &[crate::types::NetworkInsightsAccessScope] {
        self.network_insights_access_scopes.as_deref().unwrap_or_default()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeNetworkInsightsAccessScopesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeNetworkInsightsAccessScopesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeNetworkInsightsAccessScopesOutput`](crate::operation::describe_network_insights_access_scopes::DescribeNetworkInsightsAccessScopesOutput).
    pub fn builder() -> crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesOutputBuilder {
        crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesOutputBuilder::default()
    }
}

/// A builder for [`DescribeNetworkInsightsAccessScopesOutput`](crate::operation::describe_network_insights_access_scopes::DescribeNetworkInsightsAccessScopesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeNetworkInsightsAccessScopesOutputBuilder {
    pub(crate) network_insights_access_scopes: ::std::option::Option<::std::vec::Vec<crate::types::NetworkInsightsAccessScope>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeNetworkInsightsAccessScopesOutputBuilder {
    /// Appends an item to `network_insights_access_scopes`.
    ///
    /// To override the contents of this collection use [`set_network_insights_access_scopes`](Self::set_network_insights_access_scopes).
    ///
    /// <p>The Network Access Scopes.</p>
    pub fn network_insights_access_scopes(mut self, input: crate::types::NetworkInsightsAccessScope) -> Self {
        let mut v = self.network_insights_access_scopes.unwrap_or_default();
        v.push(input);
        self.network_insights_access_scopes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Network Access Scopes.</p>
    pub fn set_network_insights_access_scopes(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NetworkInsightsAccessScope>>,
    ) -> Self {
        self.network_insights_access_scopes = input;
        self
    }
    /// <p>The Network Access Scopes.</p>
    pub fn get_network_insights_access_scopes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::NetworkInsightsAccessScope>> {
        &self.network_insights_access_scopes
    }
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
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeNetworkInsightsAccessScopesOutput`](crate::operation::describe_network_insights_access_scopes::DescribeNetworkInsightsAccessScopesOutput).
    pub fn build(self) -> crate::operation::describe_network_insights_access_scopes::DescribeNetworkInsightsAccessScopesOutput {
        crate::operation::describe_network_insights_access_scopes::DescribeNetworkInsightsAccessScopesOutput {
            network_insights_access_scopes: self.network_insights_access_scopes,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
