// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an IPv6 CIDR block associated with a VPC.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcIpv6CidrBlockAssociation {
    /// <p>The association ID for the IPv6 CIDR block.</p>
    pub association_id: ::std::option::Option<::std::string::String>,
    /// <p>The IPv6 CIDR block.</p>
    pub ipv6_cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>Information about the state of the CIDR block.</p>
    pub ipv6_cidr_block_state: ::std::option::Option<crate::types::VpcCidrBlockState>,
    /// <p>The name of the unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses, for example, <code>us-east-1-wl1-bos-wlz-1</code>.</p>
    pub network_border_group: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the IPv6 address pool from which the IPv6 CIDR block is allocated.</p>
    pub ipv6_pool: ::std::option::Option<::std::string::String>,
}
impl VpcIpv6CidrBlockAssociation {
    /// <p>The association ID for the IPv6 CIDR block.</p>
    pub fn association_id(&self) -> ::std::option::Option<&str> {
        self.association_id.as_deref()
    }
    /// <p>The IPv6 CIDR block.</p>
    pub fn ipv6_cidr_block(&self) -> ::std::option::Option<&str> {
        self.ipv6_cidr_block.as_deref()
    }
    /// <p>Information about the state of the CIDR block.</p>
    pub fn ipv6_cidr_block_state(&self) -> ::std::option::Option<&crate::types::VpcCidrBlockState> {
        self.ipv6_cidr_block_state.as_ref()
    }
    /// <p>The name of the unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses, for example, <code>us-east-1-wl1-bos-wlz-1</code>.</p>
    pub fn network_border_group(&self) -> ::std::option::Option<&str> {
        self.network_border_group.as_deref()
    }
    /// <p>The ID of the IPv6 address pool from which the IPv6 CIDR block is allocated.</p>
    pub fn ipv6_pool(&self) -> ::std::option::Option<&str> {
        self.ipv6_pool.as_deref()
    }
}
impl VpcIpv6CidrBlockAssociation {
    /// Creates a new builder-style object to manufacture [`VpcIpv6CidrBlockAssociation`](crate::types::VpcIpv6CidrBlockAssociation).
    pub fn builder() -> crate::types::builders::VpcIpv6CidrBlockAssociationBuilder {
        crate::types::builders::VpcIpv6CidrBlockAssociationBuilder::default()
    }
}

/// A builder for [`VpcIpv6CidrBlockAssociation`](crate::types::VpcIpv6CidrBlockAssociation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VpcIpv6CidrBlockAssociationBuilder {
    pub(crate) association_id: ::std::option::Option<::std::string::String>,
    pub(crate) ipv6_cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) ipv6_cidr_block_state: ::std::option::Option<crate::types::VpcCidrBlockState>,
    pub(crate) network_border_group: ::std::option::Option<::std::string::String>,
    pub(crate) ipv6_pool: ::std::option::Option<::std::string::String>,
}
impl VpcIpv6CidrBlockAssociationBuilder {
    /// <p>The association ID for the IPv6 CIDR block.</p>
    pub fn association_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.association_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The association ID for the IPv6 CIDR block.</p>
    pub fn set_association_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.association_id = input;
        self
    }
    /// <p>The association ID for the IPv6 CIDR block.</p>
    pub fn get_association_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.association_id
    }
    /// <p>The IPv6 CIDR block.</p>
    pub fn ipv6_cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv6_cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv6 CIDR block.</p>
    pub fn set_ipv6_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv6_cidr_block = input;
        self
    }
    /// <p>The IPv6 CIDR block.</p>
    pub fn get_ipv6_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipv6_cidr_block
    }
    /// <p>Information about the state of the CIDR block.</p>
    pub fn ipv6_cidr_block_state(mut self, input: crate::types::VpcCidrBlockState) -> Self {
        self.ipv6_cidr_block_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the state of the CIDR block.</p>
    pub fn set_ipv6_cidr_block_state(mut self, input: ::std::option::Option<crate::types::VpcCidrBlockState>) -> Self {
        self.ipv6_cidr_block_state = input;
        self
    }
    /// <p>Information about the state of the CIDR block.</p>
    pub fn get_ipv6_cidr_block_state(&self) -> &::std::option::Option<crate::types::VpcCidrBlockState> {
        &self.ipv6_cidr_block_state
    }
    /// <p>The name of the unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses, for example, <code>us-east-1-wl1-bos-wlz-1</code>.</p>
    pub fn network_border_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_border_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses, for example, <code>us-east-1-wl1-bos-wlz-1</code>.</p>
    pub fn set_network_border_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_border_group = input;
        self
    }
    /// <p>The name of the unique set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses, for example, <code>us-east-1-wl1-bos-wlz-1</code>.</p>
    pub fn get_network_border_group(&self) -> &::std::option::Option<::std::string::String> {
        &self.network_border_group
    }
    /// <p>The ID of the IPv6 address pool from which the IPv6 CIDR block is allocated.</p>
    pub fn ipv6_pool(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv6_pool = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the IPv6 address pool from which the IPv6 CIDR block is allocated.</p>
    pub fn set_ipv6_pool(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv6_pool = input;
        self
    }
    /// <p>The ID of the IPv6 address pool from which the IPv6 CIDR block is allocated.</p>
    pub fn get_ipv6_pool(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipv6_pool
    }
    /// Consumes the builder and constructs a [`VpcIpv6CidrBlockAssociation`](crate::types::VpcIpv6CidrBlockAssociation).
    pub fn build(self) -> crate::types::VpcIpv6CidrBlockAssociation {
        crate::types::VpcIpv6CidrBlockAssociation {
            association_id: self.association_id,
            ipv6_cidr_block: self.ipv6_cidr_block,
            ipv6_cidr_block_state: self.ipv6_cidr_block_state,
            network_border_group: self.network_border_group,
            ipv6_pool: self.ipv6_pool,
        }
    }
}
