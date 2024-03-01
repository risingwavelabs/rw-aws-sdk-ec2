// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a VPC endpoint.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcEndpoint {
    /// <p>The ID of the endpoint.</p>
    pub vpc_endpoint_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of endpoint.</p>
    pub vpc_endpoint_type: ::std::option::Option<crate::types::VpcEndpointType>,
    /// <p>The ID of the VPC to which the endpoint is associated.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the service to which the endpoint is associated.</p>
    pub service_name: ::std::option::Option<::std::string::String>,
    /// <p>The state of the endpoint.</p>
    pub state: ::std::option::Option<crate::types::State>,
    /// <p>The policy document associated with the endpoint, if applicable.</p>
    pub policy_document: ::std::option::Option<::std::string::String>,
    /// <p>(Gateway endpoint) The IDs of the route tables associated with the endpoint.</p>
    pub route_table_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Interface endpoint) The subnets for the endpoint.</p>
    pub subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Interface endpoint) Information about the security groups that are associated with the network interface.</p>
    pub groups: ::std::option::Option<::std::vec::Vec<crate::types::SecurityGroupIdentifier>>,
    /// <p>The IP address type for the endpoint.</p>
    pub ip_address_type: ::std::option::Option<crate::types::IpAddressType>,
    /// <p>The DNS options for the endpoint.</p>
    pub dns_options: ::std::option::Option<crate::types::DnsOptions>,
    /// <p>(Interface endpoint) Indicates whether the VPC is associated with a private hosted zone.</p>
    pub private_dns_enabled: ::std::option::Option<bool>,
    /// <p>Indicates whether the endpoint is being managed by its service.</p>
    pub requester_managed: ::std::option::Option<bool>,
    /// <p>(Interface endpoint) The network interfaces for the endpoint.</p>
    pub network_interface_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Interface endpoint) The DNS entries for the endpoint.</p>
    pub dns_entries: ::std::option::Option<::std::vec::Vec<crate::types::DnsEntry>>,
    /// <p>The date and time that the endpoint was created.</p>
    pub creation_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The tags assigned to the endpoint.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The ID of the Amazon Web Services account that owns the endpoint.</p>
    pub owner_id: ::std::option::Option<::std::string::String>,
    /// <p>The last error that occurred for endpoint.</p>
    pub last_error: ::std::option::Option<crate::types::LastError>,
}
impl VpcEndpoint {
    /// <p>The ID of the endpoint.</p>
    pub fn vpc_endpoint_id(&self) -> ::std::option::Option<&str> {
        self.vpc_endpoint_id.as_deref()
    }
    /// <p>The type of endpoint.</p>
    pub fn vpc_endpoint_type(&self) -> ::std::option::Option<&crate::types::VpcEndpointType> {
        self.vpc_endpoint_type.as_ref()
    }
    /// <p>The ID of the VPC to which the endpoint is associated.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The name of the service to which the endpoint is associated.</p>
    pub fn service_name(&self) -> ::std::option::Option<&str> {
        self.service_name.as_deref()
    }
    /// <p>The state of the endpoint.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::State> {
        self.state.as_ref()
    }
    /// <p>The policy document associated with the endpoint, if applicable.</p>
    pub fn policy_document(&self) -> ::std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>(Gateway endpoint) The IDs of the route tables associated with the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.route_table_ids.is_none()`.
    pub fn route_table_ids(&self) -> &[::std::string::String] {
        self.route_table_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Interface endpoint) The subnets for the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subnet_ids.is_none()`.
    pub fn subnet_ids(&self) -> &[::std::string::String] {
        self.subnet_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Interface endpoint) Information about the security groups that are associated with the network interface.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.groups.is_none()`.
    pub fn groups(&self) -> &[crate::types::SecurityGroupIdentifier] {
        self.groups.as_deref().unwrap_or_default()
    }
    /// <p>The IP address type for the endpoint.</p>
    pub fn ip_address_type(&self) -> ::std::option::Option<&crate::types::IpAddressType> {
        self.ip_address_type.as_ref()
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn dns_options(&self) -> ::std::option::Option<&crate::types::DnsOptions> {
        self.dns_options.as_ref()
    }
    /// <p>(Interface endpoint) Indicates whether the VPC is associated with a private hosted zone.</p>
    pub fn private_dns_enabled(&self) -> ::std::option::Option<bool> {
        self.private_dns_enabled
    }
    /// <p>Indicates whether the endpoint is being managed by its service.</p>
    pub fn requester_managed(&self) -> ::std::option::Option<bool> {
        self.requester_managed
    }
    /// <p>(Interface endpoint) The network interfaces for the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.network_interface_ids.is_none()`.
    pub fn network_interface_ids(&self) -> &[::std::string::String] {
        self.network_interface_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Interface endpoint) The DNS entries for the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.dns_entries.is_none()`.
    pub fn dns_entries(&self) -> &[crate::types::DnsEntry] {
        self.dns_entries.as_deref().unwrap_or_default()
    }
    /// <p>The date and time that the endpoint was created.</p>
    pub fn creation_timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_timestamp.as_ref()
    }
    /// <p>The tags assigned to the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
    /// <p>The ID of the Amazon Web Services account that owns the endpoint.</p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The last error that occurred for endpoint.</p>
    pub fn last_error(&self) -> ::std::option::Option<&crate::types::LastError> {
        self.last_error.as_ref()
    }
}
impl VpcEndpoint {
    /// Creates a new builder-style object to manufacture [`VpcEndpoint`](crate::types::VpcEndpoint).
    pub fn builder() -> crate::types::builders::VpcEndpointBuilder {
        crate::types::builders::VpcEndpointBuilder::default()
    }
}

/// A builder for [`VpcEndpoint`](crate::types::VpcEndpoint).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VpcEndpointBuilder {
    pub(crate) vpc_endpoint_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_endpoint_type: ::std::option::Option<crate::types::VpcEndpointType>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) service_name: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::State>,
    pub(crate) policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) route_table_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) groups: ::std::option::Option<::std::vec::Vec<crate::types::SecurityGroupIdentifier>>,
    pub(crate) ip_address_type: ::std::option::Option<crate::types::IpAddressType>,
    pub(crate) dns_options: ::std::option::Option<crate::types::DnsOptions>,
    pub(crate) private_dns_enabled: ::std::option::Option<bool>,
    pub(crate) requester_managed: ::std::option::Option<bool>,
    pub(crate) network_interface_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) dns_entries: ::std::option::Option<::std::vec::Vec<crate::types::DnsEntry>>,
    pub(crate) creation_timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) last_error: ::std::option::Option<crate::types::LastError>,
}
impl VpcEndpointBuilder {
    /// <p>The ID of the endpoint.</p>
    pub fn vpc_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_endpoint_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn set_vpc_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_endpoint_id = input;
        self
    }
    /// <p>The ID of the endpoint.</p>
    pub fn get_vpc_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_endpoint_id
    }
    /// <p>The type of endpoint.</p>
    pub fn vpc_endpoint_type(mut self, input: crate::types::VpcEndpointType) -> Self {
        self.vpc_endpoint_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of endpoint.</p>
    pub fn set_vpc_endpoint_type(mut self, input: ::std::option::Option<crate::types::VpcEndpointType>) -> Self {
        self.vpc_endpoint_type = input;
        self
    }
    /// <p>The type of endpoint.</p>
    pub fn get_vpc_endpoint_type(&self) -> &::std::option::Option<crate::types::VpcEndpointType> {
        &self.vpc_endpoint_type
    }
    /// <p>The ID of the VPC to which the endpoint is associated.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC to which the endpoint is associated.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>The ID of the VPC to which the endpoint is associated.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_id
    }
    /// <p>The name of the service to which the endpoint is associated.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the service to which the endpoint is associated.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_name = input;
        self
    }
    /// <p>The name of the service to which the endpoint is associated.</p>
    pub fn get_service_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.service_name
    }
    /// <p>The state of the endpoint.</p>
    pub fn state(mut self, input: crate::types::State) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the endpoint.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::State>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the endpoint.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::State> {
        &self.state
    }
    /// <p>The policy document associated with the endpoint, if applicable.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The policy document associated with the endpoint, if applicable.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>The policy document associated with the endpoint, if applicable.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_document
    }
    /// Appends an item to `route_table_ids`.
    ///
    /// To override the contents of this collection use [`set_route_table_ids`](Self::set_route_table_ids).
    ///
    /// <p>(Gateway endpoint) The IDs of the route tables associated with the endpoint.</p>
    pub fn route_table_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.route_table_ids.unwrap_or_default();
        v.push(input.into());
        self.route_table_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables associated with the endpoint.</p>
    pub fn set_route_table_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.route_table_ids = input;
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables associated with the endpoint.</p>
    pub fn get_route_table_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.route_table_ids
    }
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>(Interface endpoint) The subnets for the endpoint.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface endpoint) The subnets for the endpoint.</p>
    pub fn set_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.subnet_ids = input;
        self
    }
    /// <p>(Interface endpoint) The subnets for the endpoint.</p>
    pub fn get_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.subnet_ids
    }
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>(Interface endpoint) Information about the security groups that are associated with the network interface.</p>
    pub fn groups(mut self, input: crate::types::SecurityGroupIdentifier) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input);
        self.groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface endpoint) Information about the security groups that are associated with the network interface.</p>
    pub fn set_groups(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SecurityGroupIdentifier>>) -> Self {
        self.groups = input;
        self
    }
    /// <p>(Interface endpoint) Information about the security groups that are associated with the network interface.</p>
    pub fn get_groups(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SecurityGroupIdentifier>> {
        &self.groups
    }
    /// <p>The IP address type for the endpoint.</p>
    pub fn ip_address_type(mut self, input: crate::types::IpAddressType) -> Self {
        self.ip_address_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The IP address type for the endpoint.</p>
    pub fn set_ip_address_type(mut self, input: ::std::option::Option<crate::types::IpAddressType>) -> Self {
        self.ip_address_type = input;
        self
    }
    /// <p>The IP address type for the endpoint.</p>
    pub fn get_ip_address_type(&self) -> &::std::option::Option<crate::types::IpAddressType> {
        &self.ip_address_type
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn dns_options(mut self, input: crate::types::DnsOptions) -> Self {
        self.dns_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn set_dns_options(mut self, input: ::std::option::Option<crate::types::DnsOptions>) -> Self {
        self.dns_options = input;
        self
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn get_dns_options(&self) -> &::std::option::Option<crate::types::DnsOptions> {
        &self.dns_options
    }
    /// <p>(Interface endpoint) Indicates whether the VPC is associated with a private hosted zone.</p>
    pub fn private_dns_enabled(mut self, input: bool) -> Self {
        self.private_dns_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>(Interface endpoint) Indicates whether the VPC is associated with a private hosted zone.</p>
    pub fn set_private_dns_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.private_dns_enabled = input;
        self
    }
    /// <p>(Interface endpoint) Indicates whether the VPC is associated with a private hosted zone.</p>
    pub fn get_private_dns_enabled(&self) -> &::std::option::Option<bool> {
        &self.private_dns_enabled
    }
    /// <p>Indicates whether the endpoint is being managed by its service.</p>
    pub fn requester_managed(mut self, input: bool) -> Self {
        self.requester_managed = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the endpoint is being managed by its service.</p>
    pub fn set_requester_managed(mut self, input: ::std::option::Option<bool>) -> Self {
        self.requester_managed = input;
        self
    }
    /// <p>Indicates whether the endpoint is being managed by its service.</p>
    pub fn get_requester_managed(&self) -> &::std::option::Option<bool> {
        &self.requester_managed
    }
    /// Appends an item to `network_interface_ids`.
    ///
    /// To override the contents of this collection use [`set_network_interface_ids`](Self::set_network_interface_ids).
    ///
    /// <p>(Interface endpoint) The network interfaces for the endpoint.</p>
    pub fn network_interface_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.network_interface_ids.unwrap_or_default();
        v.push(input.into());
        self.network_interface_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface endpoint) The network interfaces for the endpoint.</p>
    pub fn set_network_interface_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.network_interface_ids = input;
        self
    }
    /// <p>(Interface endpoint) The network interfaces for the endpoint.</p>
    pub fn get_network_interface_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.network_interface_ids
    }
    /// Appends an item to `dns_entries`.
    ///
    /// To override the contents of this collection use [`set_dns_entries`](Self::set_dns_entries).
    ///
    /// <p>(Interface endpoint) The DNS entries for the endpoint.</p>
    pub fn dns_entries(mut self, input: crate::types::DnsEntry) -> Self {
        let mut v = self.dns_entries.unwrap_or_default();
        v.push(input);
        self.dns_entries = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface endpoint) The DNS entries for the endpoint.</p>
    pub fn set_dns_entries(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::DnsEntry>>) -> Self {
        self.dns_entries = input;
        self
    }
    /// <p>(Interface endpoint) The DNS entries for the endpoint.</p>
    pub fn get_dns_entries(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::DnsEntry>> {
        &self.dns_entries
    }
    /// <p>The date and time that the endpoint was created.</p>
    pub fn creation_timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time that the endpoint was created.</p>
    pub fn set_creation_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_timestamp = input;
        self
    }
    /// <p>The date and time that the endpoint was created.</p>
    pub fn get_creation_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_timestamp
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags assigned to the endpoint.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags assigned to the endpoint.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags assigned to the endpoint.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// <p>The ID of the Amazon Web Services account that owns the endpoint.</p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the endpoint.</p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the endpoint.</p>
    pub fn get_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner_id
    }
    /// <p>The last error that occurred for endpoint.</p>
    pub fn last_error(mut self, input: crate::types::LastError) -> Self {
        self.last_error = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last error that occurred for endpoint.</p>
    pub fn set_last_error(mut self, input: ::std::option::Option<crate::types::LastError>) -> Self {
        self.last_error = input;
        self
    }
    /// <p>The last error that occurred for endpoint.</p>
    pub fn get_last_error(&self) -> &::std::option::Option<crate::types::LastError> {
        &self.last_error
    }
    /// Consumes the builder and constructs a [`VpcEndpoint`](crate::types::VpcEndpoint).
    pub fn build(self) -> crate::types::VpcEndpoint {
        crate::types::VpcEndpoint {
            vpc_endpoint_id: self.vpc_endpoint_id,
            vpc_endpoint_type: self.vpc_endpoint_type,
            vpc_id: self.vpc_id,
            service_name: self.service_name,
            state: self.state,
            policy_document: self.policy_document,
            route_table_ids: self.route_table_ids,
            subnet_ids: self.subnet_ids,
            groups: self.groups,
            ip_address_type: self.ip_address_type,
            dns_options: self.dns_options,
            private_dns_enabled: self.private_dns_enabled,
            requester_managed: self.requester_managed,
            network_interface_ids: self.network_interface_ids,
            dns_entries: self.dns_entries,
            creation_timestamp: self.creation_timestamp,
            tags: self.tags,
            owner_id: self.owner_id,
            last_error: self.last_error,
        }
    }
}
