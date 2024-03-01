// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateAddressOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DisassociateAddressOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisassociateAddressOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateAddressOutput`](crate::operation::disassociate_address::DisassociateAddressOutput).
    pub fn builder() -> crate::operation::disassociate_address::builders::DisassociateAddressOutputBuilder {
        crate::operation::disassociate_address::builders::DisassociateAddressOutputBuilder::default()
    }
}

/// A builder for [`DisassociateAddressOutput`](crate::operation::disassociate_address::DisassociateAddressOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisassociateAddressOutputBuilder {
    _request_id: Option<String>,
}
impl DisassociateAddressOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateAddressOutput`](crate::operation::disassociate_address::DisassociateAddressOutput).
    pub fn build(self) -> crate::operation::disassociate_address::DisassociateAddressOutput {
        crate::operation::disassociate_address::DisassociateAddressOutput {
            _request_id: self._request_id,
        }
    }
}
