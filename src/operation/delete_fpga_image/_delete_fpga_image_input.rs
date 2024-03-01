// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteFpgaImageInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the AFI.</p>
    pub fpga_image_id: ::std::option::Option<::std::string::String>,
}
impl DeleteFpgaImageInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the AFI.</p>
    pub fn fpga_image_id(&self) -> ::std::option::Option<&str> {
        self.fpga_image_id.as_deref()
    }
}
impl DeleteFpgaImageInput {
    /// Creates a new builder-style object to manufacture [`DeleteFpgaImageInput`](crate::operation::delete_fpga_image::DeleteFpgaImageInput).
    pub fn builder() -> crate::operation::delete_fpga_image::builders::DeleteFpgaImageInputBuilder {
        crate::operation::delete_fpga_image::builders::DeleteFpgaImageInputBuilder::default()
    }
}

/// A builder for [`DeleteFpgaImageInput`](crate::operation::delete_fpga_image::DeleteFpgaImageInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteFpgaImageInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) fpga_image_id: ::std::option::Option<::std::string::String>,
}
impl DeleteFpgaImageInputBuilder {
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
    /// <p>The ID of the AFI.</p>
    /// This field is required.
    pub fn fpga_image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fpga_image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the AFI.</p>
    pub fn set_fpga_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fpga_image_id = input;
        self
    }
    /// <p>The ID of the AFI.</p>
    pub fn get_fpga_image_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.fpga_image_id
    }
    /// Consumes the builder and constructs a [`DeleteFpgaImageInput`](crate::operation::delete_fpga_image::DeleteFpgaImageInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_fpga_image::DeleteFpgaImageInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_fpga_image::DeleteFpgaImageInput {
            dry_run: self.dry_run,
            fpga_image_id: self.fpga_image_id,
        })
    }
}
