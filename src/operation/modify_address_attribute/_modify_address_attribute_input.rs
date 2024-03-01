// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyAddressAttributeInput {
    /// <p>[EC2-VPC] The allocation ID.</p>
    pub allocation_id: ::std::option::Option<::std::string::String>,
    /// <p>The domain name to modify for the IP address.</p>
    pub domain_name: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl ModifyAddressAttributeInput {
    /// <p>[EC2-VPC] The allocation ID.</p>
    pub fn allocation_id(&self) -> ::std::option::Option<&str> {
        self.allocation_id.as_deref()
    }
    /// <p>The domain name to modify for the IP address.</p>
    pub fn domain_name(&self) -> ::std::option::Option<&str> {
        self.domain_name.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl ModifyAddressAttributeInput {
    /// Creates a new builder-style object to manufacture [`ModifyAddressAttributeInput`](crate::operation::modify_address_attribute::ModifyAddressAttributeInput).
    pub fn builder() -> crate::operation::modify_address_attribute::builders::ModifyAddressAttributeInputBuilder {
        crate::operation::modify_address_attribute::builders::ModifyAddressAttributeInputBuilder::default()
    }
}

/// A builder for [`ModifyAddressAttributeInput`](crate::operation::modify_address_attribute::ModifyAddressAttributeInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyAddressAttributeInputBuilder {
    pub(crate) allocation_id: ::std::option::Option<::std::string::String>,
    pub(crate) domain_name: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl ModifyAddressAttributeInputBuilder {
    /// <p>[EC2-VPC] The allocation ID.</p>
    /// This field is required.
    pub fn allocation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.allocation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>[EC2-VPC] The allocation ID.</p>
    pub fn set_allocation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.allocation_id = input;
        self
    }
    /// <p>[EC2-VPC] The allocation ID.</p>
    pub fn get_allocation_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.allocation_id
    }
    /// <p>The domain name to modify for the IP address.</p>
    pub fn domain_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.domain_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The domain name to modify for the IP address.</p>
    pub fn set_domain_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.domain_name = input;
        self
    }
    /// <p>The domain name to modify for the IP address.</p>
    pub fn get_domain_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.domain_name
    }
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
    /// Consumes the builder and constructs a [`ModifyAddressAttributeInput`](crate::operation::modify_address_attribute::ModifyAddressAttributeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_address_attribute::ModifyAddressAttributeInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::modify_address_attribute::ModifyAddressAttributeInput {
            allocation_id: self.allocation_id,
            domain_name: self.domain_name,
            dry_run: self.dry_run,
        })
    }
}