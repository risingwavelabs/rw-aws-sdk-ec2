// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ProvisionIpamByoasnOutput {
    /// <p>An ASN and BYOIP CIDR association.</p>
    pub byoasn: ::std::option::Option<crate::types::Byoasn>,
    _request_id: Option<String>,
}
impl ProvisionIpamByoasnOutput {
    /// <p>An ASN and BYOIP CIDR association.</p>
    pub fn byoasn(&self) -> ::std::option::Option<&crate::types::Byoasn> {
        self.byoasn.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ProvisionIpamByoasnOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ProvisionIpamByoasnOutput {
    /// Creates a new builder-style object to manufacture [`ProvisionIpamByoasnOutput`](crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput).
    pub fn builder() -> crate::operation::provision_ipam_byoasn::builders::ProvisionIpamByoasnOutputBuilder {
        crate::operation::provision_ipam_byoasn::builders::ProvisionIpamByoasnOutputBuilder::default()
    }
}

/// A builder for [`ProvisionIpamByoasnOutput`](crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ProvisionIpamByoasnOutputBuilder {
    pub(crate) byoasn: ::std::option::Option<crate::types::Byoasn>,
    _request_id: Option<String>,
}
impl ProvisionIpamByoasnOutputBuilder {
    /// <p>An ASN and BYOIP CIDR association.</p>
    pub fn byoasn(mut self, input: crate::types::Byoasn) -> Self {
        self.byoasn = ::std::option::Option::Some(input);
        self
    }
    /// <p>An ASN and BYOIP CIDR association.</p>
    pub fn set_byoasn(mut self, input: ::std::option::Option<crate::types::Byoasn>) -> Self {
        self.byoasn = input;
        self
    }
    /// <p>An ASN and BYOIP CIDR association.</p>
    pub fn get_byoasn(&self) -> &::std::option::Option<crate::types::Byoasn> {
        &self.byoasn
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ProvisionIpamByoasnOutput`](crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput).
    pub fn build(self) -> crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput {
        crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnOutput {
            byoasn: self.byoasn,
            _request_id: self._request_id,
        }
    }
}
