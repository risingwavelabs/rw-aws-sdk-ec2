// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeprovisionPublicIpv4PoolCidrInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the pool that you want to deprovision the CIDR from.</p>
    pub pool_id: ::std::option::Option<::std::string::String>,
    /// <p>The CIDR you want to deprovision from the pool. Enter the CIDR you want to deprovision with a netmask of <code>/32</code>. You must rerun this command for each IP address in the CIDR range. If your CIDR is a <code>/24</code>, you will have to run this command to deprovision each of the 256 IP addresses in the <code>/24</code> CIDR.</p>
    pub cidr: ::std::option::Option<::std::string::String>,
}
impl DeprovisionPublicIpv4PoolCidrInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the pool that you want to deprovision the CIDR from.</p>
    pub fn pool_id(&self) -> ::std::option::Option<&str> {
        self.pool_id.as_deref()
    }
    /// <p>The CIDR you want to deprovision from the pool. Enter the CIDR you want to deprovision with a netmask of <code>/32</code>. You must rerun this command for each IP address in the CIDR range. If your CIDR is a <code>/24</code>, you will have to run this command to deprovision each of the 256 IP addresses in the <code>/24</code> CIDR.</p>
    pub fn cidr(&self) -> ::std::option::Option<&str> {
        self.cidr.as_deref()
    }
}
impl DeprovisionPublicIpv4PoolCidrInput {
    /// Creates a new builder-style object to manufacture [`DeprovisionPublicIpv4PoolCidrInput`](crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrInput).
    pub fn builder() -> crate::operation::deprovision_public_ipv4_pool_cidr::builders::DeprovisionPublicIpv4PoolCidrInputBuilder {
        crate::operation::deprovision_public_ipv4_pool_cidr::builders::DeprovisionPublicIpv4PoolCidrInputBuilder::default()
    }
}

/// A builder for [`DeprovisionPublicIpv4PoolCidrInput`](crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeprovisionPublicIpv4PoolCidrInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) cidr: ::std::option::Option<::std::string::String>,
}
impl DeprovisionPublicIpv4PoolCidrInputBuilder {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// <p>The ID of the pool that you want to deprovision the CIDR from.</p>
    /// This field is required.
    pub fn pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the pool that you want to deprovision the CIDR from.</p>
    pub fn set_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.pool_id = input;
        self
    }
    /// <p>The ID of the pool that you want to deprovision the CIDR from.</p>
    pub fn get_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.pool_id
    }
    /// <p>The CIDR you want to deprovision from the pool. Enter the CIDR you want to deprovision with a netmask of <code>/32</code>. You must rerun this command for each IP address in the CIDR range. If your CIDR is a <code>/24</code>, you will have to run this command to deprovision each of the 256 IP addresses in the <code>/24</code> CIDR.</p>
    /// This field is required.
    pub fn cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The CIDR you want to deprovision from the pool. Enter the CIDR you want to deprovision with a netmask of <code>/32</code>. You must rerun this command for each IP address in the CIDR range. If your CIDR is a <code>/24</code>, you will have to run this command to deprovision each of the 256 IP addresses in the <code>/24</code> CIDR.</p>
    pub fn set_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr = input;
        self
    }
    /// <p>The CIDR you want to deprovision from the pool. Enter the CIDR you want to deprovision with a netmask of <code>/32</code>. You must rerun this command for each IP address in the CIDR range. If your CIDR is a <code>/24</code>, you will have to run this command to deprovision each of the 256 IP addresses in the <code>/24</code> CIDR.</p>
    pub fn get_cidr(&self) -> &::std::option::Option<::std::string::String> {
        &self.cidr
    }
    /// Consumes the builder and constructs a [`DeprovisionPublicIpv4PoolCidrInput`](crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::deprovision_public_ipv4_pool_cidr::DeprovisionPublicIpv4PoolCidrInput {
            dry_run: self.dry_run,
            pool_id: self.pool_id,
            cidr: self.cidr,
        })
    }
}
