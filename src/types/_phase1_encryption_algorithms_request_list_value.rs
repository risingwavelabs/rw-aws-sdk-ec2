// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Specifies the encryption algorithm for the VPN tunnel for phase 1 IKE negotiations.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Phase1EncryptionAlgorithmsRequestListValue {
    /// <p>The value for the encryption algorithm.</p>
    pub value: ::std::option::Option<::std::string::String>,
}
impl Phase1EncryptionAlgorithmsRequestListValue {
    /// <p>The value for the encryption algorithm.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl Phase1EncryptionAlgorithmsRequestListValue {
    /// Creates a new builder-style object to manufacture [`Phase1EncryptionAlgorithmsRequestListValue`](crate::types::Phase1EncryptionAlgorithmsRequestListValue).
    pub fn builder() -> crate::types::builders::Phase1EncryptionAlgorithmsRequestListValueBuilder {
        crate::types::builders::Phase1EncryptionAlgorithmsRequestListValueBuilder::default()
    }
}

/// A builder for [`Phase1EncryptionAlgorithmsRequestListValue`](crate::types::Phase1EncryptionAlgorithmsRequestListValue).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct Phase1EncryptionAlgorithmsRequestListValueBuilder {
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl Phase1EncryptionAlgorithmsRequestListValueBuilder {
    /// <p>The value for the encryption algorithm.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The value for the encryption algorithm.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>The value for the encryption algorithm.</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.value
    }
    /// Consumes the builder and constructs a [`Phase1EncryptionAlgorithmsRequestListValue`](crate::types::Phase1EncryptionAlgorithmsRequestListValue).
    pub fn build(self) -> crate::types::Phase1EncryptionAlgorithmsRequestListValue {
        crate::types::Phase1EncryptionAlgorithmsRequestListValue { value: self.value }
    }
}
