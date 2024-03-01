// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the options when modifying a Verified Access endpoint with the <code>network-interface</code> type.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyVerifiedAccessEndpointEniOptions {
    /// <p>The IP protocol.</p>
    pub protocol: ::std::option::Option<crate::types::VerifiedAccessEndpointProtocol>,
    /// <p>The IP port number.</p>
    pub port: ::std::option::Option<i32>,
}
impl ModifyVerifiedAccessEndpointEniOptions {
    /// <p>The IP protocol.</p>
    pub fn protocol(&self) -> ::std::option::Option<&crate::types::VerifiedAccessEndpointProtocol> {
        self.protocol.as_ref()
    }
    /// <p>The IP port number.</p>
    pub fn port(&self) -> ::std::option::Option<i32> {
        self.port
    }
}
impl ModifyVerifiedAccessEndpointEniOptions {
    /// Creates a new builder-style object to manufacture [`ModifyVerifiedAccessEndpointEniOptions`](crate::types::ModifyVerifiedAccessEndpointEniOptions).
    pub fn builder() -> crate::types::builders::ModifyVerifiedAccessEndpointEniOptionsBuilder {
        crate::types::builders::ModifyVerifiedAccessEndpointEniOptionsBuilder::default()
    }
}

/// A builder for [`ModifyVerifiedAccessEndpointEniOptions`](crate::types::ModifyVerifiedAccessEndpointEniOptions).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyVerifiedAccessEndpointEniOptionsBuilder {
    pub(crate) protocol: ::std::option::Option<crate::types::VerifiedAccessEndpointProtocol>,
    pub(crate) port: ::std::option::Option<i32>,
}
impl ModifyVerifiedAccessEndpointEniOptionsBuilder {
    /// <p>The IP protocol.</p>
    pub fn protocol(mut self, input: crate::types::VerifiedAccessEndpointProtocol) -> Self {
        self.protocol = ::std::option::Option::Some(input);
        self
    }
    /// <p>The IP protocol.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<crate::types::VerifiedAccessEndpointProtocol>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The IP protocol.</p>
    pub fn get_protocol(&self) -> &::std::option::Option<crate::types::VerifiedAccessEndpointProtocol> {
        &self.protocol
    }
    /// <p>The IP port number.</p>
    pub fn port(mut self, input: i32) -> Self {
        self.port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The IP port number.</p>
    pub fn set_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.port = input;
        self
    }
    /// <p>The IP port number.</p>
    pub fn get_port(&self) -> &::std::option::Option<i32> {
        &self.port
    }
    /// Consumes the builder and constructs a [`ModifyVerifiedAccessEndpointEniOptions`](crate::types::ModifyVerifiedAccessEndpointEniOptions).
    pub fn build(self) -> crate::types::ModifyVerifiedAccessEndpointEniOptions {
        crate::types::ModifyVerifiedAccessEndpointEniOptions {
            protocol: self.protocol,
            port: self.port,
        }
    }
}