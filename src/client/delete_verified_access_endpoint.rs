// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVerifiedAccessEndpoint`](crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`verified_access_endpoint_id(impl Into<String>)`](crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointFluentBuilder::verified_access_endpoint_id) / [`set_verified_access_endpoint_id(Option<String>)`](crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointFluentBuilder::set_verified_access_endpoint_id):<br>required: **true**<br><p>The ID of the Verified Access endpoint.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DeleteVerifiedAccessEndpointOutput`](crate::operation::delete_verified_access_endpoint::DeleteVerifiedAccessEndpointOutput) with field(s):
    ///   - [`verified_access_endpoint(Option<VerifiedAccessEndpoint>)`](crate::operation::delete_verified_access_endpoint::DeleteVerifiedAccessEndpointOutput::verified_access_endpoint): <p>Details about the Verified Access endpoint.</p>
    /// - On failure, responds with [`SdkError<DeleteVerifiedAccessEndpointError>`](crate::operation::delete_verified_access_endpoint::DeleteVerifiedAccessEndpointError)
    pub fn delete_verified_access_endpoint(
        &self,
    ) -> crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointFluentBuilder {
        crate::operation::delete_verified_access_endpoint::builders::DeleteVerifiedAccessEndpointFluentBuilder::new(self.handle.clone())
    }
}
