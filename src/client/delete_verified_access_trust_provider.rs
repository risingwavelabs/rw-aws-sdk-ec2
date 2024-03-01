// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVerifiedAccessTrustProvider`](crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`verified_access_trust_provider_id(impl Into<String>)`](crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderFluentBuilder::verified_access_trust_provider_id) / [`set_verified_access_trust_provider_id(Option<String>)`](crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderFluentBuilder::set_verified_access_trust_provider_id):<br>required: **true**<br><p>The ID of the Verified Access trust provider.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    /// - On success, responds with [`DeleteVerifiedAccessTrustProviderOutput`](crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput) with field(s):
    ///   - [`verified_access_trust_provider(Option<VerifiedAccessTrustProvider>)`](crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderOutput::verified_access_trust_provider): <p>Details about the Verified Access trust provider.</p>
    /// - On failure, responds with [`SdkError<DeleteVerifiedAccessTrustProviderError>`](crate::operation::delete_verified_access_trust_provider::DeleteVerifiedAccessTrustProviderError)
    pub fn delete_verified_access_trust_provider(
        &self,
    ) -> crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderFluentBuilder {
        crate::operation::delete_verified_access_trust_provider::builders::DeleteVerifiedAccessTrustProviderFluentBuilder::new(self.handle.clone())
    }
}
