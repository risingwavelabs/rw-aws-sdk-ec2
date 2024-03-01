// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an instance's Capacity Reservation targeting option. You can specify only one parameter at a time. If you specify <code>CapacityReservationPreference</code> and <code>CapacityReservationTarget</code>, the request fails.</p>
/// <p>Use the <code>CapacityReservationPreference</code> parameter to configure the instance to run as an On-Demand Instance or to run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone). Use the <code>CapacityReservationTarget</code> parameter to explicitly target a specific Capacity Reservation or a Capacity Reservation group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CapacityReservationSpecification {
    /// <p>Indicates the instance's Capacity Reservation preferences. Possible preferences include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The instance can run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p> </li>
    /// <li> <p> <code>none</code> - The instance avoids running in a Capacity Reservation even if one is available. The instance runs as an On-Demand Instance.</p> </li>
    /// </ul>
    pub capacity_reservation_preference: ::std::option::Option<crate::types::CapacityReservationPreference>,
    /// <p>Information about the target Capacity Reservation or Capacity Reservation group.</p>
    pub capacity_reservation_target: ::std::option::Option<crate::types::CapacityReservationTarget>,
}
impl CapacityReservationSpecification {
    /// <p>Indicates the instance's Capacity Reservation preferences. Possible preferences include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The instance can run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p> </li>
    /// <li> <p> <code>none</code> - The instance avoids running in a Capacity Reservation even if one is available. The instance runs as an On-Demand Instance.</p> </li>
    /// </ul>
    pub fn capacity_reservation_preference(&self) -> ::std::option::Option<&crate::types::CapacityReservationPreference> {
        self.capacity_reservation_preference.as_ref()
    }
    /// <p>Information about the target Capacity Reservation or Capacity Reservation group.</p>
    pub fn capacity_reservation_target(&self) -> ::std::option::Option<&crate::types::CapacityReservationTarget> {
        self.capacity_reservation_target.as_ref()
    }
}
impl CapacityReservationSpecification {
    /// Creates a new builder-style object to manufacture [`CapacityReservationSpecification`](crate::types::CapacityReservationSpecification).
    pub fn builder() -> crate::types::builders::CapacityReservationSpecificationBuilder {
        crate::types::builders::CapacityReservationSpecificationBuilder::default()
    }
}

/// A builder for [`CapacityReservationSpecification`](crate::types::CapacityReservationSpecification).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CapacityReservationSpecificationBuilder {
    pub(crate) capacity_reservation_preference: ::std::option::Option<crate::types::CapacityReservationPreference>,
    pub(crate) capacity_reservation_target: ::std::option::Option<crate::types::CapacityReservationTarget>,
}
impl CapacityReservationSpecificationBuilder {
    /// <p>Indicates the instance's Capacity Reservation preferences. Possible preferences include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The instance can run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p> </li>
    /// <li> <p> <code>none</code> - The instance avoids running in a Capacity Reservation even if one is available. The instance runs as an On-Demand Instance.</p> </li>
    /// </ul>
    pub fn capacity_reservation_preference(mut self, input: crate::types::CapacityReservationPreference) -> Self {
        self.capacity_reservation_preference = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates the instance's Capacity Reservation preferences. Possible preferences include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The instance can run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p> </li>
    /// <li> <p> <code>none</code> - The instance avoids running in a Capacity Reservation even if one is available. The instance runs as an On-Demand Instance.</p> </li>
    /// </ul>
    pub fn set_capacity_reservation_preference(mut self, input: ::std::option::Option<crate::types::CapacityReservationPreference>) -> Self {
        self.capacity_reservation_preference = input;
        self
    }
    /// <p>Indicates the instance's Capacity Reservation preferences. Possible preferences include:</p>
    /// <ul>
    /// <li> <p> <code>open</code> - The instance can run in any <code>open</code> Capacity Reservation that has matching attributes (instance type, platform, Availability Zone).</p> </li>
    /// <li> <p> <code>none</code> - The instance avoids running in a Capacity Reservation even if one is available. The instance runs as an On-Demand Instance.</p> </li>
    /// </ul>
    pub fn get_capacity_reservation_preference(&self) -> &::std::option::Option<crate::types::CapacityReservationPreference> {
        &self.capacity_reservation_preference
    }
    /// <p>Information about the target Capacity Reservation or Capacity Reservation group.</p>
    pub fn capacity_reservation_target(mut self, input: crate::types::CapacityReservationTarget) -> Self {
        self.capacity_reservation_target = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the target Capacity Reservation or Capacity Reservation group.</p>
    pub fn set_capacity_reservation_target(mut self, input: ::std::option::Option<crate::types::CapacityReservationTarget>) -> Self {
        self.capacity_reservation_target = input;
        self
    }
    /// <p>Information about the target Capacity Reservation or Capacity Reservation group.</p>
    pub fn get_capacity_reservation_target(&self) -> &::std::option::Option<crate::types::CapacityReservationTarget> {
        &self.capacity_reservation_target
    }
    /// Consumes the builder and constructs a [`CapacityReservationSpecification`](crate::types::CapacityReservationSpecification).
    pub fn build(self) -> crate::types::CapacityReservationSpecification {
        crate::types::CapacityReservationSpecification {
            capacity_reservation_preference: self.capacity_reservation_preference,
            capacity_reservation_target: self.capacity_reservation_target,
        }
    }
}