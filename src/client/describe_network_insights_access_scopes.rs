// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeNetworkInsightsAccessScopes`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_insights_access_scope_ids(impl Into<String>)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::network_insights_access_scope_ids) / [`set_network_insights_access_scope_ids(Option<Vec::<String>>)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::set_network_insights_access_scope_ids):<br>required: **false**<br><p>The IDs of the Network Access Scopes.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::set_filters):<br>required: **false**<br><p>There are no supported filters.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    /// - On success, responds with [`DescribeNetworkInsightsAccessScopesOutput`](crate::operation::describe_network_insights_access_scopes::DescribeNetworkInsightsAccessScopesOutput) with field(s):
    ///   - [`network_insights_access_scopes(Option<Vec::<NetworkInsightsAccessScope>>)`](crate::operation::describe_network_insights_access_scopes::DescribeNetworkInsightsAccessScopesOutput::network_insights_access_scopes): <p>The Network Access Scopes.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_network_insights_access_scopes::DescribeNetworkInsightsAccessScopesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeNetworkInsightsAccessScopesError>`](crate::operation::describe_network_insights_access_scopes::DescribeNetworkInsightsAccessScopesError)
    pub fn describe_network_insights_access_scopes(
        &self,
    ) -> crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder {
        crate::operation::describe_network_insights_access_scopes::builders::DescribeNetworkInsightsAccessScopesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
