// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetVerifiedAccessGroupPolicy`](crate::operation::get_verified_access_group_policy::builders::GetVerifiedAccessGroupPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`verified_access_group_id(impl Into<String>)`](crate::operation::get_verified_access_group_policy::builders::GetVerifiedAccessGroupPolicyFluentBuilder::verified_access_group_id) / [`set_verified_access_group_id(Option<String>)`](crate::operation::get_verified_access_group_policy::builders::GetVerifiedAccessGroupPolicyFluentBuilder::set_verified_access_group_id):<br>required: **true**<br><p>The ID of the Verified Access group.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::get_verified_access_group_policy::builders::GetVerifiedAccessGroupPolicyFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_verified_access_group_policy::builders::GetVerifiedAccessGroupPolicyFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`GetVerifiedAccessGroupPolicyOutput`](crate::operation::get_verified_access_group_policy::GetVerifiedAccessGroupPolicyOutput) with field(s):
    ///   - [`policy_enabled(Option<bool>)`](crate::operation::get_verified_access_group_policy::GetVerifiedAccessGroupPolicyOutput::policy_enabled): <p>The status of the Verified Access policy.</p>
    ///   - [`policy_document(Option<String>)`](crate::operation::get_verified_access_group_policy::GetVerifiedAccessGroupPolicyOutput::policy_document): <p>The Verified Access policy document.</p>
    /// - On failure, responds with [`SdkError<GetVerifiedAccessGroupPolicyError>`](crate::operation::get_verified_access_group_policy::GetVerifiedAccessGroupPolicyError)
    pub fn get_verified_access_group_policy(
        &self,
    ) -> crate::operation::get_verified_access_group_policy::builders::GetVerifiedAccessGroupPolicyFluentBuilder {
        crate::operation::get_verified_access_group_policy::builders::GetVerifiedAccessGroupPolicyFluentBuilder::new(self.handle.clone())
    }
}
