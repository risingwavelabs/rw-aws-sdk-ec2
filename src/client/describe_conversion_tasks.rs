// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeConversionTasks`](crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`conversion_task_ids(impl Into<String>)`](crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksFluentBuilder::conversion_task_ids) / [`set_conversion_task_ids(Option<Vec::<String>>)`](crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksFluentBuilder::set_conversion_task_ids):<br>required: **false**<br><p>The conversion task IDs.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DescribeConversionTasksOutput`](crate::operation::describe_conversion_tasks::DescribeConversionTasksOutput) with field(s):
    ///   - [`conversion_tasks(Option<Vec::<ConversionTask>>)`](crate::operation::describe_conversion_tasks::DescribeConversionTasksOutput::conversion_tasks): <p>Information about the conversion tasks.</p>
    /// - On failure, responds with [`SdkError<DescribeConversionTasksError>`](crate::operation::describe_conversion_tasks::DescribeConversionTasksError)
    pub fn describe_conversion_tasks(&self) -> crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksFluentBuilder {
        crate::operation::describe_conversion_tasks::builders::DescribeConversionTasksFluentBuilder::new(self.handle.clone())
    }
}
