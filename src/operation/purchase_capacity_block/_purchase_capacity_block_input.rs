// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PurchaseCapacityBlockInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The tags to apply to the Capacity Block during launch.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>The ID of the Capacity Block offering.</p>
    pub capacity_block_offering_id: ::std::option::Option<::std::string::String>,
    /// <p>The type of operating system for which to reserve capacity.</p>
    pub instance_platform: ::std::option::Option<crate::types::CapacityReservationInstancePlatform>,
}
impl PurchaseCapacityBlockInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The tags to apply to the Capacity Block during launch.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>The ID of the Capacity Block offering.</p>
    pub fn capacity_block_offering_id(&self) -> ::std::option::Option<&str> {
        self.capacity_block_offering_id.as_deref()
    }
    /// <p>The type of operating system for which to reserve capacity.</p>
    pub fn instance_platform(&self) -> ::std::option::Option<&crate::types::CapacityReservationInstancePlatform> {
        self.instance_platform.as_ref()
    }
}
impl PurchaseCapacityBlockInput {
    /// Creates a new builder-style object to manufacture [`PurchaseCapacityBlockInput`](crate::operation::purchase_capacity_block::PurchaseCapacityBlockInput).
    pub fn builder() -> crate::operation::purchase_capacity_block::builders::PurchaseCapacityBlockInputBuilder {
        crate::operation::purchase_capacity_block::builders::PurchaseCapacityBlockInputBuilder::default()
    }
}

/// A builder for [`PurchaseCapacityBlockInput`](crate::operation::purchase_capacity_block::PurchaseCapacityBlockInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PurchaseCapacityBlockInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) capacity_block_offering_id: ::std::option::Option<::std::string::String>,
    pub(crate) instance_platform: ::std::option::Option<crate::types::CapacityReservationInstancePlatform>,
}
impl PurchaseCapacityBlockInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the Capacity Block during launch.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the Capacity Block during launch.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to apply to the Capacity Block during launch.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
    /// <p>The ID of the Capacity Block offering.</p>
    /// This field is required.
    pub fn capacity_block_offering_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.capacity_block_offering_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Capacity Block offering.</p>
    pub fn set_capacity_block_offering_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.capacity_block_offering_id = input;
        self
    }
    /// <p>The ID of the Capacity Block offering.</p>
    pub fn get_capacity_block_offering_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.capacity_block_offering_id
    }
    /// <p>The type of operating system for which to reserve capacity.</p>
    /// This field is required.
    pub fn instance_platform(mut self, input: crate::types::CapacityReservationInstancePlatform) -> Self {
        self.instance_platform = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of operating system for which to reserve capacity.</p>
    pub fn set_instance_platform(mut self, input: ::std::option::Option<crate::types::CapacityReservationInstancePlatform>) -> Self {
        self.instance_platform = input;
        self
    }
    /// <p>The type of operating system for which to reserve capacity.</p>
    pub fn get_instance_platform(&self) -> &::std::option::Option<crate::types::CapacityReservationInstancePlatform> {
        &self.instance_platform
    }
    /// Consumes the builder and constructs a [`PurchaseCapacityBlockInput`](crate::operation::purchase_capacity_block::PurchaseCapacityBlockInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::purchase_capacity_block::PurchaseCapacityBlockInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::purchase_capacity_block::PurchaseCapacityBlockInput {
            dry_run: self.dry_run,
            tag_specifications: self.tag_specifications,
            capacity_block_offering_id: self.capacity_block_offering_id,
            instance_platform: self.instance_platform,
        })
    }
}