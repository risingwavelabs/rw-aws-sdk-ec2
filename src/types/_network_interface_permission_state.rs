// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the state of a network interface permission.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NetworkInterfacePermissionState {
    /// <p>The state of the permission.</p>
    pub state: ::std::option::Option<crate::types::NetworkInterfacePermissionStateCode>,
    /// <p>A status message, if applicable.</p>
    pub status_message: ::std::option::Option<::std::string::String>,
}
impl NetworkInterfacePermissionState {
    /// <p>The state of the permission.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::NetworkInterfacePermissionStateCode> {
        self.state.as_ref()
    }
    /// <p>A status message, if applicable.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
}
impl NetworkInterfacePermissionState {
    /// Creates a new builder-style object to manufacture [`NetworkInterfacePermissionState`](crate::types::NetworkInterfacePermissionState).
    pub fn builder() -> crate::types::builders::NetworkInterfacePermissionStateBuilder {
        crate::types::builders::NetworkInterfacePermissionStateBuilder::default()
    }
}

/// A builder for [`NetworkInterfacePermissionState`](crate::types::NetworkInterfacePermissionState).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct NetworkInterfacePermissionStateBuilder {
    pub(crate) state: ::std::option::Option<crate::types::NetworkInterfacePermissionStateCode>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
}
impl NetworkInterfacePermissionStateBuilder {
    /// <p>The state of the permission.</p>
    pub fn state(mut self, input: crate::types::NetworkInterfacePermissionStateCode) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the permission.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::NetworkInterfacePermissionStateCode>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the permission.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::NetworkInterfacePermissionStateCode> {
        &self.state
    }
    /// <p>A status message, if applicable.</p>
    pub fn status_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A status message, if applicable.</p>
    pub fn set_status_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// <p>A status message, if applicable.</p>
    pub fn get_status_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_message
    }
    /// Consumes the builder and constructs a [`NetworkInterfacePermissionState`](crate::types::NetworkInterfacePermissionState).
    pub fn build(self) -> crate::types::NetworkInterfacePermissionState {
        crate::types::NetworkInterfacePermissionState {
            state: self.state,
            status_message: self.status_message,
        }
    }
}
