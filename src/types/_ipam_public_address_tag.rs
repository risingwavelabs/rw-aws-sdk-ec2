// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A tag for a public IP address discovered by IPAM.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IpamPublicAddressTag {
    /// <p>The tag's key.</p>
    pub key: ::std::option::Option<::std::string::String>,
    /// <p>The tag's value.</p>
    pub value: ::std::option::Option<::std::string::String>,
}
impl IpamPublicAddressTag {
    /// <p>The tag's key.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
    /// <p>The tag's value.</p>
    pub fn value(&self) -> ::std::option::Option<&str> {
        self.value.as_deref()
    }
}
impl IpamPublicAddressTag {
    /// Creates a new builder-style object to manufacture [`IpamPublicAddressTag`](crate::types::IpamPublicAddressTag).
    pub fn builder() -> crate::types::builders::IpamPublicAddressTagBuilder {
        crate::types::builders::IpamPublicAddressTagBuilder::default()
    }
}

/// A builder for [`IpamPublicAddressTag`](crate::types::IpamPublicAddressTag).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct IpamPublicAddressTagBuilder {
    pub(crate) key: ::std::option::Option<::std::string::String>,
    pub(crate) value: ::std::option::Option<::std::string::String>,
}
impl IpamPublicAddressTagBuilder {
    /// <p>The tag's key.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The tag's key.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The tag's key.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// <p>The tag's value.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.value = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The tag's value.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.value = input;
        self
    }
    /// <p>The tag's value.</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        &self.value
    }
    /// Consumes the builder and constructs a [`IpamPublicAddressTag`](crate::types::IpamPublicAddressTag).
    pub fn build(self) -> crate::types::IpamPublicAddressTag {
        crate::types::IpamPublicAddressTag {
            key: self.key,
            value: self.value,
        }
    }
}
