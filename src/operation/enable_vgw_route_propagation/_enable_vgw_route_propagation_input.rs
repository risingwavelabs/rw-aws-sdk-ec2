// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for EnableVgwRoutePropagation.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableVgwRoutePropagationInput {
    /// <p>The ID of the virtual private gateway that is attached to a VPC. The virtual private gateway must be attached to the same VPC that the routing tables are associated with. </p>
    pub gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the route table. The routing table must be associated with the same VPC that the virtual private gateway is attached to. </p>
    pub route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl EnableVgwRoutePropagationInput {
    /// <p>The ID of the virtual private gateway that is attached to a VPC. The virtual private gateway must be attached to the same VPC that the routing tables are associated with. </p>
    pub fn gateway_id(&self) -> ::std::option::Option<&str> {
        self.gateway_id.as_deref()
    }
    /// <p>The ID of the route table. The routing table must be associated with the same VPC that the virtual private gateway is attached to. </p>
    pub fn route_table_id(&self) -> ::std::option::Option<&str> {
        self.route_table_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl EnableVgwRoutePropagationInput {
    /// Creates a new builder-style object to manufacture [`EnableVgwRoutePropagationInput`](crate::operation::enable_vgw_route_propagation::EnableVgwRoutePropagationInput).
    pub fn builder() -> crate::operation::enable_vgw_route_propagation::builders::EnableVgwRoutePropagationInputBuilder {
        crate::operation::enable_vgw_route_propagation::builders::EnableVgwRoutePropagationInputBuilder::default()
    }
}

/// A builder for [`EnableVgwRoutePropagationInput`](crate::operation::enable_vgw_route_propagation::EnableVgwRoutePropagationInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EnableVgwRoutePropagationInputBuilder {
    pub(crate) gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl EnableVgwRoutePropagationInputBuilder {
    /// <p>The ID of the virtual private gateway that is attached to a VPC. The virtual private gateway must be attached to the same VPC that the routing tables are associated with. </p>
    /// This field is required.
    pub fn gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the virtual private gateway that is attached to a VPC. The virtual private gateway must be attached to the same VPC that the routing tables are associated with. </p>
    pub fn set_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.gateway_id = input;
        self
    }
    /// <p>The ID of the virtual private gateway that is attached to a VPC. The virtual private gateway must be attached to the same VPC that the routing tables are associated with. </p>
    pub fn get_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.gateway_id
    }
    /// <p>The ID of the route table. The routing table must be associated with the same VPC that the virtual private gateway is attached to. </p>
    /// This field is required.
    pub fn route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.route_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the route table. The routing table must be associated with the same VPC that the virtual private gateway is attached to. </p>
    pub fn set_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.route_table_id = input;
        self
    }
    /// <p>The ID of the route table. The routing table must be associated with the same VPC that the virtual private gateway is attached to. </p>
    pub fn get_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.route_table_id
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`EnableVgwRoutePropagationInput`](crate::operation::enable_vgw_route_propagation::EnableVgwRoutePropagationInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_vgw_route_propagation::EnableVgwRoutePropagationInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::enable_vgw_route_propagation::EnableVgwRoutePropagationInput {
            gateway_id: self.gateway_id,
            route_table_id: self.route_table_id,
            dry_run: self.dry_run,
        })
    }
}
