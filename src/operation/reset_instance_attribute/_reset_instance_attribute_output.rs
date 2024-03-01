// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResetInstanceAttributeOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for ResetInstanceAttributeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ResetInstanceAttributeOutput {
    /// Creates a new builder-style object to manufacture [`ResetInstanceAttributeOutput`](crate::operation::reset_instance_attribute::ResetInstanceAttributeOutput).
    pub fn builder() -> crate::operation::reset_instance_attribute::builders::ResetInstanceAttributeOutputBuilder {
        crate::operation::reset_instance_attribute::builders::ResetInstanceAttributeOutputBuilder::default()
    }
}

/// A builder for [`ResetInstanceAttributeOutput`](crate::operation::reset_instance_attribute::ResetInstanceAttributeOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ResetInstanceAttributeOutputBuilder {
    _request_id: Option<String>,
}
impl ResetInstanceAttributeOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ResetInstanceAttributeOutput`](crate::operation::reset_instance_attribute::ResetInstanceAttributeOutput).
    pub fn build(self) -> crate::operation::reset_instance_attribute::ResetInstanceAttributeOutput {
        crate::operation::reset_instance_attribute::ResetInstanceAttributeOutput {
            _request_id: self._request_id,
        }
    }
}
