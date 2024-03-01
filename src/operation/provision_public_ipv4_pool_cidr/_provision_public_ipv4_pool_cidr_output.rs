// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProvisionPublicIpv4PoolCidrOutput {
    /// <p>The ID of the pool that you want to provision the CIDR to.</p>
    pub pool_id: ::std::option::Option<::std::string::String>,
    /// <p>Information about the address range of the public IPv4 pool.</p>
    pub pool_address_range: ::std::option::Option<crate::types::PublicIpv4PoolRange>,
    _request_id: Option<String>,
}
impl ProvisionPublicIpv4PoolCidrOutput {
    /// <p>The ID of the pool that you want to provision the CIDR to.</p>
    pub fn pool_id(&self) -> ::std::option::Option<&str> {
        self.pool_id.as_deref()
    }
    /// <p>Information about the address range of the public IPv4 pool.</p>
    pub fn pool_address_range(&self) -> ::std::option::Option<&crate::types::PublicIpv4PoolRange> {
        self.pool_address_range.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ProvisionPublicIpv4PoolCidrOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ProvisionPublicIpv4PoolCidrOutput {
    /// Creates a new builder-style object to manufacture [`ProvisionPublicIpv4PoolCidrOutput`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput).
    pub fn builder() -> crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrOutputBuilder {
        crate::operation::provision_public_ipv4_pool_cidr::builders::ProvisionPublicIpv4PoolCidrOutputBuilder::default()
    }
}

/// A builder for [`ProvisionPublicIpv4PoolCidrOutput`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ProvisionPublicIpv4PoolCidrOutputBuilder {
    pub(crate) pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) pool_address_range: ::std::option::Option<crate::types::PublicIpv4PoolRange>,
    _request_id: Option<String>,
}
impl ProvisionPublicIpv4PoolCidrOutputBuilder {
    /// <p>The ID of the pool that you want to provision the CIDR to.</p>
    pub fn pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the pool that you want to provision the CIDR to.</p>
    pub fn set_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pool_id = input;
        self
    }
    /// <p>The ID of the pool that you want to provision the CIDR to.</p>
    pub fn get_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.pool_id
    }
    /// <p>Information about the address range of the public IPv4 pool.</p>
    pub fn pool_address_range(mut self, input: crate::types::PublicIpv4PoolRange) -> Self {
        self.pool_address_range = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the address range of the public IPv4 pool.</p>
    pub fn set_pool_address_range(mut self, input: ::std::option::Option<crate::types::PublicIpv4PoolRange>) -> Self {
        self.pool_address_range = input;
        self
    }
    /// <p>Information about the address range of the public IPv4 pool.</p>
    pub fn get_pool_address_range(&self) -> &::std::option::Option<crate::types::PublicIpv4PoolRange> {
        &self.pool_address_range
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ProvisionPublicIpv4PoolCidrOutput`](crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput).
    pub fn build(self) -> crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput {
        crate::operation::provision_public_ipv4_pool_cidr::ProvisionPublicIpv4PoolCidrOutput {
            pool_id: self.pool_id,
            pool_address_range: self.pool_address_range,
            _request_id: self._request_id,
        }
    }
}
