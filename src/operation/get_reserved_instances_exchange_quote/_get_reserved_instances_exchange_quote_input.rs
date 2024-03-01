// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for GetReservedInstanceExchangeQuote.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetReservedInstancesExchangeQuoteInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The IDs of the Convertible Reserved Instances to exchange.</p>
    pub reserved_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    pub target_configurations: ::std::option::Option<::std::vec::Vec<crate::types::TargetConfigurationRequest>>,
}
impl GetReservedInstancesExchangeQuoteInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IDs of the Convertible Reserved Instances to exchange.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.reserved_instance_ids.is_none()`.
    pub fn reserved_instance_ids(&self) -> &[::std::string::String] {
        self.reserved_instance_ids.as_deref().unwrap_or_default()
    }
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.target_configurations.is_none()`.
    pub fn target_configurations(&self) -> &[crate::types::TargetConfigurationRequest] {
        self.target_configurations.as_deref().unwrap_or_default()
    }
}
impl GetReservedInstancesExchangeQuoteInput {
    /// Creates a new builder-style object to manufacture [`GetReservedInstancesExchangeQuoteInput`](crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteInput).
    pub fn builder() -> crate::operation::get_reserved_instances_exchange_quote::builders::GetReservedInstancesExchangeQuoteInputBuilder {
        crate::operation::get_reserved_instances_exchange_quote::builders::GetReservedInstancesExchangeQuoteInputBuilder::default()
    }
}

/// A builder for [`GetReservedInstancesExchangeQuoteInput`](crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetReservedInstancesExchangeQuoteInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) reserved_instance_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) target_configurations: ::std::option::Option<::std::vec::Vec<crate::types::TargetConfigurationRequest>>,
}
impl GetReservedInstancesExchangeQuoteInputBuilder {
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
    /// Appends an item to `reserved_instance_ids`.
    ///
    /// To override the contents of this collection use [`set_reserved_instance_ids`](Self::set_reserved_instance_ids).
    ///
    /// <p>The IDs of the Convertible Reserved Instances to exchange.</p>
    pub fn reserved_instance_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.reserved_instance_ids.unwrap_or_default();
        v.push(input.into());
        self.reserved_instance_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the Convertible Reserved Instances to exchange.</p>
    pub fn set_reserved_instance_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.reserved_instance_ids = input;
        self
    }
    /// <p>The IDs of the Convertible Reserved Instances to exchange.</p>
    pub fn get_reserved_instance_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.reserved_instance_ids
    }
    /// Appends an item to `target_configurations`.
    ///
    /// To override the contents of this collection use [`set_target_configurations`](Self::set_target_configurations).
    ///
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    pub fn target_configurations(mut self, input: crate::types::TargetConfigurationRequest) -> Self {
        let mut v = self.target_configurations.unwrap_or_default();
        v.push(input);
        self.target_configurations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    pub fn set_target_configurations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TargetConfigurationRequest>>) -> Self {
        self.target_configurations = input;
        self
    }
    /// <p>The configuration of the target Convertible Reserved Instance to exchange for your current Convertible Reserved Instances.</p>
    pub fn get_target_configurations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TargetConfigurationRequest>> {
        &self.target_configurations
    }
    /// Consumes the builder and constructs a [`GetReservedInstancesExchangeQuoteInput`](crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_reserved_instances_exchange_quote::GetReservedInstancesExchangeQuoteInput {
                dry_run: self.dry_run,
                reserved_instance_ids: self.reserved_instance_ids,
                target_configurations: self.target_configurations,
            },
        )
    }
}
