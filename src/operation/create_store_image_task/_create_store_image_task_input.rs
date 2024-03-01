// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateStoreImageTaskInput {
    /// <p>The ID of the AMI.</p>
    pub image_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Amazon S3 bucket in which the AMI object will be stored. The bucket must be in the Region in which the request is being made. The AMI object appears in the bucket only after the upload task has completed. </p>
    pub bucket: ::std::option::Option<::std::string::String>,
    /// <p>The tags to apply to the AMI object that will be stored in the Amazon S3 bucket. </p>
    pub s3_object_tags: ::std::option::Option<::std::vec::Vec<crate::types::S3ObjectTag>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl CreateStoreImageTaskInput {
    /// <p>The ID of the AMI.</p>
    pub fn image_id(&self) -> ::std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>The name of the Amazon S3 bucket in which the AMI object will be stored. The bucket must be in the Region in which the request is being made. The AMI object appears in the bucket only after the upload task has completed. </p>
    pub fn bucket(&self) -> ::std::option::Option<&str> {
        self.bucket.as_deref()
    }
    /// <p>The tags to apply to the AMI object that will be stored in the Amazon S3 bucket. </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.s3_object_tags.is_none()`.
    pub fn s3_object_tags(&self) -> &[crate::types::S3ObjectTag] {
        self.s3_object_tags.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl CreateStoreImageTaskInput {
    /// Creates a new builder-style object to manufacture [`CreateStoreImageTaskInput`](crate::operation::create_store_image_task::CreateStoreImageTaskInput).
    pub fn builder() -> crate::operation::create_store_image_task::builders::CreateStoreImageTaskInputBuilder {
        crate::operation::create_store_image_task::builders::CreateStoreImageTaskInputBuilder::default()
    }
}

/// A builder for [`CreateStoreImageTaskInput`](crate::operation::create_store_image_task::CreateStoreImageTaskInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateStoreImageTaskInputBuilder {
    pub(crate) image_id: ::std::option::Option<::std::string::String>,
    pub(crate) bucket: ::std::option::Option<::std::string::String>,
    pub(crate) s3_object_tags: ::std::option::Option<::std::vec::Vec<crate::types::S3ObjectTag>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl CreateStoreImageTaskInputBuilder {
    /// <p>The ID of the AMI.</p>
    /// This field is required.
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.image_id
    }
    /// <p>The name of the Amazon S3 bucket in which the AMI object will be stored. The bucket must be in the Region in which the request is being made. The AMI object appears in the bucket only after the upload task has completed. </p>
    /// This field is required.
    pub fn bucket(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.bucket = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket in which the AMI object will be stored. The bucket must be in the Region in which the request is being made. The AMI object appears in the bucket only after the upload task has completed. </p>
    pub fn set_bucket(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.bucket = input;
        self
    }
    /// <p>The name of the Amazon S3 bucket in which the AMI object will be stored. The bucket must be in the Region in which the request is being made. The AMI object appears in the bucket only after the upload task has completed. </p>
    pub fn get_bucket(&self) -> &::std::option::Option<::std::string::String> {
        &self.bucket
    }
    /// Appends an item to `s3_object_tags`.
    ///
    /// To override the contents of this collection use [`set_s3_object_tags`](Self::set_s3_object_tags).
    ///
    /// <p>The tags to apply to the AMI object that will be stored in the Amazon S3 bucket. </p>
    pub fn s3_object_tags(mut self, input: crate::types::S3ObjectTag) -> Self {
        let mut v = self.s3_object_tags.unwrap_or_default();
        v.push(input);
        self.s3_object_tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the AMI object that will be stored in the Amazon S3 bucket. </p>
    pub fn set_s3_object_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::S3ObjectTag>>) -> Self {
        self.s3_object_tags = input;
        self
    }
    /// <p>The tags to apply to the AMI object that will be stored in the Amazon S3 bucket. </p>
    pub fn get_s3_object_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::S3ObjectTag>> {
        &self.s3_object_tags
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`CreateStoreImageTaskInput`](crate::operation::create_store_image_task::CreateStoreImageTaskInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_store_image_task::CreateStoreImageTaskInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::create_store_image_task::CreateStoreImageTaskInput {
            image_id: self.image_id,
            bucket: self.bucket,
            s3_object_tags: self.s3_object_tags,
            dry_run: self.dry_run,
        })
    }
}
