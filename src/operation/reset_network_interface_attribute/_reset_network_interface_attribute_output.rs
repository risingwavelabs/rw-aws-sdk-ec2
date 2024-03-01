// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResetNetworkInterfaceAttributeOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for ResetNetworkInterfaceAttributeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ResetNetworkInterfaceAttributeOutput {
    /// Creates a new builder-style object to manufacture [`ResetNetworkInterfaceAttributeOutput`](crate::operation::reset_network_interface_attribute::ResetNetworkInterfaceAttributeOutput).
    pub fn builder() -> crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeOutputBuilder {
        crate::operation::reset_network_interface_attribute::builders::ResetNetworkInterfaceAttributeOutputBuilder::default()
    }
}

/// A builder for [`ResetNetworkInterfaceAttributeOutput`](crate::operation::reset_network_interface_attribute::ResetNetworkInterfaceAttributeOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ResetNetworkInterfaceAttributeOutputBuilder {
    _request_id: Option<String>,
}
impl ResetNetworkInterfaceAttributeOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ResetNetworkInterfaceAttributeOutput`](crate::operation::reset_network_interface_attribute::ResetNetworkInterfaceAttributeOutput).
    pub fn build(self) -> crate::operation::reset_network_interface_attribute::ResetNetworkInterfaceAttributeOutput {
        crate::operation::reset_network_interface_attribute::ResetNetworkInterfaceAttributeOutput {
            _request_id: self._request_id,
        }
    }
}
