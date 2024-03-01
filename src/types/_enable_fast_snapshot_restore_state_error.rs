// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an error that occurred when enabling fast snapshot restores.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableFastSnapshotRestoreStateError {
    /// <p>The error code.</p>
    pub code: ::std::option::Option<::std::string::String>,
    /// <p>The error message.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl EnableFastSnapshotRestoreStateError {
    /// <p>The error code.</p>
    pub fn code(&self) -> ::std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>The error message.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl EnableFastSnapshotRestoreStateError {
    /// Creates a new builder-style object to manufacture [`EnableFastSnapshotRestoreStateError`](crate::types::EnableFastSnapshotRestoreStateError).
    pub fn builder() -> crate::types::builders::EnableFastSnapshotRestoreStateErrorBuilder {
        crate::types::builders::EnableFastSnapshotRestoreStateErrorBuilder::default()
    }
}

/// A builder for [`EnableFastSnapshotRestoreStateError`](crate::types::EnableFastSnapshotRestoreStateError).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EnableFastSnapshotRestoreStateErrorBuilder {
    pub(crate) code: ::std::option::Option<::std::string::String>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl EnableFastSnapshotRestoreStateErrorBuilder {
    /// <p>The error code.</p>
    pub fn code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The error code.</p>
    pub fn set_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>The error code.</p>
    pub fn get_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.code
    }
    /// <p>The error message.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The error message.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The error message.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Consumes the builder and constructs a [`EnableFastSnapshotRestoreStateError`](crate::types::EnableFastSnapshotRestoreStateError).
    pub fn build(self) -> crate::types::EnableFastSnapshotRestoreStateError {
        crate::types::EnableFastSnapshotRestoreStateError {
            code: self.code,
            message: self.message,
        }
    }
}
