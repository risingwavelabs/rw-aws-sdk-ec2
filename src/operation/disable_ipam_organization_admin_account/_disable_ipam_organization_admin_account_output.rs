// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableIpamOrganizationAdminAccountOutput {
    /// <p>The result of disabling the IPAM account.</p>
    pub success: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl DisableIpamOrganizationAdminAccountOutput {
    /// <p>The result of disabling the IPAM account.</p>
    pub fn success(&self) -> ::std::option::Option<bool> {
        self.success
    }
}
impl ::aws_types::request_id::RequestId for DisableIpamOrganizationAdminAccountOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisableIpamOrganizationAdminAccountOutput {
    /// Creates a new builder-style object to manufacture [`DisableIpamOrganizationAdminAccountOutput`](crate::operation::disable_ipam_organization_admin_account::DisableIpamOrganizationAdminAccountOutput).
    pub fn builder() -> crate::operation::disable_ipam_organization_admin_account::builders::DisableIpamOrganizationAdminAccountOutputBuilder {
        crate::operation::disable_ipam_organization_admin_account::builders::DisableIpamOrganizationAdminAccountOutputBuilder::default()
    }
}

/// A builder for [`DisableIpamOrganizationAdminAccountOutput`](crate::operation::disable_ipam_organization_admin_account::DisableIpamOrganizationAdminAccountOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisableIpamOrganizationAdminAccountOutputBuilder {
    pub(crate) success: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl DisableIpamOrganizationAdminAccountOutputBuilder {
    /// <p>The result of disabling the IPAM account.</p>
    pub fn success(mut self, input: bool) -> Self {
        self.success = ::std::option::Option::Some(input);
        self
    }
    /// <p>The result of disabling the IPAM account.</p>
    pub fn set_success(mut self, input: ::std::option::Option<bool>) -> Self {
        self.success = input;
        self
    }
    /// <p>The result of disabling the IPAM account.</p>
    pub fn get_success(&self) -> &::std::option::Option<bool> {
        &self.success
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisableIpamOrganizationAdminAccountOutput`](crate::operation::disable_ipam_organization_admin_account::DisableIpamOrganizationAdminAccountOutput).
    pub fn build(self) -> crate::operation::disable_ipam_organization_admin_account::DisableIpamOrganizationAdminAccountOutput {
        crate::operation::disable_ipam_organization_admin_account::DisableIpamOrganizationAdminAccountOutput {
            success: self.success,
            _request_id: self._request_id,
        }
    }
}
