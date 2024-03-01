// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAssociatedEnclaveCertificateIamRoles`](crate::operation::get_associated_enclave_certificate_iam_roles::builders::GetAssociatedEnclaveCertificateIamRolesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`certificate_arn(impl Into<String>)`](crate::operation::get_associated_enclave_certificate_iam_roles::builders::GetAssociatedEnclaveCertificateIamRolesFluentBuilder::certificate_arn) / [`set_certificate_arn(Option<String>)`](crate::operation::get_associated_enclave_certificate_iam_roles::builders::GetAssociatedEnclaveCertificateIamRolesFluentBuilder::set_certificate_arn):<br>required: **true**<br><p>The ARN of the ACM certificate for which to view the associated IAM roles, encryption keys, and Amazon S3 object information.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::get_associated_enclave_certificate_iam_roles::builders::GetAssociatedEnclaveCertificateIamRolesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_associated_enclave_certificate_iam_roles::builders::GetAssociatedEnclaveCertificateIamRolesFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`GetAssociatedEnclaveCertificateIamRolesOutput`](crate::operation::get_associated_enclave_certificate_iam_roles::GetAssociatedEnclaveCertificateIamRolesOutput) with field(s):
    ///   - [`associated_roles(Option<Vec::<AssociatedRole>>)`](crate::operation::get_associated_enclave_certificate_iam_roles::GetAssociatedEnclaveCertificateIamRolesOutput::associated_roles): <p>Information about the associated IAM roles.</p>
    /// - On failure, responds with [`SdkError<GetAssociatedEnclaveCertificateIamRolesError>`](crate::operation::get_associated_enclave_certificate_iam_roles::GetAssociatedEnclaveCertificateIamRolesError)
    pub fn get_associated_enclave_certificate_iam_roles(
        &self,
    ) -> crate::operation::get_associated_enclave_certificate_iam_roles::builders::GetAssociatedEnclaveCertificateIamRolesFluentBuilder {
        crate::operation::get_associated_enclave_certificate_iam_roles::builders::GetAssociatedEnclaveCertificateIamRolesFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
