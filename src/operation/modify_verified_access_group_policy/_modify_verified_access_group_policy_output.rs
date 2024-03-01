// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyVerifiedAccessGroupPolicyOutput {
    /// <p>The status of the Verified Access policy.</p>
    pub policy_enabled: ::std::option::Option<bool>,
    /// <p>The Verified Access policy document.</p>
    pub policy_document: ::std::option::Option<::std::string::String>,
    /// <p>The options in use for server side encryption.</p>
    pub sse_specification: ::std::option::Option<crate::types::VerifiedAccessSseSpecificationResponse>,
    _request_id: Option<String>,
}
impl ModifyVerifiedAccessGroupPolicyOutput {
    /// <p>The status of the Verified Access policy.</p>
    pub fn policy_enabled(&self) -> ::std::option::Option<bool> {
        self.policy_enabled
    }
    /// <p>The Verified Access policy document.</p>
    pub fn policy_document(&self) -> ::std::option::Option<&str> {
        self.policy_document.as_deref()
    }
    /// <p>The options in use for server side encryption.</p>
    pub fn sse_specification(&self) -> ::std::option::Option<&crate::types::VerifiedAccessSseSpecificationResponse> {
        self.sse_specification.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ModifyVerifiedAccessGroupPolicyOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyVerifiedAccessGroupPolicyOutput {
    /// Creates a new builder-style object to manufacture [`ModifyVerifiedAccessGroupPolicyOutput`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyOutput).
    pub fn builder() -> crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyOutputBuilder {
        crate::operation::modify_verified_access_group_policy::builders::ModifyVerifiedAccessGroupPolicyOutputBuilder::default()
    }
}

/// A builder for [`ModifyVerifiedAccessGroupPolicyOutput`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyVerifiedAccessGroupPolicyOutputBuilder {
    pub(crate) policy_enabled: ::std::option::Option<bool>,
    pub(crate) policy_document: ::std::option::Option<::std::string::String>,
    pub(crate) sse_specification: ::std::option::Option<crate::types::VerifiedAccessSseSpecificationResponse>,
    _request_id: Option<String>,
}
impl ModifyVerifiedAccessGroupPolicyOutputBuilder {
    /// <p>The status of the Verified Access policy.</p>
    pub fn policy_enabled(mut self, input: bool) -> Self {
        self.policy_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the Verified Access policy.</p>
    pub fn set_policy_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.policy_enabled = input;
        self
    }
    /// <p>The status of the Verified Access policy.</p>
    pub fn get_policy_enabled(&self) -> &::std::option::Option<bool> {
        &self.policy_enabled
    }
    /// <p>The Verified Access policy document.</p>
    pub fn policy_document(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.policy_document = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Verified Access policy document.</p>
    pub fn set_policy_document(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.policy_document = input;
        self
    }
    /// <p>The Verified Access policy document.</p>
    pub fn get_policy_document(&self) -> &::std::option::Option<::std::string::String> {
        &self.policy_document
    }
    /// <p>The options in use for server side encryption.</p>
    pub fn sse_specification(mut self, input: crate::types::VerifiedAccessSseSpecificationResponse) -> Self {
        self.sse_specification = ::std::option::Option::Some(input);
        self
    }
    /// <p>The options in use for server side encryption.</p>
    pub fn set_sse_specification(mut self, input: ::std::option::Option<crate::types::VerifiedAccessSseSpecificationResponse>) -> Self {
        self.sse_specification = input;
        self
    }
    /// <p>The options in use for server side encryption.</p>
    pub fn get_sse_specification(&self) -> &::std::option::Option<crate::types::VerifiedAccessSseSpecificationResponse> {
        &self.sse_specification
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifyVerifiedAccessGroupPolicyOutput`](crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyOutput).
    pub fn build(self) -> crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyOutput {
        crate::operation::modify_verified_access_group_policy::ModifyVerifiedAccessGroupPolicyOutput {
            policy_enabled: self.policy_enabled,
            policy_document: self.policy_document,
            sse_specification: self.sse_specification,
            _request_id: self._request_id,
        }
    }
}