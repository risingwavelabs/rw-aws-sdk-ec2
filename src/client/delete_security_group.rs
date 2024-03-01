// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteSecurityGroup`](crate::operation::delete_security_group::builders::DeleteSecurityGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`group_id(impl Into<String>)`](crate::operation::delete_security_group::builders::DeleteSecurityGroupFluentBuilder::group_id) / [`set_group_id(Option<String>)`](crate::operation::delete_security_group::builders::DeleteSecurityGroupFluentBuilder::set_group_id):<br>required: **false**<br><p>The ID of the security group.</p><br>
    ///   - [`group_name(impl Into<String>)`](crate::operation::delete_security_group::builders::DeleteSecurityGroupFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::delete_security_group::builders::DeleteSecurityGroupFluentBuilder::set_group_name):<br>required: **false**<br><p>[Default VPC] The name of the security group. You can specify either the security group name or the security group ID. For security groups in a nondefault VPC, you must specify the security group ID.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::delete_security_group::builders::DeleteSecurityGroupFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_security_group::builders::DeleteSecurityGroupFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DeleteSecurityGroupOutput`](crate::operation::delete_security_group::DeleteSecurityGroupOutput)
    /// - On failure, responds with [`SdkError<DeleteSecurityGroupError>`](crate::operation::delete_security_group::DeleteSecurityGroupError)
    pub fn delete_security_group(&self) -> crate::operation::delete_security_group::builders::DeleteSecurityGroupFluentBuilder {
        crate::operation::delete_security_group::builders::DeleteSecurityGroupFluentBuilder::new(self.handle.clone())
    }
}