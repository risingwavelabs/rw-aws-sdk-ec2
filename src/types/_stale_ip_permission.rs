// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a stale rule in a security group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StaleIpPermission {
    /// <p>The start of the port range for the TCP and UDP protocols, or an ICMP type number. A value of -1 indicates all ICMP types. </p>
    pub from_port: ::std::option::Option<i32>,
    /// <p>The IP protocol name (for <code>tcp</code>, <code>udp</code>, and <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers)</a>.</p>
    pub ip_protocol: ::std::option::Option<::std::string::String>,
    /// <p>The IP ranges. Not applicable for stale security group rules.</p>
    pub ip_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The prefix list IDs. Not applicable for stale security group rules.</p>
    pub prefix_list_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The end of the port range for the TCP and UDP protocols, or an ICMP type number. A value of <code>-1</code> indicates all ICMP types. </p>
    pub to_port: ::std::option::Option<i32>,
    /// <p>The security group pairs. Returns the ID of the referenced security group and VPC, and the ID and status of the VPC peering connection.</p>
    pub user_id_group_pairs: ::std::option::Option<::std::vec::Vec<crate::types::UserIdGroupPair>>,
}
impl StaleIpPermission {
    /// <p>The start of the port range for the TCP and UDP protocols, or an ICMP type number. A value of -1 indicates all ICMP types. </p>
    pub fn from_port(&self) -> ::std::option::Option<i32> {
        self.from_port
    }
    /// <p>The IP protocol name (for <code>tcp</code>, <code>udp</code>, and <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers)</a>.</p>
    pub fn ip_protocol(&self) -> ::std::option::Option<&str> {
        self.ip_protocol.as_deref()
    }
    /// <p>The IP ranges. Not applicable for stale security group rules.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ip_ranges.is_none()`.
    pub fn ip_ranges(&self) -> &[::std::string::String] {
        self.ip_ranges.as_deref().unwrap_or_default()
    }
    /// <p>The prefix list IDs. Not applicable for stale security group rules.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.prefix_list_ids.is_none()`.
    pub fn prefix_list_ids(&self) -> &[::std::string::String] {
        self.prefix_list_ids.as_deref().unwrap_or_default()
    }
    /// <p>The end of the port range for the TCP and UDP protocols, or an ICMP type number. A value of <code>-1</code> indicates all ICMP types. </p>
    pub fn to_port(&self) -> ::std::option::Option<i32> {
        self.to_port
    }
    /// <p>The security group pairs. Returns the ID of the referenced security group and VPC, and the ID and status of the VPC peering connection.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.user_id_group_pairs.is_none()`.
    pub fn user_id_group_pairs(&self) -> &[crate::types::UserIdGroupPair] {
        self.user_id_group_pairs.as_deref().unwrap_or_default()
    }
}
impl StaleIpPermission {
    /// Creates a new builder-style object to manufacture [`StaleIpPermission`](crate::types::StaleIpPermission).
    pub fn builder() -> crate::types::builders::StaleIpPermissionBuilder {
        crate::types::builders::StaleIpPermissionBuilder::default()
    }
}

/// A builder for [`StaleIpPermission`](crate::types::StaleIpPermission).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StaleIpPermissionBuilder {
    pub(crate) from_port: ::std::option::Option<i32>,
    pub(crate) ip_protocol: ::std::option::Option<::std::string::String>,
    pub(crate) ip_ranges: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) prefix_list_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) to_port: ::std::option::Option<i32>,
    pub(crate) user_id_group_pairs: ::std::option::Option<::std::vec::Vec<crate::types::UserIdGroupPair>>,
}
impl StaleIpPermissionBuilder {
    /// <p>The start of the port range for the TCP and UDP protocols, or an ICMP type number. A value of -1 indicates all ICMP types. </p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.from_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The start of the port range for the TCP and UDP protocols, or an ICMP type number. A value of -1 indicates all ICMP types. </p>
    pub fn set_from_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.from_port = input;
        self
    }
    /// <p>The start of the port range for the TCP and UDP protocols, or an ICMP type number. A value of -1 indicates all ICMP types. </p>
    pub fn get_from_port(&self) -> &::std::option::Option<i32> {
        &self.from_port
    }
    /// <p>The IP protocol name (for <code>tcp</code>, <code>udp</code>, and <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers)</a>.</p>
    pub fn ip_protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ip_protocol = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP protocol name (for <code>tcp</code>, <code>udp</code>, and <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers)</a>.</p>
    pub fn set_ip_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ip_protocol = input;
        self
    }
    /// <p>The IP protocol name (for <code>tcp</code>, <code>udp</code>, and <code>icmp</code>) or number (see <a href="http://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml">Protocol Numbers)</a>.</p>
    pub fn get_ip_protocol(&self) -> &::std::option::Option<::std::string::String> {
        &self.ip_protocol
    }
    /// Appends an item to `ip_ranges`.
    ///
    /// To override the contents of this collection use [`set_ip_ranges`](Self::set_ip_ranges).
    ///
    /// <p>The IP ranges. Not applicable for stale security group rules.</p>
    pub fn ip_ranges(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.ip_ranges.unwrap_or_default();
        v.push(input.into());
        self.ip_ranges = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IP ranges. Not applicable for stale security group rules.</p>
    pub fn set_ip_ranges(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.ip_ranges = input;
        self
    }
    /// <p>The IP ranges. Not applicable for stale security group rules.</p>
    pub fn get_ip_ranges(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.ip_ranges
    }
    /// Appends an item to `prefix_list_ids`.
    ///
    /// To override the contents of this collection use [`set_prefix_list_ids`](Self::set_prefix_list_ids).
    ///
    /// <p>The prefix list IDs. Not applicable for stale security group rules.</p>
    pub fn prefix_list_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.prefix_list_ids.unwrap_or_default();
        v.push(input.into());
        self.prefix_list_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The prefix list IDs. Not applicable for stale security group rules.</p>
    pub fn set_prefix_list_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.prefix_list_ids = input;
        self
    }
    /// <p>The prefix list IDs. Not applicable for stale security group rules.</p>
    pub fn get_prefix_list_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.prefix_list_ids
    }
    /// <p>The end of the port range for the TCP and UDP protocols, or an ICMP type number. A value of <code>-1</code> indicates all ICMP types. </p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.to_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The end of the port range for the TCP and UDP protocols, or an ICMP type number. A value of <code>-1</code> indicates all ICMP types. </p>
    pub fn set_to_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.to_port = input;
        self
    }
    /// <p>The end of the port range for the TCP and UDP protocols, or an ICMP type number. A value of <code>-1</code> indicates all ICMP types. </p>
    pub fn get_to_port(&self) -> &::std::option::Option<i32> {
        &self.to_port
    }
    /// Appends an item to `user_id_group_pairs`.
    ///
    /// To override the contents of this collection use [`set_user_id_group_pairs`](Self::set_user_id_group_pairs).
    ///
    /// <p>The security group pairs. Returns the ID of the referenced security group and VPC, and the ID and status of the VPC peering connection.</p>
    pub fn user_id_group_pairs(mut self, input: crate::types::UserIdGroupPair) -> Self {
        let mut v = self.user_id_group_pairs.unwrap_or_default();
        v.push(input);
        self.user_id_group_pairs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The security group pairs. Returns the ID of the referenced security group and VPC, and the ID and status of the VPC peering connection.</p>
    pub fn set_user_id_group_pairs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UserIdGroupPair>>) -> Self {
        self.user_id_group_pairs = input;
        self
    }
    /// <p>The security group pairs. Returns the ID of the referenced security group and VPC, and the ID and status of the VPC peering connection.</p>
    pub fn get_user_id_group_pairs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UserIdGroupPair>> {
        &self.user_id_group_pairs
    }
    /// Consumes the builder and constructs a [`StaleIpPermission`](crate::types::StaleIpPermission).
    pub fn build(self) -> crate::types::StaleIpPermission {
        crate::types::StaleIpPermission {
            from_port: self.from_port,
            ip_protocol: self.ip_protocol,
            ip_ranges: self.ip_ranges,
            prefix_list_ids: self.prefix_list_ids,
            to_port: self.to_port,
            user_id_group_pairs: self.user_id_group_pairs,
        }
    }
}
