// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an IPv4 CIDR block associated with a VPC.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcCidrBlockAssociation {
    /// <p>The association ID for the IPv4 CIDR block.</p>
    pub association_id: ::std::option::Option<::std::string::String>,
    /// <p>The IPv4 CIDR block.</p>
    pub cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>Information about the state of the CIDR block.</p>
    pub cidr_block_state: ::std::option::Option<crate::types::VpcCidrBlockState>,
}
impl VpcCidrBlockAssociation {
    /// <p>The association ID for the IPv4 CIDR block.</p>
    pub fn association_id(&self) -> ::std::option::Option<&str> {
        self.association_id.as_deref()
    }
    /// <p>The IPv4 CIDR block.</p>
    pub fn cidr_block(&self) -> ::std::option::Option<&str> {
        self.cidr_block.as_deref()
    }
    /// <p>Information about the state of the CIDR block.</p>
    pub fn cidr_block_state(&self) -> ::std::option::Option<&crate::types::VpcCidrBlockState> {
        self.cidr_block_state.as_ref()
    }
}
impl VpcCidrBlockAssociation {
    /// Creates a new builder-style object to manufacture [`VpcCidrBlockAssociation`](crate::types::VpcCidrBlockAssociation).
    pub fn builder() -> crate::types::builders::VpcCidrBlockAssociationBuilder {
        crate::types::builders::VpcCidrBlockAssociationBuilder::default()
    }
}

/// A builder for [`VpcCidrBlockAssociation`](crate::types::VpcCidrBlockAssociation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VpcCidrBlockAssociationBuilder {
    pub(crate) association_id: ::std::option::Option<::std::string::String>,
    pub(crate) cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) cidr_block_state: ::std::option::Option<crate::types::VpcCidrBlockState>,
}
impl VpcCidrBlockAssociationBuilder {
    /// <p>The association ID for the IPv4 CIDR block.</p>
    pub fn association_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.association_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The association ID for the IPv4 CIDR block.</p>
    pub fn set_association_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.association_id = input;
        self
    }
    /// <p>The association ID for the IPv4 CIDR block.</p>
    pub fn get_association_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.association_id
    }
    /// <p>The IPv4 CIDR block.</p>
    pub fn cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 CIDR block.</p>
    pub fn set_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr_block = input;
        self
    }
    /// <p>The IPv4 CIDR block.</p>
    pub fn get_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.cidr_block
    }
    /// <p>Information about the state of the CIDR block.</p>
    pub fn cidr_block_state(mut self, input: crate::types::VpcCidrBlockState) -> Self {
        self.cidr_block_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the state of the CIDR block.</p>
    pub fn set_cidr_block_state(mut self, input: ::std::option::Option<crate::types::VpcCidrBlockState>) -> Self {
        self.cidr_block_state = input;
        self
    }
    /// <p>Information about the state of the CIDR block.</p>
    pub fn get_cidr_block_state(&self) -> &::std::option::Option<crate::types::VpcCidrBlockState> {
        &self.cidr_block_state
    }
    /// Consumes the builder and constructs a [`VpcCidrBlockAssociation`](crate::types::VpcCidrBlockAssociation).
    pub fn build(self) -> crate::types::VpcCidrBlockAssociation {
        crate::types::VpcCidrBlockAssociation {
            association_id: self.association_id,
            cidr_block: self.cidr_block,
            cidr_block_state: self.cidr_block_state,
        }
    }
}
