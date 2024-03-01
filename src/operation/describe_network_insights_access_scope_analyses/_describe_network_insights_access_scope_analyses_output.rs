// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeNetworkInsightsAccessScopeAnalysesOutput {
    /// <p>The Network Access Scope analyses.</p>
    pub network_insights_access_scope_analyses: ::std::option::Option<::std::vec::Vec<crate::types::NetworkInsightsAccessScopeAnalysis>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeNetworkInsightsAccessScopeAnalysesOutput {
    /// <p>The Network Access Scope analyses.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.network_insights_access_scope_analyses.is_none()`.
    pub fn network_insights_access_scope_analyses(&self) -> &[crate::types::NetworkInsightsAccessScopeAnalysis] {
        self.network_insights_access_scope_analyses.as_deref().unwrap_or_default()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeNetworkInsightsAccessScopeAnalysesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeNetworkInsightsAccessScopeAnalysesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeNetworkInsightsAccessScopeAnalysesOutput`](crate::operation::describe_network_insights_access_scope_analyses::DescribeNetworkInsightsAccessScopeAnalysesOutput).
    pub fn builder(
    ) -> crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesOutputBuilder {
        crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesOutputBuilder::default(
        )
    }
}

/// A builder for [`DescribeNetworkInsightsAccessScopeAnalysesOutput`](crate::operation::describe_network_insights_access_scope_analyses::DescribeNetworkInsightsAccessScopeAnalysesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeNetworkInsightsAccessScopeAnalysesOutputBuilder {
    pub(crate) network_insights_access_scope_analyses: ::std::option::Option<::std::vec::Vec<crate::types::NetworkInsightsAccessScopeAnalysis>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeNetworkInsightsAccessScopeAnalysesOutputBuilder {
    /// Appends an item to `network_insights_access_scope_analyses`.
    ///
    /// To override the contents of this collection use [`set_network_insights_access_scope_analyses`](Self::set_network_insights_access_scope_analyses).
    ///
    /// <p>The Network Access Scope analyses.</p>
    pub fn network_insights_access_scope_analyses(mut self, input: crate::types::NetworkInsightsAccessScopeAnalysis) -> Self {
        let mut v = self.network_insights_access_scope_analyses.unwrap_or_default();
        v.push(input);
        self.network_insights_access_scope_analyses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Network Access Scope analyses.</p>
    pub fn set_network_insights_access_scope_analyses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::NetworkInsightsAccessScopeAnalysis>>,
    ) -> Self {
        self.network_insights_access_scope_analyses = input;
        self
    }
    /// <p>The Network Access Scope analyses.</p>
    pub fn get_network_insights_access_scope_analyses(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::NetworkInsightsAccessScopeAnalysis>> {
        &self.network_insights_access_scope_analyses
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
    /// Consumes the builder and constructs a [`DescribeNetworkInsightsAccessScopeAnalysesOutput`](crate::operation::describe_network_insights_access_scope_analyses::DescribeNetworkInsightsAccessScopeAnalysesOutput).
    pub fn build(self) -> crate::operation::describe_network_insights_access_scope_analyses::DescribeNetworkInsightsAccessScopeAnalysesOutput {
        crate::operation::describe_network_insights_access_scope_analyses::DescribeNetworkInsightsAccessScopeAnalysesOutput {
            network_insights_access_scope_analyses: self.network_insights_access_scope_analyses,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}