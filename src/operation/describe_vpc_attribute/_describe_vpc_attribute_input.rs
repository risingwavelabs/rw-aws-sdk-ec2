// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVpcAttributeInput {
    /// <p>The VPC attribute.</p>
    pub attribute: ::std::option::Option<crate::types::VpcAttributeName>,
    /// <p>The ID of the VPC.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DescribeVpcAttributeInput {
    /// <p>The VPC attribute.</p>
    pub fn attribute(&self) -> ::std::option::Option<&crate::types::VpcAttributeName> {
        self.attribute.as_ref()
    }
    /// <p>The ID of the VPC.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeVpcAttributeInput {
    /// Creates a new builder-style object to manufacture [`DescribeVpcAttributeInput`](crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput).
    pub fn builder() -> crate::operation::describe_vpc_attribute::builders::DescribeVpcAttributeInputBuilder {
        crate::operation::describe_vpc_attribute::builders::DescribeVpcAttributeInputBuilder::default()
    }
}

/// A builder for [`DescribeVpcAttributeInput`](crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeVpcAttributeInputBuilder {
    pub(crate) attribute: ::std::option::Option<crate::types::VpcAttributeName>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DescribeVpcAttributeInputBuilder {
    /// <p>The VPC attribute.</p>
    /// This field is required.
    pub fn attribute(mut self, input: crate::types::VpcAttributeName) -> Self {
        self.attribute = ::std::option::Option::Some(input);
        self
    }
    /// <p>The VPC attribute.</p>
    pub fn set_attribute(mut self, input: ::std::option::Option<crate::types::VpcAttributeName>) -> Self {
        self.attribute = input;
        self
    }
    /// <p>The VPC attribute.</p>
    pub fn get_attribute(&self) -> &::std::option::Option<crate::types::VpcAttributeName> {
        &self.attribute
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
    /// Consumes the builder and constructs a [`DescribeVpcAttributeInput`](crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::describe_vpc_attribute::DescribeVpcAttributeInput {
            attribute: self.attribute,
            vpc_id: self.vpc_id,
            dry_run: self.dry_run,
        })
    }
}