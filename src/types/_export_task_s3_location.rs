// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the destination for an export image task.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ExportTaskS3Location {
    /// <p>The destination Amazon S3 bucket.</p>
    pub s3_bucket: ::std::option::Option<::std::string::String>,
    /// <p>The prefix (logical hierarchy) in the bucket.</p>
    pub s3_prefix: ::std::option::Option<::std::string::String>,
}
impl ExportTaskS3Location {
    /// <p>The destination Amazon S3 bucket.</p>
    pub fn s3_bucket(&self) -> ::std::option::Option<&str> {
        self.s3_bucket.as_deref()
    }
    /// <p>The prefix (logical hierarchy) in the bucket.</p>
    pub fn s3_prefix(&self) -> ::std::option::Option<&str> {
        self.s3_prefix.as_deref()
    }
}
impl ExportTaskS3Location {
    /// Creates a new builder-style object to manufacture [`ExportTaskS3Location`](crate::types::ExportTaskS3Location).
    pub fn builder() -> crate::types::builders::ExportTaskS3LocationBuilder {
        crate::types::builders::ExportTaskS3LocationBuilder::default()
    }
}

/// A builder for [`ExportTaskS3Location`](crate::types::ExportTaskS3Location).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ExportTaskS3LocationBuilder {
    pub(crate) s3_bucket: ::std::option::Option<::std::string::String>,
    pub(crate) s3_prefix: ::std::option::Option<::std::string::String>,
}
impl ExportTaskS3LocationBuilder {
    /// <p>The destination Amazon S3 bucket.</p>
    pub fn s3_bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The destination Amazon S3 bucket.</p>
    pub fn set_s3_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_bucket = input;
        self
    }
    /// <p>The destination Amazon S3 bucket.</p>
    pub fn get_s3_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.s3_bucket
    }
    /// <p>The prefix (logical hierarchy) in the bucket.</p>
    pub fn s3_prefix(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.s3_prefix = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The prefix (logical hierarchy) in the bucket.</p>
    pub fn set_s3_prefix(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.s3_prefix = input;
        self
    }
    /// <p>The prefix (logical hierarchy) in the bucket.</p>
    pub fn get_s3_prefix(&self) -> &::std::option::Option<::std::string::String> {
        &self.s3_prefix
    }
    /// Consumes the builder and constructs a [`ExportTaskS3Location`](crate::types::ExportTaskS3Location).
    pub fn build(self) -> crate::types::ExportTaskS3Location {
        crate::types::ExportTaskS3Location {
            s3_bucket: self.s3_bucket,
            s3_prefix: self.s3_prefix,
        }
    }
}
