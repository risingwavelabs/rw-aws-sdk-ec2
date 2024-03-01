// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an IPv4 prefix.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Ipv4PrefixSpecification {
    /// <p>The IPv4 prefix. For information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-prefix-eni.html"> Assigning prefixes to Amazon EC2 network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub ipv4_prefix: ::std::option::Option<::std::string::String>,
}
impl Ipv4PrefixSpecification {
    /// <p>The IPv4 prefix. For information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-prefix-eni.html"> Assigning prefixes to Amazon EC2 network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn ipv4_prefix(&self) -> ::std::option::Option<&str> {
        self.ipv4_prefix.as_deref()
    }
}
impl Ipv4PrefixSpecification {
    /// Creates a new builder-style object to manufacture [`Ipv4PrefixSpecification`](crate::types::Ipv4PrefixSpecification).
    pub fn builder() -> crate::types::builders::Ipv4PrefixSpecificationBuilder {
        crate::types::builders::Ipv4PrefixSpecificationBuilder::default()
    }
}

/// A builder for [`Ipv4PrefixSpecification`](crate::types::Ipv4PrefixSpecification).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct Ipv4PrefixSpecificationBuilder {
    pub(crate) ipv4_prefix: ::std::option::Option<::std::string::String>,
}
impl Ipv4PrefixSpecificationBuilder {
    /// <p>The IPv4 prefix. For information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-prefix-eni.html"> Assigning prefixes to Amazon EC2 network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn ipv4_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipv4_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPv4 prefix. For information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-prefix-eni.html"> Assigning prefixes to Amazon EC2 network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn set_ipv4_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipv4_prefix = input;
        self
    }
    /// <p>The IPv4 prefix. For information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-prefix-eni.html"> Assigning prefixes to Amazon EC2 network interfaces</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn get_ipv4_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipv4_prefix
    }
    /// Consumes the builder and constructs a [`Ipv4PrefixSpecification`](crate::types::Ipv4PrefixSpecification).
    pub fn build(self) -> crate::types::Ipv4PrefixSpecification {
        crate::types::Ipv4PrefixSpecification {
            ipv4_prefix: self.ipv4_prefix,
        }
    }
}
