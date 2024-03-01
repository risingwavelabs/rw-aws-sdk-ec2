// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for ModifyReservedInstances.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyReservedInstancesInput {
    /// <p>The IDs of the Reserved Instances to modify.</p>
    pub reserved_instances_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>A unique, case-sensitive token you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The configuration settings for the Reserved Instances to modify.</p>
    pub target_configurations: ::std::option::Option<::std::vec::Vec<crate::types::ReservedInstancesConfiguration>>,
}
impl ModifyReservedInstancesInput {
    /// <p>The IDs of the Reserved Instances to modify.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.reserved_instances_ids.is_none()`.
    pub fn reserved_instances_ids(&self) -> &[::std::string::String] {
        self.reserved_instances_ids.as_deref().unwrap_or_default()
    }
    /// <p>A unique, case-sensitive token you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The configuration settings for the Reserved Instances to modify.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.target_configurations.is_none()`.
    pub fn target_configurations(&self) -> &[crate::types::ReservedInstancesConfiguration] {
        self.target_configurations.as_deref().unwrap_or_default()
    }
}
impl ModifyReservedInstancesInput {
    /// Creates a new builder-style object to manufacture [`ModifyReservedInstancesInput`](crate::operation::modify_reserved_instances::ModifyReservedInstancesInput).
    pub fn builder() -> crate::operation::modify_reserved_instances::builders::ModifyReservedInstancesInputBuilder {
        crate::operation::modify_reserved_instances::builders::ModifyReservedInstancesInputBuilder::default()
    }
}

/// A builder for [`ModifyReservedInstancesInput`](crate::operation::modify_reserved_instances::ModifyReservedInstancesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyReservedInstancesInputBuilder {
    pub(crate) reserved_instances_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) target_configurations: ::std::option::Option<::std::vec::Vec<crate::types::ReservedInstancesConfiguration>>,
}
impl ModifyReservedInstancesInputBuilder {
    /// Appends an item to `reserved_instances_ids`.
    ///
    /// To override the contents of this collection use [`set_reserved_instances_ids`](Self::set_reserved_instances_ids).
    ///
    /// <p>The IDs of the Reserved Instances to modify.</p>
    pub fn reserved_instances_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.reserved_instances_ids.unwrap_or_default();
        v.push(input.into());
        self.reserved_instances_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the Reserved Instances to modify.</p>
    pub fn set_reserved_instances_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.reserved_instances_ids = input;
        self
    }
    /// <p>The IDs of the Reserved Instances to modify.</p>
    pub fn get_reserved_instances_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.reserved_instances_ids
    }
    /// <p>A unique, case-sensitive token you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive token you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>A unique, case-sensitive token you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// Appends an item to `target_configurations`.
    ///
    /// To override the contents of this collection use [`set_target_configurations`](Self::set_target_configurations).
    ///
    /// <p>The configuration settings for the Reserved Instances to modify.</p>
    pub fn target_configurations(mut self, input: crate::types::ReservedInstancesConfiguration) -> Self {
        let mut v = self.target_configurations.unwrap_or_default();
        v.push(input);
        self.target_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The configuration settings for the Reserved Instances to modify.</p>
    pub fn set_target_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ReservedInstancesConfiguration>>) -> Self {
        self.target_configurations = input;
        self
    }
    /// <p>The configuration settings for the Reserved Instances to modify.</p>
    pub fn get_target_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ReservedInstancesConfiguration>> {
        &self.target_configurations
    }
    /// Consumes the builder and constructs a [`ModifyReservedInstancesInput`](crate::operation::modify_reserved_instances::ModifyReservedInstancesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_reserved_instances::ModifyReservedInstancesInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::modify_reserved_instances::ModifyReservedInstancesInput {
            reserved_instances_ids: self.reserved_instances_ids,
            client_token: self.client_token,
            target_configurations: self.target_configurations,
        })
    }
}
