// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a private IPv4 address.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstancePrivateIpAddress {
    /// <p>The association information for an Elastic IP address for the network interface.</p>
    pub association: ::std::option::Option<crate::types::InstanceNetworkInterfaceAssociation>,
    /// <p>Indicates whether this IPv4 address is the primary private IP address of the network interface.</p>
    pub primary: ::std::option::Option<bool>,
    /// <p>The private IPv4 DNS name.</p>
    pub private_dns_name: ::std::option::Option<::std::string::String>,
    /// <p>The private IPv4 address of the network interface.</p>
    pub private_ip_address: ::std::option::Option<::std::string::String>,
}
impl InstancePrivateIpAddress {
    /// <p>The association information for an Elastic IP address for the network interface.</p>
    pub fn association(&self) -> ::std::option::Option<&crate::types::InstanceNetworkInterfaceAssociation> {
        self.association.as_ref()
    }
    /// <p>Indicates whether this IPv4 address is the primary private IP address of the network interface.</p>
    pub fn primary(&self) -> ::std::option::Option<bool> {
        self.primary
    }
    /// <p>The private IPv4 DNS name.</p>
    pub fn private_dns_name(&self) -> ::std::option::Option<&str> {
        self.private_dns_name.as_deref()
    }
    /// <p>The private IPv4 address of the network interface.</p>
    pub fn private_ip_address(&self) -> ::std::option::Option<&str> {
        self.private_ip_address.as_deref()
    }
}
impl InstancePrivateIpAddress {
    /// Creates a new builder-style object to manufacture [`InstancePrivateIpAddress`](crate::types::InstancePrivateIpAddress).
    pub fn builder() -> crate::types::builders::InstancePrivateIpAddressBuilder {
        crate::types::builders::InstancePrivateIpAddressBuilder::default()
    }
}

/// A builder for [`InstancePrivateIpAddress`](crate::types::InstancePrivateIpAddress).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InstancePrivateIpAddressBuilder {
    pub(crate) association: ::std::option::Option<crate::types::InstanceNetworkInterfaceAssociation>,
    pub(crate) primary: ::std::option::Option<bool>,
    pub(crate) private_dns_name: ::std::option::Option<::std::string::String>,
    pub(crate) private_ip_address: ::std::option::Option<::std::string::String>,
}
impl InstancePrivateIpAddressBuilder {
    /// <p>The association information for an Elastic IP address for the network interface.</p>
    pub fn association(mut self, input: crate::types::InstanceNetworkInterfaceAssociation) -> Self {
        self.association = ::std::option::Option::Some(input);
        self
    }
    /// <p>The association information for an Elastic IP address for the network interface.</p>
    pub fn set_association(mut self, input: ::std::option::Option<crate::types::InstanceNetworkInterfaceAssociation>) -> Self {
        self.association = input;
        self
    }
    /// <p>The association information for an Elastic IP address for the network interface.</p>
    pub fn get_association(&self) -> &::std::option::Option<crate::types::InstanceNetworkInterfaceAssociation> {
        &self.association
    }
    /// <p>Indicates whether this IPv4 address is the primary private IP address of the network interface.</p>
    pub fn primary(mut self, input: bool) -> Self {
        self.primary = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether this IPv4 address is the primary private IP address of the network interface.</p>
    pub fn set_primary(mut self, input: ::std::option::Option<bool>) -> Self {
        self.primary = input;
        self
    }
    /// <p>Indicates whether this IPv4 address is the primary private IP address of the network interface.</p>
    pub fn get_primary(&self) -> &::std::option::Option<bool> {
        &self.primary
    }
    /// <p>The private IPv4 DNS name.</p>
    pub fn private_dns_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.private_dns_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The private IPv4 DNS name.</p>
    pub fn set_private_dns_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.private_dns_name = input;
        self
    }
    /// <p>The private IPv4 DNS name.</p>
    pub fn get_private_dns_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.private_dns_name
    }
    /// <p>The private IPv4 address of the network interface.</p>
    pub fn private_ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.private_ip_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The private IPv4 address of the network interface.</p>
    pub fn set_private_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.private_ip_address = input;
        self
    }
    /// <p>The private IPv4 address of the network interface.</p>
    pub fn get_private_ip_address(&self) -> &::std::option::Option<::std::string::String> {
        &self.private_ip_address
    }
    /// Consumes the builder and constructs a [`InstancePrivateIpAddress`](crate::types::InstancePrivateIpAddress).
    pub fn build(self) -> crate::types::InstancePrivateIpAddress {
        crate::types::InstancePrivateIpAddress {
            association: self.association,
            primary: self.primary,
            private_dns_name: self.private_dns_name,
            private_ip_address: self.private_ip_address,
        }
    }
}