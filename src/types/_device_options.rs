// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the options for an Amazon Web Services Verified Access device-identity based trust provider.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeviceOptions {
    /// <p>The ID of the tenant application with the device-identity provider.</p>
    pub tenant_id: ::std::option::Option<::std::string::String>,
    /// <p> The URL Amazon Web Services Verified Access will use to verify the authenticity of the device tokens. </p>
    pub public_signing_key_url: ::std::option::Option<::std::string::String>,
}
impl DeviceOptions {
    /// <p>The ID of the tenant application with the device-identity provider.</p>
    pub fn tenant_id(&self) -> ::std::option::Option<&str> {
        self.tenant_id.as_deref()
    }
    /// <p> The URL Amazon Web Services Verified Access will use to verify the authenticity of the device tokens. </p>
    pub fn public_signing_key_url(&self) -> ::std::option::Option<&str> {
        self.public_signing_key_url.as_deref()
    }
}
impl DeviceOptions {
    /// Creates a new builder-style object to manufacture [`DeviceOptions`](crate::types::DeviceOptions).
    pub fn builder() -> crate::types::builders::DeviceOptionsBuilder {
        crate::types::builders::DeviceOptionsBuilder::default()
    }
}

/// A builder for [`DeviceOptions`](crate::types::DeviceOptions).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeviceOptionsBuilder {
    pub(crate) tenant_id: ::std::option::Option<::std::string::String>,
    pub(crate) public_signing_key_url: ::std::option::Option<::std::string::String>,
}
impl DeviceOptionsBuilder {
    /// <p>The ID of the tenant application with the device-identity provider.</p>
    pub fn tenant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.tenant_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the tenant application with the device-identity provider.</p>
    pub fn set_tenant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.tenant_id = input;
        self
    }
    /// <p>The ID of the tenant application with the device-identity provider.</p>
    pub fn get_tenant_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.tenant_id
    }
    /// <p> The URL Amazon Web Services Verified Access will use to verify the authenticity of the device tokens. </p>
    pub fn public_signing_key_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.public_signing_key_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p> The URL Amazon Web Services Verified Access will use to verify the authenticity of the device tokens. </p>
    pub fn set_public_signing_key_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.public_signing_key_url = input;
        self
    }
    /// <p> The URL Amazon Web Services Verified Access will use to verify the authenticity of the device tokens. </p>
    pub fn get_public_signing_key_url(&self) -> &::std::option::Option<::std::string::String> {
        &self.public_signing_key_url
    }
    /// Consumes the builder and constructs a [`DeviceOptions`](crate::types::DeviceOptions).
    pub fn build(self) -> crate::types::DeviceOptions {
        crate::types::DeviceOptions {
            tenant_id: self.tenant_id,
            public_signing_key_url: self.public_signing_key_url,
        }
    }
}