// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyFpgaImageAttributeOutput {
    /// <p>Information about the attribute.</p>
    pub fpga_image_attribute: ::std::option::Option<crate::types::FpgaImageAttribute>,
    _request_id: Option<String>,
}
impl ModifyFpgaImageAttributeOutput {
    /// <p>Information about the attribute.</p>
    pub fn fpga_image_attribute(&self) -> ::std::option::Option<&crate::types::FpgaImageAttribute> {
        self.fpga_image_attribute.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ModifyFpgaImageAttributeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyFpgaImageAttributeOutput {
    /// Creates a new builder-style object to manufacture [`ModifyFpgaImageAttributeOutput`](crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeOutput).
    pub fn builder() -> crate::operation::modify_fpga_image_attribute::builders::ModifyFpgaImageAttributeOutputBuilder {
        crate::operation::modify_fpga_image_attribute::builders::ModifyFpgaImageAttributeOutputBuilder::default()
    }
}

/// A builder for [`ModifyFpgaImageAttributeOutput`](crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyFpgaImageAttributeOutputBuilder {
    pub(crate) fpga_image_attribute: ::std::option::Option<crate::types::FpgaImageAttribute>,
    _request_id: Option<String>,
}
impl ModifyFpgaImageAttributeOutputBuilder {
    /// <p>Information about the attribute.</p>
    pub fn fpga_image_attribute(mut self, input: crate::types::FpgaImageAttribute) -> Self {
        self.fpga_image_attribute = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the attribute.</p>
    pub fn set_fpga_image_attribute(mut self, input: ::std::option::Option<crate::types::FpgaImageAttribute>) -> Self {
        self.fpga_image_attribute = input;
        self
    }
    /// <p>Information about the attribute.</p>
    pub fn get_fpga_image_attribute(&self) -> &::std::option::Option<crate::types::FpgaImageAttribute> {
        &self.fpga_image_attribute
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifyFpgaImageAttributeOutput`](crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeOutput).
    pub fn build(self) -> crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeOutput {
        crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeOutput {
            fpga_image_attribute: self.fpga_image_attribute,
            _request_id: self._request_id,
        }
    }
}
