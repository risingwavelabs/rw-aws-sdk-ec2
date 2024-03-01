// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DetachInternetGatewayInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the internet gateway.</p>
    pub internet_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the VPC.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
}
impl DetachInternetGatewayInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the internet gateway.</p>
    pub fn internet_gateway_id(&self) -> ::std::option::Option<&str> {
        self.internet_gateway_id.as_deref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
}
impl DetachInternetGatewayInput {
    /// Creates a new builder-style object to manufacture [`DetachInternetGatewayInput`](crate::operation::detach_internet_gateway::DetachInternetGatewayInput).
    pub fn builder() -> crate::operation::detach_internet_gateway::builders::DetachInternetGatewayInputBuilder {
        crate::operation::detach_internet_gateway::builders::DetachInternetGatewayInputBuilder::default()
    }
}

/// A builder for [`DetachInternetGatewayInput`](crate::operation::detach_internet_gateway::DetachInternetGatewayInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DetachInternetGatewayInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) internet_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
}
impl DetachInternetGatewayInputBuilder {
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
    /// <p>The ID of the internet gateway.</p>
    /// This field is required.
    pub fn internet_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.internet_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the internet gateway.</p>
    pub fn set_internet_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.internet_gateway_id = input;
        self
    }
    /// <p>The ID of the internet gateway.</p>
    pub fn get_internet_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.internet_gateway_id
    }
    /// <p>The ID of the VPC.</p>
    /// This field is required.
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>The ID of the VPC.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_id
    }
    /// Consumes the builder and constructs a [`DetachInternetGatewayInput`](crate::operation::detach_internet_gateway::DetachInternetGatewayInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::detach_internet_gateway::DetachInternetGatewayInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::detach_internet_gateway::DetachInternetGatewayInput {
            dry_run: self.dry_run,
            internet_gateway_id: self.internet_gateway_id,
            vpc_id: self.vpc_id,
        })
    }
}
