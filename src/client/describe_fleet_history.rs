// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeFleetHistory`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`event_type(FleetEventType)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::event_type) / [`set_event_type(Option<FleetEventType>)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::set_event_type):<br>required: **false**<br><p>The type of events to describe. By default, all events are described.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::set_next_token):<br>required: **false**<br><p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p><br>
    ///   - [`fleet_id(impl Into<String>)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::fleet_id) / [`set_fleet_id(Option<String>)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::set_fleet_id):<br>required: **true**<br><p>The ID of the EC2 Fleet.</p><br>
    ///   - [`start_time(DateTime)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::start_time) / [`set_start_time(Option<DateTime>)`](crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::set_start_time):<br>required: **true**<br><p>The start date and time for the events, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p><br>
    /// - On success, responds with [`DescribeFleetHistoryOutput`](crate::operation::describe_fleet_history::DescribeFleetHistoryOutput) with field(s):
    ///   - [`history_records(Option<Vec::<HistoryRecordEntry>>)`](crate::operation::describe_fleet_history::DescribeFleetHistoryOutput::history_records): <p>Information about the events in the history of the EC2 Fleet.</p>
    ///   - [`last_evaluated_time(Option<DateTime>)`](crate::operation::describe_fleet_history::DescribeFleetHistoryOutput::last_evaluated_time): <p>The last date and time for the events, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). All records up to this time were retrieved.</p>  <p>If <code>nextToken</code> indicates that there are more items, this value is not present.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_fleet_history::DescribeFleetHistoryOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    ///   - [`fleet_id(Option<String>)`](crate::operation::describe_fleet_history::DescribeFleetHistoryOutput::fleet_id): <p>The ID of the EC Fleet.</p>
    ///   - [`start_time(Option<DateTime>)`](crate::operation::describe_fleet_history::DescribeFleetHistoryOutput::start_time): <p>The start date and time for the events, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// - On failure, responds with [`SdkError<DescribeFleetHistoryError>`](crate::operation::describe_fleet_history::DescribeFleetHistoryError)
    pub fn describe_fleet_history(&self) -> crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder {
        crate::operation::describe_fleet_history::builders::DescribeFleetHistoryFluentBuilder::new(self.handle.clone())
    }
}
