// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReleaseAddressInput {
    /// <p>The allocation ID. This parameter is required.</p>
    pub allocation_id: ::std::option::Option<::std::string::String>,
    /// <p>Deprecated.</p>
    pub public_ip: ::std::option::Option<::std::string::String>,
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    pub network_border_group: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl ReleaseAddressInput {
    /// <p>The allocation ID. This parameter is required.</p>
    pub fn allocation_id(&self) -> ::std::option::Option<&str> {
        self.allocation_id.as_deref()
    }
    /// <p>Deprecated.</p>
    pub fn public_ip(&self) -> ::std::option::Option<&str> {
        self.public_ip.as_deref()
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    pub fn network_border_group(&self) -> ::std::option::Option<&str> {
        self.network_border_group.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl ReleaseAddressInput {
    /// Creates a new builder-style object to manufacture [`ReleaseAddressInput`](crate::operation::release_address::ReleaseAddressInput).
    pub fn builder() -> crate::operation::release_address::builders::ReleaseAddressInputBuilder {
        crate::operation::release_address::builders::ReleaseAddressInputBuilder::default()
    }
}

/// A builder for [`ReleaseAddressInput`](crate::operation::release_address::ReleaseAddressInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ReleaseAddressInputBuilder {
    pub(crate) allocation_id: ::std::option::Option<::std::string::String>,
    pub(crate) public_ip: ::std::option::Option<::std::string::String>,
    pub(crate) network_border_group: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl ReleaseAddressInputBuilder {
    /// <p>The allocation ID. This parameter is required.</p>
    pub fn allocation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.allocation_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The allocation ID. This parameter is required.</p>
    pub fn set_allocation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.allocation_id = input;
        self
    }
    /// <p>The allocation ID. This parameter is required.</p>
    pub fn get_allocation_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.allocation_id
    }
    /// <p>Deprecated.</p>
    pub fn public_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.public_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Deprecated.</p>
    pub fn set_public_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.public_ip = input;
        self
    }
    /// <p>Deprecated.</p>
    pub fn get_public_ip(&self) -> &::std::option::Option<::std::string::String> {
        &self.public_ip
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    pub fn network_border_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_border_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    pub fn set_network_border_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_border_group = input;
        self
    }
    /// <p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>
    /// <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p>
    pub fn get_network_border_group(&self) -> &::std::option::Option<::std::string::String> {
        &self.network_border_group
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
    /// Consumes the builder and constructs a [`ReleaseAddressInput`](crate::operation::release_address::ReleaseAddressInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::release_address::ReleaseAddressInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::release_address::ReleaseAddressInput {
            allocation_id: self.allocation_id,
            public_ip: self.public_ip,
            network_border_group: self.network_border_group,
            dry_run: self.dry_run,
        })
    }
}