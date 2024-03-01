// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the options for instance hostnames.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PrivateDnsNameOptionsRequest {
    /// <p>The type of hostname for EC2 instances. For IPv4 only subnets, an instance DNS name must be based on the instance IPv4 address. For IPv6 only subnets, an instance DNS name must be based on the instance ID. For dual-stack subnets, you can specify whether DNS names use the instance IPv4 address or the instance ID.</p>
    pub hostname_type: ::std::option::Option<crate::types::HostnameType>,
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p>
    pub enable_resource_name_dns_a_record: ::std::option::Option<bool>,
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p>
    pub enable_resource_name_dns_aaaa_record: ::std::option::Option<bool>,
}
impl PrivateDnsNameOptionsRequest {
    /// <p>The type of hostname for EC2 instances. For IPv4 only subnets, an instance DNS name must be based on the instance IPv4 address. For IPv6 only subnets, an instance DNS name must be based on the instance ID. For dual-stack subnets, you can specify whether DNS names use the instance IPv4 address or the instance ID.</p>
    pub fn hostname_type(&self) -> ::std::option::Option<&crate::types::HostnameType> {
        self.hostname_type.as_ref()
    }
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p>
    pub fn enable_resource_name_dns_a_record(&self) -> ::std::option::Option<bool> {
        self.enable_resource_name_dns_a_record
    }
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p>
    pub fn enable_resource_name_dns_aaaa_record(&self) -> ::std::option::Option<bool> {
        self.enable_resource_name_dns_aaaa_record
    }
}
impl PrivateDnsNameOptionsRequest {
    /// Creates a new builder-style object to manufacture [`PrivateDnsNameOptionsRequest`](crate::types::PrivateDnsNameOptionsRequest).
    pub fn builder() -> crate::types::builders::PrivateDnsNameOptionsRequestBuilder {
        crate::types::builders::PrivateDnsNameOptionsRequestBuilder::default()
    }
}

/// A builder for [`PrivateDnsNameOptionsRequest`](crate::types::PrivateDnsNameOptionsRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PrivateDnsNameOptionsRequestBuilder {
    pub(crate) hostname_type: ::std::option::Option<crate::types::HostnameType>,
    pub(crate) enable_resource_name_dns_a_record: ::std::option::Option<bool>,
    pub(crate) enable_resource_name_dns_aaaa_record: ::std::option::Option<bool>,
}
impl PrivateDnsNameOptionsRequestBuilder {
    /// <p>The type of hostname for EC2 instances. For IPv4 only subnets, an instance DNS name must be based on the instance IPv4 address. For IPv6 only subnets, an instance DNS name must be based on the instance ID. For dual-stack subnets, you can specify whether DNS names use the instance IPv4 address or the instance ID.</p>
    pub fn hostname_type(mut self, input: crate::types::HostnameType) -> Self {
        self.hostname_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of hostname for EC2 instances. For IPv4 only subnets, an instance DNS name must be based on the instance IPv4 address. For IPv6 only subnets, an instance DNS name must be based on the instance ID. For dual-stack subnets, you can specify whether DNS names use the instance IPv4 address or the instance ID.</p>
    pub fn set_hostname_type(mut self, input: ::std::option::Option<crate::types::HostnameType>) -> Self {
        self.hostname_type = input;
        self
    }
    /// <p>The type of hostname for EC2 instances. For IPv4 only subnets, an instance DNS name must be based on the instance IPv4 address. For IPv6 only subnets, an instance DNS name must be based on the instance ID. For dual-stack subnets, you can specify whether DNS names use the instance IPv4 address or the instance ID.</p>
    pub fn get_hostname_type(&self) -> &::std::option::Option<crate::types::HostnameType> {
        &self.hostname_type
    }
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p>
    pub fn enable_resource_name_dns_a_record(mut self, input: bool) -> Self {
        self.enable_resource_name_dns_a_record = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p>
    pub fn set_enable_resource_name_dns_a_record(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enable_resource_name_dns_a_record = input;
        self
    }
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS A records.</p>
    pub fn get_enable_resource_name_dns_a_record(&self) -> &::std::option::Option<bool> {
        &self.enable_resource_name_dns_a_record
    }
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p>
    pub fn enable_resource_name_dns_aaaa_record(mut self, input: bool) -> Self {
        self.enable_resource_name_dns_aaaa_record = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p>
    pub fn set_enable_resource_name_dns_aaaa_record(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enable_resource_name_dns_aaaa_record = input;
        self
    }
    /// <p>Indicates whether to respond to DNS queries for instance hostnames with DNS AAAA records.</p>
    pub fn get_enable_resource_name_dns_aaaa_record(&self) -> &::std::option::Option<bool> {
        &self.enable_resource_name_dns_aaaa_record
    }
    /// Consumes the builder and constructs a [`PrivateDnsNameOptionsRequest`](crate::types::PrivateDnsNameOptionsRequest).
    pub fn build(self) -> crate::types::PrivateDnsNameOptionsRequest {
        crate::types::PrivateDnsNameOptionsRequest {
            hostname_type: self.hostname_type,
            enable_resource_name_dns_a_record: self.enable_resource_name_dns_a_record,
            enable_resource_name_dns_aaaa_record: self.enable_resource_name_dns_aaaa_record,
        }
    }
}
