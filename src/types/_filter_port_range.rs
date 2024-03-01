// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a port range.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FilterPortRange {
    /// <p>The first port in the range.</p>
    pub from_port: ::std::option::Option<i32>,
    /// <p>The last port in the range.</p>
    pub to_port: ::std::option::Option<i32>,
}
impl FilterPortRange {
    /// <p>The first port in the range.</p>
    pub fn from_port(&self) -> ::std::option::Option<i32> {
        self.from_port
    }
    /// <p>The last port in the range.</p>
    pub fn to_port(&self) -> ::std::option::Option<i32> {
        self.to_port
    }
}
impl FilterPortRange {
    /// Creates a new builder-style object to manufacture [`FilterPortRange`](crate::types::FilterPortRange).
    pub fn builder() -> crate::types::builders::FilterPortRangeBuilder {
        crate::types::builders::FilterPortRangeBuilder::default()
    }
}

/// A builder for [`FilterPortRange`](crate::types::FilterPortRange).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct FilterPortRangeBuilder {
    pub(crate) from_port: ::std::option::Option<i32>,
    pub(crate) to_port: ::std::option::Option<i32>,
}
impl FilterPortRangeBuilder {
    /// <p>The first port in the range.</p>
    pub fn from_port(mut self, input: i32) -> Self {
        self.from_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The first port in the range.</p>
    pub fn set_from_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.from_port = input;
        self
    }
    /// <p>The first port in the range.</p>
    pub fn get_from_port(&self) -> &::std::option::Option<i32> {
        &self.from_port
    }
    /// <p>The last port in the range.</p>
    pub fn to_port(mut self, input: i32) -> Self {
        self.to_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last port in the range.</p>
    pub fn set_to_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.to_port = input;
        self
    }
    /// <p>The last port in the range.</p>
    pub fn get_to_port(&self) -> &::std::option::Option<i32> {
        &self.to_port
    }
    /// Consumes the builder and constructs a [`FilterPortRange`](crate::types::FilterPortRange).
    pub fn build(self) -> crate::types::FilterPortRange {
        crate::types::FilterPortRange {
            from_port: self.from_port,
            to_port: self.to_port,
        }
    }
}