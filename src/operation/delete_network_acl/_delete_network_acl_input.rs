// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteNetworkAclInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the network ACL.</p>
    pub network_acl_id: ::std::option::Option<::std::string::String>,
}
impl DeleteNetworkAclInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the network ACL.</p>
    pub fn network_acl_id(&self) -> ::std::option::Option<&str> {
        self.network_acl_id.as_deref()
    }
}
impl DeleteNetworkAclInput {
    /// Creates a new builder-style object to manufacture [`DeleteNetworkAclInput`](crate::operation::delete_network_acl::DeleteNetworkAclInput).
    pub fn builder() -> crate::operation::delete_network_acl::builders::DeleteNetworkAclInputBuilder {
        crate::operation::delete_network_acl::builders::DeleteNetworkAclInputBuilder::default()
    }
}

/// A builder for [`DeleteNetworkAclInput`](crate::operation::delete_network_acl::DeleteNetworkAclInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteNetworkAclInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) network_acl_id: ::std::option::Option<::std::string::String>,
}
impl DeleteNetworkAclInputBuilder {
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
    /// <p>The ID of the network ACL.</p>
    /// This field is required.
    pub fn network_acl_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_acl_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the network ACL.</p>
    pub fn set_network_acl_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_acl_id = input;
        self
    }
    /// <p>The ID of the network ACL.</p>
    pub fn get_network_acl_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.network_acl_id
    }
    /// Consumes the builder and constructs a [`DeleteNetworkAclInput`](crate::operation::delete_network_acl::DeleteNetworkAclInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_network_acl::DeleteNetworkAclInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_network_acl::DeleteNetworkAclInput {
            dry_run: self.dry_run,
            network_acl_id: self.network_acl_id,
        })
    }
}
