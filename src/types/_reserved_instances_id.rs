// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the ID of a Reserved Instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReservedInstancesId {
    /// <p>The ID of the Reserved Instance.</p>
    pub reserved_instances_id: ::std::option::Option<::std::string::String>,
}
impl ReservedInstancesId {
    /// <p>The ID of the Reserved Instance.</p>
    pub fn reserved_instances_id(&self) -> ::std::option::Option<&str> {
        self.reserved_instances_id.as_deref()
    }
}
impl ReservedInstancesId {
    /// Creates a new builder-style object to manufacture [`ReservedInstancesId`](crate::types::ReservedInstancesId).
    pub fn builder() -> crate::types::builders::ReservedInstancesIdBuilder {
        crate::types::builders::ReservedInstancesIdBuilder::default()
    }
}

/// A builder for [`ReservedInstancesId`](crate::types::ReservedInstancesId).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ReservedInstancesIdBuilder {
    pub(crate) reserved_instances_id: ::std::option::Option<::std::string::String>,
}
impl ReservedInstancesIdBuilder {
    /// <p>The ID of the Reserved Instance.</p>
    pub fn reserved_instances_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.reserved_instances_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Reserved Instance.</p>
    pub fn set_reserved_instances_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.reserved_instances_id = input;
        self
    }
    /// <p>The ID of the Reserved Instance.</p>
    pub fn get_reserved_instances_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.reserved_instances_id
    }
    /// Consumes the builder and constructs a [`ReservedInstancesId`](crate::types::ReservedInstancesId).
    pub fn build(self) -> crate::types::ReservedInstancesId {
        crate::types::ReservedInstancesId {
            reserved_instances_id: self.reserved_instances_id,
        }
    }
}
