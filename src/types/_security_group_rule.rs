// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a security group rule.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SecurityGroupRule {
    /// <p>The ID of the security group rule.</p>
    pub security_group_rule_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the security group.</p>
    pub group_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Web Services account that owns the security group. </p>
    pub group_owner_id: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the security group rule is an outbound rule.</p>
    pub is_egress: ::std::option::Option<bool>,
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). </p>
    /// <p>Use <code>-1</code> to specify all protocols.</p>
    pub ip_protocol: ::std::option::Option<::std::string::String>,
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub from_port: ::std::option::Option<i32>,
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub to_port: ::std::option::Option<i32>,
    /// <p>The IPv4 CIDR range.</p>
    pub cidr_ipv4: ::std::option::Option<::std::string::String>,
    /// <p>The IPv6 CIDR range.</p>
    pub cidr_ipv6: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the prefix list.</p>
    pub prefix_list_id: ::std::option::Option<::std::string::String>,
    /// <p>Describes the security group that is referenced in the rule.</p>
    pub referenced_group_info: ::std::option::Option<crate::types::ReferencedSecurityGroup>,
    /// <p>The security group rule description.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The tags applied to the security group rule.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl SecurityGroupRule {
    /// <p>The ID of the security group rule.</p>
    pub fn security_group_rule_id(&self) -> ::std::option::Option<&str> {
        self.security_group_rule_id.as_deref()
    }
    /// <p>The ID of the security group.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the security group. </p>
    pub fn group_owner_id(&self) -> ::std::option::Option<&str> {
        self.group_owner_id.as_deref()
    }
    /// <p>Indicates whether the security group rule is an outbound rule.</p>
    pub fn is_egress(&self) -> ::std::option::Option<bool> {
        self.is_egress
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). </p>
    /// <p>Use <code>-1</code> to specify all protocols.</p>
    pub fn ip_protocol(&self) -> ::std::option::Option<&str> {
        self.ip_protocol.as_deref()
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn from_port(&self) -> ::std::option::Option<i32> {
        self.from_port
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn to_port(&self) -> ::std::option::Option<i32> {
        self.to_port
    }
    /// <p>The IPv4 CIDR range.</p>
    pub fn cidr_ipv4(&self) -> ::std::option::Option<&str> {
        self.cidr_ipv4.as_deref()
    }
    /// <p>The IPv6 CIDR range.</p>
    pub fn cidr_ipv6(&self) -> ::std::option::Option<&str> {
        self.cidr_ipv6.as_deref()
    }
    /// <p>The ID of the prefix list.</p>
    pub fn prefix_list_id(&self) -> ::std::option::Option<&str> {
        self.prefix_list_id.as_deref()
    }
    /// <p>Describes the security group that is referenced in the rule.</p>
    pub fn referenced_group_info(&self) -> ::std::option::Option<&crate::types::ReferencedSecurityGroup> {
        self.referenced_group_info.as_ref()
    }
    /// <p>The security group rule description.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The tags applied to the security group rule.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl SecurityGroupRule {
    /// Creates a new builder-style object to manufacture [`SecurityGroupRule`](crate::types::SecurityGroupRule).
    pub fn builder() -> crate::types::builders::SecurityGroupRuleBuilder {
        crate::types::builders::SecurityGroupRuleBuilder::default()
    }
}

/// A builder for [`SecurityGroupRule`](crate::types::SecurityGroupRule).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SecurityGroupRuleBuilder {
    pub(crate) security_group_rule_id: ::std::option::Option<::std::string::String>,
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    pub(crate) group_owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) is_egress: ::std::option::Option<bool>,
    pub(crate) ip_protocol: ::std::option::Option<::std::string::String>,
    pub(crate) from_port: ::std::option::Option<i32>,
    pub(crate) to_port: ::std::option::Option<i32>,
    pub(crate) cidr_ipv4: ::std::option::Option<::std::string::String>,
    pub(crate) cidr_ipv6: ::std::option::Option<::std::string::String>,
    pub(crate) prefix_list_id: ::std::option::Option<::std::string::String>,
    pub(crate) referenced_group_info: ::std::option::Option<crate::types::ReferencedSecurityGroup>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl SecurityGroupRuleBuilder {
    /// <p>The ID of the security group rule.</p>
    pub fn security_group_rule_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.security_group_rule_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the security group rule.</p>
    pub fn set_security_group_rule_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.security_group_rule_id = input;
        self
    }
    /// <p>The ID of the security group rule.</p>
    pub fn get_security_group_rule_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.security_group_rule_id
    }
    /// <p>The ID of the security group.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn get_group_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_id
    }
    /// <p>The ID of the Amazon Web Services account that owns the security group. </p>
    pub fn group_owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the security group. </p>
    pub fn set_group_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_owner_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the security group. </p>
    pub fn get_group_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_owner_id
    }
    /// <p>Indicates whether the security group rule is an outbound rule.</p>
    pub fn is_egress(mut self, input: bool) -> Self {
        self.is_egress = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the security group rule is an outbound rule.</p>
    pub fn set_is_egress(mut self, input: ::std::option::Option<bool>) -> Self {
        self.is_egress = input;
        self
    }
    /// <p>Indicates whether the security group rule is an outbound rule.</p>
    pub fn get_is_egress(&self) -> &::std::option::Option<bool> {
        &self.is_egress
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). </p>
    /// <p>Use <code>-1</code> to specify all protocols.</p>
    pub fn ip_protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ip_protocol = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). </p>
    /// <p>Use <code>-1</code> to specify all protocols.</p>
    pub fn set_ip_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ip_protocol = input;
        self
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>, <code>icmpv6</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). </p>
    /// <p>Use <code>-1</code> to specify all protocols.</p>
    pub fn get_ip_protocol(&self) -> &::std::option::Option<::std::string::String> {
        &self.ip_protocol
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.from_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn set_from_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.from_port = input;
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 types. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn get_from_port(&self) -> &::std::option::Option<i32> {
        &self.from_port
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.to_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn set_to_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.to_port = input;
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP or ICMPv6, this is the type number. A value of -1 indicates all ICMP/ICMPv6 codes. If you specify all ICMP/ICMPv6 types, you must specify all ICMP/ICMPv6 codes.</p>
    pub fn get_to_port(&self) -> &::std::option::Option<i32> {
        &self.to_port
    }
    /// <p>The IPv4 CIDR range.</p>
    pub fn cidr_ipv4(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr_ipv4 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 CIDR range.</p>
    pub fn set_cidr_ipv4(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr_ipv4 = input;
        self
    }
    /// <p>The IPv4 CIDR range.</p>
    pub fn get_cidr_ipv4(&self) -> &::std::option::Option<::std::string::String> {
        &self.cidr_ipv4
    }
    /// <p>The IPv6 CIDR range.</p>
    pub fn cidr_ipv6(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr_ipv6 = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv6 CIDR range.</p>
    pub fn set_cidr_ipv6(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr_ipv6 = input;
        self
    }
    /// <p>The IPv6 CIDR range.</p>
    pub fn get_cidr_ipv6(&self) -> &::std::option::Option<::std::string::String> {
        &self.cidr_ipv6
    }
    /// <p>The ID of the prefix list.</p>
    pub fn prefix_list_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix_list_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn set_prefix_list_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix_list_id = input;
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn get_prefix_list_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix_list_id
    }
    /// <p>Describes the security group that is referenced in the rule.</p>
    pub fn referenced_group_info(mut self, input: crate::types::ReferencedSecurityGroup) -> Self {
        self.referenced_group_info = ::std::option::Option::Some(input);
        self
    }
    /// <p>Describes the security group that is referenced in the rule.</p>
    pub fn set_referenced_group_info(mut self, input: ::std::option::Option<crate::types::ReferencedSecurityGroup>) -> Self {
        self.referenced_group_info = input;
        self
    }
    /// <p>Describes the security group that is referenced in the rule.</p>
    pub fn get_referenced_group_info(&self) -> &::std::option::Option<crate::types::ReferencedSecurityGroup> {
        &self.referenced_group_info
    }
    /// <p>The security group rule description.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The security group rule description.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The security group rule description.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags applied to the security group rule.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags applied to the security group rule.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags applied to the security group rule.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`SecurityGroupRule`](crate::types::SecurityGroupRule).
    pub fn build(self) -> crate::types::SecurityGroupRule {
        crate::types::SecurityGroupRule {
            security_group_rule_id: self.security_group_rule_id,
            group_id: self.group_id,
            group_owner_id: self.group_owner_id,
            is_egress: self.is_egress,
            ip_protocol: self.ip_protocol,
            from_port: self.from_port,
            to_port: self.to_port,
            cidr_ipv4: self.cidr_ipv4,
            cidr_ipv6: self.cidr_ipv6,
            prefix_list_id: self.prefix_list_id,
            referenced_group_info: self.referenced_group_info,
            description: self.description,
            tags: self.tags,
        }
    }
}