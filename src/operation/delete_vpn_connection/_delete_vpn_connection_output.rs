// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteVpnConnectionOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteVpnConnectionOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteVpnConnectionOutput {
    /// Creates a new builder-style object to manufacture [`DeleteVpnConnectionOutput`](crate::operation::delete_vpn_connection::DeleteVpnConnectionOutput).
    pub fn builder() -> crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionOutputBuilder {
        crate::operation::delete_vpn_connection::builders::DeleteVpnConnectionOutputBuilder::default()
    }
}

/// A builder for [`DeleteVpnConnectionOutput`](crate::operation::delete_vpn_connection::DeleteVpnConnectionOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteVpnConnectionOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteVpnConnectionOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVpnConnectionOutput`](crate::operation::delete_vpn_connection::DeleteVpnConnectionOutput).
    pub fn build(self) -> crate::operation::delete_vpn_connection::DeleteVpnConnectionOutput {
        crate::operation::delete_vpn_connection::DeleteVpnConnectionOutput {
            _request_id: self._request_id,
        }
    }
}
