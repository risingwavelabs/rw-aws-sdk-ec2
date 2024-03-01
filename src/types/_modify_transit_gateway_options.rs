// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The transit gateway options.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyTransitGatewayOptions {
    /// <p>Adds IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    pub add_transit_gateway_cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Removes CIDR blocks for the transit gateway.</p>
    pub remove_transit_gateway_cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Enable or disable Equal Cost Multipath Protocol support.</p>
    pub vpn_ecmp_support: ::std::option::Option<crate::types::VpnEcmpSupportValue>,
    /// <p>Enable or disable DNS support.</p>
    pub dns_support: ::std::option::Option<crate::types::DnsSupportValue>,
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway (TGW). Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>For important information about this feature, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-transit-gateways.html#create-tgw">Create a transit gateway</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    pub security_group_referencing_support: ::std::option::Option<crate::types::SecurityGroupReferencingSupportValue>,
    /// <p>Enable or disable automatic acceptance of attachment requests.</p>
    pub auto_accept_shared_attachments: ::std::option::Option<crate::types::AutoAcceptSharedAttachmentsValue>,
    /// <p>Enable or disable automatic association with the default association route table.</p>
    pub default_route_table_association: ::std::option::Option<crate::types::DefaultRouteTableAssociationValue>,
    /// <p>The ID of the default association route table.</p>
    pub association_default_route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table.</p>
    pub default_route_table_propagation: ::std::option::Option<crate::types::DefaultRouteTablePropagationValue>,
    /// <p>The ID of the default propagation route table.</p>
    pub propagation_default_route_table_id: ::std::option::Option<::std::string::String>,
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs.</p>
    /// <p>The modify ASN operation is not allowed on a transit gateway with active BGP sessions. You must first delete all transit gateway attachments that have BGP configured prior to modifying the ASN on the transit gateway.</p>
    pub amazon_side_asn: ::std::option::Option<i64>,
}
impl ModifyTransitGatewayOptions {
    /// <p>Adds IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.add_transit_gateway_cidr_blocks.is_none()`.
    pub fn add_transit_gateway_cidr_blocks(&self) -> &[::std::string::String] {
        self.add_transit_gateway_cidr_blocks.as_deref().unwrap_or_default()
    }
    /// <p>Removes CIDR blocks for the transit gateway.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.remove_transit_gateway_cidr_blocks.is_none()`.
    pub fn remove_transit_gateway_cidr_blocks(&self) -> &[::std::string::String] {
        self.remove_transit_gateway_cidr_blocks.as_deref().unwrap_or_default()
    }
    /// <p>Enable or disable Equal Cost Multipath Protocol support.</p>
    pub fn vpn_ecmp_support(&self) -> ::std::option::Option<&crate::types::VpnEcmpSupportValue> {
        self.vpn_ecmp_support.as_ref()
    }
    /// <p>Enable or disable DNS support.</p>
    pub fn dns_support(&self) -> ::std::option::Option<&crate::types::DnsSupportValue> {
        self.dns_support.as_ref()
    }
    /// <p>Enables you to reference a security group across VPCs attached to a transit gateway (TGW). Use this option to simplify security group management and control of instance-to-instance traffic across VPCs that are connected by transit gateway. You can also use this option to migrate from VPC peering (which was the only option that supported security group referencing) to transit gateways (which now also support security group referencing). This option is disabled by default and there are no additional costs to use this feature.</p>
    /// <p>For important information about this feature, see <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-transit-gateways.html#create-tgw">Create a transit gateway</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    pub fn security_group_referencing_support(&self) -> ::std::option::Option<&crate::types::SecurityGroupReferencingSupportValue> {
        self.security_group_referencing_support.as_ref()
    }
    /// <p>Enable or disable automatic acceptance of attachment requests.</p>
    pub fn auto_accept_shared_attachments(&self) -> ::std::option::Option<&crate::types::AutoAcceptSharedAttachmentsValue> {
        self.auto_accept_shared_attachments.as_ref()
    }
    /// <p>Enable or disable automatic association with the default association route table.</p>
    pub fn default_route_table_association(&self) -> ::std::option::Option<&crate::types::DefaultRouteTableAssociationValue> {
        self.default_route_table_association.as_ref()
    }
    /// <p>The ID of the default association route table.</p>
    pub fn association_default_route_table_id(&self) -> ::std::option::Option<&str> {
        self.association_default_route_table_id.as_deref()
    }
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table.</p>
    pub fn default_route_table_propagation(&self) -> ::std::option::Option<&crate::types::DefaultRouteTablePropagationValue> {
        self.default_route_table_propagation.as_ref()
    }
    /// <p>The ID of the default propagation route table.</p>
    pub fn propagation_default_route_table_id(&self) -> ::std::option::Option<&str> {
        self.propagation_default_route_table_id.as_deref()
    }
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs.</p>
    /// <p>The modify ASN operation is not allowed on a transit gateway with active BGP sessions. You must first delete all transit gateway attachments that have BGP configured prior to modifying the ASN on the transit gateway.</p>
    pub fn amazon_side_asn(&self) -> ::std::option::Option<i64> {
        self.amazon_side_asn
    }
}
impl ModifyTransitGatewayOptions {
    /// Creates a new builder-style object to manufacture [`ModifyTransitGatewayOptions`](crate::types::ModifyTransitGatewayOptions).
    pub fn builder() -> crate::types::builders::ModifyTransitGatewayOptionsBuilder {
        crate::types::builders::ModifyTransitGatewayOptionsBuilder::default()
    }
}

/// A builder for [`ModifyTransitGatewayOptions`](crate::types::ModifyTransitGatewayOptions).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyTransitGatewayOptionsBuilder {
    pub(crate) add_transit_gateway_cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) remove_transit_gateway_cidr_blocks: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) vpn_ecmp_support: ::std::option::Option<crate::types::VpnEcmpSupportValue>,
    pub(crate) dns_support: ::std::option::Option<crate::types::DnsSupportValue>,
    pub(crate) security_group_referencing_support: ::std::option::Option<crate::types::SecurityGroupReferencingSupportValue>,
    pub(crate) auto_accept_shared_attachments: ::std::option::Option<crate::types::AutoAcceptSharedAttachmentsValue>,
    pub(crate) default_route_table_association: ::std::option::Option<crate::types::DefaultRouteTableAssociationValue>,
    pub(crate) association_default_route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) default_route_table_propagation: ::std::option::Option<crate::types::DefaultRouteTablePropagationValue>,
    pub(crate) propagation_default_route_table_id: ::std::option::Option<::std::string::String>,
    pub(crate) amazon_side_asn: ::std::option::Option<i64>,
}
impl ModifyTransitGatewayOptionsBuilder {
    /// Appends an item to `add_transit_gateway_cidr_blocks`.
    ///
    /// To override the contents of this collection use [`set_add_transit_gateway_cidr_blocks`](Self::set_add_transit_gateway_cidr_blocks).
    ///
    /// <p>Adds IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    pub fn add_transit_gateway_cidr_blocks(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.add_transit_gateway_cidr_blocks.unwrap_or_default();
        v.push(input.into());
        self.add_transit_gateway_cidr_blocks = ::std::option::Option::Some(v);
        self
    }
    /// <p>Adds IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    pub fn set_add_transit_gateway_cidr_blocks(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.add_transit_gateway_cidr_blocks = input;
        self
    }
    /// <p>Adds IPv4 or IPv6 CIDR blocks for the transit gateway. Must be a size /24 CIDR block or larger for IPv4, or a size /64 CIDR block or larger for IPv6.</p>
    pub fn get_add_transit_gateway_cidr_blocks(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.add_transit_gateway_cidr_blocks
    }
    /// Appends an item to `remove_transit_gateway_cidr_blocks`.
    ///
    /// To override the contents of this collection use [`set_remove_transit_gateway_cidr_blocks`](Self::set_remove_transit_gateway_cidr_blocks).
    ///
    /// <p>Removes CIDR blocks for the transit gateway.</p>
    pub fn remove_transit_gateway_cidr_blocks(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.remove_transit_gateway_cidr_blocks.unwrap_or_default();
        v.push(input.into());
        self.remove_transit_gateway_cidr_blocks = ::std::option::Option::Some(v);
        self
    }
    /// <p>Removes CIDR blocks for the transit gateway.</p>
    pub fn set_remove_transit_gateway_cidr_blocks(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.remove_transit_gateway_cidr_blocks = input;
        self
    }
    /// <p>Removes CIDR blocks for the transit gateway.</p>
    pub fn get_remove_transit_gateway_cidr_blocks(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.remove_transit_gateway_cidr_blocks
    }
    /// <p>Enable or disable Equal Cost Multipath Protocol support.</p>
    pub fn vpn_ecmp_support(mut self, input: crate::types::VpnEcmpSupportValue) -> Self {
        self.vpn_ecmp_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable Equal Cost Multipath Protocol support.</p>
    pub fn set_vpn_ecmp_support(mut self, input: ::std::option::Option<crate::types::VpnEcmpSupportValue>) -> Self {
        self.vpn_ecmp_support = input;
        self
    }
    /// <p>Enable or disable Equal Cost Multipath Protocol support.</p>
    pub fn get_vpn_ecmp_support(&self) -> &::std::option::Option<crate::types::VpnEcmpSupportValue> {
        &self.vpn_ecmp_support
    }
    /// <p>Enable or disable DNS support.</p>
    pub fn dns_support(mut self, input: crate::types::DnsSupportValue) -> Self {
        self.dns_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable DNS support.</p>
    pub fn set_dns_support(mut self, input: ::std::option::Option<crate::types::DnsSupportValue>) -> Self {
        self.dns_support = input;
        self
    }
    /// <p>Enable or disable DNS support.</p>
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
    /// <p>Enable or disable automatic acceptance of attachment requests.</p>
    pub fn auto_accept_shared_attachments(mut self, input: crate::types::AutoAcceptSharedAttachmentsValue) -> Self {
        self.auto_accept_shared_attachments = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable automatic acceptance of attachment requests.</p>
    pub fn set_auto_accept_shared_attachments(mut self, input: ::std::option::Option<crate::types::AutoAcceptSharedAttachmentsValue>) -> Self {
        self.auto_accept_shared_attachments = input;
        self
    }
    /// <p>Enable or disable automatic acceptance of attachment requests.</p>
    pub fn get_auto_accept_shared_attachments(&self) -> &::std::option::Option<crate::types::AutoAcceptSharedAttachmentsValue> {
        &self.auto_accept_shared_attachments
    }
    /// <p>Enable or disable automatic association with the default association route table.</p>
    pub fn default_route_table_association(mut self, input: crate::types::DefaultRouteTableAssociationValue) -> Self {
        self.default_route_table_association = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable automatic association with the default association route table.</p>
    pub fn set_default_route_table_association(mut self, input: ::std::option::Option<crate::types::DefaultRouteTableAssociationValue>) -> Self {
        self.default_route_table_association = input;
        self
    }
    /// <p>Enable or disable automatic association with the default association route table.</p>
    pub fn get_default_route_table_association(&self) -> &::std::option::Option<crate::types::DefaultRouteTableAssociationValue> {
        &self.default_route_table_association
    }
    /// <p>The ID of the default association route table.</p>
    pub fn association_default_route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.association_default_route_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the default association route table.</p>
    pub fn set_association_default_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.association_default_route_table_id = input;
        self
    }
    /// <p>The ID of the default association route table.</p>
    pub fn get_association_default_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.association_default_route_table_id
    }
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table.</p>
    pub fn default_route_table_propagation(mut self, input: crate::types::DefaultRouteTablePropagationValue) -> Self {
        self.default_route_table_propagation = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table.</p>
    pub fn set_default_route_table_propagation(mut self, input: ::std::option::Option<crate::types::DefaultRouteTablePropagationValue>) -> Self {
        self.default_route_table_propagation = input;
        self
    }
    /// <p>Enable or disable automatic propagation of routes to the default propagation route table.</p>
    pub fn get_default_route_table_propagation(&self) -> &::std::option::Option<crate::types::DefaultRouteTablePropagationValue> {
        &self.default_route_table_propagation
    }
    /// <p>The ID of the default propagation route table.</p>
    pub fn propagation_default_route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.propagation_default_route_table_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the default propagation route table.</p>
    pub fn set_propagation_default_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.propagation_default_route_table_id = input;
        self
    }
    /// <p>The ID of the default propagation route table.</p>
    pub fn get_propagation_default_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.propagation_default_route_table_id
    }
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs.</p>
    /// <p>The modify ASN operation is not allowed on a transit gateway with active BGP sessions. You must first delete all transit gateway attachments that have BGP configured prior to modifying the ASN on the transit gateway.</p>
    pub fn amazon_side_asn(mut self, input: i64) -> Self {
        self.amazon_side_asn = ::std::option::Option::Some(input);
        self
    }
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs.</p>
    /// <p>The modify ASN operation is not allowed on a transit gateway with active BGP sessions. You must first delete all transit gateway attachments that have BGP configured prior to modifying the ASN on the transit gateway.</p>
    pub fn set_amazon_side_asn(mut self, input: ::std::option::Option<i64>) -> Self {
        self.amazon_side_asn = input;
        self
    }
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. The range is 64512 to 65534 for 16-bit ASNs and 4200000000 to 4294967294 for 32-bit ASNs.</p>
    /// <p>The modify ASN operation is not allowed on a transit gateway with active BGP sessions. You must first delete all transit gateway attachments that have BGP configured prior to modifying the ASN on the transit gateway.</p>
    pub fn get_amazon_side_asn(&self) -> &::std::option::Option<i64> {
        &self.amazon_side_asn
    }
    /// Consumes the builder and constructs a [`ModifyTransitGatewayOptions`](crate::types::ModifyTransitGatewayOptions).
    pub fn build(self) -> crate::types::ModifyTransitGatewayOptions {
        crate::types::ModifyTransitGatewayOptions {
            add_transit_gateway_cidr_blocks: self.add_transit_gateway_cidr_blocks,
            remove_transit_gateway_cidr_blocks: self.remove_transit_gateway_cidr_blocks,
            vpn_ecmp_support: self.vpn_ecmp_support,
            dns_support: self.dns_support,
            security_group_referencing_support: self.security_group_referencing_support,
            auto_accept_shared_attachments: self.auto_accept_shared_attachments,
            default_route_table_association: self.default_route_table_association,
            association_default_route_table_id: self.association_default_route_table_id,
            default_route_table_propagation: self.default_route_table_propagation,
            propagation_default_route_table_id: self.propagation_default_route_table_id,
            amazon_side_asn: self.amazon_side_asn,
        }
    }
}