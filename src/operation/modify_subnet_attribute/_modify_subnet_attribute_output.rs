// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifySubnetAttributeOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for ModifySubnetAttributeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifySubnetAttributeOutput {
    /// Creates a new builder-style object to manufacture [`ModifySubnetAttributeOutput`](crate::operation::modify_subnet_attribute::ModifySubnetAttributeOutput).
    pub fn builder() -> crate::operation::modify_subnet_attribute::builders::ModifySubnetAttributeOutputBuilder {
        crate::operation::modify_subnet_attribute::builders::ModifySubnetAttributeOutputBuilder::default()
    }
}

/// A builder for [`ModifySubnetAttributeOutput`](crate::operation::modify_subnet_attribute::ModifySubnetAttributeOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifySubnetAttributeOutputBuilder {
    _request_id: Option<String>,
}
impl ModifySubnetAttributeOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifySubnetAttributeOutput`](crate::operation::modify_subnet_attribute::ModifySubnetAttributeOutput).
    pub fn build(self) -> crate::operation::modify_subnet_attribute::ModifySubnetAttributeOutput {
        crate::operation::modify_subnet_attribute::ModifySubnetAttributeOutput {
            _request_id: self._request_id,
        }
    }
}