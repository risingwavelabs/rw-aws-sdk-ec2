// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeVpcEndpointConnectionNotificationsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the notification.</p>
    pub connection_notification_id: ::std::option::Option<::std::string::String>,
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>connection-notification-arn</code> - The ARN of the SNS topic for the notification.</p> </li>
    /// <li> <p> <code>connection-notification-id</code> - The ID of the notification.</p> </li>
    /// <li> <p> <code>connection-notification-state</code> - The state of the notification (<code>Enabled</code> | <code>Disabled</code>).</p> </li>
    /// <li> <p> <code>connection-notification-type</code> - The type of notification (<code>Topic</code>).</p> </li>
    /// <li> <p> <code>service-id</code> - The ID of the endpoint service.</p> </li>
    /// <li> <p> <code>vpc-endpoint-id</code> - The ID of the VPC endpoint.</p> </li>
    /// </ul>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token to request the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeVpcEndpointConnectionNotificationsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the notification.</p>
    pub fn connection_notification_id(&self) -> ::std::option::Option<&str> {
        self.connection_notification_id.as_deref()
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>connection-notification-arn</code> - The ARN of the SNS topic for the notification.</p> </li>
    /// <li> <p> <code>connection-notification-id</code> - The ID of the notification.</p> </li>
    /// <li> <p> <code>connection-notification-state</code> - The state of the notification (<code>Enabled</code> | <code>Disabled</code>).</p> </li>
    /// <li> <p> <code>connection-notification-type</code> - The type of notification (<code>Topic</code>).</p> </li>
    /// <li> <p> <code>service-id</code> - The ID of the endpoint service.</p> </li>
    /// <li> <p> <code>vpc-endpoint-id</code> - The ID of the VPC endpoint.</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token to request the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl DescribeVpcEndpointConnectionNotificationsInput {
    /// Creates a new builder-style object to manufacture [`DescribeVpcEndpointConnectionNotificationsInput`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsInput).
    pub fn builder(
    ) -> crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsInputBuilder {
        crate::operation::describe_vpc_endpoint_connection_notifications::builders::DescribeVpcEndpointConnectionNotificationsInputBuilder::default()
    }
}

/// A builder for [`DescribeVpcEndpointConnectionNotificationsInput`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeVpcEndpointConnectionNotificationsInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) connection_notification_id: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
}
impl DescribeVpcEndpointConnectionNotificationsInputBuilder {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// <p>The ID of the notification.</p>
    pub fn connection_notification_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.connection_notification_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the notification.</p>
    pub fn set_connection_notification_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.connection_notification_id = input;
        self
    }
    /// <p>The ID of the notification.</p>
    pub fn get_connection_notification_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.connection_notification_id
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>connection-notification-arn</code> - The ARN of the SNS topic for the notification.</p> </li>
    /// <li> <p> <code>connection-notification-id</code> - The ID of the notification.</p> </li>
    /// <li> <p> <code>connection-notification-state</code> - The state of the notification (<code>Enabled</code> | <code>Disabled</code>).</p> </li>
    /// <li> <p> <code>connection-notification-type</code> - The type of notification (<code>Topic</code>).</p> </li>
    /// <li> <p> <code>service-id</code> - The ID of the endpoint service.</p> </li>
    /// <li> <p> <code>vpc-endpoint-id</code> - The ID of the VPC endpoint.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>connection-notification-arn</code> - The ARN of the SNS topic for the notification.</p> </li>
    /// <li> <p> <code>connection-notification-id</code> - The ID of the notification.</p> </li>
    /// <li> <p> <code>connection-notification-state</code> - The state of the notification (<code>Enabled</code> | <code>Disabled</code>).</p> </li>
    /// <li> <p> <code>connection-notification-type</code> - The type of notification (<code>Topic</code>).</p> </li>
    /// <li> <p> <code>service-id</code> - The ID of the endpoint service.</p> </li>
    /// <li> <p> <code>vpc-endpoint-id</code> - The ID of the VPC endpoint.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>connection-notification-arn</code> - The ARN of the SNS topic for the notification.</p> </li>
    /// <li> <p> <code>connection-notification-id</code> - The ID of the notification.</p> </li>
    /// <li> <p> <code>connection-notification-state</code> - The state of the notification (<code>Enabled</code> | <code>Disabled</code>).</p> </li>
    /// <li> <p> <code>connection-notification-type</code> - The type of notification (<code>Topic</code>).</p> </li>
    /// <li> <p> <code>service-id</code> - The ID of the endpoint service.</p> </li>
    /// <li> <p> <code>vpc-endpoint-id</code> - The ID of the VPC endpoint.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of results to return in a single call. To retrieve the remaining results, make another request with the returned <code>NextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token to request the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to request the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to request the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Consumes the builder and constructs a [`DescribeVpcEndpointConnectionNotificationsInput`](crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::describe_vpc_endpoint_connection_notifications::DescribeVpcEndpointConnectionNotificationsInput {
                dry_run: self.dry_run,
                connection_notification_id: self.connection_notification_id,
                filters: self.filters,
                max_results: self.max_results,
                next_token: self.next_token,
            },
        )
    }
}