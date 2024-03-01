// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyAddressAttributeOutput {
    /// <p>Information about the Elastic IP address.</p>
    pub address: ::std::option::Option<crate::types::AddressAttribute>,
    _request_id: Option<String>,
}
impl ModifyAddressAttributeOutput {
    /// <p>Information about the Elastic IP address.</p>
    pub fn address(&self) -> ::std::option::Option<&crate::types::AddressAttribute> {
        self.address.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ModifyAddressAttributeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyAddressAttributeOutput {
    /// Creates a new builder-style object to manufacture [`ModifyAddressAttributeOutput`](crate::operation::modify_address_attribute::ModifyAddressAttributeOutput).
    pub fn builder() -> crate::operation::modify_address_attribute::builders::ModifyAddressAttributeOutputBuilder {
        crate::operation::modify_address_attribute::builders::ModifyAddressAttributeOutputBuilder::default()
    }
}

/// A builder for [`ModifyAddressAttributeOutput`](crate::operation::modify_address_attribute::ModifyAddressAttributeOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyAddressAttributeOutputBuilder {
    pub(crate) address: ::std::option::Option<crate::types::AddressAttribute>,
    _request_id: Option<String>,
}
impl ModifyAddressAttributeOutputBuilder {
    /// <p>Information about the Elastic IP address.</p>
    pub fn address(mut self, input: crate::types::AddressAttribute) -> Self {
        self.address = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the Elastic IP address.</p>
    pub fn set_address(mut self, input: ::std::option::Option<crate::types::AddressAttribute>) -> Self {
        self.address = input;
        self
    }
    /// <p>Information about the Elastic IP address.</p>
    pub fn get_address(&self) -> &::std::option::Option<crate::types::AddressAttribute> {
        &self.address
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifyAddressAttributeOutput`](crate::operation::modify_address_attribute::ModifyAddressAttributeOutput).
    pub fn build(self) -> crate::operation::modify_address_attribute::ModifyAddressAttributeOutput {
        crate::operation::modify_address_attribute::ModifyAddressAttributeOutput {
            address: self.address,
            _request_id: self._request_id,
        }
    }
}