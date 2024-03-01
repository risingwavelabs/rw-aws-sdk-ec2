// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteIpamPoolInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the pool to delete.</p>
    pub ipam_pool_id: ::std::option::Option<::std::string::String>,
    /// <p>Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.</p> <important>
    /// <p>You can only use this option to delete pools in the private scope or pools in the public scope with a source resource. A source resource is a resource used to provision CIDRs to a resource planning pool.</p>
    /// </important>
    pub cascade: ::std::option::Option<bool>,
}
impl DeleteIpamPoolInput {
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the pool to delete.</p>
    pub fn ipam_pool_id(&self) -> ::std::option::Option<&str> {
        self.ipam_pool_id.as_deref()
    }
    /// <p>Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.</p> <important>
    /// <p>You can only use this option to delete pools in the private scope or pools in the public scope with a source resource. A source resource is a resource used to provision CIDRs to a resource planning pool.</p>
    /// </important>
    pub fn cascade(&self) -> ::std::option::Option<bool> {
        self.cascade
    }
}
impl DeleteIpamPoolInput {
    /// Creates a new builder-style object to manufacture [`DeleteIpamPoolInput`](crate::operation::delete_ipam_pool::DeleteIpamPoolInput).
    pub fn builder() -> crate::operation::delete_ipam_pool::builders::DeleteIpamPoolInputBuilder {
        crate::operation::delete_ipam_pool::builders::DeleteIpamPoolInputBuilder::default()
    }
}

/// A builder for [`DeleteIpamPoolInput`](crate::operation::delete_ipam_pool::DeleteIpamPoolInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteIpamPoolInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) ipam_pool_id: ::std::option::Option<::std::string::String>,
    pub(crate) cascade: ::std::option::Option<bool>,
}
impl DeleteIpamPoolInputBuilder {
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
    /// <p>The ID of the pool to delete.</p>
    /// This field is required.
    pub fn ipam_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipam_pool_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the pool to delete.</p>
    pub fn set_ipam_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipam_pool_id = input;
        self
    }
    /// <p>The ID of the pool to delete.</p>
    pub fn get_ipam_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipam_pool_id
    }
    /// <p>Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.</p> <important>
    /// <p>You can only use this option to delete pools in the private scope or pools in the public scope with a source resource. A source resource is a resource used to provision CIDRs to a resource planning pool.</p>
    /// </important>
    pub fn cascade(mut self, input: bool) -> Self {
        self.cascade = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.</p> <important>
    /// <p>You can only use this option to delete pools in the private scope or pools in the public scope with a source resource. A source resource is a resource used to provision CIDRs to a resource planning pool.</p>
    /// </important>
    pub fn set_cascade(mut self, input: ::std::option::Option<bool>) -> Self {
        self.cascade = input;
        self
    }
    /// <p>Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.</p> <important>
    /// <p>You can only use this option to delete pools in the private scope or pools in the public scope with a source resource. A source resource is a resource used to provision CIDRs to a resource planning pool.</p>
    /// </important>
    pub fn get_cascade(&self) -> &::std::option::Option<bool> {
        &self.cascade
    }
    /// Consumes the builder and constructs a [`DeleteIpamPoolInput`](crate::operation::delete_ipam_pool::DeleteIpamPoolInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_ipam_pool::DeleteIpamPoolInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_ipam_pool::DeleteIpamPoolInput {
            dry_run: self.dry_run,
            ipam_pool_id: self.ipam_pool_id,
            cascade: self.cascade,
        })
    }
}
