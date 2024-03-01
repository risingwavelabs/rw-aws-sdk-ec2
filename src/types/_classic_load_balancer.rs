// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Classic Load Balancer.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ClassicLoadBalancer {
    /// <p>The name of the load balancer.</p>
    pub name: ::std::option::Option<::std::string::String>,
}
impl ClassicLoadBalancer {
    /// <p>The name of the load balancer.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl ClassicLoadBalancer {
    /// Creates a new builder-style object to manufacture [`ClassicLoadBalancer`](crate::types::ClassicLoadBalancer).
    pub fn builder() -> crate::types::builders::ClassicLoadBalancerBuilder {
        crate::types::builders::ClassicLoadBalancerBuilder::default()
    }
}

/// A builder for [`ClassicLoadBalancer`](crate::types::ClassicLoadBalancer).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ClassicLoadBalancerBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl ClassicLoadBalancerBuilder {
    /// <p>The name of the load balancer.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the load balancer.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the load balancer.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Consumes the builder and constructs a [`ClassicLoadBalancer`](crate::types::ClassicLoadBalancer).
    pub fn build(self) -> crate::types::ClassicLoadBalancer {
        crate::types::ClassicLoadBalancer { name: self.name }
    }
}
