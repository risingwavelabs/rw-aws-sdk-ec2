// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a security group and Amazon Web Services account ID pair.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UserIdGroupPair {
    /// <p>A description for the security group rule that references this user ID group pair.</p>
    /// <p>Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=;{}!$*</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the security group.</p>
    pub group_id: ::std::option::Option<::std::string::String>,
    /// <p>[Default VPC] The name of the security group. For a security group in a nondefault VPC, use the security group ID. </p>
    /// <p>For a referenced security group in another VPC, this value is not returned if the referenced security group is deleted.</p>
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The status of a VPC peering connection, if applicable.</p>
    pub peering_status: ::std::option::Option<::std::string::String>,
    /// <p>The ID of an Amazon Web Services account.</p>
    /// <p>For a referenced security group in another VPC, the account ID of the referenced security group is returned in the response. If the referenced security group is deleted, this value is not returned.</p>
    pub user_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the VPC for the referenced security group, if applicable.</p>
    pub vpc_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the VPC peering connection, if applicable.</p>
    pub vpc_peering_connection_id: ::std::option::Option<::std::string::String>,
}
impl UserIdGroupPair {
    /// <p>A description for the security group rule that references this user ID group pair.</p>
    /// <p>Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=;{}!$*</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The ID of the security group.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>[Default VPC] The name of the security group. For a security group in a nondefault VPC, use the security group ID. </p>
    /// <p>For a referenced security group in another VPC, this value is not returned if the referenced security group is deleted.</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The status of a VPC peering connection, if applicable.</p>
    pub fn peering_status(&self) -> ::std::option::Option<&str> {
        self.peering_status.as_deref()
    }
    /// <p>The ID of an Amazon Web Services account.</p>
    /// <p>For a referenced security group in another VPC, the account ID of the referenced security group is returned in the response. If the referenced security group is deleted, this value is not returned.</p>
    pub fn user_id(&self) -> ::std::option::Option<&str> {
        self.user_id.as_deref()
    }
    /// <p>The ID of the VPC for the referenced security group, if applicable.</p>
    pub fn vpc_id(&self) -> ::std::option::Option<&str> {
        self.vpc_id.as_deref()
    }
    /// <p>The ID of the VPC peering connection, if applicable.</p>
    pub fn vpc_peering_connection_id(&self) -> ::std::option::Option<&str> {
        self.vpc_peering_connection_id.as_deref()
    }
}
impl UserIdGroupPair {
    /// Creates a new builder-style object to manufacture [`UserIdGroupPair`](crate::types::UserIdGroupPair).
    pub fn builder() -> crate::types::builders::UserIdGroupPairBuilder {
        crate::types::builders::UserIdGroupPairBuilder::default()
    }
}

/// A builder for [`UserIdGroupPair`](crate::types::UserIdGroupPair).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct UserIdGroupPairBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) peering_status: ::std::option::Option<::std::string::String>,
    pub(crate) user_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpc_peering_connection_id: ::std::option::Option<::std::string::String>,
}
impl UserIdGroupPairBuilder {
    /// <p>A description for the security group rule that references this user ID group pair.</p>
    /// <p>Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=;{}!$*</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the security group rule that references this user ID group pair.</p>
    /// <p>Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=;{}!$*</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description for the security group rule that references this user ID group pair.</p>
    /// <p>Constraints: Up to 255 characters in length. Allowed characters are a-z, A-Z, 0-9, spaces, and ._-:/()#,@[]+=;{}!$*</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The ID of the security group.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>The ID of the security group.</p>
    pub fn get_group_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_id
    }
    /// <p>[Default VPC] The name of the security group. For a security group in a nondefault VPC, use the security group ID. </p>
    /// <p>For a referenced security group in another VPC, this value is not returned if the referenced security group is deleted.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>[Default VPC] The name of the security group. For a security group in a nondefault VPC, use the security group ID. </p>
    /// <p>For a referenced security group in another VPC, this value is not returned if the referenced security group is deleted.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>[Default VPC] The name of the security group. For a security group in a nondefault VPC, use the security group ID. </p>
    /// <p>For a referenced security group in another VPC, this value is not returned if the referenced security group is deleted.</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_name
    }
    /// <p>The status of a VPC peering connection, if applicable.</p>
    pub fn peering_status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.peering_status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The status of a VPC peering connection, if applicable.</p>
    pub fn set_peering_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.peering_status = input;
        self
    }
    /// <p>The status of a VPC peering connection, if applicable.</p>
    pub fn get_peering_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.peering_status
    }
    /// <p>The ID of an Amazon Web Services account.</p>
    /// <p>For a referenced security group in another VPC, the account ID of the referenced security group is returned in the response. If the referenced security group is deleted, this value is not returned.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of an Amazon Web Services account.</p>
    /// <p>For a referenced security group in another VPC, the account ID of the referenced security group is returned in the response. If the referenced security group is deleted, this value is not returned.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_id = input;
        self
    }
    /// <p>The ID of an Amazon Web Services account.</p>
    /// <p>For a referenced security group in another VPC, the account ID of the referenced security group is returned in the response. If the referenced security group is deleted, this value is not returned.</p>
    pub fn get_user_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.user_id
    }
    /// <p>The ID of the VPC for the referenced security group, if applicable.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC for the referenced security group, if applicable.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_id = input;
        self
    }
    /// <p>The ID of the VPC for the referenced security group, if applicable.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_id
    }
    /// <p>The ID of the VPC peering connection, if applicable.</p>
    pub fn vpc_peering_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpc_peering_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPC peering connection, if applicable.</p>
    pub fn set_vpc_peering_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpc_peering_connection_id = input;
        self
    }
    /// <p>The ID of the VPC peering connection, if applicable.</p>
    pub fn get_vpc_peering_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpc_peering_connection_id
    }
    /// Consumes the builder and constructs a [`UserIdGroupPair`](crate::types::UserIdGroupPair).
    pub fn build(self) -> crate::types::UserIdGroupPair {
        crate::types::UserIdGroupPair {
            description: self.description,
            group_id: self.group_id,
            group_name: self.group_name,
            peering_status: self.peering_status,
            user_id: self.user_id,
            vpc_id: self.vpc_id,
            vpc_peering_connection_id: self.vpc_peering_connection_id,
        }
    }
}
