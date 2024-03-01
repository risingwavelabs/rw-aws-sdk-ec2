// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateReplaceRootVolumeTaskOutput {
    /// <p>Information about the root volume replacement task.</p>
    pub replace_root_volume_task: ::std::option::Option<crate::types::ReplaceRootVolumeTask>,
    _request_id: Option<String>,
}
impl CreateReplaceRootVolumeTaskOutput {
    /// <p>Information about the root volume replacement task.</p>
    pub fn replace_root_volume_task(&self) -> ::std::option::Option<&crate::types::ReplaceRootVolumeTask> {
        self.replace_root_volume_task.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for CreateReplaceRootVolumeTaskOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateReplaceRootVolumeTaskOutput {
    /// Creates a new builder-style object to manufacture [`CreateReplaceRootVolumeTaskOutput`](crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskOutput).
    pub fn builder() -> crate::operation::create_replace_root_volume_task::builders::CreateReplaceRootVolumeTaskOutputBuilder {
        crate::operation::create_replace_root_volume_task::builders::CreateReplaceRootVolumeTaskOutputBuilder::default()
    }
}

/// A builder for [`CreateReplaceRootVolumeTaskOutput`](crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateReplaceRootVolumeTaskOutputBuilder {
    pub(crate) replace_root_volume_task: ::std::option::Option<crate::types::ReplaceRootVolumeTask>,
    _request_id: Option<String>,
}
impl CreateReplaceRootVolumeTaskOutputBuilder {
    /// <p>Information about the root volume replacement task.</p>
    pub fn replace_root_volume_task(mut self, input: crate::types::ReplaceRootVolumeTask) -> Self {
        self.replace_root_volume_task = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the root volume replacement task.</p>
    pub fn set_replace_root_volume_task(mut self, input: ::std::option::Option<crate::types::ReplaceRootVolumeTask>) -> Self {
        self.replace_root_volume_task = input;
        self
    }
    /// <p>Information about the root volume replacement task.</p>
    pub fn get_replace_root_volume_task(&self) -> &::std::option::Option<crate::types::ReplaceRootVolumeTask> {
        &self.replace_root_volume_task
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateReplaceRootVolumeTaskOutput`](crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskOutput).
    pub fn build(self) -> crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskOutput {
        crate::operation::create_replace_root_volume_task::CreateReplaceRootVolumeTaskOutput {
            replace_root_volume_task: self.replace_root_volume_task,
            _request_id: self._request_id,
        }
    }
}
