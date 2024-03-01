// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the DNS server to be used.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DnsServersOptionsModifyStructure {
    /// <p>The IPv4 address range, in CIDR notation, of the DNS servers to be used. You can specify up to two DNS servers. Ensure that the DNS servers can be reached by the clients. The specified values overwrite the existing values.</p>
    pub custom_dns_servers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Indicates whether DNS servers should be used. Specify <code>False</code> to delete the existing DNS servers.</p>
    pub enabled: ::std::option::Option<bool>,
}
impl DnsServersOptionsModifyStructure {
    /// <p>The IPv4 address range, in CIDR notation, of the DNS servers to be used. You can specify up to two DNS servers. Ensure that the DNS servers can be reached by the clients. The specified values overwrite the existing values.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.custom_dns_servers.is_none()`.
    pub fn custom_dns_servers(&self) -> &[::std::string::String] {
        self.custom_dns_servers.as_deref().unwrap_or_default()
    }
    /// <p>Indicates whether DNS servers should be used. Specify <code>False</code> to delete the existing DNS servers.</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
}
impl DnsServersOptionsModifyStructure {
    /// Creates a new builder-style object to manufacture [`DnsServersOptionsModifyStructure`](crate::types::DnsServersOptionsModifyStructure).
    pub fn builder() -> crate::types::builders::DnsServersOptionsModifyStructureBuilder {
        crate::types::builders::DnsServersOptionsModifyStructureBuilder::default()
    }
}

/// A builder for [`DnsServersOptionsModifyStructure`](crate::types::DnsServersOptionsModifyStructure).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DnsServersOptionsModifyStructureBuilder {
    pub(crate) custom_dns_servers: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) enabled: ::std::option::Option<bool>,
}
impl DnsServersOptionsModifyStructureBuilder {
    /// Appends an item to `custom_dns_servers`.
    ///
    /// To override the contents of this collection use [`set_custom_dns_servers`](Self::set_custom_dns_servers).
    ///
    /// <p>The IPv4 address range, in CIDR notation, of the DNS servers to be used. You can specify up to two DNS servers. Ensure that the DNS servers can be reached by the clients. The specified values overwrite the existing values.</p>
    pub fn custom_dns_servers(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.custom_dns_servers.unwrap_or_default();
        v.push(input.into());
        self.custom_dns_servers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, of the DNS servers to be used. You can specify up to two DNS servers. Ensure that the DNS servers can be reached by the clients. The specified values overwrite the existing values.</p>
    pub fn set_custom_dns_servers(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.custom_dns_servers = input;
        self
    }
    /// <p>The IPv4 address range, in CIDR notation, of the DNS servers to be used. You can specify up to two DNS servers. Ensure that the DNS servers can be reached by the clients. The specified values overwrite the existing values.</p>
    pub fn get_custom_dns_servers(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.custom_dns_servers
    }
    /// <p>Indicates whether DNS servers should be used. Specify <code>False</code> to delete the existing DNS servers.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether DNS servers should be used. Specify <code>False</code> to delete the existing DNS servers.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>Indicates whether DNS servers should be used. Specify <code>False</code> to delete the existing DNS servers.</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }
    /// Consumes the builder and constructs a [`DnsServersOptionsModifyStructure`](crate::types::DnsServersOptionsModifyStructure).
    pub fn build(self) -> crate::types::DnsServersOptionsModifyStructure {
        crate::types::DnsServersOptionsModifyStructure {
            custom_dns_servers: self.custom_dns_servers,
            enabled: self.enabled,
        }
    }
}
