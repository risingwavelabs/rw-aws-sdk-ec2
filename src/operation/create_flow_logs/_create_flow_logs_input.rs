// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateFlowLogsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs to a CloudWatch Logs log group in your account.</p>
    /// <p>This parameter is required if the destination type is <code>cloud-watch-logs</code> and unsupported otherwise.</p>
    pub deliver_logs_permission_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.</p>
    pub deliver_cross_account_role: ::std::option::Option<::std::string::String>,
    /// <p>The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs.</p>
    /// <p>This parameter is valid only if the destination type is <code>cloud-watch-logs</code>.</p>
    pub log_group_name: ::std::option::Option<::std::string::String>,
    /// <p>The IDs of the resources to monitor. For example, if the resource type is <code>VPC</code>, specify the IDs of the VPCs.</p>
    /// <p>Constraints: Maximum of 25 for transit gateway resource types. Maximum of 1000 for the other resource types.</p>
    pub resource_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The type of resource to monitor.</p>
    pub resource_type: ::std::option::Option<crate::types::FlowLogsResourceType>,
    /// <p>The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic). This parameter is not supported for transit gateway resource types. It is required for the other resource types.</p>
    pub traffic_type: ::std::option::Option<crate::types::TrafficType>,
    /// <p>The type of destination for the flow log data.</p>
    /// <p>Default: <code>cloud-watch-logs</code> </p>
    pub log_destination_type: ::std::option::Option<crate::types::LogDestinationType>,
    /// <p>The destination for the flow log data. The meaning of this parameter depends on the destination type.</p>
    /// <ul>
    /// <li> <p>If the destination type is <code>cloud-watch-logs</code>, specify the ARN of a CloudWatch Logs log group. For example:</p> <p>arn:aws:logs:<i>region</i>:<i>account_id</i>:log-group:<i>my_group</i> </p> <p>Alternatively, use the <code>LogGroupName</code> parameter.</p> </li>
    /// <li> <p>If the destination type is <code>s3</code>, specify the ARN of an S3 bucket. For example:</p> <p>arn:aws:s3:::<i>my_bucket</i>/<i>my_subfolder</i>/</p> <p>The subfolder is optional. Note that you can't use <code>AWSLogs</code> as a subfolder name.</p> </li>
    /// <li> <p>If the destination type is <code>kinesis-data-firehose</code>, specify the ARN of a Kinesis Data Firehose delivery stream. For example:</p> <p>arn:aws:firehose:<i>region</i>:<i>account_id</i>:deliverystream:<i>my_stream</i> </p> </li>
    /// </ul>
    pub log_destination: ::std::option::Option<::std::string::String>,
    /// <p>The fields to include in the flow log record. List the fields in the order in which they should appear. If you omit this parameter, the flow log is created using the default format. If you specify this parameter, you must include at least one field. For more information about the available fields, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs.html#flow-log-records">Flow log records</a> in the <i>Amazon VPC User Guide</i> or <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-flow-logs.html#flow-log-records">Transit Gateway Flow Log records</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    /// <p>Specify the fields using the <code>${field-id}</code> format, separated by spaces.</p>
    pub log_format: ::std::option::Option<::std::string::String>,
    /// <p>The tags to apply to the flow logs.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record. The possible values are 60 seconds (1 minute) or 600 seconds (10 minutes). This parameter must be 60 seconds for transit gateway resource types.</p>
    /// <p>When a network interface is attached to a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Nitro-based instance</a>, the aggregation interval is always 60 seconds or less, regardless of the value that you specify.</p>
    /// <p>Default: 600</p>
    pub max_aggregation_interval: ::std::option::Option<i32>,
    /// <p>The destination options.</p>
    pub destination_options: ::std::option::Option<crate::types::DestinationOptionsRequest>,
}
impl CreateFlowLogsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs to a CloudWatch Logs log group in your account.</p>
    /// <p>This parameter is required if the destination type is <code>cloud-watch-logs</code> and unsupported otherwise.</p>
    pub fn deliver_logs_permission_arn(&self) -> ::std::option::Option<&str> {
        self.deliver_logs_permission_arn.as_deref()
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.</p>
    pub fn deliver_cross_account_role(&self) -> ::std::option::Option<&str> {
        self.deliver_cross_account_role.as_deref()
    }
    /// <p>The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs.</p>
    /// <p>This parameter is valid only if the destination type is <code>cloud-watch-logs</code>.</p>
    pub fn log_group_name(&self) -> ::std::option::Option<&str> {
        self.log_group_name.as_deref()
    }
    /// <p>The IDs of the resources to monitor. For example, if the resource type is <code>VPC</code>, specify the IDs of the VPCs.</p>
    /// <p>Constraints: Maximum of 25 for transit gateway resource types. Maximum of 1000 for the other resource types.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.resource_ids.is_none()`.
    pub fn resource_ids(&self) -> &[::std::string::String] {
        self.resource_ids.as_deref().unwrap_or_default()
    }
    /// <p>The type of resource to monitor.</p>
    pub fn resource_type(&self) -> ::std::option::Option<&crate::types::FlowLogsResourceType> {
        self.resource_type.as_ref()
    }
    /// <p>The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic). This parameter is not supported for transit gateway resource types. It is required for the other resource types.</p>
    pub fn traffic_type(&self) -> ::std::option::Option<&crate::types::TrafficType> {
        self.traffic_type.as_ref()
    }
    /// <p>The type of destination for the flow log data.</p>
    /// <p>Default: <code>cloud-watch-logs</code> </p>
    pub fn log_destination_type(&self) -> ::std::option::Option<&crate::types::LogDestinationType> {
        self.log_destination_type.as_ref()
    }
    /// <p>The destination for the flow log data. The meaning of this parameter depends on the destination type.</p>
    /// <ul>
    /// <li> <p>If the destination type is <code>cloud-watch-logs</code>, specify the ARN of a CloudWatch Logs log group. For example:</p> <p>arn:aws:logs:<i>region</i>:<i>account_id</i>:log-group:<i>my_group</i> </p> <p>Alternatively, use the <code>LogGroupName</code> parameter.</p> </li>
    /// <li> <p>If the destination type is <code>s3</code>, specify the ARN of an S3 bucket. For example:</p> <p>arn:aws:s3:::<i>my_bucket</i>/<i>my_subfolder</i>/</p> <p>The subfolder is optional. Note that you can't use <code>AWSLogs</code> as a subfolder name.</p> </li>
    /// <li> <p>If the destination type is <code>kinesis-data-firehose</code>, specify the ARN of a Kinesis Data Firehose delivery stream. For example:</p> <p>arn:aws:firehose:<i>region</i>:<i>account_id</i>:deliverystream:<i>my_stream</i> </p> </li>
    /// </ul>
    pub fn log_destination(&self) -> ::std::option::Option<&str> {
        self.log_destination.as_deref()
    }
    /// <p>The fields to include in the flow log record. List the fields in the order in which they should appear. If you omit this parameter, the flow log is created using the default format. If you specify this parameter, you must include at least one field. For more information about the available fields, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs.html#flow-log-records">Flow log records</a> in the <i>Amazon VPC User Guide</i> or <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-flow-logs.html#flow-log-records">Transit Gateway Flow Log records</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    /// <p>Specify the fields using the <code>${field-id}</code> format, separated by spaces.</p>
    pub fn log_format(&self) -> ::std::option::Option<&str> {
        self.log_format.as_deref()
    }
    /// <p>The tags to apply to the flow logs.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record. The possible values are 60 seconds (1 minute) or 600 seconds (10 minutes). This parameter must be 60 seconds for transit gateway resource types.</p>
    /// <p>When a network interface is attached to a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Nitro-based instance</a>, the aggregation interval is always 60 seconds or less, regardless of the value that you specify.</p>
    /// <p>Default: 600</p>
    pub fn max_aggregation_interval(&self) -> ::std::option::Option<i32> {
        self.max_aggregation_interval
    }
    /// <p>The destination options.</p>
    pub fn destination_options(&self) -> ::std::option::Option<&crate::types::DestinationOptionsRequest> {
        self.destination_options.as_ref()
    }
}
impl CreateFlowLogsInput {
    /// Creates a new builder-style object to manufacture [`CreateFlowLogsInput`](crate::operation::create_flow_logs::CreateFlowLogsInput).
    pub fn builder() -> crate::operation::create_flow_logs::builders::CreateFlowLogsInputBuilder {
        crate::operation::create_flow_logs::builders::CreateFlowLogsInputBuilder::default()
    }
}

/// A builder for [`CreateFlowLogsInput`](crate::operation::create_flow_logs::CreateFlowLogsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateFlowLogsInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) deliver_logs_permission_arn: ::std::option::Option<::std::string::String>,
    pub(crate) deliver_cross_account_role: ::std::option::Option<::std::string::String>,
    pub(crate) log_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) resource_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) resource_type: ::std::option::Option<crate::types::FlowLogsResourceType>,
    pub(crate) traffic_type: ::std::option::Option<crate::types::TrafficType>,
    pub(crate) log_destination_type: ::std::option::Option<crate::types::LogDestinationType>,
    pub(crate) log_destination: ::std::option::Option<::std::string::String>,
    pub(crate) log_format: ::std::option::Option<::std::string::String>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) max_aggregation_interval: ::std::option::Option<i32>,
    pub(crate) destination_options: ::std::option::Option<crate::types::DestinationOptionsRequest>,
}
impl CreateFlowLogsInputBuilder {
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs to a CloudWatch Logs log group in your account.</p>
    /// <p>This parameter is required if the destination type is <code>cloud-watch-logs</code> and unsupported otherwise.</p>
    pub fn deliver_logs_permission_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.deliver_logs_permission_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs to a CloudWatch Logs log group in your account.</p>
    /// <p>This parameter is required if the destination type is <code>cloud-watch-logs</code> and unsupported otherwise.</p>
    pub fn set_deliver_logs_permission_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.deliver_logs_permission_arn = input;
        self
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs to a CloudWatch Logs log group in your account.</p>
    /// <p>This parameter is required if the destination type is <code>cloud-watch-logs</code> and unsupported otherwise.</p>
    pub fn get_deliver_logs_permission_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.deliver_logs_permission_arn
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.</p>
    pub fn deliver_cross_account_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.deliver_cross_account_role = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.</p>
    pub fn set_deliver_cross_account_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.deliver_cross_account_role = input;
        self
    }
    /// <p>The ARN of the IAM role that allows Amazon EC2 to publish flow logs across accounts.</p>
    pub fn get_deliver_cross_account_role(&self) -> &::std::option::Option<::std::string::String> {
        &self.deliver_cross_account_role
    }
    /// <p>The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs.</p>
    /// <p>This parameter is valid only if the destination type is <code>cloud-watch-logs</code>.</p>
    pub fn log_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs.</p>
    /// <p>This parameter is valid only if the destination type is <code>cloud-watch-logs</code>.</p>
    pub fn set_log_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group_name = input;
        self
    }
    /// <p>The name of a new or existing CloudWatch Logs log group where Amazon EC2 publishes your flow logs.</p>
    /// <p>This parameter is valid only if the destination type is <code>cloud-watch-logs</code>.</p>
    pub fn get_log_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group_name
    }
    /// Appends an item to `resource_ids`.
    ///
    /// To override the contents of this collection use [`set_resource_ids`](Self::set_resource_ids).
    ///
    /// <p>The IDs of the resources to monitor. For example, if the resource type is <code>VPC</code>, specify the IDs of the VPCs.</p>
    /// <p>Constraints: Maximum of 25 for transit gateway resource types. Maximum of 1000 for the other resource types.</p>
    pub fn resource_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.resource_ids.unwrap_or_default();
        v.push(input.into());
        self.resource_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the resources to monitor. For example, if the resource type is <code>VPC</code>, specify the IDs of the VPCs.</p>
    /// <p>Constraints: Maximum of 25 for transit gateway resource types. Maximum of 1000 for the other resource types.</p>
    pub fn set_resource_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.resource_ids = input;
        self
    }
    /// <p>The IDs of the resources to monitor. For example, if the resource type is <code>VPC</code>, specify the IDs of the VPCs.</p>
    /// <p>Constraints: Maximum of 25 for transit gateway resource types. Maximum of 1000 for the other resource types.</p>
    pub fn get_resource_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.resource_ids
    }
    /// <p>The type of resource to monitor.</p>
    /// This field is required.
    pub fn resource_type(mut self, input: crate::types::FlowLogsResourceType) -> Self {
        self.resource_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of resource to monitor.</p>
    pub fn set_resource_type(mut self, input: ::std::option::Option<crate::types::FlowLogsResourceType>) -> Self {
        self.resource_type = input;
        self
    }
    /// <p>The type of resource to monitor.</p>
    pub fn get_resource_type(&self) -> &::std::option::Option<crate::types::FlowLogsResourceType> {
        &self.resource_type
    }
    /// <p>The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic). This parameter is not supported for transit gateway resource types. It is required for the other resource types.</p>
    pub fn traffic_type(mut self, input: crate::types::TrafficType) -> Self {
        self.traffic_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic). This parameter is not supported for transit gateway resource types. It is required for the other resource types.</p>
    pub fn set_traffic_type(mut self, input: ::std::option::Option<crate::types::TrafficType>) -> Self {
        self.traffic_type = input;
        self
    }
    /// <p>The type of traffic to monitor (accepted traffic, rejected traffic, or all traffic). This parameter is not supported for transit gateway resource types. It is required for the other resource types.</p>
    pub fn get_traffic_type(&self) -> &::std::option::Option<crate::types::TrafficType> {
        &self.traffic_type
    }
    /// <p>The type of destination for the flow log data.</p>
    /// <p>Default: <code>cloud-watch-logs</code> </p>
    pub fn log_destination_type(mut self, input: crate::types::LogDestinationType) -> Self {
        self.log_destination_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of destination for the flow log data.</p>
    /// <p>Default: <code>cloud-watch-logs</code> </p>
    pub fn set_log_destination_type(mut self, input: ::std::option::Option<crate::types::LogDestinationType>) -> Self {
        self.log_destination_type = input;
        self
    }
    /// <p>The type of destination for the flow log data.</p>
    /// <p>Default: <code>cloud-watch-logs</code> </p>
    pub fn get_log_destination_type(&self) -> &::std::option::Option<crate::types::LogDestinationType> {
        &self.log_destination_type
    }
    /// <p>The destination for the flow log data. The meaning of this parameter depends on the destination type.</p>
    /// <ul>
    /// <li> <p>If the destination type is <code>cloud-watch-logs</code>, specify the ARN of a CloudWatch Logs log group. For example:</p> <p>arn:aws:logs:<i>region</i>:<i>account_id</i>:log-group:<i>my_group</i> </p> <p>Alternatively, use the <code>LogGroupName</code> parameter.</p> </li>
    /// <li> <p>If the destination type is <code>s3</code>, specify the ARN of an S3 bucket. For example:</p> <p>arn:aws:s3:::<i>my_bucket</i>/<i>my_subfolder</i>/</p> <p>The subfolder is optional. Note that you can't use <code>AWSLogs</code> as a subfolder name.</p> </li>
    /// <li> <p>If the destination type is <code>kinesis-data-firehose</code>, specify the ARN of a Kinesis Data Firehose delivery stream. For example:</p> <p>arn:aws:firehose:<i>region</i>:<i>account_id</i>:deliverystream:<i>my_stream</i> </p> </li>
    /// </ul>
    pub fn log_destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_destination = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The destination for the flow log data. The meaning of this parameter depends on the destination type.</p>
    /// <ul>
    /// <li> <p>If the destination type is <code>cloud-watch-logs</code>, specify the ARN of a CloudWatch Logs log group. For example:</p> <p>arn:aws:logs:<i>region</i>:<i>account_id</i>:log-group:<i>my_group</i> </p> <p>Alternatively, use the <code>LogGroupName</code> parameter.</p> </li>
    /// <li> <p>If the destination type is <code>s3</code>, specify the ARN of an S3 bucket. For example:</p> <p>arn:aws:s3:::<i>my_bucket</i>/<i>my_subfolder</i>/</p> <p>The subfolder is optional. Note that you can't use <code>AWSLogs</code> as a subfolder name.</p> </li>
    /// <li> <p>If the destination type is <code>kinesis-data-firehose</code>, specify the ARN of a Kinesis Data Firehose delivery stream. For example:</p> <p>arn:aws:firehose:<i>region</i>:<i>account_id</i>:deliverystream:<i>my_stream</i> </p> </li>
    /// </ul>
    pub fn set_log_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_destination = input;
        self
    }
    /// <p>The destination for the flow log data. The meaning of this parameter depends on the destination type.</p>
    /// <ul>
    /// <li> <p>If the destination type is <code>cloud-watch-logs</code>, specify the ARN of a CloudWatch Logs log group. For example:</p> <p>arn:aws:logs:<i>region</i>:<i>account_id</i>:log-group:<i>my_group</i> </p> <p>Alternatively, use the <code>LogGroupName</code> parameter.</p> </li>
    /// <li> <p>If the destination type is <code>s3</code>, specify the ARN of an S3 bucket. For example:</p> <p>arn:aws:s3:::<i>my_bucket</i>/<i>my_subfolder</i>/</p> <p>The subfolder is optional. Note that you can't use <code>AWSLogs</code> as a subfolder name.</p> </li>
    /// <li> <p>If the destination type is <code>kinesis-data-firehose</code>, specify the ARN of a Kinesis Data Firehose delivery stream. For example:</p> <p>arn:aws:firehose:<i>region</i>:<i>account_id</i>:deliverystream:<i>my_stream</i> </p> </li>
    /// </ul>
    pub fn get_log_destination(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_destination
    }
    /// <p>The fields to include in the flow log record. List the fields in the order in which they should appear. If you omit this parameter, the flow log is created using the default format. If you specify this parameter, you must include at least one field. For more information about the available fields, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs.html#flow-log-records">Flow log records</a> in the <i>Amazon VPC User Guide</i> or <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-flow-logs.html#flow-log-records">Transit Gateway Flow Log records</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    /// <p>Specify the fields using the <code>${field-id}</code> format, separated by spaces.</p>
    pub fn log_format(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_format = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The fields to include in the flow log record. List the fields in the order in which they should appear. If you omit this parameter, the flow log is created using the default format. If you specify this parameter, you must include at least one field. For more information about the available fields, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs.html#flow-log-records">Flow log records</a> in the <i>Amazon VPC User Guide</i> or <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-flow-logs.html#flow-log-records">Transit Gateway Flow Log records</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    /// <p>Specify the fields using the <code>${field-id}</code> format, separated by spaces.</p>
    pub fn set_log_format(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_format = input;
        self
    }
    /// <p>The fields to include in the flow log record. List the fields in the order in which they should appear. If you omit this parameter, the flow log is created using the default format. If you specify this parameter, you must include at least one field. For more information about the available fields, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/flow-logs.html#flow-log-records">Flow log records</a> in the <i>Amazon VPC User Guide</i> or <a href="https://docs.aws.amazon.com/vpc/latest/tgw/tgw-flow-logs.html#flow-log-records">Transit Gateway Flow Log records</a> in the <i>Amazon Web Services Transit Gateway Guide</i>.</p>
    /// <p>Specify the fields using the <code>${field-id}</code> format, separated by spaces.</p>
    pub fn get_log_format(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_format
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the flow logs.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the flow logs.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to apply to the flow logs.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
    /// <p>The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record. The possible values are 60 seconds (1 minute) or 600 seconds (10 minutes). This parameter must be 60 seconds for transit gateway resource types.</p>
    /// <p>When a network interface is attached to a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Nitro-based instance</a>, the aggregation interval is always 60 seconds or less, regardless of the value that you specify.</p>
    /// <p>Default: 600</p>
    pub fn max_aggregation_interval(mut self, input: i32) -> Self {
        self.max_aggregation_interval = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record. The possible values are 60 seconds (1 minute) or 600 seconds (10 minutes). This parameter must be 60 seconds for transit gateway resource types.</p>
    /// <p>When a network interface is attached to a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Nitro-based instance</a>, the aggregation interval is always 60 seconds or less, regardless of the value that you specify.</p>
    /// <p>Default: 600</p>
    pub fn set_max_aggregation_interval(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_aggregation_interval = input;
        self
    }
    /// <p>The maximum interval of time during which a flow of packets is captured and aggregated into a flow log record. The possible values are 60 seconds (1 minute) or 600 seconds (10 minutes). This parameter must be 60 seconds for transit gateway resource types.</p>
    /// <p>When a network interface is attached to a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Nitro-based instance</a>, the aggregation interval is always 60 seconds or less, regardless of the value that you specify.</p>
    /// <p>Default: 600</p>
    pub fn get_max_aggregation_interval(&self) -> &::std::option::Option<i32> {
        &self.max_aggregation_interval
    }
    /// <p>The destination options.</p>
    pub fn destination_options(mut self, input: crate::types::DestinationOptionsRequest) -> Self {
        self.destination_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination options.</p>
    pub fn set_destination_options(mut self, input: ::std::option::Option<crate::types::DestinationOptionsRequest>) -> Self {
        self.destination_options = input;
        self
    }
    /// <p>The destination options.</p>
    pub fn get_destination_options(&self) -> &::std::option::Option<crate::types::DestinationOptionsRequest> {
        &self.destination_options
    }
    /// Consumes the builder and constructs a [`CreateFlowLogsInput`](crate::operation::create_flow_logs::CreateFlowLogsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_flow_logs::CreateFlowLogsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_flow_logs::CreateFlowLogsInput {
            dry_run: self.dry_run,
            client_token: self.client_token,
            deliver_logs_permission_arn: self.deliver_logs_permission_arn,
            deliver_cross_account_role: self.deliver_cross_account_role,
            log_group_name: self.log_group_name,
            resource_ids: self.resource_ids,
            resource_type: self.resource_type,
            traffic_type: self.traffic_type,
            log_destination_type: self.log_destination_type,
            log_destination: self.log_destination,
            log_format: self.log_format,
            tag_specifications: self.tag_specifications,
            max_aggregation_interval: self.max_aggregation_interval,
            destination_options: self.destination_options,
        })
    }
}