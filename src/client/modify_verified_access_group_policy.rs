// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVerifiedAccessGroupPolicy`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`verified_access_group_id(impl Into<String>)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::verified_access_group_id) / [`set_verified_access_group_id(Option<String>)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::set_verified_access_group_id):<br>required: **true**<br><p>The ID of the Verified Access group.</p><br>
    ///   - [`policy_enabled(bool)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::policy_enabled) / [`set_policy_enabled(Option<bool>)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::set_policy_enabled):<br>required: **false**<br><p>The status of the Verified Access policy.</p><br>
    ///   - [`policy_document(impl Into<String>)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::policy_document) / [`set_policy_document(Option<String>)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::set_policy_document):<br>required: **false**<br><p>The Verified Access policy document.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`sse_specification(VerifiedAccessSseSpecificationRequest)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::sse_specification) / [`set_sse_specification(Option<VerifiedAccessSseSpecificationRequest>)`](crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::set_sse_specification):<br>required: **false**<br><p>The options for server side encryption.</p><br>
    /// - On success, responds with [`ModifyVerifiedAccessGroupPolicyOutput`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyOutput) with field(s):
    ///   - [`policy_enabled(Option<bool>)`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyOutput::policy_enabled): <p>The status of the Verified Access policy.</p>
    ///   - [`policy_document(Option<String>)`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyOutput::policy_document): <p>The Verified Access policy document.</p>
    ///   - [`sse_specification(Option<VerifiedAccessSseSpecificationResponse>)`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyOutput::sse_specification): <p>The options in use for server side encryption.</p>
    /// - On failure, responds with [`SdkError<ModifyVerifiedAccessGroupPolicyError>`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyError)
    pub fn modify_verified_access_group_policy(
        &self,
    ) -> crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder {
        crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyFluentBuilder::new(self.handle.clone())
    }
}