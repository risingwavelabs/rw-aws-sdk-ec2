// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVerifiedAccessInstance`](crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`verified_access_instance_id(impl Into<String>)`](crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder::verified_access_instance_id) / [`set_verified_access_instance_id(Option<String>)`](crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder::set_verified_access_instance_id):<br>required: **true**<br><p>The ID of the Verified Access instance.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder::set_description):<br>required: **false**<br><p>A description for the Verified Access instance.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    /// - On success, responds with [`ModifyVerifiedAccessInstanceOutput`](crate::operation::modify_verified_access_instance::ModifyVerifiedAccessInstanceOutput) with field(s):
    ///   - [`verified_access_instance(Option<VerifiedAccessInstance>)`](crate::operation::modify_verified_access_instance::ModifyVerifiedAccessInstanceOutput::verified_access_instance): <p>Details about the Verified Access instance.</p>
    /// - On failure, responds with [`SdkError<ModifyVerifiedAccessInstanceError>`](crate::operation::modify_verified_access_instance::ModifyVerifiedAccessInstanceError)
    pub fn modify_verified_access_instance(
        &self,
    ) -> crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder {
        crate::operation::modify_verified_access_instance::builders::ModifyVerifiedAccessInstanceFluentBuilder::new(self.handle.clone())
    }
}