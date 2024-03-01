// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the status of a client connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ClientVpnConnectionStatus {
    /// <p>The state of the client connection.</p>
    pub code: ::std::option::Option<crate::types::ClientVpnConnectionStatusCode>,
    /// <p>A message about the status of the client connection, if applicable.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl ClientVpnConnectionStatus {
    /// <p>The state of the client connection.</p>
    pub fn code(&self) -> ::std::option::Option<&crate::types::ClientVpnConnectionStatusCode> {
        self.code.as_ref()
    }
    /// <p>A message about the status of the client connection, if applicable.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ClientVpnConnectionStatus {
    /// Creates a new builder-style object to manufacture [`ClientVpnConnectionStatus`](crate::types::ClientVpnConnectionStatus).
    pub fn builder() -> crate::types::builders::ClientVpnConnectionStatusBuilder {
        crate::types::builders::ClientVpnConnectionStatusBuilder::default()
    }
}

/// A builder for [`ClientVpnConnectionStatus`](crate::types::ClientVpnConnectionStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ClientVpnConnectionStatusBuilder {
    pub(crate) code: ::std::option::Option<crate::types::ClientVpnConnectionStatusCode>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl ClientVpnConnectionStatusBuilder {
    /// <p>The state of the client connection.</p>
    pub fn code(mut self, input: crate::types::ClientVpnConnectionStatusCode) -> Self {
        self.code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the client connection.</p>
    pub fn set_code(mut self, input: ::std::option::Option<crate::types::ClientVpnConnectionStatusCode>) -> Self {
        self.code = input;
        self
    }
    /// <p>The state of the client connection.</p>
    pub fn get_code(&self) -> &::std::option::Option<crate::types::ClientVpnConnectionStatusCode> {
        &self.code
    }
    /// <p>A message about the status of the client connection, if applicable.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A message about the status of the client connection, if applicable.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>A message about the status of the client connection, if applicable.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Consumes the builder and constructs a [`ClientVpnConnectionStatus`](crate::types::ClientVpnConnectionStatus).
    pub fn build(self) -> crate::types::ClientVpnConnectionStatus {
        crate::types::ClientVpnConnectionStatus {
            code: self.code,
            message: self.message,
        }
    }
}