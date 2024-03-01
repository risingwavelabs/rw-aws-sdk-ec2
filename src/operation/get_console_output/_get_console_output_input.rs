// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetConsoleOutputInput {
    /// <p>The ID of the instance.</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>When enabled, retrieves the latest console output for the instance.</p>
    /// <p>Default: disabled (<code>false</code>)</p>
    pub latest: ::std::option::Option<bool>,
}
impl GetConsoleOutputInput {
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>When enabled, retrieves the latest console output for the instance.</p>
    /// <p>Default: disabled (<code>false</code>)</p>
    pub fn latest(&self) -> ::std::option::Option<bool> {
        self.latest
    }
}
impl GetConsoleOutputInput {
    /// Creates a new builder-style object to manufacture [`GetConsoleOutputInput`](crate::operation::get_console_output::GetConsoleOutputInput).
    pub fn builder() -> crate::operation::get_console_output::builders::GetConsoleOutputInputBuilder {
        crate::operation::get_console_output::builders::GetConsoleOutputInputBuilder::default()
    }
}

/// A builder for [`GetConsoleOutputInput`](crate::operation::get_console_output::GetConsoleOutputInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetConsoleOutputInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) latest: ::std::option::Option<bool>,
}
impl GetConsoleOutputInputBuilder {
    /// <p>The ID of the instance.</p>
    /// This field is required.
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_id
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
    /// <p>When enabled, retrieves the latest console output for the instance.</p>
    /// <p>Default: disabled (<code>false</code>)</p>
    pub fn latest(mut self, input: bool) -> Self {
        self.latest = ::std::option::Option::Some(input);
        self
    }
    /// <p>When enabled, retrieves the latest console output for the instance.</p>
    /// <p>Default: disabled (<code>false</code>)</p>
    pub fn set_latest(mut self, input: ::std::option::Option<bool>) -> Self {
        self.latest = input;
        self
    }
    /// <p>When enabled, retrieves the latest console output for the instance.</p>
    /// <p>Default: disabled (<code>false</code>)</p>
    pub fn get_latest(&self) -> &::std::option::Option<bool> {
        &self.latest
    }
    /// Consumes the builder and constructs a [`GetConsoleOutputInput`](crate::operation::get_console_output::GetConsoleOutputInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_console_output::GetConsoleOutputInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_console_output::GetConsoleOutputInput {
            instance_id: self.instance_id,
            dry_run: self.dry_run,
            latest: self.latest,
        })
    }
}
