// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyIpamResourceCidrOutput {
    /// <p>The CIDR of the resource.</p>
    pub ipam_resource_cidr: ::std::option::Option<crate::types::IpamResourceCidr>,
    _request_id: Option<String>,
}
impl ModifyIpamResourceCidrOutput {
    /// <p>The CIDR of the resource.</p>
    pub fn ipam_resource_cidr(&self) -> ::std::option::Option<&crate::types::IpamResourceCidr> {
        self.ipam_resource_cidr.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ModifyIpamResourceCidrOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyIpamResourceCidrOutput {
    /// Creates a new builder-style object to manufacture [`ModifyIpamResourceCidrOutput`](crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput).
    pub fn builder() -> crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrOutputBuilder {
        crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrOutputBuilder::default()
    }
}

/// A builder for [`ModifyIpamResourceCidrOutput`](crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyIpamResourceCidrOutputBuilder {
    pub(crate) ipam_resource_cidr: ::std::option::Option<crate::types::IpamResourceCidr>,
    _request_id: Option<String>,
}
impl ModifyIpamResourceCidrOutputBuilder {
    /// <p>The CIDR of the resource.</p>
    pub fn ipam_resource_cidr(mut self, input: crate::types::IpamResourceCidr) -> Self {
        self.ipam_resource_cidr = ::std::option::Option::Some(input);
        self
    }
    /// <p>The CIDR of the resource.</p>
    pub fn set_ipam_resource_cidr(mut self, input: ::std::option::Option<crate::types::IpamResourceCidr>) -> Self {
        self.ipam_resource_cidr = input;
        self
    }
    /// <p>The CIDR of the resource.</p>
    pub fn get_ipam_resource_cidr(&self) -> &::std::option::Option<crate::types::IpamResourceCidr> {
        &self.ipam_resource_cidr
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifyIpamResourceCidrOutput`](crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput).
    pub fn build(self) -> crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput {
        crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput {
            ipam_resource_cidr: self.ipam_resource_cidr,
            _request_id: self._request_id,
        }
    }
}
