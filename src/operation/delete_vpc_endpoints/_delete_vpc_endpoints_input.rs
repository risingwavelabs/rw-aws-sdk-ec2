// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteVpcEndpointsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The IDs of the VPC endpoints.</p>
    pub vpc_endpoint_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DeleteVpcEndpointsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IDs of the VPC endpoints.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.vpc_endpoint_ids.is_none()`.
    pub fn vpc_endpoint_ids(&self) -> &[::std::string::String] {
        self.vpc_endpoint_ids.as_deref().unwrap_or_default()
    }
}
impl DeleteVpcEndpointsInput {
    /// Creates a new builder-style object to manufacture [`DeleteVpcEndpointsInput`](crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsInput).
    pub fn builder() -> crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsInputBuilder {
        crate::operation::delete_vpc_endpoints::builders::DeleteVpcEndpointsInputBuilder::default()
    }
}

/// A builder for [`DeleteVpcEndpointsInput`](crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteVpcEndpointsInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) vpc_endpoint_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl DeleteVpcEndpointsInputBuilder {
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
    /// Appends an item to `vpc_endpoint_ids`.
    ///
    /// To override the contents of this collection use [`set_vpc_endpoint_ids`](Self::set_vpc_endpoint_ids).
    ///
    /// <p>The IDs of the VPC endpoints.</p>
    pub fn vpc_endpoint_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.vpc_endpoint_ids.unwrap_or_default();
        v.push(input.into());
        self.vpc_endpoint_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the VPC endpoints.</p>
    pub fn set_vpc_endpoint_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.vpc_endpoint_ids = input;
        self
    }
    /// <p>The IDs of the VPC endpoints.</p>
    pub fn get_vpc_endpoint_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.vpc_endpoint_ids
    }
    /// Consumes the builder and constructs a [`DeleteVpcEndpointsInput`](crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::delete_vpc_endpoints::DeleteVpcEndpointsInput {
            dry_run: self.dry_run,
            vpc_endpoint_ids: self.vpc_endpoint_ids,
        })
    }
}