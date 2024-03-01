// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAggregateIdFormat`](crate::operation::describe_aggregate_id_format::builders::DescribeAggregateIdFormatFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_aggregate_id_format::builders::DescribeAggregateIdFormatFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_aggregate_id_format::builders::DescribeAggregateIdFormatFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DescribeAggregateIdFormatOutput`](crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatOutput) with field(s):
    ///   - [`use_long_ids_aggregated(Option<bool>)`](crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatOutput::use_long_ids_aggregated): <p>Indicates whether all resource types in the Region are configured to use longer IDs. This value is only <code>true</code> if all users are configured to use longer IDs for all resources types in the Region.</p>
    ///   - [`statuses(Option<Vec::<IdFormat>>)`](crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatOutput::statuses): <p>Information about each resource's ID format.</p>
    /// - On failure, responds with [`SdkError<DescribeAggregateIdFormatError>`](crate::operation::describe_aggregate_id_format::DescribeAggregateIdFormatError)
    pub fn describe_aggregate_id_format(&self) -> crate::operation::describe_aggregate_id_format::builders::DescribeAggregateIdFormatFluentBuilder {
        crate::operation::describe_aggregate_id_format::builders::DescribeAggregateIdFormatFluentBuilder::new(self.handle.clone())
    }
}
