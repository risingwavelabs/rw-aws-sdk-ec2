// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RestoreAddressToClassicInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The Elastic IP address.</p>
    pub public_ip: ::std::option::Option<::std::string::String>,
}
impl RestoreAddressToClassicInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The Elastic IP address.</p>
    pub fn public_ip(&self) -> ::std::option::Option<&str> {
        self.public_ip.as_deref()
    }
}
impl RestoreAddressToClassicInput {
    /// Creates a new builder-style object to manufacture [`RestoreAddressToClassicInput`](crate::operation::restore_address_to_classic::RestoreAddressToClassicInput).
    pub fn builder() -> crate::operation::restore_address_to_classic::builders::RestoreAddressToClassicInputBuilder {
        crate::operation::restore_address_to_classic::builders::RestoreAddressToClassicInputBuilder::default()
    }
}

/// A builder for [`RestoreAddressToClassicInput`](crate::operation::restore_address_to_classic::RestoreAddressToClassicInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RestoreAddressToClassicInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) public_ip: ::std::option::Option<::std::string::String>,
}
impl RestoreAddressToClassicInputBuilder {
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
    /// <p>The Elastic IP address.</p>
    /// This field is required.
    pub fn public_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.public_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Elastic IP address.</p>
    pub fn set_public_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.public_ip = input;
        self
    }
    /// <p>The Elastic IP address.</p>
    pub fn get_public_ip(&self) -> &::std::option::Option<::std::string::String> {
        &self.public_ip
    }
    /// Consumes the builder and constructs a [`RestoreAddressToClassicInput`](crate::operation::restore_address_to_classic::RestoreAddressToClassicInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::restore_address_to_classic::RestoreAddressToClassicInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::restore_address_to_classic::RestoreAddressToClassicInput {
            dry_run: self.dry_run,
            public_ip: self.public_ip,
        })
    }
}