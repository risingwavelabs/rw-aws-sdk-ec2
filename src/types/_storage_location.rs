// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a storage location in Amazon S3.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StorageLocation {
    /// <p>The name of the S3 bucket.</p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The key.</p>
    pub key: ::std::option::Option<::std::string::String>,
}
impl StorageLocation {
    /// <p>The name of the S3 bucket.</p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The key.</p>
    pub fn key(&self) -> ::std::option::Option<&str> {
        self.key.as_deref()
    }
}
impl StorageLocation {
    /// Creates a new builder-style object to manufacture [`StorageLocation`](crate::types::StorageLocation).
    pub fn builder() -> crate::types::builders::StorageLocationBuilder {
        crate::types::builders::StorageLocationBuilder::default()
    }
}

/// A builder for [`StorageLocation`](crate::types::StorageLocation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StorageLocationBuilder {
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) key: ::std::option::Option<::std::string::String>,
}
impl StorageLocationBuilder {
    /// <p>The name of the S3 bucket.</p>
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the S3 bucket.</p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the S3 bucket.</p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// <p>The key.</p>
    pub fn key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The key.</p>
    pub fn set_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key = input;
        self
    }
    /// <p>The key.</p>
    pub fn get_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.key
    }
    /// Consumes the builder and constructs a [`StorageLocation`](crate::types::StorageLocation).
    pub fn build(self) -> crate::types::StorageLocation {
        crate::types::StorageLocation {
            bucket: self.bucket,
            key: self.key,
        }
    }
}