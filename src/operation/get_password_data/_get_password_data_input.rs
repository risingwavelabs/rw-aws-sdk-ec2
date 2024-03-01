// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetPasswordDataInput {
    /// <p>The ID of the Windows instance.</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl GetPasswordDataInput {
    /// <p>The ID of the Windows instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl GetPasswordDataInput {
    /// Creates a new builder-style object to manufacture [`GetPasswordDataInput`](crate::operation::get_password_data::GetPasswordDataInput).
    pub fn builder() -> crate::operation::get_password_data::builders::GetPasswordDataInputBuilder {
        crate::operation::get_password_data::builders::GetPasswordDataInputBuilder::default()
    }
}

/// A builder for [`GetPasswordDataInput`](crate::operation::get_password_data::GetPasswordDataInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetPasswordDataInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl GetPasswordDataInputBuilder {
    /// <p>The ID of the Windows instance.</p>
    /// This field is required.
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Windows instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The ID of the Windows instance.</p>
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
    /// Consumes the builder and constructs a [`GetPasswordDataInput`](crate::operation::get_password_data::GetPasswordDataInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::get_password_data::GetPasswordDataInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::get_password_data::GetPasswordDataInput {
            instance_id: self.instance_id,
            dry_run: self.dry_run,
        })
    }
}
