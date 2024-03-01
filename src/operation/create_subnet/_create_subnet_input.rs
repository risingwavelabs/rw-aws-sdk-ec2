// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateSubnetInput {
    /// <p>The tags to assign to the subnet.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>The Availability Zone or Local Zone for the subnet.</p>
    /// <p>Default: Amazon Web Services selects one for you. If you create more than one subnet in your VPC, we do not necessarily select a different zone for each subnet.</p>
    /// <p>To create a subnet in a Local Zone, set this value to the Local Zone ID, for example <code>us-west-2-lax-1a</code>. For information about the Regions that support Local Zones, see <a href="http://aws.amazon.com/about-aws/global-infrastructure/localzones/locations/">Local Zones locations</a>.</p>
    /// <p>To create a subnet in an Outpost, set this value to the Availability Zone for the Outpost and specify the Outpost ARN.</p>
    pub availability_zone: ::std::option::Option<::std::string::String>,
    /// <p>The AZ ID or the Local Zone ID of the subnet.</p>
    pub availability_zone_id: ::std::option::Option<::std::string::String>,
    /// <p>The IPv4 network range for the subnet, in CIDR notation. For example, <code>10.0.0.0/24</code>. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    /// <p>This parameter is not supported for an IPv6 only subnet.</p>
    pub cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>The IPv6 network range for the subnet, in CIDR notation. This parameter is required for an IPv6 only subnet.</p>
    pub ipv6_cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify an Outpost ARN, you must also specify the Availability Zone of the Outpost subnet.</p>
    pub outpost_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the VPC.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Indicates whether to create an IPv6 only subnet.</p>
    pub ipv6_native: ::std::option::Option<bool>,
    /// <p>An IPv4 IPAM pool ID for the subnet.</p>
    pub ipv4_ipam_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>An IPv4 netmask length for the subnet.</p>
    pub ipv4_netmask_length: ::std::option::Option<i32>,
    /// <p>An IPv6 IPAM pool ID for the subnet.</p>
    pub ipv6_ipam_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>An IPv6 netmask length for the subnet.</p>
    pub ipv6_netmask_length: ::std::option::Option<i32>,
}
impl CreateSubnetInput {
    /// <p>The tags to assign to the subnet.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>The Availability Zone or Local Zone for the subnet.</p>
    /// <p>Default: Amazon Web Services selects one for you. If you create more than one subnet in your VPC, we do not necessarily select a different zone for each subnet.</p>
    /// <p>To create a subnet in a Local Zone, set this value to the Local Zone ID, for example <code>us-west-2-lax-1a</code>. For information about the Regions that support Local Zones, see <a href="http://aws.amazon.com/about-aws/global-infrastructure/localzones/locations/">Local Zones locations</a>.</p>
    /// <p>To create a subnet in an Outpost, set this value to the Availability Zone for the Outpost and specify the Outpost ARN.</p>
    pub fn availability_zone(&self) -> ::std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>The AZ ID or the Local Zone ID of the subnet.</p>
    pub fn availability_zone_id(&self) -> ::std::option::Option<&str> {
        self.availability_zone_id.as_deref()
    }
    /// <p>The IPv4 network range for the subnet, in CIDR notation. For example, <code>10.0.0.0/24</code>. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    /// <p>This parameter is not supported for an IPv6 only subnet.</p>
    pub fn cidr_block(&self) -> ::std::option::Option<&str> {
        self.cidr_block.as_deref()
    }
    /// <p>The IPv6 network range for the subnet, in CIDR notation. This parameter is required for an IPv6 only subnet.</p>
    pub fn ipv6_cidr_block(&self) -> ::std::option::Option<&str> {
        self.ipv6_cidr_block.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify an Outpost ARN, you must also specify the Availability Zone of the Outpost subnet.</p>
    pub fn outpost_arn(&self) -> ::std::option::Option<&str> {
        self.outpost_arn.as_deref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Indicates whether to create an IPv6 only subnet.</p>
    pub fn ipv6_native(&self) -> ::std::option::Option<bool> {
        self.ipv6_native
    }
    /// <p>An IPv4 IPAM pool ID for the subnet.</p>
    pub fn ipv4_ipam_pool_id(&self) -> ::std::option::Option<&str> {
        self.ipv4_ipam_pool_id.as_deref()
    }
    /// <p>An IPv4 netmask length for the subnet.</p>
    pub fn ipv4_netmask_length(&self) -> ::std::option::Option<i32> {
        self.ipv4_netmask_length
    }
    /// <p>An IPv6 IPAM pool ID for the subnet.</p>
    pub fn ipv6_ipam_pool_id(&self) -> ::std::option::Option<&str> {
        self.ipv6_ipam_pool_id.as_deref()
    }
    /// <p>An IPv6 netmask length for the subnet.</p>
    pub fn ipv6_netmask_length(&self) -> ::std::option::Option<i32> {
        self.ipv6_netmask_length
    }
}
impl CreateSubnetInput {
    /// Creates a new builder-style object to manufacture [`CreateSubnetInput`](crate::operation::create_subnet::CreateSubnetInput).
    pub fn builder() -> crate::operation::create_subnet::builders::CreateSubnetInputBuilder {
        crate::operation::create_subnet::builders::CreateSubnetInputBuilder::default()
    }
}

/// A builder for [`CreateSubnetInput`](crate::operation::create_subnet::CreateSubnetInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateSubnetInputBuilder {
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) availability_zone: ::std::option::Option<::std::string::String>,
    pub(crate) availability_zone_id: ::std::option::Option<::std::string::String>,
    pub(crate) cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) ipv6_cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) outpost_arn: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) ipv6_native: ::std::option::Option<bool>,
    pub(crate) ipv4_ipam_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) ipv4_netmask_length: ::std::option::Option<i32>,
    pub(crate) ipv6_ipam_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) ipv6_netmask_length: ::std::option::Option<i32>,
}
impl CreateSubnetInputBuilder {
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the subnet.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to assign to the subnet.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to assign to the subnet.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
    /// <p>The Availability Zone or Local Zone for the subnet.</p>
    /// <p>Default: Amazon Web Services selects one for you. If you create more than one subnet in your VPC, we do not necessarily select a different zone for each subnet.</p>
    /// <p>To create a subnet in a Local Zone, set this value to the Local Zone ID, for example <code>us-west-2-lax-1a</code>. For information about the Regions that support Local Zones, see <a href="http://aws.amazon.com/about-aws/global-infrastructure/localzones/locations/">Local Zones locations</a>.</p>
    /// <p>To create a subnet in an Outpost, set this value to the Availability Zone for the Outpost and specify the Outpost ARN.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.availability_zone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Availability Zone or Local Zone for the subnet.</p>
    /// <p>Default: Amazon Web Services selects one for you. If you create more than one subnet in your VPC, we do not necessarily select a different zone for each subnet.</p>
    /// <p>To create a subnet in a Local Zone, set this value to the Local Zone ID, for example <code>us-west-2-lax-1a</code>. For information about the Regions that support Local Zones, see <a href="http://aws.amazon.com/about-aws/global-infrastructure/localzones/locations/">Local Zones locations</a>.</p>
    /// <p>To create a subnet in an Outpost, set this value to the Availability Zone for the Outpost and specify the Outpost ARN.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>The Availability Zone or Local Zone for the subnet.</p>
    /// <p>Default: Amazon Web Services selects one for you. If you create more than one subnet in your VPC, we do not necessarily select a different zone for each subnet.</p>
    /// <p>To create a subnet in a Local Zone, set this value to the Local Zone ID, for example <code>us-west-2-lax-1a</code>. For information about the Regions that support Local Zones, see <a href="http://aws.amazon.com/about-aws/global-infrastructure/localzones/locations/">Local Zones locations</a>.</p>
    /// <p>To create a subnet in an Outpost, set this value to the Availability Zone for the Outpost and specify the Outpost ARN.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        &self.availability_zone
    }
    /// <p>The AZ ID or the Local Zone ID of the subnet.</p>
    pub fn availability_zone_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.availability_zone_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The AZ ID or the Local Zone ID of the subnet.</p>
    pub fn set_availability_zone_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.availability_zone_id = input;
        self
    }
    /// <p>The AZ ID or the Local Zone ID of the subnet.</p>
    pub fn get_availability_zone_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.availability_zone_id
    }
    /// <p>The IPv4 network range for the subnet, in CIDR notation. For example, <code>10.0.0.0/24</code>. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    /// <p>This parameter is not supported for an IPv6 only subnet.</p>
    pub fn cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 network range for the subnet, in CIDR notation. For example, <code>10.0.0.0/24</code>. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    /// <p>This parameter is not supported for an IPv6 only subnet.</p>
    pub fn set_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr_block = input;
        self
    }
    /// <p>The IPv4 network range for the subnet, in CIDR notation. For example, <code>10.0.0.0/24</code>. We modify the specified CIDR block to its canonical form; for example, if you specify <code>100.68.0.18/18</code>, we modify it to <code>100.68.0.0/18</code>.</p>
    /// <p>This parameter is not supported for an IPv6 only subnet.</p>
    pub fn get_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.cidr_block
    }
    /// <p>The IPv6 network range for the subnet, in CIDR notation. This parameter is required for an IPv6 only subnet.</p>
    pub fn ipv6_cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv6_cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv6 network range for the subnet, in CIDR notation. This parameter is required for an IPv6 only subnet.</p>
    pub fn set_ipv6_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv6_cidr_block = input;
        self
    }
    /// <p>The IPv6 network range for the subnet, in CIDR notation. This parameter is required for an IPv6 only subnet.</p>
    pub fn get_ipv6_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipv6_cidr_block
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify an Outpost ARN, you must also specify the Availability Zone of the Outpost subnet.</p>
    pub fn outpost_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.outpost_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify an Outpost ARN, you must also specify the Availability Zone of the Outpost subnet.</p>
    pub fn set_outpost_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.outpost_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost. If you specify an Outpost ARN, you must also specify the Availability Zone of the Outpost subnet.</p>
    pub fn get_outpost_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.outpost_arn
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
    /// <p>Indicates whether to create an IPv6 only subnet.</p>
    pub fn ipv6_native(mut self, input: bool) -> Self {
        self.ipv6_native = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to create an IPv6 only subnet.</p>
    pub fn set_ipv6_native(mut self, input: ::std::option::Option<bool>) -> Self {
        self.ipv6_native = input;
        self
    }
    /// <p>Indicates whether to create an IPv6 only subnet.</p>
    pub fn get_ipv6_native(&self) -> &::std::option::Option<bool> {
        &self.ipv6_native
    }
    /// <p>An IPv4 IPAM pool ID for the subnet.</p>
    pub fn ipv4_ipam_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv4_ipam_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An IPv4 IPAM pool ID for the subnet.</p>
    pub fn set_ipv4_ipam_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv4_ipam_pool_id = input;
        self
    }
    /// <p>An IPv4 IPAM pool ID for the subnet.</p>
    pub fn get_ipv4_ipam_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipv4_ipam_pool_id
    }
    /// <p>An IPv4 netmask length for the subnet.</p>
    pub fn ipv4_netmask_length(mut self, input: i32) -> Self {
        self.ipv4_netmask_length = ::std::option::Option::Some(input);
        self
    }
    /// <p>An IPv4 netmask length for the subnet.</p>
    pub fn set_ipv4_netmask_length(mut self, input: ::std::option::Option<i32>) -> Self {
        self.ipv4_netmask_length = input;
        self
    }
    /// <p>An IPv4 netmask length for the subnet.</p>
    pub fn get_ipv4_netmask_length(&self) -> &::std::option::Option<i32> {
        &self.ipv4_netmask_length
    }
    /// <p>An IPv6 IPAM pool ID for the subnet.</p>
    pub fn ipv6_ipam_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv6_ipam_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An IPv6 IPAM pool ID for the subnet.</p>
    pub fn set_ipv6_ipam_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv6_ipam_pool_id = input;
        self
    }
    /// <p>An IPv6 IPAM pool ID for the subnet.</p>
    pub fn get_ipv6_ipam_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipv6_ipam_pool_id
    }
    /// <p>An IPv6 netmask length for the subnet.</p>
    pub fn ipv6_netmask_length(mut self, input: i32) -> Self {
        self.ipv6_netmask_length = ::std::option::Option::Some(input);
        self
    }
    /// <p>An IPv6 netmask length for the subnet.</p>
    pub fn set_ipv6_netmask_length(mut self, input: ::std::option::Option<i32>) -> Self {
        self.ipv6_netmask_length = input;
        self
    }
    /// <p>An IPv6 netmask length for the subnet.</p>
    pub fn get_ipv6_netmask_length(&self) -> &::std::option::Option<i32> {
        &self.ipv6_netmask_length
    }
    /// Consumes the builder and constructs a [`CreateSubnetInput`](crate::operation::create_subnet::CreateSubnetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_subnet::CreateSubnetInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_subnet::CreateSubnetInput {
            tag_specifications: self.tag_specifications,
            availability_zone: self.availability_zone,
            availability_zone_id: self.availability_zone_id,
            cidr_block: self.cidr_block,
            ipv6_cidr_block: self.ipv6_cidr_block,
            outpost_arn: self.outpost_arn,
            vpc_id: self.vpc_id,
            dry_run: self.dry_run,
            ipv6_native: self.ipv6_native,
            ipv4_ipam_pool_id: self.ipv4_ipam_pool_id,
            ipv4_netmask_length: self.ipv4_netmask_length,
            ipv6_ipam_pool_id: self.ipv6_ipam_pool_id,
            ipv6_netmask_length: self.ipv6_netmask_length,
        })
    }
}