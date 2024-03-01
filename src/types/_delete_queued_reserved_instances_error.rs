// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the error for a Reserved Instance whose queued purchase could not be deleted.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteQueuedReservedInstancesError {
    /// <p>The error code.</p>
    pub code: ::std::option::Option<crate::types::DeleteQueuedReservedInstancesErrorCode>,
    /// <p>The error message.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl DeleteQueuedReservedInstancesError {
    /// <p>The error code.</p>
    pub fn code(&self) -> ::std::option::Option<&crate::types::DeleteQueuedReservedInstancesErrorCode> {
        self.code.as_ref()
    }
    /// <p>The error message.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl DeleteQueuedReservedInstancesError {
    /// Creates a new builder-style object to manufacture [`DeleteQueuedReservedInstancesError`](crate::types::DeleteQueuedReservedInstancesError).
    pub fn builder() -> crate::types::builders::DeleteQueuedReservedInstancesErrorBuilder {
        crate::types::builders::DeleteQueuedReservedInstancesErrorBuilder::default()
    }
}

/// A builder for [`DeleteQueuedReservedInstancesError`](crate::types::DeleteQueuedReservedInstancesError).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteQueuedReservedInstancesErrorBuilder {
    pub(crate) code: ::std::option::Option<crate::types::DeleteQueuedReservedInstancesErrorCode>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl DeleteQueuedReservedInstancesErrorBuilder {
    /// <p>The error code.</p>
    pub fn code(mut self, input: crate::types::DeleteQueuedReservedInstancesErrorCode) -> Self {
        self.code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The error code.</p>
    pub fn set_code(mut self, input: ::std::option::Option<crate::types::DeleteQueuedReservedInstancesErrorCode>) -> Self {
        self.code = input;
        self
    }
    /// <p>The error code.</p>
    pub fn get_code(&self) -> &::std::option::Option<crate::types::DeleteQueuedReservedInstancesErrorCode> {
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
    /// Consumes the builder and constructs a [`DeleteQueuedReservedInstancesError`](crate::types::DeleteQueuedReservedInstancesError).
    pub fn build(self) -> crate::types::DeleteQueuedReservedInstancesError {
        crate::types::DeleteQueuedReservedInstancesError {
            code: self.code,
            message: self.message,
        }
    }
}