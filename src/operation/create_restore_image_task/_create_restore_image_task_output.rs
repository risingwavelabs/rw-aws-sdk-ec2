// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateRestoreImageTaskOutput {
    /// <p>The AMI ID.</p>
    pub image_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateRestoreImageTaskOutput {
    /// <p>The AMI ID.</p>
    pub fn image_id(&self) -> ::std::option::Option<&str> {
        self.image_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for CreateRestoreImageTaskOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateRestoreImageTaskOutput {
    /// Creates a new builder-style object to manufacture [`CreateRestoreImageTaskOutput`](crate::operation::create_restore_image_task::CreateRestoreImageTaskOutput).
    pub fn builder() -> crate::operation::create_restore_image_task::builders::CreateRestoreImageTaskOutputBuilder {
        crate::operation::create_restore_image_task::builders::CreateRestoreImageTaskOutputBuilder::default()
    }
}

/// A builder for [`CreateRestoreImageTaskOutput`](crate::operation::create_restore_image_task::CreateRestoreImageTaskOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateRestoreImageTaskOutputBuilder {
    pub(crate) image_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateRestoreImageTaskOutputBuilder {
    /// <p>The AMI ID.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The AMI ID.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>The AMI ID.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.image_id
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateRestoreImageTaskOutput`](crate::operation::create_restore_image_task::CreateRestoreImageTaskOutput).
    pub fn build(self) -> crate::operation::create_restore_image_task::CreateRestoreImageTaskOutput {
        crate::operation::create_restore_image_task::CreateRestoreImageTaskOutput {
            image_id: self.image_id,
            _request_id: self._request_id,
        }
    }
}
