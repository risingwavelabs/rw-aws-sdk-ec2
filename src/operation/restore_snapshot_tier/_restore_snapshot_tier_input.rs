// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RestoreSnapshotTierInput {
    /// <p>The ID of the snapshot to restore.</p>
    pub snapshot_id: ::std::option::Option<::std::string::String>,
    /// <p>Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.</p>
    /// <p>To temporarily restore an archived snapshot, specify the number of days and omit the <b>PermanentRestore</b> parameter or set it to <code>false</code>.</p>
    pub temporary_restore_days: ::std::option::Option<i32>,
    /// <p>Indicates whether to permanently restore an archived snapshot. To permanently restore an archived snapshot, specify <code>true</code> and omit the <b>RestoreSnapshotTierRequest$TemporaryRestoreDays</b> parameter.</p>
    pub permanent_restore: ::std::option::Option<bool>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl RestoreSnapshotTierInput {
    /// <p>The ID of the snapshot to restore.</p>
    pub fn snapshot_id(&self) -> ::std::option::Option<&str> {
        self.snapshot_id.as_deref()
    }
    /// <p>Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.</p>
    /// <p>To temporarily restore an archived snapshot, specify the number of days and omit the <b>PermanentRestore</b> parameter or set it to <code>false</code>.</p>
    pub fn temporary_restore_days(&self) -> ::std::option::Option<i32> {
        self.temporary_restore_days
    }
    /// <p>Indicates whether to permanently restore an archived snapshot. To permanently restore an archived snapshot, specify <code>true</code> and omit the <b>RestoreSnapshotTierRequest$TemporaryRestoreDays</b> parameter.</p>
    pub fn permanent_restore(&self) -> ::std::option::Option<bool> {
        self.permanent_restore
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl RestoreSnapshotTierInput {
    /// Creates a new builder-style object to manufacture [`RestoreSnapshotTierInput`](crate::operation::restore_snapshot_tier::RestoreSnapshotTierInput).
    pub fn builder() -> crate::operation::restore_snapshot_tier::builders::RestoreSnapshotTierInputBuilder {
        crate::operation::restore_snapshot_tier::builders::RestoreSnapshotTierInputBuilder::default()
    }
}

/// A builder for [`RestoreSnapshotTierInput`](crate::operation::restore_snapshot_tier::RestoreSnapshotTierInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RestoreSnapshotTierInputBuilder {
    pub(crate) snapshot_id: ::std::option::Option<::std::string::String>,
    pub(crate) temporary_restore_days: ::std::option::Option<i32>,
    pub(crate) permanent_restore: ::std::option::Option<bool>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl RestoreSnapshotTierInputBuilder {
    /// <p>The ID of the snapshot to restore.</p>
    /// This field is required.
    pub fn snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.snapshot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the snapshot to restore.</p>
    pub fn set_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.snapshot_id = input;
        self
    }
    /// <p>The ID of the snapshot to restore.</p>
    pub fn get_snapshot_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.snapshot_id
    }
    /// <p>Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.</p>
    /// <p>To temporarily restore an archived snapshot, specify the number of days and omit the <b>PermanentRestore</b> parameter or set it to <code>false</code>.</p>
    pub fn temporary_restore_days(mut self, input: i32) -> Self {
        self.temporary_restore_days = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.</p>
    /// <p>To temporarily restore an archived snapshot, specify the number of days and omit the <b>PermanentRestore</b> parameter or set it to <code>false</code>.</p>
    pub fn set_temporary_restore_days(mut self, input: ::std::option::Option<i32>) -> Self {
        self.temporary_restore_days = input;
        self
    }
    /// <p>Specifies the number of days for which to temporarily restore an archived snapshot. Required for temporary restores only. The snapshot will be automatically re-archived after this period.</p>
    /// <p>To temporarily restore an archived snapshot, specify the number of days and omit the <b>PermanentRestore</b> parameter or set it to <code>false</code>.</p>
    pub fn get_temporary_restore_days(&self) -> &::std::option::Option<i32> {
        &self.temporary_restore_days
    }
    /// <p>Indicates whether to permanently restore an archived snapshot. To permanently restore an archived snapshot, specify <code>true</code> and omit the <b>RestoreSnapshotTierRequest$TemporaryRestoreDays</b> parameter.</p>
    pub fn permanent_restore(mut self, input: bool) -> Self {
        self.permanent_restore = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to permanently restore an archived snapshot. To permanently restore an archived snapshot, specify <code>true</code> and omit the <b>RestoreSnapshotTierRequest$TemporaryRestoreDays</b> parameter.</p>
    pub fn set_permanent_restore(mut self, input: ::std::option::Option<bool>) -> Self {
        self.permanent_restore = input;
        self
    }
    /// <p>Indicates whether to permanently restore an archived snapshot. To permanently restore an archived snapshot, specify <code>true</code> and omit the <b>RestoreSnapshotTierRequest$TemporaryRestoreDays</b> parameter.</p>
    pub fn get_permanent_restore(&self) -> &::std::option::Option<bool> {
        &self.permanent_restore
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`RestoreSnapshotTierInput`](crate::operation::restore_snapshot_tier::RestoreSnapshotTierInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::restore_snapshot_tier::RestoreSnapshotTierInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::restore_snapshot_tier::RestoreSnapshotTierInput {
            snapshot_id: self.snapshot_id,
            temporary_restore_days: self.temporary_restore_days,
            permanent_restore: self.permanent_restore,
            dry_run: self.dry_run,
        })
    }
}