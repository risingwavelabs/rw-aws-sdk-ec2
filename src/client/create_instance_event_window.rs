// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateInstanceEventWindow`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::set_name):<br>required: **false**<br><p>The name of the event window.</p><br>
    ///   - [`time_ranges(InstanceEventWindowTimeRangeRequest)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::time_ranges) / [`set_time_ranges(Option<Vec::<InstanceEventWindowTimeRangeRequest>>)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::set_time_ranges):<br>required: **false**<br><p>The time range for the event window. If you specify a time range, you can't specify a cron expression.</p><br>
    ///   - [`cron_expression(impl Into<String>)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::cron_expression) / [`set_cron_expression(Option<String>)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::set_cron_expression):<br>required: **false**<br><p>The cron expression for the event window, for example, <code>* 0-4,20-23 * * 1,5</code>. If you specify a cron expression, you can't specify a time range.</p>  <p>Constraints:</p>  <ul>   <li> <p>Only hour and day of the week values are supported.</p> </li>   <li> <p>For day of the week values, you can specify either integers <code>0</code> through <code>6</code>, or alternative single values <code>SUN</code> through <code>SAT</code>.</p> </li>   <li> <p>The minute, month, and year must be specified by <code>*</code>.</p> </li>   <li> <p>The hour value must be one or a multiple range, for example, <code>0-4</code> or <code>0-4,20-23</code>.</p> </li>   <li> <p>Each hour range must be &gt;= 2 hours, for example, <code>0-2</code> or <code>20-23</code>.</p> </li>   <li> <p>The event window must be &gt;= 4 hours. The combined total time ranges in the event window must be &gt;= 4 hours.</p> </li>  </ul>  <p>For more information about cron expressions, see <a href="https://en.wikipedia.org/wiki/Cron">cron</a> on the <i>Wikipedia website</i>.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to apply to the event window.</p><br>
    /// - On success, responds with [`CreateInstanceEventWindowOutput`](crate::operation::create_instance_event_window::CreateInstanceEventWindowOutput) with field(s):
    ///   - [`instance_event_window(Option<InstanceEventWindow>)`](crate::operation::create_instance_event_window::CreateInstanceEventWindowOutput::instance_event_window): <p>Information about the event window.</p>
    /// - On failure, responds with [`SdkError<CreateInstanceEventWindowError>`](crate::operation::create_instance_event_window::CreateInstanceEventWindowError)
    pub fn create_instance_event_window(&self) -> crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder {
        crate::operation::create_instance_event_window::builders::CreateInstanceEventWindowFluentBuilder::new(self.handle.clone())
    }
}