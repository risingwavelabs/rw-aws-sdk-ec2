// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateStoreImageTaskOutput {
    /// <p>The name of the stored AMI object in the S3 bucket.</p>
    pub object_key: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateStoreImageTaskOutput {
    /// <p>The name of the stored AMI object in the S3 bucket.</p>
    pub fn object_key(&self) -> ::std::option::Option<&str> {
        self.object_key.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for CreateStoreImageTaskOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateStoreImageTaskOutput {
    /// Creates a new builder-style object to manufacture [`CreateStoreImageTaskOutput`](crate::operation::create_store_image_task::CreateStoreImageTaskOutput).
    pub fn builder() -> crate::operation::create_store_image_task::builders::CreateStoreImageTaskOutputBuilder {
        crate::operation::create_store_image_task::builders::CreateStoreImageTaskOutputBuilder::default()
    }
}

/// A builder for [`CreateStoreImageTaskOutput`](crate::operation::create_store_image_task::CreateStoreImageTaskOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateStoreImageTaskOutputBuilder {
    pub(crate) object_key: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateStoreImageTaskOutputBuilder {
    /// <p>The name of the stored AMI object in the S3 bucket.</p>
    pub fn object_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.object_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the stored AMI object in the S3 bucket.</p>
    pub fn set_object_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.object_key = input;
        self
    }
    /// <p>The name of the stored AMI object in the S3 bucket.</p>
    pub fn get_object_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.object_key
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateStoreImageTaskOutput`](crate::operation::create_store_image_task::CreateStoreImageTaskOutput).
    pub fn build(self) -> crate::operation::create_store_image_task::CreateStoreImageTaskOutput {
        crate::operation::create_store_image_task::CreateStoreImageTaskOutput {
            object_key: self.object_key,
            _request_id: self._request_id,
        }
    }
}