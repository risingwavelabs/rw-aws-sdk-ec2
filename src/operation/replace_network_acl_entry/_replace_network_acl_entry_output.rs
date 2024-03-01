// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplaceNetworkAclEntryOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for ReplaceNetworkAclEntryOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ReplaceNetworkAclEntryOutput {
    /// Creates a new builder-style object to manufacture [`ReplaceNetworkAclEntryOutput`](crate::operation::replace_network_acl_entry::ReplaceNetworkAclEntryOutput).
    pub fn builder() -> crate::operation::replace_network_acl_entry::builders::ReplaceNetworkAclEntryOutputBuilder {
        crate::operation::replace_network_acl_entry::builders::ReplaceNetworkAclEntryOutputBuilder::default()
    }
}

/// A builder for [`ReplaceNetworkAclEntryOutput`](crate::operation::replace_network_acl_entry::ReplaceNetworkAclEntryOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ReplaceNetworkAclEntryOutputBuilder {
    _request_id: Option<String>,
}
impl ReplaceNetworkAclEntryOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ReplaceNetworkAclEntryOutput`](crate::operation::replace_network_acl_entry::ReplaceNetworkAclEntryOutput).
    pub fn build(self) -> crate::operation::replace_network_acl_entry::ReplaceNetworkAclEntryOutput {
        crate::operation::replace_network_acl_entry::ReplaceNetworkAclEntryOutput {
            _request_id: self._request_id,
        }
    }
}