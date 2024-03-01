// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AcceptVpcPeeringConnectionInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the VPC peering connection. You must specify this parameter in the request.</p>
    pub vpc_peering_connection_id: ::std::option::Option<::std::string::String>,
}
impl AcceptVpcPeeringConnectionInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the VPC peering connection. You must specify this parameter in the request.</p>
    pub fn vpc_peering_connection_id(&self) -> ::std::option::Option<&str> {
        self.vpc_peering_connection_id.as_deref()
    }
}
impl AcceptVpcPeeringConnectionInput {
    /// Creates a new builder-style object to manufacture [`AcceptVpcPeeringConnectionInput`](crate::operation::accept_vpc_peering_connection::AcceptVpcPeeringConnectionInput).
    pub fn builder() -> crate::operation::accept_vpc_peering_connection::builders::AcceptVpcPeeringConnectionInputBuilder {
        crate::operation::accept_vpc_peering_connection::builders::AcceptVpcPeeringConnectionInputBuilder::default()
    }
}

/// A builder for [`AcceptVpcPeeringConnectionInput`](crate::operation::accept_vpc_peering_connection::AcceptVpcPeeringConnectionInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AcceptVpcPeeringConnectionInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) vpc_peering_connection_id: ::std::option::Option<::std::string::String>,
}
impl AcceptVpcPeeringConnectionInputBuilder {
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
    /// <p>The ID of the VPC peering connection. You must specify this parameter in the request.</p>
    /// This field is required.
    pub fn vpc_peering_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_peering_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC peering connection. You must specify this parameter in the request.</p>
    pub fn set_vpc_peering_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_peering_connection_id = input;
        self
    }
    /// <p>The ID of the VPC peering connection. You must specify this parameter in the request.</p>
    pub fn get_vpc_peering_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_peering_connection_id
    }
    /// Consumes the builder and constructs a [`AcceptVpcPeeringConnectionInput`](crate::operation::accept_vpc_peering_connection::AcceptVpcPeeringConnectionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::accept_vpc_peering_connection::AcceptVpcPeeringConnectionInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::accept_vpc_peering_connection::AcceptVpcPeeringConnectionInput {
            dry_run: self.dry_run,
            vpc_peering_connection_id: self.vpc_peering_connection_id,
        })
    }
}