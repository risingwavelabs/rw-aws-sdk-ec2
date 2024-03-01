// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeCarrierGatewaysOutput {
    /// <p>Information about the carrier gateway.</p>
    pub carrier_gateways: ::std::option::Option<::std::vec::Vec<crate::types::CarrierGateway>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeCarrierGatewaysOutput {
    /// <p>Information about the carrier gateway.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.carrier_gateways.is_none()`.
    pub fn carrier_gateways(&self) -> &[crate::types::CarrierGateway] {
        self.carrier_gateways.as_deref().unwrap_or_default()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeCarrierGatewaysOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeCarrierGatewaysOutput {
    /// Creates a new builder-style object to manufacture [`DescribeCarrierGatewaysOutput`](crate::operation::describe_carrier_gateways::DescribeCarrierGatewaysOutput).
    pub fn builder() -> crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysOutputBuilder {
        crate::operation::describe_carrier_gateways::builders::DescribeCarrierGatewaysOutputBuilder::default()
    }
}

/// A builder for [`DescribeCarrierGatewaysOutput`](crate::operation::describe_carrier_gateways::DescribeCarrierGatewaysOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeCarrierGatewaysOutputBuilder {
    pub(crate) carrier_gateways: ::std::option::Option<::std::vec::Vec<crate::types::CarrierGateway>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeCarrierGatewaysOutputBuilder {
    /// Appends an item to `carrier_gateways`.
    ///
    /// To override the contents of this collection use [`set_carrier_gateways`](Self::set_carrier_gateways).
    ///
    /// <p>Information about the carrier gateway.</p>
    pub fn carrier_gateways(mut self, input: crate::types::CarrierGateway) -> Self {
        let mut v = self.carrier_gateways.unwrap_or_default();
        v.push(input);
        self.carrier_gateways = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the carrier gateway.</p>
    pub fn set_carrier_gateways(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::CarrierGateway>>) -> Self {
        self.carrier_gateways = input;
        self
    }
    /// <p>Information about the carrier gateway.</p>
    pub fn get_carrier_gateways(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CarrierGateway>> {
        &self.carrier_gateways
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeCarrierGatewaysOutput`](crate::operation::describe_carrier_gateways::DescribeCarrierGatewaysOutput).
    pub fn build(self) -> crate::operation::describe_carrier_gateways::DescribeCarrierGatewaysOutput {
        crate::operation::describe_carrier_gateways::DescribeCarrierGatewaysOutput {
            carrier_gateways: self.carrier_gateways,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
