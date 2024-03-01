// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeSnapshots`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Filter)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::set_filters):<br>required: **false**<br><p>The filters.</p>  <ul>   <li> <p> <code>description</code> - A description of the snapshot.</p> </li>   <li> <p> <code>encrypted</code> - Indicates whether the snapshot is encrypted (<code>true</code> | <code>false</code>)</p> </li>   <li> <p> <code>owner-alias</code> - The owner alias, from an Amazon-maintained list (<code>amazon</code>). This is not the user-configured Amazon Web Services account alias set using the IAM console. We recommend that you use the related parameter instead of this filter.</p> </li>   <li> <p> <code>owner-id</code> - The Amazon Web Services account ID of the owner. We recommend that you use the related parameter instead of this filter.</p> </li>   <li> <p> <code>progress</code> - The progress of the snapshot, as a percentage (for example, 80%).</p> </li>   <li> <p> <code>snapshot-id</code> - The snapshot ID.</p> </li>   <li> <p> <code>start-time</code> - The time stamp when the snapshot was initiated.</p> </li>   <li> <p> <code>status</code> - The status of the snapshot (<code>pending</code> | <code>completed</code> | <code>error</code>).</p> </li>   <li> <p> <code>storage-tier</code> - The storage tier of the snapshot (<code>archive</code> | <code>standard</code>).</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>volume-id</code> - The ID of the volume the snapshot is for.</p> </li>   <li> <p> <code>volume-size</code> - The size of the volume, in GiB.</p> </li>  </ul><br>
    ///   - [`max_results(i32)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of snapshots to return for this request. This value can be between 5 and 1,000; if this value is larger than 1,000, only 1,000 results are returned. If this parameter is not used, then the request returns all snapshots. You cannot specify this parameter and the snapshot IDs parameter in the same request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p><br>
    ///   - [`owner_ids(impl Into<String>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::owner_ids) / [`set_owner_ids(Option<Vec::<String>>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::set_owner_ids):<br>required: **false**<br><p>Scopes the results to snapshots with the specified owners. You can specify a combination of Amazon Web Services account IDs, <code>self</code>, and <code>amazon</code>.</p><br>
    ///   - [`restorable_by_user_ids(impl Into<String>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::restorable_by_user_ids) / [`set_restorable_by_user_ids(Option<Vec::<String>>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::set_restorable_by_user_ids):<br>required: **false**<br><p>The IDs of the Amazon Web Services accounts that can create volumes from the snapshot.</p><br>
    ///   - [`snapshot_ids(impl Into<String>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::snapshot_ids) / [`set_snapshot_ids(Option<Vec::<String>>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::set_snapshot_ids):<br>required: **false**<br><p>The snapshot IDs.</p>  <p>Default: Describes the snapshots for which you have create volume permissions.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DescribeSnapshotsOutput`](crate::operation::describe_snapshots::DescribeSnapshotsOutput) with field(s):
    ///   - [`snapshots(Option<Vec::<Snapshot>>)`](crate::operation::describe_snapshots::DescribeSnapshotsOutput::snapshots): <p>Information about the snapshots.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_snapshots::DescribeSnapshotsOutput::next_token): <p>The token to include in another request to return the next page of snapshots. This value is <code>null</code> when there are no more snapshots to return.</p>
    /// - On failure, responds with [`SdkError<DescribeSnapshotsError>`](crate::operation::describe_snapshots::DescribeSnapshotsError)
    pub fn describe_snapshots(&self) -> crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder {
        crate::operation::describe_snapshots::builders::DescribeSnapshotsFluentBuilder::new(self.handle.clone())
    }
}
