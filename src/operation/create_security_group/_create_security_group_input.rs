// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateSecurityGroupInput {
    /// <p>A description for the security group.</p>
    /// <p>Constraints: Up to 255 characters in length</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The name of the security group.</p>
    /// <p>Constraints: Up to 255 characters in length. Cannot start with <code>sg-</code>.</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the VPC. Required for a nondefault VPC.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>The tags to assign to the security group.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl CreateSecurityGroupInput {
    /// <p>A description for the security group.</p>
    /// <p>Constraints: Up to 255 characters in length</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The name of the security group.</p>
    /// <p>Constraints: Up to 255 characters in length. Cannot start with <code>sg-</code>.</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The ID of the VPC. Required for a nondefault VPC.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The tags to assign to the security group.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl CreateSecurityGroupInput {
    /// Creates a new builder-style object to manufacture [`CreateSecurityGroupInput`](crate::operation::create_security_group::CreateSecurityGroupInput).
    pub fn builder() -> crate::operation::create_security_group::builders::CreateSecurityGroupInputBuilder {
        crate::operation::create_security_group::builders::CreateSecurityGroupInputBuilder::default()
    }
}

/// A builder for [`CreateSecurityGroupInput`](crate::operation::create_security_group::CreateSecurityGroupInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateSecurityGroupInputBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl CreateSecurityGroupInputBuilder {
    /// <p>A description for the security group.</p>
    /// <p>Constraints: Up to 255 characters in length</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    /// This field is required.
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the security group.</p>
    /// <p>Constraints: Up to 255 characters in length</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description for the security group.</p>
    /// <p>Constraints: Up to 255 characters in length</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The name of the security group.</p>
    /// <p>Constraints: Up to 255 characters in length. Cannot start with <code>sg-</code>.</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    /// This field is required.
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the security group.</p>
    /// <p>Constraints: Up to 255 characters in length. Cannot start with <code>sg-</code>.</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>The name of the security group.</p>
    /// <p>Constraints: Up to 255 characters in length. Cannot start with <code>sg-</code>.</p>
    /// <p>Valid characters: a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=&amp;;{}!$*</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_name
    }
    /// <p>The ID of the VPC. Required for a nondefault VPC.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC. Required for a nondefault VPC.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>The ID of the VPC. Required for a nondefault VPC.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_id
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the security group.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to assign to the security group.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to assign to the security group.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
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
    /// Consumes the builder and constructs a [`CreateSecurityGroupInput`](crate::operation::create_security_group::CreateSecurityGroupInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_security_group::CreateSecurityGroupInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::create_security_group::CreateSecurityGroupInput {
            description: self.description,
            group_name: self.group_name,
            vpc_id: self.vpc_id,
            tag_specifications: self.tag_specifications,
            dry_run: self.dry_run,
        })
    }
}
