// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the resource with which a prefix list is associated.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PrefixListAssociation {
    /// <p>The ID of the resource.</p>
    pub resource_id: ::std::option::Option<::std::string::String>,
    /// <p>The owner of the resource.</p>
    pub resource_owner: ::std::option::Option<::std::string::String>,
}
impl PrefixListAssociation {
    /// <p>The ID of the resource.</p>
    pub fn resource_id(&self) -> ::std::option::Option<&str> {
        self.resource_id.as_deref()
    }
    /// <p>The owner of the resource.</p>
    pub fn resource_owner(&self) -> ::std::option::Option<&str> {
        self.resource_owner.as_deref()
    }
}
impl PrefixListAssociation {
    /// Creates a new builder-style object to manufacture [`PrefixListAssociation`](crate::types::PrefixListAssociation).
    pub fn builder() -> crate::types::builders::PrefixListAssociationBuilder {
        crate::types::builders::PrefixListAssociationBuilder::default()
    }
}

/// A builder for [`PrefixListAssociation`](crate::types::PrefixListAssociation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PrefixListAssociationBuilder {
    pub(crate) resource_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_owner: ::std::option::Option<::std::string::String>,
}
impl PrefixListAssociationBuilder {
    /// <p>The ID of the resource.</p>
    pub fn resource_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn set_resource_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_id = input;
        self
    }
    /// <p>The ID of the resource.</p>
    pub fn get_resource_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_id
    }
    /// <p>The owner of the resource.</p>
    pub fn resource_owner(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_owner = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The owner of the resource.</p>
    pub fn set_resource_owner(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_owner = input;
        self
    }
    /// <p>The owner of the resource.</p>
    pub fn get_resource_owner(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_owner
    }
    /// Consumes the builder and constructs a [`PrefixListAssociation`](crate::types::PrefixListAssociation).
    pub fn build(self) -> crate::types::PrefixListAssociation {
        crate::types::PrefixListAssociation {
            resource_id: self.resource_id,
            resource_owner: self.resource_owner,
        }
    }
}
