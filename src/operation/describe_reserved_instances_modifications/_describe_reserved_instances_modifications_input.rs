// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for DescribeReservedInstancesModifications.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeReservedInstancesModificationsInput {
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>client-token</code> - The idempotency token for the modification request.</p> </li>
    /// <li> <p> <code>create-date</code> - The time when the modification request was created.</p> </li>
    /// <li> <p> <code>effective-date</code> - The time when the modification becomes effective.</p> </li>
    /// <li> <p> <code>modification-result.reserved-instances-id</code> - The ID for the Reserved Instances created as part of the modification request. This ID is only available when the status of the modification is <code>fulfilled</code>.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.availability-zone</code> - The Availability Zone for the new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-count </code> - The number of new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-type</code> - The instance type of the new Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances modified.</p> </li>
    /// <li> <p> <code>reserved-instances-modification-id</code> - The ID of the modification request.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instances modification request (<code>processing</code> | <code>fulfilled</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// <li> <p> <code>update-date</code> - The time when the modification request was last updated.</p> </li>
    /// </ul>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>IDs for the submitted modification request.</p>
    pub reserved_instances_modification_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The token to retrieve the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeReservedInstancesModificationsInput {
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>client-token</code> - The idempotency token for the modification request.</p> </li>
    /// <li> <p> <code>create-date</code> - The time when the modification request was created.</p> </li>
    /// <li> <p> <code>effective-date</code> - The time when the modification becomes effective.</p> </li>
    /// <li> <p> <code>modification-result.reserved-instances-id</code> - The ID for the Reserved Instances created as part of the modification request. This ID is only available when the status of the modification is <code>fulfilled</code>.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.availability-zone</code> - The Availability Zone for the new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-count </code> - The number of new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-type</code> - The instance type of the new Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances modified.</p> </li>
    /// <li> <p> <code>reserved-instances-modification-id</code> - The ID of the modification request.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instances modification request (<code>processing</code> | <code>fulfilled</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// <li> <p> <code>update-date</code> - The time when the modification request was last updated.</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>IDs for the submitted modification request.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.reserved_instances_modification_ids.is_none()`.
    pub fn reserved_instances_modification_ids(&self) -> &[::std::string::String] {
        self.reserved_instances_modification_ids.as_deref().unwrap_or_default()
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeReservedInstancesModificationsInput {
    /// Creates a new builder-style object to manufacture [`DescribeReservedInstancesModificationsInput`](crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsInput).
    pub fn builder() -> crate::operation::describe_reserved_instances_modifications::builders::DescribeReservedInstancesModificationsInputBuilder {
        crate::operation::describe_reserved_instances_modifications::builders::DescribeReservedInstancesModificationsInputBuilder::default()
    }
}

/// A builder for [`DescribeReservedInstancesModificationsInput`](crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeReservedInstancesModificationsInputBuilder {
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) reserved_instances_modification_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeReservedInstancesModificationsInputBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>client-token</code> - The idempotency token for the modification request.</p> </li>
    /// <li> <p> <code>create-date</code> - The time when the modification request was created.</p> </li>
    /// <li> <p> <code>effective-date</code> - The time when the modification becomes effective.</p> </li>
    /// <li> <p> <code>modification-result.reserved-instances-id</code> - The ID for the Reserved Instances created as part of the modification request. This ID is only available when the status of the modification is <code>fulfilled</code>.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.availability-zone</code> - The Availability Zone for the new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-count </code> - The number of new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-type</code> - The instance type of the new Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances modified.</p> </li>
    /// <li> <p> <code>reserved-instances-modification-id</code> - The ID of the modification request.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instances modification request (<code>processing</code> | <code>fulfilled</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// <li> <p> <code>update-date</code> - The time when the modification request was last updated.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>client-token</code> - The idempotency token for the modification request.</p> </li>
    /// <li> <p> <code>create-date</code> - The time when the modification request was created.</p> </li>
    /// <li> <p> <code>effective-date</code> - The time when the modification becomes effective.</p> </li>
    /// <li> <p> <code>modification-result.reserved-instances-id</code> - The ID for the Reserved Instances created as part of the modification request. This ID is only available when the status of the modification is <code>fulfilled</code>.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.availability-zone</code> - The Availability Zone for the new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-count </code> - The number of new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-type</code> - The instance type of the new Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances modified.</p> </li>
    /// <li> <p> <code>reserved-instances-modification-id</code> - The ID of the modification request.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instances modification request (<code>processing</code> | <code>fulfilled</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// <li> <p> <code>update-date</code> - The time when the modification request was last updated.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>client-token</code> - The idempotency token for the modification request.</p> </li>
    /// <li> <p> <code>create-date</code> - The time when the modification request was created.</p> </li>
    /// <li> <p> <code>effective-date</code> - The time when the modification becomes effective.</p> </li>
    /// <li> <p> <code>modification-result.reserved-instances-id</code> - The ID for the Reserved Instances created as part of the modification request. This ID is only available when the status of the modification is <code>fulfilled</code>.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.availability-zone</code> - The Availability Zone for the new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-count </code> - The number of new Reserved Instances.</p> </li>
    /// <li> <p> <code>modification-result.target-configuration.instance-type</code> - The instance type of the new Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances modified.</p> </li>
    /// <li> <p> <code>reserved-instances-modification-id</code> - The ID of the modification request.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instances modification request (<code>processing</code> | <code>fulfilled</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// <li> <p> <code>update-date</code> - The time when the modification request was last updated.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
    }
    /// Appends an item to `reserved_instances_modification_ids`.
    ///
    /// To override the contents of this collection use [`set_reserved_instances_modification_ids`](Self::set_reserved_instances_modification_ids).
    ///
    /// <p>IDs for the submitted modification request.</p>
    pub fn reserved_instances_modification_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.reserved_instances_modification_ids.unwrap_or_default();
        v.push(input.into());
        self.reserved_instances_modification_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>IDs for the submitted modification request.</p>
    pub fn set_reserved_instances_modification_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.reserved_instances_modification_ids = input;
        self
    }
    /// <p>IDs for the submitted modification request.</p>
    pub fn get_reserved_instances_modification_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.reserved_instances_modification_ids
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to retrieve the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`DescribeReservedInstancesModificationsInput`](crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_reserved_instances_modifications::DescribeReservedInstancesModificationsInput {
                filters: self.filters,
                reserved_instances_modification_ids: self.reserved_instances_modification_ids,
                next_token: self.next_token,
            },
        )
    }
}
