// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the options for a transit gateway.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransitGatewayRequestOptions {
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs. The default is <code>64512</code>.</p>
    pub amazon_side_asn: ::std::option::Option<i64>,
    /// <p>Enable or disable automatic acceptance of attachment requests. Disabled by default.</p>
    pub auto_accept_shared_attachments: ::std::option::Option<crate::types::AutoAcceptSharedAttachmentsValue>,
    /// <p>Enable or disable automatic association with the default association route table. Enabled by default.</p>
    pub default_route_table_association: ::std::option::Option<crate::types::DefaultRouteTableAssociationValue>,
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table. Enabled by default.</p>
    pub default_route_table_propagation: ::std::option::Option<crate::types::DefaultRouteTablePropagationValue>,
    /// <p>Enable or disable Equal Cost Multipath Protocol support. Enabled by default.</p>
    pub vpn_ecmp_support: ::std::option::Option<crate::types::VpnEcmpSupportValue>,
    /// <p>Enable or disable DNS support. Enabled by default.</p>
    pub dns_support: ::std::option::Option<crate::types::DnsSupportValue>,
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway (TGW). Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>For important information about this feature, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-transit-gateways.html#create-tgw">Create a transit gateway</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    pub security_group_referencing_support: ::std::option::Option<crate::types::SecurityGroupReferencingSupportValue>,
    /// <p>Indicates whether multicast is enabled on the transit gateway</p>
    pub multicast_support: ::std::option::Option<crate::types::MulticastSupportValue>,
    /// <p>One or more IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    pub transit_gateway_cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl TransitGatewayRequestOptions {
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs. The default is <code>64512</code>.</p>
    pub fn amazon_side_asn(&self) -> ::std::option::Option<i64> {
        self.amazon_side_asn
    }
    /// <p>Enable or disable automatic acceptance of attachment requests. Disabled by default.</p>
    pub fn auto_accept_shared_attachments(&self) -> ::std::option::Option<&crate::types::AutoAcceptSharedAttachmentsValue> {
        self.auto_accept_shared_attachments.as_ref()
    }
    /// <p>Enable or disable automatic association with the default association route table. Enabled by default.</p>
    pub fn default_route_table_association(&self) -> ::std::option::Option<&crate::types::DefaultRouteTableAssociationValue> {
        self.default_route_table_association.as_ref()
    }
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table. Enabled by default.</p>
    pub fn default_route_table_propagation(&self) -> ::std::option::Option<&crate::types::DefaultRouteTablePropagationValue> {
        self.default_route_table_propagation.as_ref()
    }
    /// <p>Enable or disable Equal Cost Multipath Protocol support. Enabled by default.</p>
    pub fn vpn_ecmp_support(&self) -> ::std::option::Option<&crate::types::VpnEcmpSupportValue> {
        self.vpn_ecmp_support.as_ref()
    }
    /// <p>Enable or disable DNS support. Enabled by default.</p>
    pub fn dns_support(&self) -> ::std::option::Option<&crate::types::DnsSupportValue> {
        self.dns_support.as_ref()
    }
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway (TGW). Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>For important information about this feature, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-transit-gateways.html#create-tgw">Create a transit gateway</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    pub fn security_group_referencing_support(&self) -> ::std::option::Option<&crate::types::SecurityGroupReferencingSupportValue> {
        self.security_group_referencing_support.as_ref()
    }
    /// <p>Indicates whether multicast is enabled on the transit gateway</p>
    pub fn multicast_support(&self) -> ::std::option::Option<&crate::types::MulticastSupportValue> {
        self.multicast_support.as_ref()
    }
    /// <p>One or more IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.transit_gateway_cidr_blocks.is_none()`.
    pub fn transit_gateway_cidr_blocks(&self) -> &[::std::string::String] {
        self.transit_gateway_cidr_blocks.as_deref().unwrap_or_default()
    }
}
impl TransitGatewayRequestOptions {
    /// Creates a new builder-style object to manufacture [`TransitGatewayRequestOptions`](crate::types::TransitGatewayRequestOptions).
    pub fn builder() -> crate::types::builders::TransitGatewayRequestOptionsBuilder {
        crate::types::builders::TransitGatewayRequestOptionsBuilder::default()
    }
}

/// A builder for [`TransitGatewayRequestOptions`](crate::types::TransitGatewayRequestOptions).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TransitGatewayRequestOptionsBuilder {
    pub(crate) amazon_side_asn: ::std::option::Option<i64>,
    pub(crate) auto_accept_shared_attachments: ::std::option::Option<crate::types::AutoAcceptSharedAttachmentsValue>,
    pub(crate) default_route_table_association: ::std::option::Option<crate::types::DefaultRouteTableAssociationValue>,
    pub(crate) default_route_table_propagation: ::std::option::Option<crate::types::DefaultRouteTablePropagationValue>,
    pub(crate) vpn_ecmp_support: ::std::option::Option<crate::types::VpnEcmpSupportValue>,
    pub(crate) dns_support: ::std::option::Option<crate::types::DnsSupportValue>,
    pub(crate) security_group_referencing_support: ::std::option::Option<crate::types::SecurityGroupReferencingSupportValue>,
    pub(crate) multicast_support: ::std::option::Option<crate::types::MulticastSupportValue>,
    pub(crate) transit_gateway_cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl TransitGatewayRequestOptionsBuilder {
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs. The default is <code>64512</code>.</p>
    pub fn amazon_side_asn(mut self, input: i64) -> Self {
        self.amazon_side_asn = ::std::option::Option::Some(input);
        self
    }
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs. The default is <code>64512</code>.</p>
    pub fn set_amazon_side_asn(mut self, input: ::std::option::Option<i64>) -> Self {
        self.amazon_side_asn = input;
        self
    }
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs. The default is <code>64512</code>.</p>
    pub fn get_amazon_side_asn(&self) -> &::std::option::Option<i64> {
        &self.amazon_side_asn
    }
    /// <p>Enable or disable automatic acceptance of attachment requests. Disabled by default.</p>
    pub fn auto_accept_shared_attachments(mut self, input: crate::types::AutoAcceptSharedAttachmentsValue) -> Self {
        self.auto_accept_shared_attachments = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable automatic acceptance of attachment requests. Disabled by default.</p>
    pub fn set_auto_accept_shared_attachments(mut self, input: ::std::option::Option<crate::types::AutoAcceptSharedAttachmentsValue>) -> Self {
        self.auto_accept_shared_attachments = input;
        self
    }
    /// <p>Enable or disable automatic acceptance of attachment requests. Disabled by default.</p>
    pub fn get_auto_accept_shared_attachments(&self) -> &::std::option::Option<crate::types::AutoAcceptSharedAttachmentsValue> {
        &self.auto_accept_shared_attachments
    }
    /// <p>Enable or disable automatic association with the default association route table. Enabled by default.</p>
    pub fn default_route_table_association(mut self, input: crate::types::DefaultRouteTableAssociationValue) -> Self {
        self.default_route_table_association = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable automatic association with the default association route table. Enabled by default.</p>
    pub fn set_default_route_table_association(mut self, input: ::std::option::Option<crate::types::DefaultRouteTableAssociationValue>) -> Self {
        self.default_route_table_association = input;
        self
    }
    /// <p>Enable or disable automatic association with the default association route table. Enabled by default.</p>
    pub fn get_default_route_table_association(&self) -> &::std::option::Option<crate::types::DefaultRouteTableAssociationValue> {
        &self.default_route_table_association
    }
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table. Enabled by default.</p>
    pub fn default_route_table_propagation(mut self, input: crate::types::DefaultRouteTablePropagationValue) -> Self {
        self.default_route_table_propagation = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table. Enabled by default.</p>
    pub fn set_default_route_table_propagation(mut self, input: ::std::option::Option<crate::types::DefaultRouteTablePropagationValue>) -> Self {
        self.default_route_table_propagation = input;
        self
    }
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table. Enabled by default.</p>
    pub fn get_default_route_table_propagation(&self) -> &::std::option::Option<crate::types::DefaultRouteTablePropagationValue> {
        &self.default_route_table_propagation
    }
    /// <p>Enable or disable Equal Cost Multipath Protocol support. Enabled by default.</p>
    pub fn vpn_ecmp_support(mut self, input: crate::types::VpnEcmpSupportValue) -> Self {
        self.vpn_ecmp_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable Equal Cost Multipath Protocol support. Enabled by default.</p>
    pub fn set_vpn_ecmp_support(mut self, input: ::std::option::Option<crate::types::VpnEcmpSupportValue>) -> Self {
        self.vpn_ecmp_support = input;
        self
    }
    /// <p>Enable or disable Equal Cost Multipath Protocol support. Enabled by default.</p>
    pub fn get_vpn_ecmp_support(&self) -> &::std::option::Option<crate::types::VpnEcmpSupportValue> {
        &self.vpn_ecmp_support
    }
    /// <p>Enable or disable DNS support. Enabled by default.</p>
    pub fn dns_support(mut self, input: crate::types::DnsSupportValue) -> Self {
        self.dns_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable DNS support. Enabled by default.</p>
    pub fn set_dns_support(mut self, input: ::std::option::Option<crate::types::DnsSupportValue>) -> Self {
        self.dns_support = input;
        self
    }
    /// <p>Enable or disable DNS support. Enabled by default.</p>
    pub fn get_dns_support(&self) -> &::std::option::Option<crate::types::DnsSupportValue> {
        &self.dns_support
    }
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway (TGW). Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>For important information about this feature, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-transit-gateways.html#create-tgw">Create a transit gateway</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    pub fn security_group_referencing_support(mut self, input: crate::types::SecurityGroupReferencingSupportValue) -> Self {
        self.security_group_referencing_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway (TGW). Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>For important information about this feature, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-transit-gateways.html#create-tgw">Create a transit gateway</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    pub fn set_security_group_referencing_support(
        mut self,
        input: ::std::option::Option<crate::types::SecurityGroupReferencingSupportValue>,
    ) -> Self {
        self.security_group_referencing_support = input;
        self
    }
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway (TGW). Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>For important information about this feature, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-transit-gateways.html#create-tgw">Create a transit gateway</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    pub fn get_security_group_referencing_support(&self) -> &::std::option::Option<crate::types::SecurityGroupReferencingSupportValue> {
        &self.security_group_referencing_support
    }
    /// <p>Indicates whether multicast is enabled on the transit gateway</p>
    pub fn multicast_support(mut self, input: crate::types::MulticastSupportValue) -> Self {
        self.multicast_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether multicast is enabled on the transit gateway</p>
    pub fn set_multicast_support(mut self, input: ::std::option::Option<crate::types::MulticastSupportValue>) -> Self {
        self.multicast_support = input;
        self
    }
    /// <p>Indicates whether multicast is enabled on the transit gateway</p>
    pub fn get_multicast_support(&self) -> &::std::option::Option<crate::types::MulticastSupportValue> {
        &self.multicast_support
    }
    /// Appends an item to `transit_gateway_cidr_blocks`.
    ///
    /// To override the contents of this collection use [`set_transit_gateway_cidr_blocks`](Self::set_transit_gateway_cidr_blocks).
    ///
    /// <p>One or more IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    pub fn transit_gateway_cidr_blocks(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.transit_gateway_cidr_blocks.unwrap_or_default();
        v.push(input.into());
        self.transit_gateway_cidr_blocks = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    pub fn set_transit_gateway_cidr_blocks(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.transit_gateway_cidr_blocks = input;
        self
    }
    /// <p>One or more IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    pub fn get_transit_gateway_cidr_blocks(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.transit_gateway_cidr_blocks
    }
    /// Consumes the builder and constructs a [`TransitGatewayRequestOptions`](crate::types::TransitGatewayRequestOptions).
    pub fn build(self) -> crate::types::TransitGatewayRequestOptions {
        crate::types::TransitGatewayRequestOptions {
            amazon_side_asn: self.amazon_side_asn,
            auto_accept_shared_attachments: self.auto_accept_shared_attachments,
            default_route_table_association: self.default_route_table_association,
            default_route_table_propagation: self.default_route_table_propagation,
            vpn_ecmp_support: self.vpn_ecmp_support,
            dns_support: self.dns_support,
            security_group_referencing_support: self.security_group_referencing_support,
            multicast_support: self.multicast_support,
            transit_gateway_cidr_blocks: self.transit_gateway_cidr_blocks,
        }
    }
}
