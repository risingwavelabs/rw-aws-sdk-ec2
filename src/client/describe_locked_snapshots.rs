// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeLockedSnapshots`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Filter)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::set_filters):<br>required: **false**<br><p>The filters.</p>  <ul>   <li> <p> <code>lock-state</code> - The state of the snapshot lock (<code>compliance-cooloff</code> | <code>governance</code> | <code>compliance</code> | <code>expired</code>).</p> </li>  </ul><br>
    ///   - [`max_results(i32)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p><br>
    ///   - [`snapshot_ids(impl Into<String>)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::snapshot_ids) / [`set_snapshot_ids(Option<Vec::<String>>)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::set_snapshot_ids):<br>required: **false**<br><p>The IDs of the snapshots for which to view the lock status.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DescribeLockedSnapshotsOutput`](crate::operation::describe_locked_snapshots::DescribeLockedSnapshotsOutput) with field(s):
    ///   - [`snapshots(Option<Vec::<LockedSnapshotsInfo>>)`](crate::operation::describe_locked_snapshots::DescribeLockedSnapshotsOutput::snapshots): <p>Information about the snapshots.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_locked_snapshots::DescribeLockedSnapshotsOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<DescribeLockedSnapshotsError>`](crate::operation::describe_locked_snapshots::DescribeLockedSnapshotsError)
    pub fn describe_locked_snapshots(&self) -> crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder {
        crate::operation::describe_locked_snapshots::builders::DescribeLockedSnapshotsFluentBuilder::new(self.handle.clone())
    }
}
