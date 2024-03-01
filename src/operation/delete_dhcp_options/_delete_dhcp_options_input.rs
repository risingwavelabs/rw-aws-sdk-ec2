// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteDhcpOptionsInput {
    /// <p>The ID of the DHCP options set.</p>
    pub dhcp_options_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DeleteDhcpOptionsInput {
    /// <p>The ID of the DHCP options set.</p>
    pub fn dhcp_options_id(&self) -> ::std::option::Option<&str> {
        self.dhcp_options_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DeleteDhcpOptionsInput {
    /// Creates a new builder-style object to manufacture [`DeleteDhcpOptionsInput`](crate::operation::delete_dhcp_options::DeleteDhcpOptionsInput).
    pub fn builder() -> crate::operation::delete_dhcp_options::builders::DeleteDhcpOptionsInputBuilder {
        crate::operation::delete_dhcp_options::builders::DeleteDhcpOptionsInputBuilder::default()
    }
}

/// A builder for [`DeleteDhcpOptionsInput`](crate::operation::delete_dhcp_options::DeleteDhcpOptionsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteDhcpOptionsInputBuilder {
    pub(crate) dhcp_options_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DeleteDhcpOptionsInputBuilder {
    /// <p>The ID of the DHCP options set.</p>
    /// This field is required.
    pub fn dhcp_options_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.dhcp_options_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the DHCP options set.</p>
    pub fn set_dhcp_options_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.dhcp_options_id = input;
        self
    }
    /// <p>The ID of the DHCP options set.</p>
    pub fn get_dhcp_options_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.dhcp_options_id
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
    /// Consumes the builder and constructs a [`DeleteDhcpOptionsInput`](crate::operation::delete_dhcp_options::DeleteDhcpOptionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_dhcp_options::DeleteDhcpOptionsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_dhcp_options::DeleteDhcpOptionsInput {
            dhcp_options_id: self.dhcp_options_id,
            dry_run: self.dry_run,
        })
    }
}