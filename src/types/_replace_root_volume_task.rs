// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about a root volume replacement task.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ReplaceRootVolumeTask {
    /// <p>The ID of the root volume replacement task.</p>
    pub replace_root_volume_task_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the instance for which the root volume replacement task was created.</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The state of the task. The task can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>pending</code> - the replacement volume is being created.</p> </li>
    /// <li> <p> <code>in-progress</code> - the original volume is being detached and the replacement volume is being attached.</p> </li>
    /// <li> <p> <code>succeeded</code> - the replacement volume has been successfully attached to the instance and the instance is available.</p> </li>
    /// <li> <p> <code>failing</code> - the replacement task is in the process of failing.</p> </li>
    /// <li> <p> <code>failed</code> - the replacement task has failed but the original root volume is still attached.</p> </li>
    /// <li> <p> <code>failing-detached</code> - the replacement task is in the process of failing. The instance might have no root volume attached.</p> </li>
    /// <li> <p> <code>failed-detached</code> - the replacement task has failed and the instance has no root volume attached.</p> </li>
    /// </ul>
    pub task_state: ::std::option::Option<crate::types::ReplaceRootVolumeTaskState>,
    /// <p>The time the task was started.</p>
    pub start_time: ::std::option::Option<::std::string::String>,
    /// <p>The time the task completed.</p>
    pub complete_time: ::std::option::Option<::std::string::String>,
    /// <p>The tags assigned to the task.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The ID of the AMI used to create the replacement root volume.</p>
    pub image_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the snapshot used to create the replacement root volume.</p>
    pub snapshot_id: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the original root volume is to be deleted after the root volume replacement task completes.</p>
    pub delete_replaced_root_volume: ::std::option::Option<bool>,
}
impl ReplaceRootVolumeTask {
    /// <p>The ID of the root volume replacement task.</p>
    pub fn replace_root_volume_task_id(&self) -> ::std::option::Option<&str> {
        self.replace_root_volume_task_id.as_deref()
    }
    /// <p>The ID of the instance for which the root volume replacement task was created.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The state of the task. The task can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>pending</code> - the replacement volume is being created.</p> </li>
    /// <li> <p> <code>in-progress</code> - the original volume is being detached and the replacement volume is being attached.</p> </li>
    /// <li> <p> <code>succeeded</code> - the replacement volume has been successfully attached to the instance and the instance is available.</p> </li>
    /// <li> <p> <code>failing</code> - the replacement task is in the process of failing.</p> </li>
    /// <li> <p> <code>failed</code> - the replacement task has failed but the original root volume is still attached.</p> </li>
    /// <li> <p> <code>failing-detached</code> - the replacement task is in the process of failing. The instance might have no root volume attached.</p> </li>
    /// <li> <p> <code>failed-detached</code> - the replacement task has failed and the instance has no root volume attached.</p> </li>
    /// </ul>
    pub fn task_state(&self) -> ::std::option::Option<&crate::types::ReplaceRootVolumeTaskState> {
        self.task_state.as_ref()
    }
    /// <p>The time the task was started.</p>
    pub fn start_time(&self) -> ::std::option::Option<&str> {
        self.start_time.as_deref()
    }
    /// <p>The time the task completed.</p>
    pub fn complete_time(&self) -> ::std::option::Option<&str> {
        self.complete_time.as_deref()
    }
    /// <p>The tags assigned to the task.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
    /// <p>The ID of the AMI used to create the replacement root volume.</p>
    pub fn image_id(&self) -> ::std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>The ID of the snapshot used to create the replacement root volume.</p>
    pub fn snapshot_id(&self) -> ::std::option::Option<&str> {
        self.snapshot_id.as_deref()
    }
    /// <p>Indicates whether the original root volume is to be deleted after the root volume replacement task completes.</p>
    pub fn delete_replaced_root_volume(&self) -> ::std::option::Option<bool> {
        self.delete_replaced_root_volume
    }
}
impl ReplaceRootVolumeTask {
    /// Creates a new builder-style object to manufacture [`ReplaceRootVolumeTask`](crate::types::ReplaceRootVolumeTask).
    pub fn builder() -> crate::types::builders::ReplaceRootVolumeTaskBuilder {
        crate::types::builders::ReplaceRootVolumeTaskBuilder::default()
    }
}

/// A builder for [`ReplaceRootVolumeTask`](crate::types::ReplaceRootVolumeTask).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ReplaceRootVolumeTaskBuilder {
    pub(crate) replace_root_volume_task_id: ::std::option::Option<::std::string::String>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) task_state: ::std::option::Option<crate::types::ReplaceRootVolumeTaskState>,
    pub(crate) start_time: ::std::option::Option<::std::string::String>,
    pub(crate) complete_time: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) image_id: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_id: ::std::option::Option<::std::string::String>,
    pub(crate) delete_replaced_root_volume: ::std::option::Option<bool>,
}
impl ReplaceRootVolumeTaskBuilder {
    /// <p>The ID of the root volume replacement task.</p>
    pub fn replace_root_volume_task_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.replace_root_volume_task_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the root volume replacement task.</p>
    pub fn set_replace_root_volume_task_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.replace_root_volume_task_id = input;
        self
    }
    /// <p>The ID of the root volume replacement task.</p>
    pub fn get_replace_root_volume_task_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.replace_root_volume_task_id
    }
    /// <p>The ID of the instance for which the root volume replacement task was created.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the instance for which the root volume replacement task was created.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The ID of the instance for which the root volume replacement task was created.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_id
    }
    /// <p>The state of the task. The task can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>pending</code> - the replacement volume is being created.</p> </li>
    /// <li> <p> <code>in-progress</code> - the original volume is being detached and the replacement volume is being attached.</p> </li>
    /// <li> <p> <code>succeeded</code> - the replacement volume has been successfully attached to the instance and the instance is available.</p> </li>
    /// <li> <p> <code>failing</code> - the replacement task is in the process of failing.</p> </li>
    /// <li> <p> <code>failed</code> - the replacement task has failed but the original root volume is still attached.</p> </li>
    /// <li> <p> <code>failing-detached</code> - the replacement task is in the process of failing. The instance might have no root volume attached.</p> </li>
    /// <li> <p> <code>failed-detached</code> - the replacement task has failed and the instance has no root volume attached.</p> </li>
    /// </ul>
    pub fn task_state(mut self, input: crate::types::ReplaceRootVolumeTaskState) -> Self {
        self.task_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the task. The task can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>pending</code> - the replacement volume is being created.</p> </li>
    /// <li> <p> <code>in-progress</code> - the original volume is being detached and the replacement volume is being attached.</p> </li>
    /// <li> <p> <code>succeeded</code> - the replacement volume has been successfully attached to the instance and the instance is available.</p> </li>
    /// <li> <p> <code>failing</code> - the replacement task is in the process of failing.</p> </li>
    /// <li> <p> <code>failed</code> - the replacement task has failed but the original root volume is still attached.</p> </li>
    /// <li> <p> <code>failing-detached</code> - the replacement task is in the process of failing. The instance might have no root volume attached.</p> </li>
    /// <li> <p> <code>failed-detached</code> - the replacement task has failed and the instance has no root volume attached.</p> </li>
    /// </ul>
    pub fn set_task_state(mut self, input: ::std::option::Option<crate::types::ReplaceRootVolumeTaskState>) -> Self {
        self.task_state = input;
        self
    }
    /// <p>The state of the task. The task can be in one of the following states:</p>
    /// <ul>
    /// <li> <p> <code>pending</code> - the replacement volume is being created.</p> </li>
    /// <li> <p> <code>in-progress</code> - the original volume is being detached and the replacement volume is being attached.</p> </li>
    /// <li> <p> <code>succeeded</code> - the replacement volume has been successfully attached to the instance and the instance is available.</p> </li>
    /// <li> <p> <code>failing</code> - the replacement task is in the process of failing.</p> </li>
    /// <li> <p> <code>failed</code> - the replacement task has failed but the original root volume is still attached.</p> </li>
    /// <li> <p> <code>failing-detached</code> - the replacement task is in the process of failing. The instance might have no root volume attached.</p> </li>
    /// <li> <p> <code>failed-detached</code> - the replacement task has failed and the instance has no root volume attached.</p> </li>
    /// </ul>
    pub fn get_task_state(&self) -> &::std::option::Option<crate::types::ReplaceRootVolumeTaskState> {
        &self.task_state
    }
    /// <p>The time the task was started.</p>
    pub fn start_time(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.start_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The time the task was started.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The time the task was started.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::std::string::String> {
        &self.start_time
    }
    /// <p>The time the task completed.</p>
    pub fn complete_time(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.complete_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The time the task completed.</p>
    pub fn set_complete_time(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.complete_time = input;
        self
    }
    /// <p>The time the task completed.</p>
    pub fn get_complete_time(&self) -> &::std::option::Option<::std::string::String> {
        &self.complete_time
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags assigned to the task.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags assigned to the task.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags assigned to the task.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// <p>The ID of the AMI used to create the replacement root volume.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the AMI used to create the replacement root volume.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>The ID of the AMI used to create the replacement root volume.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.image_id
    }
    /// <p>The ID of the snapshot used to create the replacement root volume.</p>
    pub fn snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.snapshot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the snapshot used to create the replacement root volume.</p>
    pub fn set_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.snapshot_id = input;
        self
    }
    /// <p>The ID of the snapshot used to create the replacement root volume.</p>
    pub fn get_snapshot_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.snapshot_id
    }
    /// <p>Indicates whether the original root volume is to be deleted after the root volume replacement task completes.</p>
    pub fn delete_replaced_root_volume(mut self, input: bool) -> Self {
        self.delete_replaced_root_volume = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the original root volume is to be deleted after the root volume replacement task completes.</p>
    pub fn set_delete_replaced_root_volume(mut self, input: ::std::option::Option<bool>) -> Self {
        self.delete_replaced_root_volume = input;
        self
    }
    /// <p>Indicates whether the original root volume is to be deleted after the root volume replacement task completes.</p>
    pub fn get_delete_replaced_root_volume(&self) -> &::std::option::Option<bool> {
        &self.delete_replaced_root_volume
    }
    /// Consumes the builder and constructs a [`ReplaceRootVolumeTask`](crate::types::ReplaceRootVolumeTask).
    pub fn build(self) -> crate::types::ReplaceRootVolumeTask {
        crate::types::ReplaceRootVolumeTask {
            replace_root_volume_task_id: self.replace_root_volume_task_id,
            instance_id: self.instance_id,
            task_state: self.task_state,
            start_time: self.start_time,
            complete_time: self.complete_time,
            tags: self.tags,
            image_id: self.image_id,
            snapshot_id: self.snapshot_id,
            delete_replaced_root_volume: self.delete_replaced_root_volume,
        }
    }
}
