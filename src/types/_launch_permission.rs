// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a launch permission.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LaunchPermission {
    /// <p>The name of the group.</p>
    pub group: ::std::option::Option<crate::types::PermissionGroup>,
    /// <p>The Amazon Web Services account ID.</p>
    /// <p>Constraints: Up to 10 000 account IDs can be specified in a single request.</p>
    pub user_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of an organization.</p>
    pub organization_arn: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of an organizational unit (OU).</p>
    pub organizational_unit_arn: ::std::option::Option<::std::string::String>,
}
impl LaunchPermission {
    /// <p>The name of the group.</p>
    pub fn group(&self) -> ::std::option::Option<&crate::types::PermissionGroup> {
        self.group.as_ref()
    }
    /// <p>The Amazon Web Services account ID.</p>
    /// <p>Constraints: Up to 10 000 account IDs can be specified in a single request.</p>
    pub fn user_id(&self) -> ::std::option::Option<&str> {
        self.user_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of an organization.</p>
    pub fn organization_arn(&self) -> ::std::option::Option<&str> {
        self.organization_arn.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of an organizational unit (OU).</p>
    pub fn organizational_unit_arn(&self) -> ::std::option::Option<&str> {
        self.organizational_unit_arn.as_deref()
    }
}
impl LaunchPermission {
    /// Creates a new builder-style object to manufacture [`LaunchPermission`](crate::types::LaunchPermission).
    pub fn builder() -> crate::types::builders::LaunchPermissionBuilder {
        crate::types::builders::LaunchPermissionBuilder::default()
    }
}

/// A builder for [`LaunchPermission`](crate::types::LaunchPermission).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LaunchPermissionBuilder {
    pub(crate) group: ::std::option::Option<crate::types::PermissionGroup>,
    pub(crate) user_id: ::std::option::Option<::std::string::String>,
    pub(crate) organization_arn: ::std::option::Option<::std::string::String>,
    pub(crate) organizational_unit_arn: ::std::option::Option<::std::string::String>,
}
impl LaunchPermissionBuilder {
    /// <p>The name of the group.</p>
    pub fn group(mut self, input: crate::types::PermissionGroup) -> Self {
        self.group = ::std::option::Option::Some(input);
        self
    }
    /// <p>The name of the group.</p>
    pub fn set_group(mut self, input: ::std::option::Option<crate::types::PermissionGroup>) -> Self {
        self.group = input;
        self
    }
    /// <p>The name of the group.</p>
    pub fn get_group(&self) -> &::std::option::Option<crate::types::PermissionGroup> {
        &self.group
    }
    /// <p>The Amazon Web Services account ID.</p>
    /// <p>Constraints: Up to 10 000 account IDs can be specified in a single request.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID.</p>
    /// <p>Constraints: Up to 10 000 account IDs can be specified in a single request.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_id = input;
        self
    }
    /// <p>The Amazon Web Services account ID.</p>
    /// <p>Constraints: Up to 10 000 account IDs can be specified in a single request.</p>
    pub fn get_user_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.user_id
    }
    /// <p>The Amazon Resource Name (ARN) of an organization.</p>
    pub fn organization_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.organization_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an organization.</p>
    pub fn set_organization_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.organization_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an organization.</p>
    pub fn get_organization_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.organization_arn
    }
    /// <p>The Amazon Resource Name (ARN) of an organizational unit (OU).</p>
    pub fn organizational_unit_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.organizational_unit_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an organizational unit (OU).</p>
    pub fn set_organizational_unit_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.organizational_unit_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an organizational unit (OU).</p>
    pub fn get_organizational_unit_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.organizational_unit_arn
    }
    /// Consumes the builder and constructs a [`LaunchPermission`](crate::types::LaunchPermission).
    pub fn build(self) -> crate::types::LaunchPermission {
        crate::types::LaunchPermission {
            group: self.group,
            user_id: self.user_id,
            organization_arn: self.organization_arn,
            organizational_unit_arn: self.organizational_unit_arn,
        }
    }
}
