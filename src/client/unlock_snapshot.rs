// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UnlockSnapshot`](crate::operation::unlock_snapshot::builders::UnlockSnapshotFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`snapshot_id(impl Into<String>)`](crate::operation::unlock_snapshot::builders::UnlockSnapshotFluentBuilder::snapshot_id) / [`set_snapshot_id(Option<String>)`](crate::operation::unlock_snapshot::builders::UnlockSnapshotFluentBuilder::set_snapshot_id):<br>required: **true**<br><p>The ID of the snapshot to unlock.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::unlock_snapshot::builders::UnlockSnapshotFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::unlock_snapshot::builders::UnlockSnapshotFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`UnlockSnapshotOutput`](crate::operation::unlock_snapshot::UnlockSnapshotOutput) with field(s):
    ///   - [`snapshot_id(Option<String>)`](crate::operation::unlock_snapshot::UnlockSnapshotOutput::snapshot_id): <p>The ID of the snapshot.</p>
    /// - On failure, responds with [`SdkError<UnlockSnapshotError>`](crate::operation::unlock_snapshot::UnlockSnapshotError)
    pub fn unlock_snapshot(&self) -> crate::operation::unlock_snapshot::builders::UnlockSnapshotFluentBuilder {
        crate::operation::unlock_snapshot::builders::UnlockSnapshotFluentBuilder::new(self.handle.clone())
    }
}
