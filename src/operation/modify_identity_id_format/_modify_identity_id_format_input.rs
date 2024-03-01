// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyIdentityIdFormatInput {
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    pub principal_arn: ::std::option::Option<::std::string::String>,
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    pub resource: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    pub use_long_ids: ::std::option::Option<bool>,
}
impl ModifyIdentityIdFormatInput {
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    pub fn principal_arn(&self) -> ::std::option::Option<&str> {
        self.principal_arn.as_deref()
    }
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    pub fn resource(&self) -> ::std::option::Option<&str> {
        self.resource.as_deref()
    }
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    pub fn use_long_ids(&self) -> ::std::option::Option<bool> {
        self.use_long_ids
    }
}
impl ModifyIdentityIdFormatInput {
    /// Creates a new builder-style object to manufacture [`ModifyIdentityIdFormatInput`](crate::operation::modify_identity_id_format::ModifyIdentityIdFormatInput).
    pub fn builder() -> crate::operation::modify_identity_id_format::builders::ModifyIdentityIdFormatInputBuilder {
        crate::operation::modify_identity_id_format::builders::ModifyIdentityIdFormatInputBuilder::default()
    }
}

/// A builder for [`ModifyIdentityIdFormatInput`](crate::operation::modify_identity_id_format::ModifyIdentityIdFormatInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyIdentityIdFormatInputBuilder {
    pub(crate) principal_arn: ::std::option::Option<::std::string::String>,
    pub(crate) resource: ::std::option::Option<::std::string::String>,
    pub(crate) use_long_ids: ::std::option::Option<bool>,
}
impl ModifyIdentityIdFormatInputBuilder {
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    /// This field is required.
    pub fn principal_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.principal_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    pub fn set_principal_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.principal_arn = input;
        self
    }
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    pub fn get_principal_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.principal_arn
    }
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    /// This field is required.
    pub fn resource(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource = input;
        self
    }
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    pub fn get_resource(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource
    }
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    /// This field is required.
    pub fn use_long_ids(mut self, input: bool) -> Self {
        self.use_long_ids = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    pub fn set_use_long_ids(mut self, input: ::std::option::Option<bool>) -> Self {
        self.use_long_ids = input;
        self
    }
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    pub fn get_use_long_ids(&self) -> &::std::option::Option<bool> {
        &self.use_long_ids
    }
    /// Consumes the builder and constructs a [`ModifyIdentityIdFormatInput`](crate::operation::modify_identity_id_format::ModifyIdentityIdFormatInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_identity_id_format::ModifyIdentityIdFormatInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::modify_identity_id_format::ModifyIdentityIdFormatInput {
            principal_arn: self.principal_arn,
            resource: self.resource,
            use_long_ids: self.use_long_ids,
        })
    }
}
