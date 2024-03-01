// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableImageDeprecationInput {
    /// <p>The ID of the AMI.</p>
    pub image_id: ::std::option::Option<::std::string::String>,
    /// <p>The date and time to deprecate the AMI, in UTC, in the following format: <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z. If you specify a value for seconds, Amazon EC2 rounds the seconds to the nearest minute.</p>
    /// <p>You can’t specify a date in the past. The upper limit for <code>DeprecateAt</code> is 10 years from now, except for public AMIs, where the upper limit is 2 years from the creation date.</p>
    pub deprecate_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl EnableImageDeprecationInput {
    /// <p>The ID of the AMI.</p>
    pub fn image_id(&self) -> ::std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>The date and time to deprecate the AMI, in UTC, in the following format: <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z. If you specify a value for seconds, Amazon EC2 rounds the seconds to the nearest minute.</p>
    /// <p>You can’t specify a date in the past. The upper limit for <code>DeprecateAt</code> is 10 years from now, except for public AMIs, where the upper limit is 2 years from the creation date.</p>
    pub fn deprecate_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.deprecate_at.as_ref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl EnableImageDeprecationInput {
    /// Creates a new builder-style object to manufacture [`EnableImageDeprecationInput`](crate::operation::enable_image_deprecation::EnableImageDeprecationInput).
    pub fn builder() -> crate::operation::enable_image_deprecation::builders::EnableImageDeprecationInputBuilder {
        crate::operation::enable_image_deprecation::builders::EnableImageDeprecationInputBuilder::default()
    }
}

/// A builder for [`EnableImageDeprecationInput`](crate::operation::enable_image_deprecation::EnableImageDeprecationInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EnableImageDeprecationInputBuilder {
    pub(crate) image_id: ::std::option::Option<::std::string::String>,
    pub(crate) deprecate_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl EnableImageDeprecationInputBuilder {
    /// <p>The ID of the AMI.</p>
    /// This field is required.
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.image_id
    }
    /// <p>The date and time to deprecate the AMI, in UTC, in the following format: <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z. If you specify a value for seconds, Amazon EC2 rounds the seconds to the nearest minute.</p>
    /// <p>You can’t specify a date in the past. The upper limit for <code>DeprecateAt</code> is 10 years from now, except for public AMIs, where the upper limit is 2 years from the creation date.</p>
    /// This field is required.
    pub fn deprecate_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.deprecate_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time to deprecate the AMI, in UTC, in the following format: <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z. If you specify a value for seconds, Amazon EC2 rounds the seconds to the nearest minute.</p>
    /// <p>You can’t specify a date in the past. The upper limit for <code>DeprecateAt</code> is 10 years from now, except for public AMIs, where the upper limit is 2 years from the creation date.</p>
    pub fn set_deprecate_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.deprecate_at = input;
        self
    }
    /// <p>The date and time to deprecate the AMI, in UTC, in the following format: <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z. If you specify a value for seconds, Amazon EC2 rounds the seconds to the nearest minute.</p>
    /// <p>You can’t specify a date in the past. The upper limit for <code>DeprecateAt</code> is 10 years from now, except for public AMIs, where the upper limit is 2 years from the creation date.</p>
    pub fn get_deprecate_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.deprecate_at
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
    /// Consumes the builder and constructs a [`EnableImageDeprecationInput`](crate::operation::enable_image_deprecation::EnableImageDeprecationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_image_deprecation::EnableImageDeprecationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::enable_image_deprecation::EnableImageDeprecationInput {
            image_id: self.image_id,
            deprecate_at: self.deprecate_at,
            dry_run: self.dry_run,
        })
    }
}