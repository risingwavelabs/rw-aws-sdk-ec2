// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a network interface.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceNetworkInterface {
    /// <p>The association information for an Elastic IPv4 associated with the network interface.</p>
    pub association: ::std::option::Option<crate::types::InstanceNetworkInterfaceAssociation>,
    /// <p>The network interface attachment.</p>
    pub attachment: ::std::option::Option<crate::types::InstanceNetworkInterfaceAttachment>,
    /// <p>The description.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The security groups.</p>
    pub groups: ::std::option::Option<::std::vec::Vec<crate::types::GroupIdentifier>>,
    /// <p>The IPv6 addresses associated with the network interface.</p>
    pub ipv6_addresses: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv6Address>>,
    /// <p>The MAC address.</p>
    pub mac_address: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the network interface.</p>
    pub network_interface_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Web Services account that created the network interface.</p>
    pub owner_id: ::std::option::Option<::std::string::String>,
    /// <p>The private DNS name.</p>
    pub private_dns_name: ::std::option::Option<::std::string::String>,
    /// <p>The IPv4 address of the network interface within the subnet.</p>
    pub private_ip_address: ::std::option::Option<::std::string::String>,
    /// <p>The private IPv4 addresses associated with the network interface.</p>
    pub private_ip_addresses: ::std::option::Option<::std::vec::Vec<crate::types::InstancePrivateIpAddress>>,
    /// <p>Indicates whether source/destination checking is enabled.</p>
    pub source_dest_check: ::std::option::Option<bool>,
    /// <p>The status of the network interface.</p>
    pub status: ::std::option::Option<crate::types::NetworkInterfaceStatus>,
    /// <p>The ID of the subnet.</p>
    pub subnet_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the VPC.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of network interface.</p>
    /// <p>Valid values: <code>interface</code> | <code>efa</code> | <code>trunk</code> </p>
    pub interface_type: ::std::option::Option<::std::string::String>,
    /// <p>The IPv4 delegated prefixes that are assigned to the network interface.</p>
    pub ipv4_prefixes: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv4Prefix>>,
    /// <p>The IPv6 delegated prefixes that are assigned to the network interface.</p>
    pub ipv6_prefixes: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv6Prefix>>,
    /// <p>A security group connection tracking configuration that enables you to set the timeout for connection tracking on an Elastic network interface. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#connection-tracking-timeouts">Connection tracking timeouts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub connection_tracking_configuration: ::std::option::Option<crate::types::ConnectionTrackingSpecificationResponse>,
}
impl InstanceNetworkInterface {
    /// <p>The association information for an Elastic IPv4 associated with the network interface.</p>
    pub fn association(&self) -> ::std::option::Option<&crate::types::InstanceNetworkInterfaceAssociation> {
        self.association.as_ref()
    }
    /// <p>The network interface attachment.</p>
    pub fn attachment(&self) -> ::std::option::Option<&crate::types::InstanceNetworkInterfaceAttachment> {
        self.attachment.as_ref()
    }
    /// <p>The description.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The security groups.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.groups.is_none()`.
    pub fn groups(&self) -> &[crate::types::GroupIdentifier] {
        self.groups.as_deref().unwrap_or_default()
    }
    /// <p>The IPv6 addresses associated with the network interface.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ipv6_addresses.is_none()`.
    pub fn ipv6_addresses(&self) -> &[crate::types::InstanceIpv6Address] {
        self.ipv6_addresses.as_deref().unwrap_or_default()
    }
    /// <p>The MAC address.</p>
    pub fn mac_address(&self) -> ::std::option::Option<&str> {
        self.mac_address.as_deref()
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(&self) -> ::std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that created the network interface.</p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The private DNS name.</p>
    pub fn private_dns_name(&self) -> ::std::option::Option<&str> {
        self.private_dns_name.as_deref()
    }
    /// <p>The IPv4 address of the network interface within the subnet.</p>
    pub fn private_ip_address(&self) -> ::std::option::Option<&str> {
        self.private_ip_address.as_deref()
    }
    /// <p>The private IPv4 addresses associated with the network interface.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.private_ip_addresses.is_none()`.
    pub fn private_ip_addresses(&self) -> &[crate::types::InstancePrivateIpAddress] {
        self.private_ip_addresses.as_deref().unwrap_or_default()
    }
    /// <p>Indicates whether source/destination checking is enabled.</p>
    pub fn source_dest_check(&self) -> ::std::option::Option<bool> {
        self.source_dest_check
    }
    /// <p>The status of the network interface.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::NetworkInterfaceStatus> {
        self.status.as_ref()
    }
    /// <p>The ID of the subnet.</p>
    pub fn subnet_id(&self) -> ::std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The type of network interface.</p>
    /// <p>Valid values: <code>interface</code> | <code>efa</code> | <code>trunk</code> </p>
    pub fn interface_type(&self) -> ::std::option::Option<&str> {
        self.interface_type.as_deref()
    }
    /// <p>The IPv4 delegated prefixes that are assigned to the network interface.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ipv4_prefixes.is_none()`.
    pub fn ipv4_prefixes(&self) -> &[crate::types::InstanceIpv4Prefix] {
        self.ipv4_prefixes.as_deref().unwrap_or_default()
    }
    /// <p>The IPv6 delegated prefixes that are assigned to the network interface.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ipv6_prefixes.is_none()`.
    pub fn ipv6_prefixes(&self) -> &[crate::types::InstanceIpv6Prefix] {
        self.ipv6_prefixes.as_deref().unwrap_or_default()
    }
    /// <p>A security group connection tracking configuration that enables you to set the timeout for connection tracking on an Elastic network interface. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#connection-tracking-timeouts">Connection tracking timeouts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn connection_tracking_configuration(&self) -> ::std::option::Option<&crate::types::ConnectionTrackingSpecificationResponse> {
        self.connection_tracking_configuration.as_ref()
    }
}
impl InstanceNetworkInterface {
    /// Creates a new builder-style object to manufacture [`InstanceNetworkInterface`](crate::types::InstanceNetworkInterface).
    pub fn builder() -> crate::types::builders::InstanceNetworkInterfaceBuilder {
        crate::types::builders::InstanceNetworkInterfaceBuilder::default()
    }
}

/// A builder for [`InstanceNetworkInterface`](crate::types::InstanceNetworkInterface).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InstanceNetworkInterfaceBuilder {
    pub(crate) association: ::std::option::Option<crate::types::InstanceNetworkInterfaceAssociation>,
    pub(crate) attachment: ::std::option::Option<crate::types::InstanceNetworkInterfaceAttachment>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) groups: ::std::option::Option<::std::vec::Vec<crate::types::GroupIdentifier>>,
    pub(crate) ipv6_addresses: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv6Address>>,
    pub(crate) mac_address: ::std::option::Option<::std::string::String>,
    pub(crate) network_interface_id: ::std::option::Option<::std::string::String>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) private_dns_name: ::std::option::Option<::std::string::String>,
    pub(crate) private_ip_address: ::std::option::Option<::std::string::String>,
    pub(crate) private_ip_addresses: ::std::option::Option<::std::vec::Vec<crate::types::InstancePrivateIpAddress>>,
    pub(crate) source_dest_check: ::std::option::Option<bool>,
    pub(crate) status: ::std::option::Option<crate::types::NetworkInterfaceStatus>,
    pub(crate) subnet_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) interface_type: ::std::option::Option<::std::string::String>,
    pub(crate) ipv4_prefixes: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv4Prefix>>,
    pub(crate) ipv6_prefixes: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv6Prefix>>,
    pub(crate) connection_tracking_configuration: ::std::option::Option<crate::types::ConnectionTrackingSpecificationResponse>,
}
impl InstanceNetworkInterfaceBuilder {
    /// <p>The association information for an Elastic IPv4 associated with the network interface.</p>
    pub fn association(mut self, input: crate::types::InstanceNetworkInterfaceAssociation) -> Self {
        self.association = ::std::option::Option::Some(input);
        self
    }
    /// <p>The association information for an Elastic IPv4 associated with the network interface.</p>
    pub fn set_association(mut self, input: ::std::option::Option<crate::types::InstanceNetworkInterfaceAssociation>) -> Self {
        self.association = input;
        self
    }
    /// <p>The association information for an Elastic IPv4 associated with the network interface.</p>
    pub fn get_association(&self) -> &::std::option::Option<crate::types::InstanceNetworkInterfaceAssociation> {
        &self.association
    }
    /// <p>The network interface attachment.</p>
    pub fn attachment(mut self, input: crate::types::InstanceNetworkInterfaceAttachment) -> Self {
        self.attachment = ::std::option::Option::Some(input);
        self
    }
    /// <p>The network interface attachment.</p>
    pub fn set_attachment(mut self, input: ::std::option::Option<crate::types::InstanceNetworkInterfaceAttachment>) -> Self {
        self.attachment = input;
        self
    }
    /// <p>The network interface attachment.</p>
    pub fn get_attachment(&self) -> &::std::option::Option<crate::types::InstanceNetworkInterfaceAttachment> {
        &self.attachment
    }
    /// <p>The description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>The security groups.</p>
    pub fn groups(mut self, input: crate::types::GroupIdentifier) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input);
        self.groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>The security groups.</p>
    pub fn set_groups(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GroupIdentifier>>) -> Self {
        self.groups = input;
        self
    }
    /// <p>The security groups.</p>
    pub fn get_groups(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GroupIdentifier>> {
        &self.groups
    }
    /// Appends an item to `ipv6_addresses`.
    ///
    /// To override the contents of this collection use [`set_ipv6_addresses`](Self::set_ipv6_addresses).
    ///
    /// <p>The IPv6 addresses associated with the network interface.</p>
    pub fn ipv6_addresses(mut self, input: crate::types::InstanceIpv6Address) -> Self {
        let mut v = self.ipv6_addresses.unwrap_or_default();
        v.push(input);
        self.ipv6_addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IPv6 addresses associated with the network interface.</p>
    pub fn set_ipv6_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv6Address>>) -> Self {
        self.ipv6_addresses = input;
        self
    }
    /// <p>The IPv6 addresses associated with the network interface.</p>
    pub fn get_ipv6_addresses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv6Address>> {
        &self.ipv6_addresses
    }
    /// <p>The MAC address.</p>
    pub fn mac_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.mac_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The MAC address.</p>
    pub fn set_mac_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.mac_address = input;
        self
    }
    /// <p>The MAC address.</p>
    pub fn get_mac_address(&self) -> &::std::option::Option<::std::string::String> {
        &self.mac_address
    }
    /// <p>The ID of the network interface.</p>
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
    /// <p>The ID of the Amazon Web Services account that created the network interface.</p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that created the network interface.</p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that created the network interface.</p>
    pub fn get_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner_id
    }
    /// <p>The private DNS name.</p>
    pub fn private_dns_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.private_dns_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The private DNS name.</p>
    pub fn set_private_dns_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.private_dns_name = input;
        self
    }
    /// <p>The private DNS name.</p>
    pub fn get_private_dns_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.private_dns_name
    }
    /// <p>The IPv4 address of the network interface within the subnet.</p>
    pub fn private_ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.private_ip_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 address of the network interface within the subnet.</p>
    pub fn set_private_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.private_ip_address = input;
        self
    }
    /// <p>The IPv4 address of the network interface within the subnet.</p>
    pub fn get_private_ip_address(&self) -> &::std::option::Option<::std::string::String> {
        &self.private_ip_address
    }
    /// Appends an item to `private_ip_addresses`.
    ///
    /// To override the contents of this collection use [`set_private_ip_addresses`](Self::set_private_ip_addresses).
    ///
    /// <p>The private IPv4 addresses associated with the network interface.</p>
    pub fn private_ip_addresses(mut self, input: crate::types::InstancePrivateIpAddress) -> Self {
        let mut v = self.private_ip_addresses.unwrap_or_default();
        v.push(input);
        self.private_ip_addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The private IPv4 addresses associated with the network interface.</p>
    pub fn set_private_ip_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InstancePrivateIpAddress>>) -> Self {
        self.private_ip_addresses = input;
        self
    }
    /// <p>The private IPv4 addresses associated with the network interface.</p>
    pub fn get_private_ip_addresses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstancePrivateIpAddress>> {
        &self.private_ip_addresses
    }
    /// <p>Indicates whether source/destination checking is enabled.</p>
    pub fn source_dest_check(mut self, input: bool) -> Self {
        self.source_dest_check = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether source/destination checking is enabled.</p>
    pub fn set_source_dest_check(mut self, input: ::std::option::Option<bool>) -> Self {
        self.source_dest_check = input;
        self
    }
    /// <p>Indicates whether source/destination checking is enabled.</p>
    pub fn get_source_dest_check(&self) -> &::std::option::Option<bool> {
        &self.source_dest_check
    }
    /// <p>The status of the network interface.</p>
    pub fn status(mut self, input: crate::types::NetworkInterfaceStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the network interface.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::NetworkInterfaceStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status of the network interface.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::NetworkInterfaceStatus> {
        &self.status
    }
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
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_id
    }
    /// <p>The type of network interface.</p>
    /// <p>Valid values: <code>interface</code> | <code>efa</code> | <code>trunk</code> </p>
    pub fn interface_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.interface_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of network interface.</p>
    /// <p>Valid values: <code>interface</code> | <code>efa</code> | <code>trunk</code> </p>
    pub fn set_interface_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.interface_type = input;
        self
    }
    /// <p>The type of network interface.</p>
    /// <p>Valid values: <code>interface</code> | <code>efa</code> | <code>trunk</code> </p>
    pub fn get_interface_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.interface_type
    }
    /// Appends an item to `ipv4_prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv4_prefixes`](Self::set_ipv4_prefixes).
    ///
    /// <p>The IPv4 delegated prefixes that are assigned to the network interface.</p>
    pub fn ipv4_prefixes(mut self, input: crate::types::InstanceIpv4Prefix) -> Self {
        let mut v = self.ipv4_prefixes.unwrap_or_default();
        v.push(input);
        self.ipv4_prefixes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IPv4 delegated prefixes that are assigned to the network interface.</p>
    pub fn set_ipv4_prefixes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv4Prefix>>) -> Self {
        self.ipv4_prefixes = input;
        self
    }
    /// <p>The IPv4 delegated prefixes that are assigned to the network interface.</p>
    pub fn get_ipv4_prefixes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv4Prefix>> {
        &self.ipv4_prefixes
    }
    /// Appends an item to `ipv6_prefixes`.
    ///
    /// To override the contents of this collection use [`set_ipv6_prefixes`](Self::set_ipv6_prefixes).
    ///
    /// <p>The IPv6 delegated prefixes that are assigned to the network interface.</p>
    pub fn ipv6_prefixes(mut self, input: crate::types::InstanceIpv6Prefix) -> Self {
        let mut v = self.ipv6_prefixes.unwrap_or_default();
        v.push(input);
        self.ipv6_prefixes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IPv6 delegated prefixes that are assigned to the network interface.</p>
    pub fn set_ipv6_prefixes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv6Prefix>>) -> Self {
        self.ipv6_prefixes = input;
        self
    }
    /// <p>The IPv6 delegated prefixes that are assigned to the network interface.</p>
    pub fn get_ipv6_prefixes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceIpv6Prefix>> {
        &self.ipv6_prefixes
    }
    /// <p>A security group connection tracking configuration that enables you to set the timeout for connection tracking on an Elastic network interface. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#connection-tracking-timeouts">Connection tracking timeouts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn connection_tracking_configuration(mut self, input: crate::types::ConnectionTrackingSpecificationResponse) -> Self {
        self.connection_tracking_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>A security group connection tracking configuration that enables you to set the timeout for connection tracking on an Elastic network interface. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#connection-tracking-timeouts">Connection tracking timeouts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn set_connection_tracking_configuration(
        mut self,
        input: ::std::option::Option<crate::types::ConnectionTrackingSpecificationResponse>,
    ) -> Self {
        self.connection_tracking_configuration = input;
        self
    }
    /// <p>A security group connection tracking configuration that enables you to set the timeout for connection tracking on an Elastic network interface. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/security-group-connection-tracking.html#connection-tracking-timeouts">Connection tracking timeouts</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn get_connection_tracking_configuration(&self) -> &::std::option::Option<crate::types::ConnectionTrackingSpecificationResponse> {
        &self.connection_tracking_configuration
    }
    /// Consumes the builder and constructs a [`InstanceNetworkInterface`](crate::types::InstanceNetworkInterface).
    pub fn build(self) -> crate::types::InstanceNetworkInterface {
        crate::types::InstanceNetworkInterface {
            association: self.association,
            attachment: self.attachment,
            description: self.description,
            groups: self.groups,
            ipv6_addresses: self.ipv6_addresses,
            mac_address: self.mac_address,
            network_interface_id: self.network_interface_id,
            owner_id: self.owner_id,
            private_dns_name: self.private_dns_name,
            private_ip_address: self.private_ip_address,
            private_ip_addresses: self.private_ip_addresses,
            source_dest_check: self.source_dest_check,
            status: self.status,
            subnet_id: self.subnet_id,
            vpc_id: self.vpc_id,
            interface_type: self.interface_type,
            ipv4_prefixes: self.ipv4_prefixes,
            ipv6_prefixes: self.ipv6_prefixes,
            connection_tracking_configuration: self.connection_tracking_configuration,
        }
    }
}
