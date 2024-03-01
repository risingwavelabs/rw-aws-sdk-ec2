// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTransitGatewayRouteOutput {
    /// <p>Information about the route.</p>
    pub route: ::std::option::Option<crate::types::TransitGatewayRoute>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayRouteOutput {
    /// <p>Information about the route.</p>
    pub fn route(&self) -> ::std::option::Option<&crate::types::TransitGatewayRoute> {
        self.route.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for CreateTransitGatewayRouteOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateTransitGatewayRouteOutput {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayRouteOutput`](crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteOutput).
    pub fn builder() -> crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteOutputBuilder {
        crate::operation::create_transit_gateway_route::builders::CreateTransitGatewayRouteOutputBuilder::default()
    }
}

/// A builder for [`CreateTransitGatewayRouteOutput`](crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateTransitGatewayRouteOutputBuilder {
    pub(crate) route: ::std::option::Option<crate::types::TransitGatewayRoute>,
    _request_id: Option<String>,
}
impl CreateTransitGatewayRouteOutputBuilder {
    /// <p>Information about the route.</p>
    pub fn route(mut self, input: crate::types::TransitGatewayRoute) -> Self {
        self.route = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the route.</p>
    pub fn set_route(mut self, input: ::std::option::Option<crate::types::TransitGatewayRoute>) -> Self {
        self.route = input;
        self
    }
    /// <p>Information about the route.</p>
    pub fn get_route(&self) -> &::std::option::Option<crate::types::TransitGatewayRoute> {
        &self.route
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateTransitGatewayRouteOutput`](crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteOutput).
    pub fn build(self) -> crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteOutput {
        crate::operation::create_transit_gateway_route::CreateTransitGatewayRouteOutput {
            route: self.route,
            _request_id: self._request_id,
        }
    }
}
