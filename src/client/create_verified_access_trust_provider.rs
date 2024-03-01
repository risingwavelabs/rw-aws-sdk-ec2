// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVerifiedAccessTrustProvider`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`trust_provider_type(TrustProviderType)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::trust_provider_type) / [`set_trust_provider_type(Option<TrustProviderType>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_trust_provider_type):<br>required: **true**<br><p>The type of trust provider.</p><br>
    ///   - [`user_trust_provider_type(UserTrustProviderType)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::user_trust_provider_type) / [`set_user_trust_provider_type(Option<UserTrustProviderType>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_user_trust_provider_type):<br>required: **false**<br><p>The type of user-based trust provider. This parameter is required when the provider type is <code>user</code>.</p><br>
    ///   - [`device_trust_provider_type(DeviceTrustProviderType)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::device_trust_provider_type) / [`set_device_trust_provider_type(Option<DeviceTrustProviderType>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_device_trust_provider_type):<br>required: **false**<br><p>The type of device-based trust provider. This parameter is required when the provider type is <code>device</code>.</p><br>
    ///   - [`oidc_options(CreateVerifiedAccessTrustProviderOidcOptions)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::oidc_options) / [`set_oidc_options(Option<CreateVerifiedAccessTrustProviderOidcOptions>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_oidc_options):<br>required: **false**<br><p>The options for a OpenID Connect-compatible user-identity trust provider. This parameter is required when the provider type is <code>user</code>.</p><br>
    ///   - [`device_options(CreateVerifiedAccessTrustProviderDeviceOptions)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::device_options) / [`set_device_options(Option<CreateVerifiedAccessTrustProviderDeviceOptions>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_device_options):<br>required: **false**<br><p>The options for a device-based trust provider. This parameter is required when the provider type is <code>device</code>.</p><br>
    ///   - [`policy_reference_name(impl Into<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::policy_reference_name) / [`set_policy_reference_name(Option<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_policy_reference_name):<br>required: **true**<br><p>The identifier to be used when working with policy rules.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_description):<br>required: **false**<br><p>A description for the Verified Access trust provider.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to assign to the Verified Access trust provider.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`sse_specification(VerifiedAccessSseSpecificationRequest)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::sse_specification) / [`set_sse_specification(Option<VerifiedAccessSseSpecificationRequest>)`](crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::set_sse_specification):<br>required: **false**<br><p>The options for server side encryption.</p><br>
    /// - On success, responds with [`CreateVerifiedAccessTrustProviderOutput`](crate::operation::create_verified_access_trust_provider::CreateVerifiedAccessTrustProviderOutput) with field(s):
    ///   - [`verified_access_trust_provider(Option<VerifiedAccessTrustProvider>)`](crate::operation::create_verified_access_trust_provider::CreateVerifiedAccessTrustProviderOutput::verified_access_trust_provider): <p>Details about the Verified Access trust provider.</p>
    /// - On failure, responds with [`SdkError<CreateVerifiedAccessTrustProviderError>`](crate::operation::create_verified_access_trust_provider::CreateVerifiedAccessTrustProviderError)
    pub fn create_verified_access_trust_provider(
        &self,
    ) -> crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder {
        crate::operation::create_verified_access_trust_provider::builders::CreateVerifiedAccessTrustProviderFluentBuilder::new(self.handle.clone())
    }
}