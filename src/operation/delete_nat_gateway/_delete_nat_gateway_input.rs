// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteNatGatewayInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the NAT gateway.</p>
    pub nat_gateway_id: ::std::option::Option<::std::string::String>,
}
impl DeleteNatGatewayInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the NAT gateway.</p>
    pub fn nat_gateway_id(&self) -> ::std::option::Option<&str> {
        self.nat_gateway_id.as_deref()
    }
}
impl DeleteNatGatewayInput {
    /// Creates a new builder-style object to manufacture [`DeleteNatGatewayInput`](crate::operation::delete_nat_gateway::DeleteNatGatewayInput).
    pub fn builder() -> crate::operation::delete_nat_gateway::builders::DeleteNatGatewayInputBuilder {
        crate::operation::delete_nat_gateway::builders::DeleteNatGatewayInputBuilder::default()
    }
}

/// A builder for [`DeleteNatGatewayInput`](crate::operation::delete_nat_gateway::DeleteNatGatewayInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteNatGatewayInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) nat_gateway_id: ::std::option::Option<::std::string::String>,
}
impl DeleteNatGatewayInputBuilder {
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
    /// <p>The ID of the NAT gateway.</p>
    /// This field is required.
    pub fn nat_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.nat_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the NAT gateway.</p>
    pub fn set_nat_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.nat_gateway_id = input;
        self
    }
    /// <p>The ID of the NAT gateway.</p>
    pub fn get_nat_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.nat_gateway_id
    }
    /// Consumes the builder and constructs a [`DeleteNatGatewayInput`](crate::operation::delete_nat_gateway::DeleteNatGatewayInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_nat_gateway::DeleteNatGatewayInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_nat_gateway::DeleteNatGatewayInput {
            dry_run: self.dry_run,
            nat_gateway_id: self.nat_gateway_id,
        })
    }
}
