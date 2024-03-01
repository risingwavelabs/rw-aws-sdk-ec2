// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyHostsOutput {
    /// <p>The IDs of the Dedicated Hosts that were successfully modified.</p>
    pub successful: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The IDs of the Dedicated Hosts that could not be modified. Check whether the setting you requested can be used.</p>
    pub unsuccessful: ::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulItem>>,
    _request_id: Option<String>,
}
impl ModifyHostsOutput {
    /// <p>The IDs of the Dedicated Hosts that were successfully modified.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.successful.is_none()`.
    pub fn successful(&self) -> &[::std::string::String] {
        self.successful.as_deref().unwrap_or_default()
    }
    /// <p>The IDs of the Dedicated Hosts that could not be modified. Check whether the setting you requested can be used.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.unsuccessful.is_none()`.
    pub fn unsuccessful(&self) -> &[crate::types::UnsuccessfulItem] {
        self.unsuccessful.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for ModifyHostsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyHostsOutput {
    /// Creates a new builder-style object to manufacture [`ModifyHostsOutput`](crate::operation::modify_hosts::ModifyHostsOutput).
    pub fn builder() -> crate::operation::modify_hosts::builders::ModifyHostsOutputBuilder {
        crate::operation::modify_hosts::builders::ModifyHostsOutputBuilder::default()
    }
}

/// A builder for [`ModifyHostsOutput`](crate::operation::modify_hosts::ModifyHostsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyHostsOutputBuilder {
    pub(crate) successful: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) unsuccessful: ::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulItem>>,
    _request_id: Option<String>,
}
impl ModifyHostsOutputBuilder {
    /// Appends an item to `successful`.
    ///
    /// To override the contents of this collection use [`set_successful`](Self::set_successful).
    ///
    /// <p>The IDs of the Dedicated Hosts that were successfully modified.</p>
    pub fn successful(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.successful.unwrap_or_default();
        v.push(input.into());
        self.successful = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the Dedicated Hosts that were successfully modified.</p>
    pub fn set_successful(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.successful = input;
        self
    }
    /// <p>The IDs of the Dedicated Hosts that were successfully modified.</p>
    pub fn get_successful(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.successful
    }
    /// Appends an item to `unsuccessful`.
    ///
    /// To override the contents of this collection use [`set_unsuccessful`](Self::set_unsuccessful).
    ///
    /// <p>The IDs of the Dedicated Hosts that could not be modified. Check whether the setting you requested can be used.</p>
    pub fn unsuccessful(mut self, input: crate::types::UnsuccessfulItem) -> Self {
        let mut v = self.unsuccessful.unwrap_or_default();
        v.push(input);
        self.unsuccessful = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the Dedicated Hosts that could not be modified. Check whether the setting you requested can be used.</p>
    pub fn set_unsuccessful(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulItem>>) -> Self {
        self.unsuccessful = input;
        self
    }
    /// <p>The IDs of the Dedicated Hosts that could not be modified. Check whether the setting you requested can be used.</p>
    pub fn get_unsuccessful(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulItem>> {
        &self.unsuccessful
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifyHostsOutput`](crate::operation::modify_hosts::ModifyHostsOutput).
    pub fn build(self) -> crate::operation::modify_hosts::ModifyHostsOutput {
        crate::operation::modify_hosts::ModifyHostsOutput {
            successful: self.successful,
            unsuccessful: self.unsuccessful,
            _request_id: self._request_id,
        }
    }
}
