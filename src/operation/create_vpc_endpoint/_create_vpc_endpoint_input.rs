// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVpcEndpointInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The type of endpoint.</p>
    /// <p>Default: Gateway</p>
    pub vpc_endpoint_type: ::std::option::Option<crate::types::VpcEndpointType>,
    /// <p>The ID of the VPC.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the endpoint service.</p>
    pub service_name: ::std::option::Option<::std::string::String>,
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format. If this parameter is not specified, we attach a default policy that allows full access to the service.</p>
    pub policy_document: ::std::option::Option<::std::string::String>,
    /// <p>(Gateway endpoint) The route table IDs.</p>
    pub route_table_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to create endpoint network interfaces. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces. If this parameter is not specified, we use the default security group for the VPC.</p>
    pub security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The IP address type for the endpoint.</p>
    pub ip_address_type: ::std::option::Option<crate::types::IpAddressType>,
    /// <p>The DNS options for the endpoint.</p>
    pub dns_options: ::std::option::Option<crate::types::DnsOptionsSpecification>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>(Interface endpoint) Indicates whether to associate a private hosted zone with the specified VPC. The private hosted zone contains a record set for the default public DNS name for the service for the Region (for example, <code>kinesis.us-east-1.amazonaws.com</code>), which resolves to the private IP addresses of the endpoint network interfaces in the VPC. This enables you to make requests to the default public DNS name for the service instead of the public DNS names that are automatically generated by the VPC endpoint service.</p>
    /// <p>To use a private hosted zone, you must set the following VPC attributes to <code>true</code>: <code>enableDnsHostnames</code> and <code>enableDnsSupport</code>. Use <code>ModifyVpcAttribute</code> to set the VPC attributes.</p>
    /// <p>Default: <code>true</code> </p>
    pub private_dns_enabled: ::std::option::Option<bool>,
    /// <p>The tags to associate with the endpoint.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>The subnet configurations for the endpoint.</p>
    pub subnet_configurations: ::std::option::Option<::std::vec::Vec<crate::types::SubnetConfiguration>>,
}
impl CreateVpcEndpointInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The type of endpoint.</p>
    /// <p>Default: Gateway</p>
    pub fn vpc_endpoint_type(&self) -> ::std::option::Option<&crate::types::VpcEndpointType> {
        self.vpc_endpoint_type.as_ref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The name of the endpoint service.</p>
    pub fn service_name(&self) -> ::std::option::Option<&str> {
        self.service_name.as_deref()
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format. If this parameter is not specified, we attach a default policy that allows full access to the service.</p>
    pub fn policy_document(&self) -> ::std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>(Gateway endpoint) The route table IDs.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.route_table_ids.is_none()`.
    pub fn route_table_ids(&self) -> &[::std::string::String] {
        self.route_table_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to create endpoint network interfaces. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subnet_ids.is_none()`.
    pub fn subnet_ids(&self) -> &[::std::string::String] {
        self.subnet_ids.as_deref().unwrap_or_default()
    }
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces. If this parameter is not specified, we use the default security group for the VPC.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.security_group_ids.is_none()`.
    pub fn security_group_ids(&self) -> &[::std::string::String] {
        self.security_group_ids.as_deref().unwrap_or_default()
    }
    /// <p>The IP address type for the endpoint.</p>
    pub fn ip_address_type(&self) -> ::std::option::Option<&crate::types::IpAddressType> {
        self.ip_address_type.as_ref()
    }
    /// <p>The DNS options for the endpoint.</p>
    pub fn dns_options(&self) -> ::std::option::Option<&crate::types::DnsOptionsSpecification> {
        self.dns_options.as_ref()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>(Interface endpoint) Indicates whether to associate a private hosted zone with the specified VPC. The private hosted zone contains a record set for the default public DNS name for the service for the Region (for example, <code>kinesis.us-east-1.amazonaws.com</code>), which resolves to the private IP addresses of the endpoint network interfaces in the VPC. This enables you to make requests to the default public DNS name for the service instead of the public DNS names that are automatically generated by the VPC endpoint service.</p>
    /// <p>To use a private hosted zone, you must set the following VPC attributes to <code>true</code>: <code>enableDnsHostnames</code> and <code>enableDnsSupport</code>. Use <code>ModifyVpcAttribute</code> to set the VPC attributes.</p>
    /// <p>Default: <code>true</code> </p>
    pub fn private_dns_enabled(&self) -> ::std::option::Option<bool> {
        self.private_dns_enabled
    }
    /// <p>The tags to associate with the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>The subnet configurations for the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subnet_configurations.is_none()`.
    pub fn subnet_configurations(&self) -> &[crate::types::SubnetConfiguration] {
        self.subnet_configurations.as_deref().unwrap_or_default()
    }
}
impl CreateVpcEndpointInput {
    /// Creates a new builder-style object to manufacture [`CreateVpcEndpointInput`](crate::operation::create_vpc_endpoint::CreateVpcEndpointInput).
    pub fn builder() -> crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointInputBuilder {
        crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointInputBuilder::default()
    }
}

/// A builder for [`CreateVpcEndpointInput`](crate::operation::create_vpc_endpoint::CreateVpcEndpointInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateVpcEndpointInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) vpc_endpoint_type: ::std::option::Option<crate::types::VpcEndpointType>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) service_name: ::std::option::Option<::std::string::String>,
    pub(crate) policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) route_table_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) ip_address_type: ::std::option::Option<crate::types::IpAddressType>,
    pub(crate) dns_options: ::std::option::Option<crate::types::DnsOptionsSpecification>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) private_dns_enabled: ::std::option::Option<bool>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) subnet_configurations: ::std::option::Option<::std::vec::Vec<crate::types::SubnetConfiguration>>,
}
impl CreateVpcEndpointInputBuilder {
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
    /// <p>The type of endpoint.</p>
    /// <p>Default: Gateway</p>
    pub fn vpc_endpoint_type(mut self, input: crate::types::VpcEndpointType) -> Self {
        self.vpc_endpoint_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of endpoint.</p>
    /// <p>Default: Gateway</p>
    pub fn set_vpc_endpoint_type(mut self, input: ::std::option::Option<crate::types::VpcEndpointType>) -> Self {
        self.vpc_endpoint_type = input;
        self
    }
    /// <p>The type of endpoint.</p>
    /// <p>Default: Gateway</p>
    pub fn get_vpc_endpoint_type(&self) -> &::std::option::Option<crate::types::VpcEndpointType> {
        &self.vpc_endpoint_type
    }
    /// <p>The ID of the VPC.</p>
    /// This field is required.
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
    /// <p>The name of the endpoint service.</p>
    /// This field is required.
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the endpoint service.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_name = input;
        self
    }
    /// <p>The name of the endpoint service.</p>
    pub fn get_service_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.service_name
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format. If this parameter is not specified, we attach a default policy that allows full access to the service.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format. If this parameter is not specified, we attach a default policy that allows full access to the service.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format. If this parameter is not specified, we attach a default policy that allows full access to the service.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_document
    }
    /// Appends an item to `route_table_ids`.
    ///
    /// To override the contents of this collection use [`set_route_table_ids`](Self::set_route_table_ids).
    ///
    /// <p>(Gateway endpoint) The route table IDs.</p>
    pub fn route_table_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.route_table_ids.unwrap_or_default();
        v.push(input.into());
        self.route_table_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Gateway endpoint) The route table IDs.</p>
    pub fn set_route_table_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.route_table_ids = input;
        self
    }
    /// <p>(Gateway endpoint) The route table IDs.</p>
    pub fn get_route_table_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.route_table_ids
    }
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to create endpoint network interfaces. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to create endpoint network interfaces. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub fn set_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.subnet_ids = input;
        self
    }
    /// <p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to create endpoint network interfaces. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p>
    pub fn get_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.subnet_ids
    }
    /// Appends an item to `security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces. If this parameter is not specified, we use the default security group for the VPC.</p>
    pub fn security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces. If this parameter is not specified, we use the default security group for the VPC.</p>
    pub fn set_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.security_group_ids = input;
        self
    }
    /// <p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces. If this parameter is not specified, we use the default security group for the VPC.</p>
    pub fn get_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.security_group_ids
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// <p>(Interface endpoint) Indicates whether to associate a private hosted zone with the specified VPC. The private hosted zone contains a record set for the default public DNS name for the service for the Region (for example, <code>kinesis.us-east-1.amazonaws.com</code>), which resolves to the private IP addresses of the endpoint network interfaces in the VPC. This enables you to make requests to the default public DNS name for the service instead of the public DNS names that are automatically generated by the VPC endpoint service.</p>
    /// <p>To use a private hosted zone, you must set the following VPC attributes to <code>true</code>: <code>enableDnsHostnames</code> and <code>enableDnsSupport</code>. Use <code>ModifyVpcAttribute</code> to set the VPC attributes.</p>
    /// <p>Default: <code>true</code> </p>
    pub fn private_dns_enabled(mut self, input: bool) -> Self {
        self.private_dns_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>(Interface endpoint) Indicates whether to associate a private hosted zone with the specified VPC. The private hosted zone contains a record set for the default public DNS name for the service for the Region (for example, <code>kinesis.us-east-1.amazonaws.com</code>), which resolves to the private IP addresses of the endpoint network interfaces in the VPC. This enables you to make requests to the default public DNS name for the service instead of the public DNS names that are automatically generated by the VPC endpoint service.</p>
    /// <p>To use a private hosted zone, you must set the following VPC attributes to <code>true</code>: <code>enableDnsHostnames</code> and <code>enableDnsSupport</code>. Use <code>ModifyVpcAttribute</code> to set the VPC attributes.</p>
    /// <p>Default: <code>true</code> </p>
    pub fn set_private_dns_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.private_dns_enabled = input;
        self
    }
    /// <p>(Interface endpoint) Indicates whether to associate a private hosted zone with the specified VPC. The private hosted zone contains a record set for the default public DNS name for the service for the Region (for example, <code>kinesis.us-east-1.amazonaws.com</code>), which resolves to the private IP addresses of the endpoint network interfaces in the VPC. This enables you to make requests to the default public DNS name for the service instead of the public DNS names that are automatically generated by the VPC endpoint service.</p>
    /// <p>To use a private hosted zone, you must set the following VPC attributes to <code>true</code>: <code>enableDnsHostnames</code> and <code>enableDnsSupport</code>. Use <code>ModifyVpcAttribute</code> to set the VPC attributes.</p>
    /// <p>Default: <code>true</code> </p>
    pub fn get_private_dns_enabled(&self) -> &::std::option::Option<bool> {
        &self.private_dns_enabled
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to associate with the endpoint.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to associate with the endpoint.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to associate with the endpoint.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
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
    /// Consumes the builder and constructs a [`CreateVpcEndpointInput`](crate::operation::create_vpc_endpoint::CreateVpcEndpointInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_vpc_endpoint::CreateVpcEndpointInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_vpc_endpoint::CreateVpcEndpointInput {
            dry_run: self.dry_run,
            vpc_endpoint_type: self.vpc_endpoint_type,
            vpc_id: self.vpc_id,
            service_name: self.service_name,
            policy_document: self.policy_document,
            route_table_ids: self.route_table_ids,
            subnet_ids: self.subnet_ids,
            security_group_ids: self.security_group_ids,
            ip_address_type: self.ip_address_type,
            dns_options: self.dns_options,
            client_token: self.client_token,
            private_dns_enabled: self.private_dns_enabled,
            tag_specifications: self.tag_specifications,
            subnet_configurations: self.subnet_configurations,
        })
    }
}