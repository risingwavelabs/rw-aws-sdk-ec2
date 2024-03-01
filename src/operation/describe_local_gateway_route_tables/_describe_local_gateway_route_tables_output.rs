// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeLocalGatewayRouteTablesOutput {
    /// <p>Information about the local gateway route tables.</p>
    pub local_gateway_route_tables: ::std::option::Option<::std::vec::Vec<crate::types::LocalGatewayRouteTable>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLocalGatewayRouteTablesOutput {
    /// <p>Information about the local gateway route tables.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.local_gateway_route_tables.is_none()`.
    pub fn local_gateway_route_tables(&self) -> &[crate::types::LocalGatewayRouteTable] {
        self.local_gateway_route_tables.as_deref().unwrap_or_default()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeLocalGatewayRouteTablesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeLocalGatewayRouteTablesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeLocalGatewayRouteTablesOutput`](crate::operation::describe_local_gateway_route_tables::DescribeLocalGatewayRouteTablesOutput).
    pub fn builder() -> crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesOutputBuilder {
        crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesOutputBuilder::default()
    }
}

/// A builder for [`DescribeLocalGatewayRouteTablesOutput`](crate::operation::describe_local_gateway_route_tables::DescribeLocalGatewayRouteTablesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeLocalGatewayRouteTablesOutputBuilder {
    pub(crate) local_gateway_route_tables: ::std::option::Option<::std::vec::Vec<crate::types::LocalGatewayRouteTable>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLocalGatewayRouteTablesOutputBuilder {
    /// Appends an item to `local_gateway_route_tables`.
    ///
    /// To override the contents of this collection use [`set_local_gateway_route_tables`](Self::set_local_gateway_route_tables).
    ///
    /// <p>Information about the local gateway route tables.</p>
    pub fn local_gateway_route_tables(mut self, input: crate::types::LocalGatewayRouteTable) -> Self {
        let mut v = self.local_gateway_route_tables.unwrap_or_default();
        v.push(input);
        self.local_gateway_route_tables = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the local gateway route tables.</p>
    pub fn set_local_gateway_route_tables(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LocalGatewayRouteTable>>) -> Self {
        self.local_gateway_route_tables = input;
        self
    }
    /// <p>Information about the local gateway route tables.</p>
    pub fn get_local_gateway_route_tables(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LocalGatewayRouteTable>> {
        &self.local_gateway_route_tables
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
    /// Consumes the builder and constructs a [`DescribeLocalGatewayRouteTablesOutput`](crate::operation::describe_local_gateway_route_tables::DescribeLocalGatewayRouteTablesOutput).
    pub fn build(self) -> crate::operation::describe_local_gateway_route_tables::DescribeLocalGatewayRouteTablesOutput {
        crate::operation::describe_local_gateway_route_tables::DescribeLocalGatewayRouteTablesOutput {
            local_gateway_route_tables: self.local_gateway_route_tables,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}