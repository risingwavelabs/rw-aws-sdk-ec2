// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the VPC peering connection options.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcPeeringConnectionOptionsDescription {
    /// <p>Indicates whether a local VPC can resolve public DNS hostnames to private IP addresses when queried from instances in a peer VPC.</p>
    pub allow_dns_resolution_from_remote_vpc: ::std::option::Option<bool>,
    /// <p>Deprecated.</p>
    pub allow_egress_from_local_classic_link_to_remote_vpc: ::std::option::Option<bool>,
    /// <p>Deprecated.</p>
    pub allow_egress_from_local_vpc_to_remote_classic_link: ::std::option::Option<bool>,
}
impl VpcPeeringConnectionOptionsDescription {
    /// <p>Indicates whether a local VPC can resolve public DNS hostnames to private IP addresses when queried from instances in a peer VPC.</p>
    pub fn allow_dns_resolution_from_remote_vpc(&self) -> ::std::option::Option<bool> {
        self.allow_dns_resolution_from_remote_vpc
    }
    /// <p>Deprecated.</p>
    pub fn allow_egress_from_local_classic_link_to_remote_vpc(&self) -> ::std::option::Option<bool> {
        self.allow_egress_from_local_classic_link_to_remote_vpc
    }
    /// <p>Deprecated.</p>
    pub fn allow_egress_from_local_vpc_to_remote_classic_link(&self) -> ::std::option::Option<bool> {
        self.allow_egress_from_local_vpc_to_remote_classic_link
    }
}
impl VpcPeeringConnectionOptionsDescription {
    /// Creates a new builder-style object to manufacture [`VpcPeeringConnectionOptionsDescription`](crate::types::VpcPeeringConnectionOptionsDescription).
    pub fn builder() -> crate::types::builders::VpcPeeringConnectionOptionsDescriptionBuilder {
        crate::types::builders::VpcPeeringConnectionOptionsDescriptionBuilder::default()
    }
}

/// A builder for [`VpcPeeringConnectionOptionsDescription`](crate::types::VpcPeeringConnectionOptionsDescription).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VpcPeeringConnectionOptionsDescriptionBuilder {
    pub(crate) allow_dns_resolution_from_remote_vpc: ::std::option::Option<bool>,
    pub(crate) allow_egress_from_local_classic_link_to_remote_vpc: ::std::option::Option<bool>,
    pub(crate) allow_egress_from_local_vpc_to_remote_classic_link: ::std::option::Option<bool>,
}
impl VpcPeeringConnectionOptionsDescriptionBuilder {
    /// <p>Indicates whether a local VPC can resolve public DNS hostnames to private IP addresses when queried from instances in a peer VPC.</p>
    pub fn allow_dns_resolution_from_remote_vpc(mut self, input: bool) -> Self {
        self.allow_dns_resolution_from_remote_vpc = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether a local VPC can resolve public DNS hostnames to private IP addresses when queried from instances in a peer VPC.</p>
    pub fn set_allow_dns_resolution_from_remote_vpc(mut self, input: ::std::option::Option<bool>) -> Self {
        self.allow_dns_resolution_from_remote_vpc = input;
        self
    }
    /// <p>Indicates whether a local VPC can resolve public DNS hostnames to private IP addresses when queried from instances in a peer VPC.</p>
    pub fn get_allow_dns_resolution_from_remote_vpc(&self) -> &::std::option::Option<bool> {
        &self.allow_dns_resolution_from_remote_vpc
    }
    /// <p>Deprecated.</p>
    pub fn allow_egress_from_local_classic_link_to_remote_vpc(mut self, input: bool) -> Self {
        self.allow_egress_from_local_classic_link_to_remote_vpc = ::std::option::Option::Some(input);
        self
    }
    /// <p>Deprecated.</p>
    pub fn set_allow_egress_from_local_classic_link_to_remote_vpc(mut self, input: ::std::option::Option<bool>) -> Self {
        self.allow_egress_from_local_classic_link_to_remote_vpc = input;
        self
    }
    /// <p>Deprecated.</p>
    pub fn get_allow_egress_from_local_classic_link_to_remote_vpc(&self) -> &::std::option::Option<bool> {
        &self.allow_egress_from_local_classic_link_to_remote_vpc
    }
    /// <p>Deprecated.</p>
    pub fn allow_egress_from_local_vpc_to_remote_classic_link(mut self, input: bool) -> Self {
        self.allow_egress_from_local_vpc_to_remote_classic_link = ::std::option::Option::Some(input);
        self
    }
    /// <p>Deprecated.</p>
    pub fn set_allow_egress_from_local_vpc_to_remote_classic_link(mut self, input: ::std::option::Option<bool>) -> Self {
        self.allow_egress_from_local_vpc_to_remote_classic_link = input;
        self
    }
    /// <p>Deprecated.</p>
    pub fn get_allow_egress_from_local_vpc_to_remote_classic_link(&self) -> &::std::option::Option<bool> {
        &self.allow_egress_from_local_vpc_to_remote_classic_link
    }
    /// Consumes the builder and constructs a [`VpcPeeringConnectionOptionsDescription`](crate::types::VpcPeeringConnectionOptionsDescription).
    pub fn build(self) -> crate::types::VpcPeeringConnectionOptionsDescription {
        crate::types::VpcPeeringConnectionOptionsDescription {
            allow_dns_resolution_from_remote_vpc: self.allow_dns_resolution_from_remote_vpc,
            allow_egress_from_local_classic_link_to_remote_vpc: self.allow_egress_from_local_classic_link_to_remote_vpc,
            allow_egress_from_local_vpc_to_remote_classic_link: self.allow_egress_from_local_vpc_to_remote_classic_link,
        }
    }
}
