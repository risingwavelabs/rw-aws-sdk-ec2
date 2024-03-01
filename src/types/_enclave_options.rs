// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Indicates whether the instance is enabled for Amazon Web Services Nitro Enclaves.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnclaveOptions {
    /// <p>If this parameter is set to <code>true</code>, the instance is enabled for Amazon Web Services Nitro Enclaves; otherwise, it is not enabled for Amazon Web Services Nitro Enclaves.</p>
    pub enabled: ::std::option::Option<bool>,
}
impl EnclaveOptions {
    /// <p>If this parameter is set to <code>true</code>, the instance is enabled for Amazon Web Services Nitro Enclaves; otherwise, it is not enabled for Amazon Web Services Nitro Enclaves.</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
}
impl EnclaveOptions {
    /// Creates a new builder-style object to manufacture [`EnclaveOptions`](crate::types::EnclaveOptions).
    pub fn builder() -> crate::types::builders::EnclaveOptionsBuilder {
        crate::types::builders::EnclaveOptionsBuilder::default()
    }
}

/// A builder for [`EnclaveOptions`](crate::types::EnclaveOptions).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EnclaveOptionsBuilder {
    pub(crate) enabled: ::std::option::Option<bool>,
}
impl EnclaveOptionsBuilder {
    /// <p>If this parameter is set to <code>true</code>, the instance is enabled for Amazon Web Services Nitro Enclaves; otherwise, it is not enabled for Amazon Web Services Nitro Enclaves.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>If this parameter is set to <code>true</code>, the instance is enabled for Amazon Web Services Nitro Enclaves; otherwise, it is not enabled for Amazon Web Services Nitro Enclaves.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>If this parameter is set to <code>true</code>, the instance is enabled for Amazon Web Services Nitro Enclaves; otherwise, it is not enabled for Amazon Web Services Nitro Enclaves.</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }
    /// Consumes the builder and constructs a [`EnclaveOptions`](crate::types::EnclaveOptions).
    pub fn build(self) -> crate::types::EnclaveOptions {
        crate::types::EnclaveOptions { enabled: self.enabled }
    }
}
