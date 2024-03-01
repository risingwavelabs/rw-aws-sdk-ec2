// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateLocalGatewayRouteOutput {
    /// <p>Information about the route.</p>
    pub route: ::std::option::Option<crate::types::LocalGatewayRoute>,
    _request_id: Option<String>,
}
impl CreateLocalGatewayRouteOutput {
    /// <p>Information about the route.</p>
    pub fn route(&self) -> ::std::option::Option<&crate::types::LocalGatewayRoute> {
        self.route.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for CreateLocalGatewayRouteOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateLocalGatewayRouteOutput {
    /// Creates a new builder-style object to manufacture [`CreateLocalGatewayRouteOutput`](crate::operation::create_local_gateway_route::CreateLocalGatewayRouteOutput).
    pub fn builder() -> crate::operation::create_local_gateway_route::builders::CreateLocalGatewayRouteOutputBuilder {
        crate::operation::create_local_gateway_route::builders::CreateLocalGatewayRouteOutputBuilder::default()
    }
}

/// A builder for [`CreateLocalGatewayRouteOutput`](crate::operation::create_local_gateway_route::CreateLocalGatewayRouteOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateLocalGatewayRouteOutputBuilder {
    pub(crate) route: ::std::option::Option<crate::types::LocalGatewayRoute>,
    _request_id: Option<String>,
}
impl CreateLocalGatewayRouteOutputBuilder {
    /// <p>Information about the route.</p>
    pub fn route(mut self, input: crate::types::LocalGatewayRoute) -> Self {
        self.route = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the route.</p>
    pub fn set_route(mut self, input: ::std::option::Option<crate::types::LocalGatewayRoute>) -> Self {
        self.route = input;
        self
    }
    /// <p>Information about the route.</p>
    pub fn get_route(&self) -> &::std::option::Option<crate::types::LocalGatewayRoute> {
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
    /// Consumes the builder and constructs a [`CreateLocalGatewayRouteOutput`](crate::operation::create_local_gateway_route::CreateLocalGatewayRouteOutput).
    pub fn build(self) -> crate::operation::create_local_gateway_route::CreateLocalGatewayRouteOutput {
        crate::operation::create_local_gateway_route::CreateLocalGatewayRouteOutput {
            route: self.route,
            _request_id: self._request_id,
        }
    }
}
