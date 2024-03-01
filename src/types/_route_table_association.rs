// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an association between a route table and a subnet or gateway.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RouteTableAssociation {
    /// <p>Indicates whether this is the main route table.</p>
    pub main: ::std::option::Option<bool>,
    /// <p>The ID of the association.</p>
    pub route_table_association_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the route table.</p>
    pub route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the subnet. A subnet ID is not returned for an implicit association.</p>
    pub subnet_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the internet gateway or virtual private gateway.</p>
    pub gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The state of the association.</p>
    pub association_state: ::std::option::Option<crate::types::RouteTableAssociationState>,
}
impl RouteTableAssociation {
    /// <p>Indicates whether this is the main route table.</p>
    pub fn main(&self) -> ::std::option::Option<bool> {
        self.main
    }
    /// <p>The ID of the association.</p>
    pub fn route_table_association_id(&self) -> ::std::option::Option<&str> {
        self.route_table_association_id.as_deref()
    }
    /// <p>The ID of the route table.</p>
    pub fn route_table_id(&self) -> ::std::option::Option<&str> {
        self.route_table_id.as_deref()
    }
    /// <p>The ID of the subnet. A subnet ID is not returned for an implicit association.</p>
    pub fn subnet_id(&self) -> ::std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The ID of the internet gateway or virtual private gateway.</p>
    pub fn gateway_id(&self) -> ::std::option::Option<&str> {
        self.gateway_id.as_deref()
    }
    /// <p>The state of the association.</p>
    pub fn association_state(&self) -> ::std::option::Option<&crate::types::RouteTableAssociationState> {
        self.association_state.as_ref()
    }
}
impl RouteTableAssociation {
    /// Creates a new builder-style object to manufacture [`RouteTableAssociation`](crate::types::RouteTableAssociation).
    pub fn builder() -> crate::types::builders::RouteTableAssociationBuilder {
        crate::types::builders::RouteTableAssociationBuilder::default()
    }
}

/// A builder for [`RouteTableAssociation`](crate::types::RouteTableAssociation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RouteTableAssociationBuilder {
    pub(crate) main: ::std::option::Option<bool>,
    pub(crate) route_table_association_id: ::std::option::Option<::std::string::String>,
    pub(crate) route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) subnet_id: ::std::option::Option<::std::string::String>,
    pub(crate) gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) association_state: ::std::option::Option<crate::types::RouteTableAssociationState>,
}
impl RouteTableAssociationBuilder {
    /// <p>Indicates whether this is the main route table.</p>
    pub fn main(mut self, input: bool) -> Self {
        self.main = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether this is the main route table.</p>
    pub fn set_main(mut self, input: ::std::option::Option<bool>) -> Self {
        self.main = input;
        self
    }
    /// <p>Indicates whether this is the main route table.</p>
    pub fn get_main(&self) -> &::std::option::Option<bool> {
        &self.main
    }
    /// <p>The ID of the association.</p>
    pub fn route_table_association_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.route_table_association_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the association.</p>
    pub fn set_route_table_association_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.route_table_association_id = input;
        self
    }
    /// <p>The ID of the association.</p>
    pub fn get_route_table_association_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.route_table_association_id
    }
    /// <p>The ID of the route table.</p>
    pub fn route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.route_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the route table.</p>
    pub fn set_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.route_table_id = input;
        self
    }
    /// <p>The ID of the route table.</p>
    pub fn get_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.route_table_id
    }
    /// <p>The ID of the subnet. A subnet ID is not returned for an implicit association.</p>
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subnet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the subnet. A subnet ID is not returned for an implicit association.</p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// <p>The ID of the subnet. A subnet ID is not returned for an implicit association.</p>
    pub fn get_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.subnet_id
    }
    /// <p>The ID of the internet gateway or virtual private gateway.</p>
    pub fn gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the internet gateway or virtual private gateway.</p>
    pub fn set_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.gateway_id = input;
        self
    }
    /// <p>The ID of the internet gateway or virtual private gateway.</p>
    pub fn get_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.gateway_id
    }
    /// <p>The state of the association.</p>
    pub fn association_state(mut self, input: crate::types::RouteTableAssociationState) -> Self {
        self.association_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the association.</p>
    pub fn set_association_state(mut self, input: ::std::option::Option<crate::types::RouteTableAssociationState>) -> Self {
        self.association_state = input;
        self
    }
    /// <p>The state of the association.</p>
    pub fn get_association_state(&self) -> &::std::option::Option<crate::types::RouteTableAssociationState> {
        &self.association_state
    }
    /// Consumes the builder and constructs a [`RouteTableAssociation`](crate::types::RouteTableAssociation).
    pub fn build(self) -> crate::types::RouteTableAssociation {
        crate::types::RouteTableAssociation {
            main: self.main,
            route_table_association_id: self.route_table_association_id,
            route_table_id: self.route_table_id,
            subnet_id: self.subnet_id,
            gateway_id: self.gateway_id,
            association_state: self.association_state,
        }
    }
}
