// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableSerialConsoleAccessInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DisableSerialConsoleAccessInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DisableSerialConsoleAccessInput {
    /// Creates a new builder-style object to manufacture [`DisableSerialConsoleAccessInput`](crate::operation::disable_serial_console_access::DisableSerialConsoleAccessInput).
    pub fn builder() -> crate::operation::disable_serial_console_access::builders::DisableSerialConsoleAccessInputBuilder {
        crate::operation::disable_serial_console_access::builders::DisableSerialConsoleAccessInputBuilder::default()
    }
}

/// A builder for [`DisableSerialConsoleAccessInput`](crate::operation::disable_serial_console_access::DisableSerialConsoleAccessInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisableSerialConsoleAccessInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DisableSerialConsoleAccessInputBuilder {
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
    /// Consumes the builder and constructs a [`DisableSerialConsoleAccessInput`](crate::operation::disable_serial_console_access::DisableSerialConsoleAccessInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disable_serial_console_access::DisableSerialConsoleAccessInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::disable_serial_console_access::DisableSerialConsoleAccessInput { dry_run: self.dry_run })
    }
}
