// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the subnet association with the transit gateway multicast domain.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubnetAssociation {
    /// <p>The ID of the subnet.</p>
    pub subnet_id: ::std::option::Option<::std::string::String>,
    /// <p>The state of the subnet association.</p>
    pub state: ::std::option::Option<crate::types::TransitGatewayMulitcastDomainAssociationState>,
}
impl SubnetAssociation {
    /// <p>The ID of the subnet.</p>
    pub fn subnet_id(&self) -> ::std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The state of the subnet association.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::TransitGatewayMulitcastDomainAssociationState> {
        self.state.as_ref()
    }
}
impl SubnetAssociation {
    /// Creates a new builder-style object to manufacture [`SubnetAssociation`](crate::types::SubnetAssociation).
    pub fn builder() -> crate::types::builders::SubnetAssociationBuilder {
        crate::types::builders::SubnetAssociationBuilder::default()
    }
}

/// A builder for [`SubnetAssociation`](crate::types::SubnetAssociation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SubnetAssociationBuilder {
    pub(crate) subnet_id: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::TransitGatewayMulitcastDomainAssociationState>,
}
impl SubnetAssociationBuilder {
    /// <p>The ID of the subnet.</p>
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subnet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the subnet.</p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// <p>The ID of the subnet.</p>
    pub fn get_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.subnet_id
    }
    /// <p>The state of the subnet association.</p>
    pub fn state(mut self, input: crate::types::TransitGatewayMulitcastDomainAssociationState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the subnet association.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::TransitGatewayMulitcastDomainAssociationState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the subnet association.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::TransitGatewayMulitcastDomainAssociationState> {
        &self.state
    }
    /// Consumes the builder and constructs a [`SubnetAssociation`](crate::types::SubnetAssociation).
    pub fn build(self) -> crate::types::SubnetAssociation {
        crate::types::SubnetAssociation {
            subnet_id: self.subnet_id,
            state: self.state,
        }
    }
}