// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for AssignPrivateIpAddresses.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssignPrivateIpAddressesInput {
    /// <p>Indicates whether to allow an IP address that is already assigned to another network interface or instance to be reassigned to the specified network interface.</p>
    pub allow_reassignment: ::std::option::Option<bool>,
    /// <p>The ID of the network interface.</p>
    pub network_interface_id: ::std::option::Option<::std::string::String>,
    /// <p>The IP addresses to be assigned as a secondary private IP address to the network interface. You can't specify this parameter when also specifying a number of secondary IP addresses.</p>
    /// <p>If you don't specify an IP address, Amazon EC2 automatically selects an IP address within the subnet range.</p>
    pub private_ip_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The number of secondary IP addresses to assign to the network interface. You can't specify this parameter when also specifying private IP addresses.</p>
    pub secondary_private_ip_address_count: ::std::option::Option<i32>,
    /// <p>One or more IPv4 prefixes assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option.</p>
    pub ipv4_prefixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigns to the network interface. You cannot use this option if you use the <code>Ipv4 Prefixes</code> option.</p>
    pub ipv4_prefix_count: ::std::option::Option<i32>,
}
impl AssignPrivateIpAddressesInput {
    /// <p>Indicates whether to allow an IP address that is already assigned to another network interface or instance to be reassigned to the specified network interface.</p>
    pub fn allow_reassignment(&self) -> ::std::option::Option<bool> {
        self.allow_reassignment
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(&self) -> ::std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The IP addresses to be assigned as a secondary private IP address to the network interface. You can't specify this parameter when also specifying a number of secondary IP addresses.</p>
    /// <p>If you don't specify an IP address, Amazon EC2 automatically selects an IP address within the subnet range.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.private_ip_addresses.is_none()`.
    pub fn private_ip_addresses(&self) -> &[::std::string::String] {
        self.private_ip_addresses.as_deref().unwrap_or_default()
    }
    /// <p>The number of secondary IP addresses to assign to the network interface. You can't specify this parameter when also specifying private IP addresses.</p>
    pub fn secondary_private_ip_address_count(&self) -> ::std::option::Option<i32> {
        self.secondary_private_ip_address_count
    }
    /// <p>One or more IPv4 prefixes assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ipv4_prefixes.is_none()`.
    pub fn ipv4_prefixes(&self) -> &[::std::string::String] {
        self.ipv4_prefixes.as_deref().unwrap_or_default()
    }
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigns to the network interface. You cannot use this option if you use the <code>Ipv4 Prefixes</code> option.</p>
    pub fn ipv4_prefix_count(&self) -> ::std::option::Option<i32> {
        self.ipv4_prefix_count
    }
}
impl AssignPrivateIpAddressesInput {
    /// Creates a new builder-style object to manufacture [`AssignPrivateIpAddressesInput`](crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesInput).
    pub fn builder() -> crate::operation::assign_private_ip_addresses::builders::AssignPrivateIpAddressesInputBuilder {
        crate::operation::assign_private_ip_addresses::builders::AssignPrivateIpAddressesInputBuilder::default()
    }
}

/// A builder for [`AssignPrivateIpAddressesInput`](crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AssignPrivateIpAddressesInputBuilder {
    pub(crate) allow_reassignment: ::std::option::Option<bool>,
    pub(crate) network_interface_id: ::std::option::Option<::std::string::String>,
    pub(crate) private_ip_addresses: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) secondary_private_ip_address_count: ::std::option::Option<i32>,
    pub(crate) ipv4_prefixes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) ipv4_prefix_count: ::std::option::Option<i32>,
}
impl AssignPrivateIpAddressesInputBuilder {
    /// <p>Indicates whether to allow an IP address that is already assigned to another network interface or instance to be reassigned to the specified network interface.</p>
    pub fn allow_reassignment(mut self, input: bool) -> Self {
        self.allow_reassignment = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to allow an IP address that is already assigned to another network interface or instance to be reassigned to the specified network interface.</p>
    pub fn set_allow_reassignment(mut self, input: ::std::option::Option<bool>) -> Self {
        self.allow_reassignment = input;
        self
    }
    /// <p>Indicates whether to allow an IP address that is already assigned to another network interface or instance to be reassigned to the specified network interface.</p>
    pub fn get_allow_reassignment(&self) -> &::std::option::Option<bool> {
        &self.allow_reassignment
    }
    /// <p>The ID of the network interface.</p>
    /// This field is required.
    pub fn network_interface_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_interface_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn get_network_interface_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.network_interface_id
    }
    /// Appends an item to `private_ip_addresses`.
    ///
    /// To override the contents of this collection use [`set_private_ip_addresses`](Self::set_private_ip_addresses).
    ///
    /// <p>The IP addresses to be assigned as a secondary private IP address to the network interface. You can't specify this parameter when also specifying a number of secondary IP addresses.</p>
    /// <p>If you don't specify an IP address, Amazon EC2 automatically selects an IP address within the subnet range.</p>
    pub fn private_ip_addresses(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.private_ip_addresses.unwrap_or_default();
        v.push(input.into());
        self.private_ip_addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IP addresses to be assigned as a secondary private IP address to the network interface. You can't specify this parameter when also specifying a number of secondary IP addresses.</p>
    /// <p>If you don't specify an IP address, Amazon EC2 automatically selects an IP address within the subnet range.</p>
    pub fn set_private_ip_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.private_ip_addresses = input;
        self
    }
    /// <p>The IP addresses to be assigned as a secondary private IP address to the network interface. You can't specify this parameter when also specifying a number of secondary IP addresses.</p>
    /// <p>If you don't specify an IP address, Amazon EC2 automatically selects an IP address within the subnet range.</p>
    pub fn get_private_ip_addresses(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.private_ip_addresses
    }
    /// <p>The number of secondary IP addresses to assign to the network interface. You can't specify this parameter when also specifying private IP addresses.</p>
    pub fn secondary_private_ip_address_count(mut self, input: i32) -> Self {
        self.secondary_private_ip_address_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of secondary IP addresses to assign to the network interface. You can't specify this parameter when also specifying private IP addresses.</p>
    pub fn set_secondary_private_ip_address_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.secondary_private_ip_address_count = input;
        self
    }
    /// <p>The number of secondary IP addresses to assign to the network interface. You can't specify this parameter when also specifying private IP addresses.</p>
    pub fn get_secondary_private_ip_address_count(&self) -> &::std::option::Option<i32> {
        &self.secondary_private_ip_address_count
    }
    /// Appends an item to `ipv4_prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv4_prefixes`](Self::set_ipv4_prefixes).
    ///
    /// <p>One or more IPv4 prefixes assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option.</p>
    pub fn ipv4_prefixes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.ipv4_prefixes.unwrap_or_default();
        v.push(input.into());
        self.ipv4_prefixes = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more IPv4 prefixes assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option.</p>
    pub fn set_ipv4_prefixes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.ipv4_prefixes = input;
        self
    }
    /// <p>One or more IPv4 prefixes assigned to the network interface. You cannot use this option if you use the <code>Ipv4PrefixCount</code> option.</p>
    pub fn get_ipv4_prefixes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.ipv4_prefixes
    }
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigns to the network interface. You cannot use this option if you use the <code>Ipv4 Prefixes</code> option.</p>
    pub fn ipv4_prefix_count(mut self, input: i32) -> Self {
        self.ipv4_prefix_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigns to the network interface. You cannot use this option if you use the <code>Ipv4 Prefixes</code> option.</p>
    pub fn set_ipv4_prefix_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.ipv4_prefix_count = input;
        self
    }
    /// <p>The number of IPv4 prefixes that Amazon Web Services automatically assigns to the network interface. You cannot use this option if you use the <code>Ipv4 Prefixes</code> option.</p>
    pub fn get_ipv4_prefix_count(&self) -> &::std::option::Option<i32> {
        &self.ipv4_prefix_count
    }
    /// Consumes the builder and constructs a [`AssignPrivateIpAddressesInput`](crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::assign_private_ip_addresses::AssignPrivateIpAddressesInput {
            allow_reassignment: self.allow_reassignment,
            network_interface_id: self.network_interface_id,
            private_ip_addresses: self.private_ip_addresses,
            secondary_private_ip_address_count: self.secondary_private_ip_address_count,
            ipv4_prefixes: self.ipv4_prefixes,
            ipv4_prefix_count: self.ipv4_prefix_count,
        })
    }
}
