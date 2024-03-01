// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ReplaceNetworkAclAssociation`](crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`association_id(impl Into<String>)`](crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationFluentBuilder::association_id) / [`set_association_id(Option<String>)`](crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationFluentBuilder::set_association_id):<br>required: **true**<br><p>The ID of the current association between the original network ACL and the subnet.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`network_acl_id(impl Into<String>)`](crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationFluentBuilder::network_acl_id) / [`set_network_acl_id(Option<String>)`](crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationFluentBuilder::set_network_acl_id):<br>required: **true**<br><p>The ID of the new network ACL to associate with the subnet.</p><br>
    /// - On success, responds with [`ReplaceNetworkAclAssociationOutput`](crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationOutput) with field(s):
    ///   - [`new_association_id(Option<String>)`](crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationOutput::new_association_id): <p>The ID of the new association.</p>
    /// - On failure, responds with [`SdkError<ReplaceNetworkAclAssociationError>`](crate::operation::replace_network_acl_association::ReplaceNetworkAclAssociationError)
    pub fn replace_network_acl_association(
        &self,
    ) -> crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationFluentBuilder {
        crate::operation::replace_network_acl_association::builders::ReplaceNetworkAclAssociationFluentBuilder::new(self.handle.clone())
    }
}
