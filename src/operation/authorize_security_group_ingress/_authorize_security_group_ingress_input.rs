// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AuthorizeSecurityGroupIngressInput {
    /// <p>The IPv4 address range, in CIDR format. You can't specify this parameter when specifying a source security group. To specify an IPv6 address range, use a set of IP permissions.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub cidr_ip: ::std::option::Option<::std::string::String>,
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP, this is the type number. A value of -1 indicates all ICMP types. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub from_port: ::std::option::Option<i32>,
    /// <p>The ID of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub group_id: ::std::option::Option<::std::string::String>,
    /// <p>[Default VPC] The name of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The sets of IP permissions.</p>
    pub ip_permissions: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>,
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). To specify <code>icmpv6</code>, use a set of IP permissions.</p>
    /// <p>Use <code>-1</code> to specify all protocols. If you specify <code>-1</code> or a protocol other than <code>tcp</code>, <code>udp</code>, or <code>icmp</code>, traffic on all ports is allowed, regardless of any ports you specify.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub ip_protocol: ::std::option::Option<::std::string::String>,
    /// <p>[Default VPC] The name of the source security group. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the start of the port range, the IP protocol, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead. The source security group must be in the same VPC.</p>
    pub source_security_group_name: ::std::option::Option<::std::string::String>,
    /// <p>[Nondefault VPC] The Amazon Web Services account ID for the source security group, if the source security group is in a different account. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the IP protocol, the start of the port range, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead.</p>
    pub source_security_group_owner_id: ::std::option::Option<::std::string::String>,
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP, this is the code. A value of -1 indicates all ICMP codes. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub to_port: ::std::option::Option<i32>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>[VPC Only] The tags applied to the security group rule.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
}
impl AuthorizeSecurityGroupIngressInput {
    /// <p>The IPv4 address range, in CIDR format. You can't specify this parameter when specifying a source security group. To specify an IPv6 address range, use a set of IP permissions.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn cidr_ip(&self) -> ::std::option::Option<&str> {
        self.cidr_ip.as_deref()
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP, this is the type number. A value of -1 indicates all ICMP types. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn from_port(&self) -> ::std::option::Option<i32> {
        self.from_port
    }
    /// <p>The ID of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>[Default VPC] The name of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The sets of IP permissions.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ip_permissions.is_none()`.
    pub fn ip_permissions(&self) -> &[crate::types::IpPermission] {
        self.ip_permissions.as_deref().unwrap_or_default()
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). To specify <code>icmpv6</code>, use a set of IP permissions.</p>
    /// <p>Use <code>-1</code> to specify all protocols. If you specify <code>-1</code> or a protocol other than <code>tcp</code>, <code>udp</code>, or <code>icmp</code>, traffic on all ports is allowed, regardless of any ports you specify.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn ip_protocol(&self) -> ::std::option::Option<&str> {
        self.ip_protocol.as_deref()
    }
    /// <p>[Default VPC] The name of the source security group. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the start of the port range, the IP protocol, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead. The source security group must be in the same VPC.</p>
    pub fn source_security_group_name(&self) -> ::std::option::Option<&str> {
        self.source_security_group_name.as_deref()
    }
    /// <p>[Nondefault VPC] The Amazon Web Services account ID for the source security group, if the source security group is in a different account. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the IP protocol, the start of the port range, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead.</p>
    pub fn source_security_group_owner_id(&self) -> ::std::option::Option<&str> {
        self.source_security_group_owner_id.as_deref()
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP, this is the code. A value of -1 indicates all ICMP codes. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn to_port(&self) -> ::std::option::Option<i32> {
        self.to_port
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>[VPC Only] The tags applied to the security group rule.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
}
impl AuthorizeSecurityGroupIngressInput {
    /// Creates a new builder-style object to manufacture [`AuthorizeSecurityGroupIngressInput`](crate::operation::authorize_security_group_ingress::AuthorizeSecurityGroupIngressInput).
    pub fn builder() -> crate::operation::authorize_security_group_ingress::builders::AuthorizeSecurityGroupIngressInputBuilder {
        crate::operation::authorize_security_group_ingress::builders::AuthorizeSecurityGroupIngressInputBuilder::default()
    }
}

/// A builder for [`AuthorizeSecurityGroupIngressInput`](crate::operation::authorize_security_group_ingress::AuthorizeSecurityGroupIngressInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AuthorizeSecurityGroupIngressInputBuilder {
    pub(crate) cidr_ip: ::std::option::Option<::std::string::String>,
    pub(crate) from_port: ::std::option::Option<i32>,
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) ip_permissions: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>,
    pub(crate) ip_protocol: ::std::option::Option<::std::string::String>,
    pub(crate) source_security_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) source_security_group_owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) to_port: ::std::option::Option<i32>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
}
impl AuthorizeSecurityGroupIngressInputBuilder {
    /// <p>The IPv4 address range, in CIDR format. You can't specify this parameter when specifying a source security group. To specify an IPv6 address range, use a set of IP permissions.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn cidr_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 address range, in CIDR format. You can't specify this parameter when specifying a source security group. To specify an IPv6 address range, use a set of IP permissions.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn set_cidr_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr_ip = input;
        self
    }
    /// <p>The IPv4 address range, in CIDR format. You can't specify this parameter when specifying a source security group. To specify an IPv6 address range, use a set of IP permissions.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn get_cidr_ip(&self) -> &::std::option::Option<::std::string::String> {
        &self.cidr_ip
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP, this is the type number. A value of -1 indicates all ICMP types. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.from_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP, this is the type number. A value of -1 indicates all ICMP types. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn set_from_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.from_port = input;
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the start of the port range. If the protocol is ICMP, this is the type number. A value of -1 indicates all ICMP types. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn get_from_port(&self) -> &::std::option::Option<i32> {
        &self.from_port
    }
    /// <p>The ID of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>The ID of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn get_group_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_id
    }
    /// <p>[Default VPC] The name of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>[Default VPC] The name of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>[Default VPC] The name of the security group. You must specify either the security group ID or the security group name in the request. For security groups in a nondefault VPC, you must specify the security group ID.</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_name
    }
    /// Appends an item to `ip_permissions`.
    ///
    /// To override the contents of this collection use [`set_ip_permissions`](Self::set_ip_permissions).
    ///
    /// <p>The sets of IP permissions.</p>
    pub fn ip_permissions(mut self, input: crate::types::IpPermission) -> Self {
        let mut v = self.ip_permissions.unwrap_or_default();
        v.push(input);
        self.ip_permissions = ::std::option::Option::Some(v);
        self
    }
    /// <p>The sets of IP permissions.</p>
    pub fn set_ip_permissions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::IpPermission>>) -> Self {
        self.ip_permissions = input;
        self
    }
    /// <p>The sets of IP permissions.</p>
    pub fn get_ip_permissions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::IpPermission>> {
        &self.ip_permissions
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). To specify <code>icmpv6</code>, use a set of IP permissions.</p>
    /// <p>Use <code>-1</code> to specify all protocols. If you specify <code>-1</code> or a protocol other than <code>tcp</code>, <code>udp</code>, or <code>icmp</code>, traffic on all ports is allowed, regardless of any ports you specify.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn ip_protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ip_protocol = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). To specify <code>icmpv6</code>, use a set of IP permissions.</p>
    /// <p>Use <code>-1</code> to specify all protocols. If you specify <code>-1</code> or a protocol other than <code>tcp</code>, <code>udp</code>, or <code>icmp</code>, traffic on all ports is allowed, regardless of any ports you specify.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn set_ip_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ip_protocol = input;
        self
    }
    /// <p>The IP protocol name (<code>tcp</code>, <code>udp</code>, <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers</a>). To specify <code>icmpv6</code>, use a set of IP permissions.</p>
    /// <p>Use <code>-1</code> to specify all protocols. If you specify <code>-1</code> or a protocol other than <code>tcp</code>, <code>udp</code>, or <code>icmp</code>, traffic on all ports is allowed, regardless of any ports you specify.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn get_ip_protocol(&self) -> &::std::option::Option<::std::string::String> {
        &self.ip_protocol
    }
    /// <p>[Default VPC] The name of the source security group. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the start of the port range, the IP protocol, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead. The source security group must be in the same VPC.</p>
    pub fn source_security_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_security_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>[Default VPC] The name of the source security group. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the start of the port range, the IP protocol, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead. The source security group must be in the same VPC.</p>
    pub fn set_source_security_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_security_group_name = input;
        self
    }
    /// <p>[Default VPC] The name of the source security group. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the start of the port range, the IP protocol, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead. The source security group must be in the same VPC.</p>
    pub fn get_source_security_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_security_group_name
    }
    /// <p>[Nondefault VPC] The Amazon Web Services account ID for the source security group, if the source security group is in a different account. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the IP protocol, the start of the port range, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead.</p>
    pub fn source_security_group_owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_security_group_owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>[Nondefault VPC] The Amazon Web Services account ID for the source security group, if the source security group is in a different account. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the IP protocol, the start of the port range, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead.</p>
    pub fn set_source_security_group_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_security_group_owner_id = input;
        self
    }
    /// <p>[Nondefault VPC] The Amazon Web Services account ID for the source security group, if the source security group is in a different account. You can't specify this parameter in combination with the following parameters: the CIDR IP address range, the IP protocol, the start of the port range, and the end of the port range. Creates rules that grant full ICMP, UDP, and TCP access. To create a rule with a specific IP protocol and port range, use a set of IP permissions instead.</p>
    pub fn get_source_security_group_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_security_group_owner_id
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP, this is the code. A value of -1 indicates all ICMP codes. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.to_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP, this is the code. A value of -1 indicates all ICMP codes. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn set_to_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.to_port = input;
        self
    }
    /// <p>If the protocol is TCP or UDP, this is the end of the port range. If the protocol is ICMP, this is the code. A value of -1 indicates all ICMP codes. If you specify all ICMP types, you must specify all ICMP codes.</p>
    /// <p>Alternatively, use a set of IP permissions to specify multiple rules and a description for the rule.</p>
    pub fn get_to_port(&self) -> &::std::option::Option<i32> {
        &self.to_port
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
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>[VPC Only] The tags applied to the security group rule.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>[VPC Only] The tags applied to the security group rule.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>[VPC Only] The tags applied to the security group rule.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
    /// Consumes the builder and constructs a [`AuthorizeSecurityGroupIngressInput`](crate::operation::authorize_security_group_ingress::AuthorizeSecurityGroupIngressInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::authorize_security_group_ingress::AuthorizeSecurityGroupIngressInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::authorize_security_group_ingress::AuthorizeSecurityGroupIngressInput {
            cidr_ip: self.cidr_ip,
            from_port: self.from_port,
            group_id: self.group_id,
            group_name: self.group_name,
            ip_permissions: self.ip_permissions,
            ip_protocol: self.ip_protocol,
            source_security_group_name: self.source_security_group_name,
            source_security_group_owner_id: self.source_security_group_owner_id,
            to_port: self.to_port,
            dry_run: self.dry_run,
            tag_specifications: self.tag_specifications,
        })
    }
}