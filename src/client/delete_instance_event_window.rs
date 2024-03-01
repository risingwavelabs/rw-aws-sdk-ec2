// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteInstanceEventWindow`](crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`force_delete(bool)`](crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowFluentBuilder::force_delete) / [`set_force_delete(Option<bool>)`](crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowFluentBuilder::set_force_delete):<br>required: **false**<br><p>Specify <code>true</code> to force delete the event window. Use the force delete parameter if the event window is currently associated with targets.</p><br>
    ///   - [`instance_event_window_id(impl Into<String>)`](crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowFluentBuilder::instance_event_window_id) / [`set_instance_event_window_id(Option<String>)`](crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowFluentBuilder::set_instance_event_window_id):<br>required: **true**<br><p>The ID of the event window.</p><br>
    /// - On success, responds with [`DeleteInstanceEventWindowOutput`](crate::operation::delete_instance_event_window::DeleteInstanceEventWindowOutput) with field(s):
    ///   - [`instance_event_window_state(Option<InstanceEventWindowStateChange>)`](crate::operation::delete_instance_event_window::DeleteInstanceEventWindowOutput::instance_event_window_state): <p>The state of the event window.</p>
    /// - On failure, responds with [`SdkError<DeleteInstanceEventWindowError>`](crate::operation::delete_instance_event_window::DeleteInstanceEventWindowError)
    pub fn delete_instance_event_window(&self) -> crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowFluentBuilder {
        crate::operation::delete_instance_event_window::builders::DeleteInstanceEventWindowFluentBuilder::new(self.handle.clone())
    }
}
