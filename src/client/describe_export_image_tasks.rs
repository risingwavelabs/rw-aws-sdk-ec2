// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeExportImageTasks`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::set_filters):<br>required: **false**<br><p>Filter tasks using the <code>task-state</code> filter and one of the following values: <code>active</code>, <code>completed</code>, <code>deleting</code>, or <code>deleted</code>.</p><br>
    ///   - [`export_image_task_ids(impl Into<String>)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::export_image_task_ids) / [`set_export_image_task_ids(Option<Vec::<String>>)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::set_export_image_task_ids):<br>required: **false**<br><p>The IDs of the export image tasks.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in a single call.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::set_next_token):<br>required: **false**<br><p>A token that indicates the next page of results.</p><br>
    /// - On success, responds with [`DescribeExportImageTasksOutput`](crate::operation::describe_export_image_tasks::DescribeExportImageTasksOutput) with field(s):
    ///   - [`export_image_tasks(Option<Vec::<ExportImageTask>>)`](crate::operation::describe_export_image_tasks::DescribeExportImageTasksOutput::export_image_tasks): <p>Information about the export image tasks.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_export_image_tasks::DescribeExportImageTasksOutput::next_token): <p>The token to use to get the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeExportImageTasksError>`](crate::operation::describe_export_image_tasks::DescribeExportImageTasksError)
    pub fn describe_export_image_tasks(&self) -> crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder {
        crate::operation::describe_export_image_tasks::builders::DescribeExportImageTasksFluentBuilder::new(self.handle.clone())
    }
}
