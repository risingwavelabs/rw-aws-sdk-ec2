// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CancelConversionTask`](crate::operation::cancel_conversion_task::builders::CancelConversionTaskFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`conversion_task_id(impl Into<String>)`](crate::operation::cancel_conversion_task::builders::CancelConversionTaskFluentBuilder::conversion_task_id) / [`set_conversion_task_id(Option<String>)`](crate::operation::cancel_conversion_task::builders::CancelConversionTaskFluentBuilder::set_conversion_task_id):<br>required: **true**<br><p>The ID of the conversion task.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::cancel_conversion_task::builders::CancelConversionTaskFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::cancel_conversion_task::builders::CancelConversionTaskFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`reason_message(impl Into<String>)`](crate::operation::cancel_conversion_task::builders::CancelConversionTaskFluentBuilder::reason_message) / [`set_reason_message(Option<String>)`](crate::operation::cancel_conversion_task::builders::CancelConversionTaskFluentBuilder::set_reason_message):<br>required: **false**<br><p>The reason for canceling the conversion task.</p><br>
    /// - On success, responds with [`CancelConversionTaskOutput`](crate::operation::cancel_conversion_task::CancelConversionTaskOutput)
    /// - On failure, responds with [`SdkError<CancelConversionTaskError>`](crate::operation::cancel_conversion_task::CancelConversionTaskError)
    pub fn cancel_conversion_task(&self) -> crate::operation::cancel_conversion_task::builders::CancelConversionTaskFluentBuilder {
        crate::operation::cancel_conversion_task::builders::CancelConversionTaskFluentBuilder::new(self.handle.clone())
    }
}