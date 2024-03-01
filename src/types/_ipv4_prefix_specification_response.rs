// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the IPv4 delegated prefixes assigned to a network interface.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Ipv4PrefixSpecificationResponse {
    /// <p>The IPv4 delegated prefixes assigned to the network interface.</p>
    pub ipv4_prefix: ::std::option::Option<::std::string::String>,
}
impl Ipv4PrefixSpecificationResponse {
    /// <p>The IPv4 delegated prefixes assigned to the network interface.</p>
    pub fn ipv4_prefix(&self) -> ::std::option::Option<&str> {
        self.ipv4_prefix.as_deref()
    }
}
impl Ipv4PrefixSpecificationResponse {
    /// Creates a new builder-style object to manufacture [`Ipv4PrefixSpecificationResponse`](crate::types::Ipv4PrefixSpecificationResponse).
    pub fn builder() -> crate::types::builders::Ipv4PrefixSpecificationResponseBuilder {
        crate::types::builders::Ipv4PrefixSpecificationResponseBuilder::default()
    }
}

/// A builder for [`Ipv4PrefixSpecificationResponse`](crate::types::Ipv4PrefixSpecificationResponse).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct Ipv4PrefixSpecificationResponseBuilder {
    pub(crate) ipv4_prefix: ::std::option::Option<::std::string::String>,
}
impl Ipv4PrefixSpecificationResponseBuilder {
    /// <p>The IPv4 delegated prefixes assigned to the network interface.</p>
    pub fn ipv4_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv4_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 delegated prefixes assigned to the network interface.</p>
    pub fn set_ipv4_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv4_prefix = input;
        self
    }
    /// <p>The IPv4 delegated prefixes assigned to the network interface.</p>
    pub fn get_ipv4_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipv4_prefix
    }
    /// Consumes the builder and constructs a [`Ipv4PrefixSpecificationResponse`](crate::types::Ipv4PrefixSpecificationResponse).
    pub fn build(self) -> crate::types::Ipv4PrefixSpecificationResponse {
        crate::types::Ipv4PrefixSpecificationResponse {
            ipv4_prefix: self.ipv4_prefix,
        }
    }
}
