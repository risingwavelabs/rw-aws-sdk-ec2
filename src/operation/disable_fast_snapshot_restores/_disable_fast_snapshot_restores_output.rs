// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableFastSnapshotRestoresOutput {
    /// <p>Information about the snapshots for which fast snapshot restores were successfully disabled.</p>
    pub successful: ::std::option::Option<::std::vec::Vec<crate::types::DisableFastSnapshotRestoreSuccessItem>>,
    /// <p>Information about the snapshots for which fast snapshot restores could not be disabled.</p>
    pub unsuccessful: ::std::option::Option<::std::vec::Vec<crate::types::DisableFastSnapshotRestoreErrorItem>>,
    _request_id: Option<String>,
}
impl DisableFastSnapshotRestoresOutput {
    /// <p>Information about the snapshots for which fast snapshot restores were successfully disabled.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.successful.is_none()`.
    pub fn successful(&self) -> &[crate::types::DisableFastSnapshotRestoreSuccessItem] {
        self.successful.as_deref().unwrap_or_default()
    }
    /// <p>Information about the snapshots for which fast snapshot restores could not be disabled.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.unsuccessful.is_none()`.
    pub fn unsuccessful(&self) -> &[crate::types::DisableFastSnapshotRestoreErrorItem] {
        self.unsuccessful.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DisableFastSnapshotRestoresOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisableFastSnapshotRestoresOutput {
    /// Creates a new builder-style object to manufacture [`DisableFastSnapshotRestoresOutput`](crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresOutput).
    pub fn builder() -> crate::operation::disable_fast_snapshot_restores::builders::DisableFastSnapshotRestoresOutputBuilder {
        crate::operation::disable_fast_snapshot_restores::builders::DisableFastSnapshotRestoresOutputBuilder::default()
    }
}

/// A builder for [`DisableFastSnapshotRestoresOutput`](crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisableFastSnapshotRestoresOutputBuilder {
    pub(crate) successful: ::std::option::Option<::std::vec::Vec<crate::types::DisableFastSnapshotRestoreSuccessItem>>,
    pub(crate) unsuccessful: ::std::option::Option<::std::vec::Vec<crate::types::DisableFastSnapshotRestoreErrorItem>>,
    _request_id: Option<String>,
}
impl DisableFastSnapshotRestoresOutputBuilder {
    /// Appends an item to `successful`.
    ///
    /// To override the contents of this collection use [`set_successful`](Self::set_successful).
    ///
    /// <p>Information about the snapshots for which fast snapshot restores were successfully disabled.</p>
    pub fn successful(mut self, input: crate::types::DisableFastSnapshotRestoreSuccessItem) -> Self {
        let mut v = self.successful.unwrap_or_default();
        v.push(input);
        self.successful = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the snapshots for which fast snapshot restores were successfully disabled.</p>
    pub fn set_successful(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DisableFastSnapshotRestoreSuccessItem>>) -> Self {
        self.successful = input;
        self
    }
    /// <p>Information about the snapshots for which fast snapshot restores were successfully disabled.</p>
    pub fn get_successful(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DisableFastSnapshotRestoreSuccessItem>> {
        &self.successful
    }
    /// Appends an item to `unsuccessful`.
    ///
    /// To override the contents of this collection use [`set_unsuccessful`](Self::set_unsuccessful).
    ///
    /// <p>Information about the snapshots for which fast snapshot restores could not be disabled.</p>
    pub fn unsuccessful(mut self, input: crate::types::DisableFastSnapshotRestoreErrorItem) -> Self {
        let mut v = self.unsuccessful.unwrap_or_default();
        v.push(input);
        self.unsuccessful = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the snapshots for which fast snapshot restores could not be disabled.</p>
    pub fn set_unsuccessful(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DisableFastSnapshotRestoreErrorItem>>) -> Self {
        self.unsuccessful = input;
        self
    }
    /// <p>Information about the snapshots for which fast snapshot restores could not be disabled.</p>
    pub fn get_unsuccessful(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DisableFastSnapshotRestoreErrorItem>> {
        &self.unsuccessful
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisableFastSnapshotRestoresOutput`](crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresOutput).
    pub fn build(self) -> crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresOutput {
        crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresOutput {
            successful: self.successful,
            unsuccessful: self.unsuccessful,
            _request_id: self._request_id,
        }
    }
}
