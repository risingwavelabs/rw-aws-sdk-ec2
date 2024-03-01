// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVpcEndpointConnectionNotification`](crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`connection_notification_id(impl Into<String>)`](crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder::connection_notification_id) / [`set_connection_notification_id(Option<String>)`](crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder::set_connection_notification_id):<br>required: **true**<br><p>The ID of the notification.</p><br>
    ///   - [`connection_notification_arn(impl Into<String>)`](crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder::connection_notification_arn) / [`set_connection_notification_arn(Option<String>)`](crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder::set_connection_notification_arn):<br>required: **false**<br><p>The ARN for the SNS topic for the notification.</p><br>
    ///   - [`connection_events(impl Into<String>)`](crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder::connection_events) / [`set_connection_events(Option<Vec::<String>>)`](crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder::set_connection_events):<br>required: **false**<br><p>The events for the endpoint. Valid values are <code>Accept</code>, <code>Connect</code>, <code>Delete</code>, and <code>Reject</code>.</p><br>
    /// - On success, responds with [`ModifyVpcEndpointConnectionNotificationOutput`](crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationOutput) with field(s):
    ///   - [`return_value(Option<bool>)`](crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationOutput::return_value): <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    /// - On failure, responds with [`SdkError<ModifyVpcEndpointConnectionNotificationError>`](crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError)
    pub fn modify_vpc_endpoint_connection_notification(
        &self,
    ) -> crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder {
        crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
