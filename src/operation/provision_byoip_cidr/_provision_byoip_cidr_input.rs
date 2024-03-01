// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProvisionByoipCidrInput {
    /// <p>The public IPv4 or IPv6 address range, in CIDR notation. The most specific IPv4 prefix that you can specify is /24. The most specific IPv6 prefix you can specify is /56. The address range cannot overlap with another address range that you've brought to this or another Region.</p>
    pub cidr: ::std::option::Option<::std::string::String>,
    /// <p>A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP.</p>
    pub cidr_authorization_context: ::std::option::Option<crate::types::CidrAuthorizationContext>,
    /// <p>(IPv6 only) Indicate whether the address range will be publicly advertised to the internet.</p>
    /// <p>Default: true</p>
    pub publicly_advertisable: ::std::option::Option<bool>,
    /// <p>A description for the address range and the address pool.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The tags to apply to the address pool.</p>
    pub pool_tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Reserved.</p>
    pub multi_region: ::std::option::Option<bool>,
}
impl ProvisionByoipCidrInput {
    /// <p>The public IPv4 or IPv6 address range, in CIDR notation. The most specific IPv4 prefix that you can specify is /24. The most specific IPv6 prefix you can specify is /56. The address range cannot overlap with another address range that you've brought to this or another Region.</p>
    pub fn cidr(&self) -> ::std::option::Option<&str> {
        self.cidr.as_deref()
    }
    /// <p>A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP.</p>
    pub fn cidr_authorization_context(&self) -> ::std::option::Option<&crate::types::CidrAuthorizationContext> {
        self.cidr_authorization_context.as_ref()
    }
    /// <p>(IPv6 only) Indicate whether the address range will be publicly advertised to the internet.</p>
    /// <p>Default: true</p>
    pub fn publicly_advertisable(&self) -> ::std::option::Option<bool> {
        self.publicly_advertisable
    }
    /// <p>A description for the address range and the address pool.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The tags to apply to the address pool.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.pool_tag_specifications.is_none()`.
    pub fn pool_tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.pool_tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>Reserved.</p>
    pub fn multi_region(&self) -> ::std::option::Option<bool> {
        self.multi_region
    }
}
impl ProvisionByoipCidrInput {
    /// Creates a new builder-style object to manufacture [`ProvisionByoipCidrInput`](crate::operation::provision_byoip_cidr::ProvisionByoipCidrInput).
    pub fn builder() -> crate::operation::provision_byoip_cidr::builders::ProvisionByoipCidrInputBuilder {
        crate::operation::provision_byoip_cidr::builders::ProvisionByoipCidrInputBuilder::default()
    }
}

/// A builder for [`ProvisionByoipCidrInput`](crate::operation::provision_byoip_cidr::ProvisionByoipCidrInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ProvisionByoipCidrInputBuilder {
    pub(crate) cidr: ::std::option::Option<::std::string::String>,
    pub(crate) cidr_authorization_context: ::std::option::Option<crate::types::CidrAuthorizationContext>,
    pub(crate) publicly_advertisable: ::std::option::Option<bool>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) pool_tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) multi_region: ::std::option::Option<bool>,
}
impl ProvisionByoipCidrInputBuilder {
    /// <p>The public IPv4 or IPv6 address range, in CIDR notation. The most specific IPv4 prefix that you can specify is /24. The most specific IPv6 prefix you can specify is /56. The address range cannot overlap with another address range that you've brought to this or another Region.</p>
    /// This field is required.
    pub fn cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The public IPv4 or IPv6 address range, in CIDR notation. The most specific IPv4 prefix that you can specify is /24. The most specific IPv6 prefix you can specify is /56. The address range cannot overlap with another address range that you've brought to this or another Region.</p>
    pub fn set_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr = input;
        self
    }
    /// <p>The public IPv4 or IPv6 address range, in CIDR notation. The most specific IPv4 prefix that you can specify is /24. The most specific IPv6 prefix you can specify is /56. The address range cannot overlap with another address range that you've brought to this or another Region.</p>
    pub fn get_cidr(&self) -> &::std::option::Option<::std::string::String> {
        &self.cidr
    }
    /// <p>A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP.</p>
    pub fn cidr_authorization_context(mut self, input: crate::types::CidrAuthorizationContext) -> Self {
        self.cidr_authorization_context = ::std::option::Option::Some(input);
        self
    }
    /// <p>A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP.</p>
    pub fn set_cidr_authorization_context(mut self, input: ::std::option::Option<crate::types::CidrAuthorizationContext>) -> Self {
        self.cidr_authorization_context = input;
        self
    }
    /// <p>A signed document that proves that you are authorized to bring the specified IP address range to Amazon using BYOIP.</p>
    pub fn get_cidr_authorization_context(&self) -> &::std::option::Option<crate::types::CidrAuthorizationContext> {
        &self.cidr_authorization_context
    }
    /// <p>(IPv6 only) Indicate whether the address range will be publicly advertised to the internet.</p>
    /// <p>Default: true</p>
    pub fn publicly_advertisable(mut self, input: bool) -> Self {
        self.publicly_advertisable = ::std::option::Option::Some(input);
        self
    }
    /// <p>(IPv6 only) Indicate whether the address range will be publicly advertised to the internet.</p>
    /// <p>Default: true</p>
    pub fn set_publicly_advertisable(mut self, input: ::std::option::Option<bool>) -> Self {
        self.publicly_advertisable = input;
        self
    }
    /// <p>(IPv6 only) Indicate whether the address range will be publicly advertised to the internet.</p>
    /// <p>Default: true</p>
    pub fn get_publicly_advertisable(&self) -> &::std::option::Option<bool> {
        &self.publicly_advertisable
    }
    /// <p>A description for the address range and the address pool.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the address range and the address pool.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description for the address range and the address pool.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
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
    /// Appends an item to `pool_tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_pool_tag_specifications`](Self::set_pool_tag_specifications).
    ///
    /// <p>The tags to apply to the address pool.</p>
    pub fn pool_tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.pool_tag_specifications.unwrap_or_default();
        v.push(input);
        self.pool_tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the address pool.</p>
    pub fn set_pool_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.pool_tag_specifications = input;
        self
    }
    /// <p>The tags to apply to the address pool.</p>
    pub fn get_pool_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.pool_tag_specifications
    }
    /// <p>Reserved.</p>
    pub fn multi_region(mut self, input: bool) -> Self {
        self.multi_region = ::std::option::Option::Some(input);
        self
    }
    /// <p>Reserved.</p>
    pub fn set_multi_region(mut self, input: ::std::option::Option<bool>) -> Self {
        self.multi_region = input;
        self
    }
    /// <p>Reserved.</p>
    pub fn get_multi_region(&self) -> &::std::option::Option<bool> {
        &self.multi_region
    }
    /// Consumes the builder and constructs a [`ProvisionByoipCidrInput`](crate::operation::provision_byoip_cidr::ProvisionByoipCidrInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::provision_byoip_cidr::ProvisionByoipCidrInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::provision_byoip_cidr::ProvisionByoipCidrInput {
            cidr: self.cidr,
            cidr_authorization_context: self.cidr_authorization_context,
            publicly_advertisable: self.publicly_advertisable,
            description: self.description,
            dry_run: self.dry_run,
            pool_tag_specifications: self.pool_tag_specifications,
            multi_region: self.multi_region,
        })
    }
}