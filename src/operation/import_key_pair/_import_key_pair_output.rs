// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportKeyPairOutput {
    /// <ul>
    /// <li> <p>For RSA key pairs, the key fingerprint is the MD5 public key fingerprint as specified in section 4 of RFC 4716.</p> </li>
    /// <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with <a href="http://www.openssh.com/txt/release-6.8">OpenSSH 6.8</a>.</p> </li>
    /// </ul>
    pub key_fingerprint: ::std::option::Option<::std::string::String>,
    /// <p>The key pair name that you provided.</p>
    pub key_name: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the resulting key pair.</p>
    pub key_pair_id: ::std::option::Option<::std::string::String>,
    /// <p>The tags applied to the imported key pair.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl ImportKeyPairOutput {
    /// <ul>
    /// <li> <p>For RSA key pairs, the key fingerprint is the MD5 public key fingerprint as specified in section 4 of RFC 4716.</p> </li>
    /// <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with <a href="http://www.openssh.com/txt/release-6.8">OpenSSH 6.8</a>.</p> </li>
    /// </ul>
    pub fn key_fingerprint(&self) -> ::std::option::Option<&str> {
        self.key_fingerprint.as_deref()
    }
    /// <p>The key pair name that you provided.</p>
    pub fn key_name(&self) -> ::std::option::Option<&str> {
        self.key_name.as_deref()
    }
    /// <p>The ID of the resulting key pair.</p>
    pub fn key_pair_id(&self) -> ::std::option::Option<&str> {
        self.key_pair_id.as_deref()
    }
    /// <p>The tags applied to the imported key pair.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for ImportKeyPairOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ImportKeyPairOutput {
    /// Creates a new builder-style object to manufacture [`ImportKeyPairOutput`](crate::operation::import_key_pair::ImportKeyPairOutput).
    pub fn builder() -> crate::operation::import_key_pair::builders::ImportKeyPairOutputBuilder {
        crate::operation::import_key_pair::builders::ImportKeyPairOutputBuilder::default()
    }
}

/// A builder for [`ImportKeyPairOutput`](crate::operation::import_key_pair::ImportKeyPairOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ImportKeyPairOutputBuilder {
    pub(crate) key_fingerprint: ::std::option::Option<::std::string::String>,
    pub(crate) key_name: ::std::option::Option<::std::string::String>,
    pub(crate) key_pair_id: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    _request_id: Option<String>,
}
impl ImportKeyPairOutputBuilder {
    /// <ul>
    /// <li> <p>For RSA key pairs, the key fingerprint is the MD5 public key fingerprint as specified in section 4 of RFC 4716.</p> </li>
    /// <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with <a href="http://www.openssh.com/txt/release-6.8">OpenSSH 6.8</a>.</p> </li>
    /// </ul>
    pub fn key_fingerprint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_fingerprint = ::std::option::Option::Some(input.into());
        self
    }
    /// <ul>
    /// <li> <p>For RSA key pairs, the key fingerprint is the MD5 public key fingerprint as specified in section 4 of RFC 4716.</p> </li>
    /// <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with <a href="http://www.openssh.com/txt/release-6.8">OpenSSH 6.8</a>.</p> </li>
    /// </ul>
    pub fn set_key_fingerprint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_fingerprint = input;
        self
    }
    /// <ul>
    /// <li> <p>For RSA key pairs, the key fingerprint is the MD5 public key fingerprint as specified in section 4 of RFC 4716.</p> </li>
    /// <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with <a href="http://www.openssh.com/txt/release-6.8">OpenSSH 6.8</a>.</p> </li>
    /// </ul>
    pub fn get_key_fingerprint(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_fingerprint
    }
    /// <p>The key pair name that you provided.</p>
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key pair name that you provided.</p>
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_name = input;
        self
    }
    /// <p>The key pair name that you provided.</p>
    pub fn get_key_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_name
    }
    /// <p>The ID of the resulting key pair.</p>
    pub fn key_pair_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_pair_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the resulting key pair.</p>
    pub fn set_key_pair_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_pair_id = input;
        self
    }
    /// <p>The ID of the resulting key pair.</p>
    pub fn get_key_pair_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_pair_id
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags applied to the imported key pair.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags applied to the imported key pair.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags applied to the imported key pair.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ImportKeyPairOutput`](crate::operation::import_key_pair::ImportKeyPairOutput).
    pub fn build(self) -> crate::operation::import_key_pair::ImportKeyPairOutput {
        crate::operation::import_key_pair::ImportKeyPairOutput {
            key_fingerprint: self.key_fingerprint,
            key_name: self.key_name,
            key_pair_id: self.key_pair_id,
            tags: self.tags,
            _request_id: self._request_id,
        }
    }
}
