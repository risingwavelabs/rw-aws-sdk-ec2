// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeSpotFleetRequests`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p><br>
    ///   - [`spot_fleet_request_ids(impl Into<String>)`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::spot_fleet_request_ids) / [`set_spot_fleet_request_ids(Option<Vec::<String>>)`](crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::set_spot_fleet_request_ids):<br>required: **false**<br><p>The IDs of the Spot Fleet requests.</p><br>
    /// - On success, responds with [`DescribeSpotFleetRequestsOutput`](crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    ///   - [`spot_fleet_request_configs(Option<Vec::<SpotFleetRequestConfig>>)`](crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsOutput::spot_fleet_request_configs): <p>Information about the configuration of your Spot Fleet.</p>
    /// - On failure, responds with [`SdkError<DescribeSpotFleetRequestsError>`](crate::operation::describe_spot_fleet_requests::DescribeSpotFleetRequestsError)
    pub fn describe_spot_fleet_requests(&self) -> crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder {
        crate::operation::describe_spot_fleet_requests::builders::DescribeSpotFleetRequestsFluentBuilder::new(self.handle.clone())
    }
}
