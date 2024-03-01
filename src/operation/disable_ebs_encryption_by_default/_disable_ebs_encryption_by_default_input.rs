// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableEbsEncryptionByDefaultInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DisableEbsEncryptionByDefaultInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DisableEbsEncryptionByDefaultInput {
    /// Creates a new builder-style object to manufacture [`DisableEbsEncryptionByDefaultInput`](crate::operation::disable_ebs_encryption_by_default::DisableEbsEncryptionByDefaultInput).
    pub fn builder() -> crate::operation::disable_ebs_encryption_by_default::builders::DisableEbsEncryptionByDefaultInputBuilder {
        crate::operation::disable_ebs_encryption_by_default::builders::DisableEbsEncryptionByDefaultInputBuilder::default()
    }
}

/// A builder for [`DisableEbsEncryptionByDefaultInput`](crate::operation::disable_ebs_encryption_by_default::DisableEbsEncryptionByDefaultInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisableEbsEncryptionByDefaultInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DisableEbsEncryptionByDefaultInputBuilder {
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
    /// Consumes the builder and constructs a [`DisableEbsEncryptionByDefaultInput`](crate::operation::disable_ebs_encryption_by_default::DisableEbsEncryptionByDefaultInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disable_ebs_encryption_by_default::DisableEbsEncryptionByDefaultInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::disable_ebs_encryption_by_default::DisableEbsEncryptionByDefaultInput { dry_run: self.dry_run })
    }
}
