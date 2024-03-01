// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeprovisionIpamByoasnInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The IPAM ID.</p>
    pub ipam_id: ::std::option::Option<::std::string::String>,
    /// <p>An ASN.</p>
    pub asn: ::std::option::Option<::std::string::String>,
}
impl DeprovisionIpamByoasnInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IPAM ID.</p>
    pub fn ipam_id(&self) -> ::std::option::Option<&str> {
        self.ipam_id.as_deref()
    }
    /// <p>An ASN.</p>
    pub fn asn(&self) -> ::std::option::Option<&str> {
        self.asn.as_deref()
    }
}
impl DeprovisionIpamByoasnInput {
    /// Creates a new builder-style object to manufacture [`DeprovisionIpamByoasnInput`](crate::operation::deprovision_ipam_byoasn::DeprovisionIpamByoasnInput).
    pub fn builder() -> crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnInputBuilder {
        crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnInputBuilder::default()
    }
}

/// A builder for [`DeprovisionIpamByoasnInput`](crate::operation::deprovision_ipam_byoasn::DeprovisionIpamByoasnInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeprovisionIpamByoasnInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) ipam_id: ::std::option::Option<::std::string::String>,
    pub(crate) asn: ::std::option::Option<::std::string::String>,
}
impl DeprovisionIpamByoasnInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// <p>The IPAM ID.</p>
    /// This field is required.
    pub fn ipam_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ipam_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IPAM ID.</p>
    pub fn set_ipam_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ipam_id = input;
        self
    }
    /// <p>The IPAM ID.</p>
    pub fn get_ipam_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ipam_id
    }
    /// <p>An ASN.</p>
    /// This field is required.
    pub fn asn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.asn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>An ASN.</p>
    pub fn set_asn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.asn = input;
        self
    }
    /// <p>An ASN.</p>
    pub fn get_asn(&self) -> &::std::option::Option<::std::string::String> {
        &self.asn
    }
    /// Consumes the builder and constructs a [`DeprovisionIpamByoasnInput`](crate::operation::deprovision_ipam_byoasn::DeprovisionIpamByoasnInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::deprovision_ipam_byoasn::DeprovisionIpamByoasnInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::deprovision_ipam_byoasn::DeprovisionIpamByoasnInput {
            dry_run: self.dry_run,
            ipam_id: self.ipam_id,
            asn: self.asn,
        })
    }
}
