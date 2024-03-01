// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AcceptVpcEndpointConnectionsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the VPC endpoint service.</p>
    pub service_id: ::std::option::Option<::std::string::String>,
    /// <p>The IDs of the interface VPC endpoints.</p>
    pub vpc_endpoint_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AcceptVpcEndpointConnectionsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the VPC endpoint service.</p>
    pub fn service_id(&self) -> ::std::option::Option<&str> {
        self.service_id.as_deref()
    }
    /// <p>The IDs of the interface VPC endpoints.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.vpc_endpoint_ids.is_none()`.
    pub fn vpc_endpoint_ids(&self) -> &[::std::string::String] {
        self.vpc_endpoint_ids.as_deref().unwrap_or_default()
    }
}
impl AcceptVpcEndpointConnectionsInput {
    /// Creates a new builder-style object to manufacture [`AcceptVpcEndpointConnectionsInput`](crate::operation::accept_vpc_endpoint_connections::AcceptVpcEndpointConnectionsInput).
    pub fn builder() -> crate::operation::accept_vpc_endpoint_connections::builders::AcceptVpcEndpointConnectionsInputBuilder {
        crate::operation::accept_vpc_endpoint_connections::builders::AcceptVpcEndpointConnectionsInputBuilder::default()
    }
}

/// A builder for [`AcceptVpcEndpointConnectionsInput`](crate::operation::accept_vpc_endpoint_connections::AcceptVpcEndpointConnectionsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AcceptVpcEndpointConnectionsInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) service_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_endpoint_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl AcceptVpcEndpointConnectionsInputBuilder {
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
    /// <p>The ID of the VPC endpoint service.</p>
    /// This field is required.
    pub fn service_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC endpoint service.</p>
    pub fn set_service_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_id = input;
        self
    }
    /// <p>The ID of the VPC endpoint service.</p>
    pub fn get_service_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.service_id
    }
    /// Appends an item to `vpc_endpoint_ids`.
    ///
    /// To override the contents of this collection use [`set_vpc_endpoint_ids`](Self::set_vpc_endpoint_ids).
    ///
    /// <p>The IDs of the interface VPC endpoints.</p>
    pub fn vpc_endpoint_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.vpc_endpoint_ids.unwrap_or_default();
        v.push(input.into());
        self.vpc_endpoint_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the interface VPC endpoints.</p>
    pub fn set_vpc_endpoint_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.vpc_endpoint_ids = input;
        self
    }
    /// <p>The IDs of the interface VPC endpoints.</p>
    pub fn get_vpc_endpoint_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.vpc_endpoint_ids
    }
    /// Consumes the builder and constructs a [`AcceptVpcEndpointConnectionsInput`](crate::operation::accept_vpc_endpoint_connections::AcceptVpcEndpointConnectionsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::accept_vpc_endpoint_connections::AcceptVpcEndpointConnectionsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::accept_vpc_endpoint_connections::AcceptVpcEndpointConnectionsInput {
            dry_run: self.dry_run,
            service_id: self.service_id,
            vpc_endpoint_ids: self.vpc_endpoint_ids,
        })
    }
}
