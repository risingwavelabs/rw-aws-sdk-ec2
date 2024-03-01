// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the IPv4 prefix option for a network interface.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Ipv6PrefixSpecificationRequest {
    /// <p>The IPv6 prefix.</p>
    pub ipv6_prefix: ::std::option::Option<::std::string::String>,
}
impl Ipv6PrefixSpecificationRequest {
    /// <p>The IPv6 prefix.</p>
    pub fn ipv6_prefix(&self) -> ::std::option::Option<&str> {
        self.ipv6_prefix.as_deref()
    }
}
impl Ipv6PrefixSpecificationRequest {
    /// Creates a new builder-style object to manufacture [`Ipv6PrefixSpecificationRequest`](crate::types::Ipv6PrefixSpecificationRequest).
    pub fn builder() -> crate::types::builders::Ipv6PrefixSpecificationRequestBuilder {
        crate::types::builders::Ipv6PrefixSpecificationRequestBuilder::default()
    }
}

/// A builder for [`Ipv6PrefixSpecificationRequest`](crate::types::Ipv6PrefixSpecificationRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct Ipv6PrefixSpecificationRequestBuilder {
    pub(crate) ipv6_prefix: ::std::option::Option<::std::string::String>,
}
impl Ipv6PrefixSpecificationRequestBuilder {
    /// <p>The IPv6 prefix.</p>
    pub fn ipv6_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv6_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv6 prefix.</p>
    pub fn set_ipv6_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv6_prefix = input;
        self
    }
    /// <p>The IPv6 prefix.</p>
    pub fn get_ipv6_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipv6_prefix
    }
    /// Consumes the builder and constructs a [`Ipv6PrefixSpecificationRequest`](crate::types::Ipv6PrefixSpecificationRequest).
    pub fn build(self) -> crate::types::Ipv6PrefixSpecificationRequest {
        crate::types::Ipv6PrefixSpecificationRequest {
            ipv6_prefix: self.ipv6_prefix,
        }
    }
}