// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyVpcEndpointInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the endpoint.</p>
    pub vpc_endpoint_id: ::std::option::Option<::std::string::String>,
    /// <p>(Gateway endpoint) Specify <code>true</code> to reset the policy document to the default policy. The default policy allows full access to the service.</p>
    pub reset_policy: ::std::option::Option<bool>,
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format.</p>
    pub policy_document: ::std::option::Option<::std::string::String>,
    /// <p>(Gateway endpoint) The IDs of the route tables to associate with the endpoint.</p>
    pub add_route_table_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Gateway endpoint) The IDs of the route tables to disassociate from the endpoint.</p>
    pub remove_route_table_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to serve the endpoint. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub add_subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Interface endpoint) The IDs of the subnets from which to remove the endpoint.</p>
    pub remove_subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces.</p>
    pub add_security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Interface endpoint) The IDs of the security groups to disassociate from the endpoint network interfaces.</p>
    pub remove_security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The IP address type for the endpoint.</p>
    pub ip_address_type: ::std::option::Option<crate::types::IpAddressType>,
    /// <p>The DNS options for the endpoint.</p>
    pub dns_options: ::std::option::Option<crate::types::DnsOptionsSpecification>,
    /// <p>(Interface endpoint) Indicates whether a private hosted zone is associated with the VPC.</p>
    pub private_dns_enabled: ::std::option::Option<bool>,
    /// <p>The subnet configurations for the endpoint.</p>
    pub subnet_configurations: ::std::option::Option<::std::vec::Vec<crate::types::SubnetConfiguration>>,
}
impl ModifyVpcEndpointInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the endpoint.</p>
    pub fn vpc_endpoint_id(&self) -> ::std::option::Option<&str> {
        self.vpc_endpoint_id.as_deref()
    }
    /// <p>(Gateway endpoint) Specify <code>true</code> to reset the policy document to the default policy. The default policy allows full access to the service.</p>
    pub fn reset_policy(&self) -> ::std::option::Option<bool> {
        self.reset_policy
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format.</p>
    pub fn policy_document(&self) -> ::std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to associate with the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.add_route_table_ids.is_none()`.
    pub fn add_route_table_ids(&self) -> &[::std::string::String] {
        self.add_route_table_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to disassociate from the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.remove_route_table_ids.is_none()`.
    pub fn remove_route_table_ids(&self) -> &[::std::string::String] {
        self.remove_route_table_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to serve the endpoint. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.add_subnet_ids.is_none()`.
    pub fn add_subnet_ids(&self) -> &[::std::string::String] {
        self.add_subnet_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Interface endpoint) The IDs of the subnets from which to remove the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.remove_subnet_ids.is_none()`.
    pub fn remove_subnet_ids(&self) -> &[::std::string::String] {
        self.remove_subnet_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.add_security_group_ids.is_none()`.
    pub fn add_security_group_ids(&self) -> &[::std::string::String] {
        self.add_security_group_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Interface endpoint) The IDs of the security groups to disassociate from the endpoint network interfaces.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.remove_security_group_ids.is_none()`.
    pub fn remove_security_group_ids(&self) -> &[::std::string::String] {
        self.remove_security_group_ids.as_deref().unwrap_or_default()
    }
    /// <p>The IP address type for the endpoint.</p>
    pub fn ip_address_type(&self) -> ::std::option::Option<&crate::types::IpAddressType> {
        self.ip_address_type.as_ref()
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn dns_options(&self) -> ::std::option::Option<&crate::types::DnsOptionsSpecification> {
        self.dns_options.as_ref()
    }
    /// <p>(Interface endpoint) Indicates whether a private hosted zone is associated with the VPC.</p>
    pub fn private_dns_enabled(&self) -> ::std::option::Option<bool> {
        self.private_dns_enabled
    }
    /// <p>The subnet configurations for the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subnet_configurations.is_none()`.
    pub fn subnet_configurations(&self) -> &[crate::types::SubnetConfiguration] {
        self.subnet_configurations.as_deref().unwrap_or_default()
    }
}
impl ModifyVpcEndpointInput {
    /// Creates a new builder-style object to manufacture [`ModifyVpcEndpointInput`](crate::operation::modify_vpc_endpoint::ModifyVpcEndpointInput).
    pub fn builder() -> crate::operation::modify_vpc_endpoint::builders::ModifyVpcEndpointInputBuilder {
        crate::operation::modify_vpc_endpoint::builders::ModifyVpcEndpointInputBuilder::default()
    }
}

/// A builder for [`ModifyVpcEndpointInput`](crate::operation::modify_vpc_endpoint::ModifyVpcEndpointInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyVpcEndpointInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) vpc_endpoint_id: ::std::option::Option<::std::string::String>,
    pub(crate) reset_policy: ::std::option::Option<bool>,
    pub(crate) policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) add_route_table_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) remove_route_table_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) add_subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) remove_subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) add_security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) remove_security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) ip_address_type: ::std::option::Option<crate::types::IpAddressType>,
    pub(crate) dns_options: ::std::option::Option<crate::types::DnsOptionsSpecification>,
    pub(crate) private_dns_enabled: ::std::option::Option<bool>,
    pub(crate) subnet_configurations: ::std::option::Option<::std::vec::Vec<crate::types::SubnetConfiguration>>,
}
impl ModifyVpcEndpointInputBuilder {
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
    /// <p>The ID of the endpoint.</p>
    /// This field is required.
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
    /// <p>(Gateway endpoint) Specify <code>true</code> to reset the policy document to the default policy. The default policy allows full access to the service.</p>
    pub fn reset_policy(mut self, input: bool) -> Self {
        self.reset_policy = ::std::option::Option::Some(input);
        self
    }
    /// <p>(Gateway endpoint) Specify <code>true</code> to reset the policy document to the default policy. The default policy allows full access to the service.</p>
    pub fn set_reset_policy(mut self, input: ::std::option::Option<bool>) -> Self {
        self.reset_policy = input;
        self
    }
    /// <p>(Gateway endpoint) Specify <code>true</code> to reset the policy document to the default policy. The default policy allows full access to the service.</p>
    pub fn get_reset_policy(&self) -> &::std::option::Option<bool> {
        &self.reset_policy
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_document
    }
    /// Appends an item to `add_route_table_ids`.
    ///
    /// To override the contents of this collection use [`set_add_route_table_ids`](Self::set_add_route_table_ids).
    ///
    /// <p>(Gateway endpoint) The IDs of the route tables to associate with the endpoint.</p>
    pub fn add_route_table_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.add_route_table_ids.unwrap_or_default();
        v.push(input.into());
        self.add_route_table_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to associate with the endpoint.</p>
    pub fn set_add_route_table_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.add_route_table_ids = input;
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to associate with the endpoint.</p>
    pub fn get_add_route_table_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.add_route_table_ids
    }
    /// Appends an item to `remove_route_table_ids`.
    ///
    /// To override the contents of this collection use [`set_remove_route_table_ids`](Self::set_remove_route_table_ids).
    ///
    /// <p>(Gateway endpoint) The IDs of the route tables to disassociate from the endpoint.</p>
    pub fn remove_route_table_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.remove_route_table_ids.unwrap_or_default();
        v.push(input.into());
        self.remove_route_table_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to disassociate from the endpoint.</p>
    pub fn set_remove_route_table_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.remove_route_table_ids = input;
        self
    }
    /// <p>(Gateway endpoint) The IDs of the route tables to disassociate from the endpoint.</p>
    pub fn get_remove_route_table_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.remove_route_table_ids
    }
    /// Appends an item to `add_subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_add_subnet_ids`](Self::set_add_subnet_ids).
    ///
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to serve the endpoint. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub fn add_subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.add_subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.add_subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to serve the endpoint. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub fn set_add_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.add_subnet_ids = input;
        self
    }
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to serve the endpoint. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub fn get_add_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.add_subnet_ids
    }
    /// Appends an item to `remove_subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_remove_subnet_ids`](Self::set_remove_subnet_ids).
    ///
    /// <p>(Interface endpoint) The IDs of the subnets from which to remove the endpoint.</p>
    pub fn remove_subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.remove_subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.remove_subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface endpoint) The IDs of the subnets from which to remove the endpoint.</p>
    pub fn set_remove_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.remove_subnet_ids = input;
        self
    }
    /// <p>(Interface endpoint) The IDs of the subnets from which to remove the endpoint.</p>
    pub fn get_remove_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.remove_subnet_ids
    }
    /// Appends an item to `add_security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_add_security_group_ids`](Self::set_add_security_group_ids).
    ///
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces.</p>
    pub fn add_security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.add_security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.add_security_group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces.</p>
    pub fn set_add_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.add_security_group_ids = input;
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces.</p>
    pub fn get_add_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.add_security_group_ids
    }
    /// Appends an item to `remove_security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_remove_security_group_ids`](Self::set_remove_security_group_ids).
    ///
    /// <p>(Interface endpoint) The IDs of the security groups to disassociate from the endpoint network interfaces.</p>
    pub fn remove_security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.remove_security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.remove_security_group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to disassociate from the endpoint network interfaces.</p>
    pub fn set_remove_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.remove_security_group_ids = input;
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to disassociate from the endpoint network interfaces.</p>
    pub fn get_remove_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.remove_security_group_ids
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
    pub fn dns_options(mut self, input: crate::types::DnsOptionsSpecification) -> Self {
        self.dns_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn set_dns_options(mut self, input: ::std::option::Option<crate::types::DnsOptionsSpecification>) -> Self {
        self.dns_options = input;
        self
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn get_dns_options(&self) -> &::std::option::Option<crate::types::DnsOptionsSpecification> {
        &self.dns_options
    }
    /// <p>(Interface endpoint) Indicates whether a private hosted zone is associated with the VPC.</p>
    pub fn private_dns_enabled(mut self, input: bool) -> Self {
        self.private_dns_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>(Interface endpoint) Indicates whether a private hosted zone is associated with the VPC.</p>
    pub fn set_private_dns_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.private_dns_enabled = input;
        self
    }
    /// <p>(Interface endpoint) Indicates whether a private hosted zone is associated with the VPC.</p>
    pub fn get_private_dns_enabled(&self) -> &::std::option::Option<bool> {
        &self.private_dns_enabled
    }
    /// Appends an item to `subnet_configurations`.
    ///
    /// To override the contents of this collection use [`set_subnet_configurations`](Self::set_subnet_configurations).
    ///
    /// <p>The subnet configurations for the endpoint.</p>
    pub fn subnet_configurations(mut self, input: crate::types::SubnetConfiguration) -> Self {
        let mut v = self.subnet_configurations.unwrap_or_default();
        v.push(input);
        self.subnet_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The subnet configurations for the endpoint.</p>
    pub fn set_subnet_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SubnetConfiguration>>) -> Self {
        self.subnet_configurations = input;
        self
    }
    /// <p>The subnet configurations for the endpoint.</p>
    pub fn get_subnet_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SubnetConfiguration>> {
        &self.subnet_configurations
    }
    /// Consumes the builder and constructs a [`ModifyVpcEndpointInput`](crate::operation::modify_vpc_endpoint::ModifyVpcEndpointInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::modify_vpc_endpoint::ModifyVpcEndpointInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::modify_vpc_endpoint::ModifyVpcEndpointInput {
            dry_run: self.dry_run,
            vpc_endpoint_id: self.vpc_endpoint_id,
            reset_policy: self.reset_policy,
            policy_document: self.policy_document,
            add_route_table_ids: self.add_route_table_ids,
            remove_route_table_ids: self.remove_route_table_ids,
            add_subnet_ids: self.add_subnet_ids,
            remove_subnet_ids: self.remove_subnet_ids,
            add_security_group_ids: self.add_security_group_ids,
            remove_security_group_ids: self.remove_security_group_ids,
            ip_address_type: self.ip_address_type,
            dns_options: self.dns_options,
            private_dns_enabled: self.private_dns_enabled,
            subnet_configurations: self.subnet_configurations,
        })
    }
}