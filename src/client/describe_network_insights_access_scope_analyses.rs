// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeNetworkInsightsAccessScopeAnalyses`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`network_insights_access_scope_analysis_ids(impl Into<String>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::network_insights_access_scope_analysis_ids) / [`set_network_insights_access_scope_analysis_ids(Option<Vec::<String>>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::set_network_insights_access_scope_analysis_ids):<br>required: **false**<br><p>The IDs of the Network Access Scope analyses.</p><br>
    ///   - [`network_insights_access_scope_id(impl Into<String>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::network_insights_access_scope_id) / [`set_network_insights_access_scope_id(Option<String>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::set_network_insights_access_scope_id):<br>required: **false**<br><p>The ID of the Network Access Scope.</p><br>
    ///   - [`analysis_start_time_begin(DateTime)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::analysis_start_time_begin) / [`set_analysis_start_time_begin(Option<DateTime>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::set_analysis_start_time_begin):<br>required: **false**<br><p>Filters the results based on the start time. The analysis must have started on or after this time.</p><br>
    ///   - [`analysis_start_time_end(DateTime)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::analysis_start_time_end) / [`set_analysis_start_time_end(Option<DateTime>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::set_analysis_start_time_end):<br>required: **false**<br><p>Filters the results based on the start time. The analysis must have started on or before this time.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::set_filters):<br>required: **false**<br><p>There are no supported filters.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    /// - On success, responds with [`DescribeNetworkInsightsAccessScopeAnalysesOutput`](crate::operation::describe_network_insights_access_scope_analyses::DescribeNetworkInsightsAccessScopeAnalysesOutput) with field(s):
    ///   - [`network_insights_access_scope_analyses(Option<Vec::<NetworkInsightsAccessScopeAnalysis>>)`](crate::operation::describe_network_insights_access_scope_analyses::DescribeNetworkInsightsAccessScopeAnalysesOutput::network_insights_access_scope_analyses): <p>The Network Access Scope analyses.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_network_insights_access_scope_analyses::DescribeNetworkInsightsAccessScopeAnalysesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeNetworkInsightsAccessScopeAnalysesError>`](crate::operation::describe_network_insights_access_scope_analyses::DescribeNetworkInsightsAccessScopeAnalysesError)
    pub fn describe_network_insights_access_scope_analyses(
        &self,
    ) -> crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder {
        crate::operation::describe_network_insights_access_scope_analyses::builders::DescribeNetworkInsightsAccessScopeAnalysesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
