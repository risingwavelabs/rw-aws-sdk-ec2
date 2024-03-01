// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteCarrierGatewayOutput {
    /// <p>Information about the carrier gateway.</p>
    pub carrier_gateway: ::std::option::Option<crate::types::CarrierGateway>,
    _request_id: Option<String>,
}
impl DeleteCarrierGatewayOutput {
    /// <p>Information about the carrier gateway.</p>
    pub fn carrier_gateway(&self) -> ::std::option::Option<&crate::types::CarrierGateway> {
        self.carrier_gateway.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for DeleteCarrierGatewayOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteCarrierGatewayOutput {
    /// Creates a new builder-style object to manufacture [`DeleteCarrierGatewayOutput`](crate::operation::delete_carrier_gateway::DeleteCarrierGatewayOutput).
    pub fn builder() -> crate::operation::delete_carrier_gateway::builders::DeleteCarrierGatewayOutputBuilder {
        crate::operation::delete_carrier_gateway::builders::DeleteCarrierGatewayOutputBuilder::default()
    }
}

/// A builder for [`DeleteCarrierGatewayOutput`](crate::operation::delete_carrier_gateway::DeleteCarrierGatewayOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteCarrierGatewayOutputBuilder {
    pub(crate) carrier_gateway: ::std::option::Option<crate::types::CarrierGateway>,
    _request_id: Option<String>,
}
impl DeleteCarrierGatewayOutputBuilder {
    /// <p>Information about the carrier gateway.</p>
    pub fn carrier_gateway(mut self, input: crate::types::CarrierGateway) -> Self {
        self.carrier_gateway = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the carrier gateway.</p>
    pub fn set_carrier_gateway(mut self, input: ::std::option::Option<crate::types::CarrierGateway>) -> Self {
        self.carrier_gateway = input;
        self
    }
    /// <p>Information about the carrier gateway.</p>
    pub fn get_carrier_gateway(&self) -> &::std::option::Option<crate::types::CarrierGateway> {
        &self.carrier_gateway
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteCarrierGatewayOutput`](crate::operation::delete_carrier_gateway::DeleteCarrierGatewayOutput).
    pub fn build(self) -> crate::operation::delete_carrier_gateway::DeleteCarrierGatewayOutput {
        crate::operation::delete_carrier_gateway::DeleteCarrierGatewayOutput {
            carrier_gateway: self.carrier_gateway,
            _request_id: self._request_id,
        }
    }
}
