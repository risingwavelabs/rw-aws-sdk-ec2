// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelConversionTaskInput {
    /// <p>The ID of the conversion task.</p>
    pub conversion_task_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The reason for canceling the conversion task.</p>
    pub reason_message: ::std::option::Option<::std::string::String>,
}
impl CancelConversionTaskInput {
    /// <p>The ID of the conversion task.</p>
    pub fn conversion_task_id(&self) -> ::std::option::Option<&str> {
        self.conversion_task_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The reason for canceling the conversion task.</p>
    pub fn reason_message(&self) -> ::std::option::Option<&str> {
        self.reason_message.as_deref()
    }
}
impl CancelConversionTaskInput {
    /// Creates a new builder-style object to manufacture [`CancelConversionTaskInput`](crate::operation::cancel_conversion_task::CancelConversionTaskInput).
    pub fn builder() -> crate::operation::cancel_conversion_task::builders::CancelConversionTaskInputBuilder {
        crate::operation::cancel_conversion_task::builders::CancelConversionTaskInputBuilder::default()
    }
}

/// A builder for [`CancelConversionTaskInput`](crate::operation::cancel_conversion_task::CancelConversionTaskInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CancelConversionTaskInputBuilder {
    pub(crate) conversion_task_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) reason_message: ::std::option::Option<::std::string::String>,
}
impl CancelConversionTaskInputBuilder {
    /// <p>The ID of the conversion task.</p>
    /// This field is required.
    pub fn conversion_task_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.conversion_task_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the conversion task.</p>
    pub fn set_conversion_task_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.conversion_task_id = input;
        self
    }
    /// <p>The ID of the conversion task.</p>
    pub fn get_conversion_task_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.conversion_task_id
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
    /// <p>The reason for canceling the conversion task.</p>
    pub fn reason_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reason_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The reason for canceling the conversion task.</p>
    pub fn set_reason_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reason_message = input;
        self
    }
    /// <p>The reason for canceling the conversion task.</p>
    pub fn get_reason_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.reason_message
    }
    /// Consumes the builder and constructs a [`CancelConversionTaskInput`](crate::operation::cancel_conversion_task::CancelConversionTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::cancel_conversion_task::CancelConversionTaskInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::cancel_conversion_task::CancelConversionTaskInput {
            conversion_task_id: self.conversion_task_id,
            dry_run: self.dry_run,
            reason_message: self.reason_message,
        })
    }
}
