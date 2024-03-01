// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSubnetsInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for the subnet. You can also use <code>availabilityZone</code> as the filter name.</p> </li>
    /// <li> <p> <code>availability-zone-id</code> - The ID of the Availability Zone for the subnet. You can also use <code>availabilityZoneId</code> as the filter name.</p> </li>
    /// <li> <p> <code>available-ip-address-count</code> - The number of IPv4 addresses in the subnet that are available.</p> </li>
    /// <li> <p> <code>cidr-block</code> - The IPv4 CIDR block of the subnet. The CIDR block you specify must exactly match the subnet's CIDR block for information to be returned for the subnet. You can also use <code>cidr</code> or <code>cidrBlock</code> as the filter names.</p> </li>
    /// <li> <p> <code>customer-owned-ipv4-pool</code> - The customer-owned IPv4 address pool associated with the subnet.</p> </li>
    /// <li> <p> <code>default-for-az</code> - Indicates whether this is the default subnet for the Availability Zone (<code>true</code> | <code>false</code>). You can also use <code>defaultForAz</code> as the filter name.</p> </li>
    /// <li> <p> <code>enable-dns64</code> - Indicates whether DNS queries made to the Amazon-provided DNS Resolver in this subnet should return synthetic IPv6 addresses for IPv4-only destinations.</p> </li>
    /// <li> <p> <code>enable-lni-at-device-index</code> - Indicates the device position for local network interfaces in this subnet. For example, <code>1</code> indicates local network interfaces in this subnet are the secondary network interface (eth1). </p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.ipv6-cidr-block</code> - An IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.association-id</code> - An association ID for an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.state</code> - The state of an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-native</code> - Indicates whether this is an IPv6 only subnet (<code>true</code> | <code>false</code>).</p> </li>
    /// <li> <p> <code>map-customer-owned-ip-on-launch</code> - Indicates whether a network interface created in this subnet (including a network interface created by <code>RunInstances</code>) receives a customer-owned IPv4 address.</p> </li>
    /// <li> <p> <code>map-public-ip-on-launch</code> - Indicates whether instances launched in this subnet receive a public IPv4 address.</p> </li>
    /// <li> <p> <code>outpost-arn</code> - The Amazon Resource Name (ARN) of the Outpost.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the subnet.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.hostname-type</code> - The type of hostname to assign to instances in the subnet at launch. For IPv4-only and dual-stack (IPv4 and IPv6) subnets, an instance DNS name can be based on the instance IPv4 address (ip-name) or the instance ID (resource-name). For IPv6 only subnets, an instance DNS name must be based on the instance ID (resource-name).</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-a-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-aaaa-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p> </li>
    /// <li> <p> <code>state</code> - The state of the subnet (<code>pending</code> | <code>available</code>).</p> </li>
    /// <li> <p> <code>subnet-arn</code> - The Amazon Resource Name (ARN) of the subnet.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// <li> <p> <code>vpc-id</code> - The ID of the VPC for the subnet.</p> </li>
    /// </ul>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The IDs of the subnets.</p>
    /// <p>Default: Describes all your subnets.</p>
    pub subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub max_results: ::std::option::Option<i32>,
}
impl DescribeSubnetsInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for the subnet. You can also use <code>availabilityZone</code> as the filter name.</p> </li>
    /// <li> <p> <code>availability-zone-id</code> - The ID of the Availability Zone for the subnet. You can also use <code>availabilityZoneId</code> as the filter name.</p> </li>
    /// <li> <p> <code>available-ip-address-count</code> - The number of IPv4 addresses in the subnet that are available.</p> </li>
    /// <li> <p> <code>cidr-block</code> - The IPv4 CIDR block of the subnet. The CIDR block you specify must exactly match the subnet's CIDR block for information to be returned for the subnet. You can also use <code>cidr</code> or <code>cidrBlock</code> as the filter names.</p> </li>
    /// <li> <p> <code>customer-owned-ipv4-pool</code> - The customer-owned IPv4 address pool associated with the subnet.</p> </li>
    /// <li> <p> <code>default-for-az</code> - Indicates whether this is the default subnet for the Availability Zone (<code>true</code> | <code>false</code>). You can also use <code>defaultForAz</code> as the filter name.</p> </li>
    /// <li> <p> <code>enable-dns64</code> - Indicates whether DNS queries made to the Amazon-provided DNS Resolver in this subnet should return synthetic IPv6 addresses for IPv4-only destinations.</p> </li>
    /// <li> <p> <code>enable-lni-at-device-index</code> - Indicates the device position for local network interfaces in this subnet. For example, <code>1</code> indicates local network interfaces in this subnet are the secondary network interface (eth1). </p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.ipv6-cidr-block</code> - An IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.association-id</code> - An association ID for an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.state</code> - The state of an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-native</code> - Indicates whether this is an IPv6 only subnet (<code>true</code> | <code>false</code>).</p> </li>
    /// <li> <p> <code>map-customer-owned-ip-on-launch</code> - Indicates whether a network interface created in this subnet (including a network interface created by <code>RunInstances</code>) receives a customer-owned IPv4 address.</p> </li>
    /// <li> <p> <code>map-public-ip-on-launch</code> - Indicates whether instances launched in this subnet receive a public IPv4 address.</p> </li>
    /// <li> <p> <code>outpost-arn</code> - The Amazon Resource Name (ARN) of the Outpost.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the subnet.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.hostname-type</code> - The type of hostname to assign to instances in the subnet at launch. For IPv4-only and dual-stack (IPv4 and IPv6) subnets, an instance DNS name can be based on the instance IPv4 address (ip-name) or the instance ID (resource-name). For IPv6 only subnets, an instance DNS name must be based on the instance ID (resource-name).</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-a-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-aaaa-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p> </li>
    /// <li> <p> <code>state</code> - The state of the subnet (<code>pending</code> | <code>available</code>).</p> </li>
    /// <li> <p> <code>subnet-arn</code> - The Amazon Resource Name (ARN) of the subnet.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// <li> <p> <code>vpc-id</code> - The ID of the VPC for the subnet.</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>The IDs of the subnets.</p>
    /// <p>Default: Describes all your subnets.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subnet_ids.is_none()`.
    pub fn subnet_ids(&self) -> &[::std::string::String] {
        self.subnet_ids.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
}
impl DescribeSubnetsInput {
    /// Creates a new builder-style object to manufacture [`DescribeSubnetsInput`](crate::operation::describe_subnets::DescribeSubnetsInput).
    pub fn builder() -> crate::operation::describe_subnets::builders::DescribeSubnetsInputBuilder {
        crate::operation::describe_subnets::builders::DescribeSubnetsInputBuilder::default()
    }
}

/// A builder for [`DescribeSubnetsInput`](crate::operation::describe_subnets::DescribeSubnetsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeSubnetsInputBuilder {
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) max_results: ::std::option::Option<i32>,
}
impl DescribeSubnetsInputBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for the subnet. You can also use <code>availabilityZone</code> as the filter name.</p> </li>
    /// <li> <p> <code>availability-zone-id</code> - The ID of the Availability Zone for the subnet. You can also use <code>availabilityZoneId</code> as the filter name.</p> </li>
    /// <li> <p> <code>available-ip-address-count</code> - The number of IPv4 addresses in the subnet that are available.</p> </li>
    /// <li> <p> <code>cidr-block</code> - The IPv4 CIDR block of the subnet. The CIDR block you specify must exactly match the subnet's CIDR block for information to be returned for the subnet. You can also use <code>cidr</code> or <code>cidrBlock</code> as the filter names.</p> </li>
    /// <li> <p> <code>customer-owned-ipv4-pool</code> - The customer-owned IPv4 address pool associated with the subnet.</p> </li>
    /// <li> <p> <code>default-for-az</code> - Indicates whether this is the default subnet for the Availability Zone (<code>true</code> | <code>false</code>). You can also use <code>defaultForAz</code> as the filter name.</p> </li>
    /// <li> <p> <code>enable-dns64</code> - Indicates whether DNS queries made to the Amazon-provided DNS Resolver in this subnet should return synthetic IPv6 addresses for IPv4-only destinations.</p> </li>
    /// <li> <p> <code>enable-lni-at-device-index</code> - Indicates the device position for local network interfaces in this subnet. For example, <code>1</code> indicates local network interfaces in this subnet are the secondary network interface (eth1). </p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.ipv6-cidr-block</code> - An IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.association-id</code> - An association ID for an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.state</code> - The state of an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-native</code> - Indicates whether this is an IPv6 only subnet (<code>true</code> | <code>false</code>).</p> </li>
    /// <li> <p> <code>map-customer-owned-ip-on-launch</code> - Indicates whether a network interface created in this subnet (including a network interface created by <code>RunInstances</code>) receives a customer-owned IPv4 address.</p> </li>
    /// <li> <p> <code>map-public-ip-on-launch</code> - Indicates whether instances launched in this subnet receive a public IPv4 address.</p> </li>
    /// <li> <p> <code>outpost-arn</code> - The Amazon Resource Name (ARN) of the Outpost.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the subnet.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.hostname-type</code> - The type of hostname to assign to instances in the subnet at launch. For IPv4-only and dual-stack (IPv4 and IPv6) subnets, an instance DNS name can be based on the instance IPv4 address (ip-name) or the instance ID (resource-name). For IPv6 only subnets, an instance DNS name must be based on the instance ID (resource-name).</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-a-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-aaaa-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p> </li>
    /// <li> <p> <code>state</code> - The state of the subnet (<code>pending</code> | <code>available</code>).</p> </li>
    /// <li> <p> <code>subnet-arn</code> - The Amazon Resource Name (ARN) of the subnet.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// <li> <p> <code>vpc-id</code> - The ID of the VPC for the subnet.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for the subnet. You can also use <code>availabilityZone</code> as the filter name.</p> </li>
    /// <li> <p> <code>availability-zone-id</code> - The ID of the Availability Zone for the subnet. You can also use <code>availabilityZoneId</code> as the filter name.</p> </li>
    /// <li> <p> <code>available-ip-address-count</code> - The number of IPv4 addresses in the subnet that are available.</p> </li>
    /// <li> <p> <code>cidr-block</code> - The IPv4 CIDR block of the subnet. The CIDR block you specify must exactly match the subnet's CIDR block for information to be returned for the subnet. You can also use <code>cidr</code> or <code>cidrBlock</code> as the filter names.</p> </li>
    /// <li> <p> <code>customer-owned-ipv4-pool</code> - The customer-owned IPv4 address pool associated with the subnet.</p> </li>
    /// <li> <p> <code>default-for-az</code> - Indicates whether this is the default subnet for the Availability Zone (<code>true</code> | <code>false</code>). You can also use <code>defaultForAz</code> as the filter name.</p> </li>
    /// <li> <p> <code>enable-dns64</code> - Indicates whether DNS queries made to the Amazon-provided DNS Resolver in this subnet should return synthetic IPv6 addresses for IPv4-only destinations.</p> </li>
    /// <li> <p> <code>enable-lni-at-device-index</code> - Indicates the device position for local network interfaces in this subnet. For example, <code>1</code> indicates local network interfaces in this subnet are the secondary network interface (eth1). </p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.ipv6-cidr-block</code> - An IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.association-id</code> - An association ID for an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.state</code> - The state of an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-native</code> - Indicates whether this is an IPv6 only subnet (<code>true</code> | <code>false</code>).</p> </li>
    /// <li> <p> <code>map-customer-owned-ip-on-launch</code> - Indicates whether a network interface created in this subnet (including a network interface created by <code>RunInstances</code>) receives a customer-owned IPv4 address.</p> </li>
    /// <li> <p> <code>map-public-ip-on-launch</code> - Indicates whether instances launched in this subnet receive a public IPv4 address.</p> </li>
    /// <li> <p> <code>outpost-arn</code> - The Amazon Resource Name (ARN) of the Outpost.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the subnet.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.hostname-type</code> - The type of hostname to assign to instances in the subnet at launch. For IPv4-only and dual-stack (IPv4 and IPv6) subnets, an instance DNS name can be based on the instance IPv4 address (ip-name) or the instance ID (resource-name). For IPv6 only subnets, an instance DNS name must be based on the instance ID (resource-name).</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-a-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-aaaa-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p> </li>
    /// <li> <p> <code>state</code> - The state of the subnet (<code>pending</code> | <code>available</code>).</p> </li>
    /// <li> <p> <code>subnet-arn</code> - The Amazon Resource Name (ARN) of the subnet.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// <li> <p> <code>vpc-id</code> - The ID of the VPC for the subnet.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for the subnet. You can also use <code>availabilityZone</code> as the filter name.</p> </li>
    /// <li> <p> <code>availability-zone-id</code> - The ID of the Availability Zone for the subnet. You can also use <code>availabilityZoneId</code> as the filter name.</p> </li>
    /// <li> <p> <code>available-ip-address-count</code> - The number of IPv4 addresses in the subnet that are available.</p> </li>
    /// <li> <p> <code>cidr-block</code> - The IPv4 CIDR block of the subnet. The CIDR block you specify must exactly match the subnet's CIDR block for information to be returned for the subnet. You can also use <code>cidr</code> or <code>cidrBlock</code> as the filter names.</p> </li>
    /// <li> <p> <code>customer-owned-ipv4-pool</code> - The customer-owned IPv4 address pool associated with the subnet.</p> </li>
    /// <li> <p> <code>default-for-az</code> - Indicates whether this is the default subnet for the Availability Zone (<code>true</code> | <code>false</code>). You can also use <code>defaultForAz</code> as the filter name.</p> </li>
    /// <li> <p> <code>enable-dns64</code> - Indicates whether DNS queries made to the Amazon-provided DNS Resolver in this subnet should return synthetic IPv6 addresses for IPv4-only destinations.</p> </li>
    /// <li> <p> <code>enable-lni-at-device-index</code> - Indicates the device position for local network interfaces in this subnet. For example, <code>1</code> indicates local network interfaces in this subnet are the secondary network interface (eth1). </p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.ipv6-cidr-block</code> - An IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.association-id</code> - An association ID for an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-cidr-block-association.state</code> - The state of an IPv6 CIDR block associated with the subnet.</p> </li>
    /// <li> <p> <code>ipv6-native</code> - Indicates whether this is an IPv6 only subnet (<code>true</code> | <code>false</code>).</p> </li>
    /// <li> <p> <code>map-customer-owned-ip-on-launch</code> - Indicates whether a network interface created in this subnet (including a network interface created by <code>RunInstances</code>) receives a customer-owned IPv4 address.</p> </li>
    /// <li> <p> <code>map-public-ip-on-launch</code> - Indicates whether instances launched in this subnet receive a public IPv4 address.</p> </li>
    /// <li> <p> <code>outpost-arn</code> - The Amazon Resource Name (ARN) of the Outpost.</p> </li>
    /// <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the subnet.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.hostname-type</code> - The type of hostname to assign to instances in the subnet at launch. For IPv4-only and dual-stack (IPv4 and IPv6) subnets, an instance DNS name can be based on the instance IPv4 address (ip-name) or the instance ID (resource-name). For IPv6 only subnets, an instance DNS name must be based on the instance ID (resource-name).</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-a-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p> </li>
    /// <li> <p> <code>private-dns-name-options-on-launch.enable-resource-name-dns-aaaa-record</code> - Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p> </li>
    /// <li> <p> <code>state</code> - The state of the subnet (<code>pending</code> | <code>available</code>).</p> </li>
    /// <li> <p> <code>subnet-arn</code> - The Amazon Resource Name (ARN) of the subnet.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>tag</code>:<key>
    /// - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key
    /// <code>Owner</code> and the value
    /// <code>TeamA</code>, specify
    /// <code>tag:Owner</code> for the filter name and
    /// <code>TeamA</code> for the filter value.
    /// </key></p> </li>
    /// <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>
    /// <li> <p> <code>vpc-id</code> - The ID of the VPC for the subnet.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
    }
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>The IDs of the subnets.</p>
    /// <p>Default: Describes all your subnets.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the subnets.</p>
    /// <p>Default: Describes all your subnets.</p>
    pub fn set_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.subnet_ids = input;
        self
    }
    /// <p>The IDs of the subnets.</p>
    /// <p>Default: Describes all your subnets.</p>
    pub fn get_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.subnet_ids
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
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// Consumes the builder and constructs a [`DescribeSubnetsInput`](crate::operation::describe_subnets::DescribeSubnetsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_subnets::DescribeSubnetsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::describe_subnets::DescribeSubnetsInput {
            filters: self.filters,
            subnet_ids: self.subnet_ids,
            dry_run: self.dry_run,
            next_token: self.next_token,
            max_results: self.max_results,
        })
    }
}
