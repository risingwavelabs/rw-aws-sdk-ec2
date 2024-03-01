// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`EnableIpamOrganizationAdminAccount`](crate::operation::enable_ipam_organization_admin_account::builders::EnableIpamOrganizationAdminAccountFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::enable_ipam_organization_admin_account::builders::EnableIpamOrganizationAdminAccountFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::enable_ipam_organization_admin_account::builders::EnableIpamOrganizationAdminAccountFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`delegated_admin_account_id(impl Into<String>)`](crate::operation::enable_ipam_organization_admin_account::builders::EnableIpamOrganizationAdminAccountFluentBuilder::delegated_admin_account_id) / [`set_delegated_admin_account_id(Option<String>)`](crate::operation::enable_ipam_organization_admin_account::builders::EnableIpamOrganizationAdminAccountFluentBuilder::set_delegated_admin_account_id):<br>required: **true**<br><p>The Organizations member account ID that you want to enable as the IPAM account.</p><br>
    /// - On success, responds with [`EnableIpamOrganizationAdminAccountOutput`](crate::operation::enable_ipam_organization_admin_account::EnableIpamOrganizationAdminAccountOutput) with field(s):
    ///   - [`success(Option<bool>)`](crate::operation::enable_ipam_organization_admin_account::EnableIpamOrganizationAdminAccountOutput::success): <p>The result of enabling the IPAM account.</p>
    /// - On failure, responds with [`SdkError<EnableIpamOrganizationAdminAccountError>`](crate::operation::enable_ipam_organization_admin_account::EnableIpamOrganizationAdminAccountError)
    pub fn enable_ipam_organization_admin_account(
        &self,
    ) -> crate::operation::enable_ipam_organization_admin_account::builders::EnableIpamOrganizationAdminAccountFluentBuilder {
        crate::operation::enable_ipam_organization_admin_account::builders::EnableIpamOrganizationAdminAccountFluentBuilder::new(self.handle.clone())
    }
}