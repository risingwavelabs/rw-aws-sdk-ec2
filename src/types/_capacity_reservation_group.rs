// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a resource group to which a Capacity Reservation has been added.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CapacityReservationGroup {
    /// <p>The ARN of the resource group.</p>
    pub group_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Web Services account that owns the resource group.</p>
    pub owner_id: ::std::option::Option<::std::string::String>,
}
impl CapacityReservationGroup {
    /// <p>The ARN of the resource group.</p>
    pub fn group_arn(&self) -> ::std::option::Option<&str> {
        self.group_arn.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the resource group.</p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
}
impl CapacityReservationGroup {
    /// Creates a new builder-style object to manufacture [`CapacityReservationGroup`](crate::types::CapacityReservationGroup).
    pub fn builder() -> crate::types::builders::CapacityReservationGroupBuilder {
        crate::types::builders::CapacityReservationGroupBuilder::default()
    }
}

/// A builder for [`CapacityReservationGroup`](crate::types::CapacityReservationGroup).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CapacityReservationGroupBuilder {
    pub(crate) group_arn: ::std::option::Option<::std::string::String>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
}
impl CapacityReservationGroupBuilder {
    /// <p>The ARN of the resource group.</p>
    pub fn group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the resource group.</p>
    pub fn set_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_arn = input;
        self
    }
    /// <p>The ARN of the resource group.</p>
    pub fn get_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_arn
    }
    /// <p>The ID of the Amazon Web Services account that owns the resource group.</p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the resource group.</p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the resource group.</p>
    pub fn get_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner_id
    }
    /// Consumes the builder and constructs a [`CapacityReservationGroup`](crate::types::CapacityReservationGroup).
    pub fn build(self) -> crate::types::CapacityReservationGroup {
        crate::types::CapacityReservationGroup {
            group_arn: self.group_arn,
            owner_id: self.owner_id,
        }
    }
}
