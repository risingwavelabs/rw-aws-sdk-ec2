// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for RequestSpotInstances.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RequestSpotInstancesInput {
    /// <p>The user-specified name for a logical grouping of requests.</p>
    /// <p>When you specify an Availability Zone group in a Spot Instance request, all Spot Instances in the request are launched in the same Availability Zone. Instance proximity is maintained with this parameter, but the choice of Availability Zone is not. The group applies only to requests for Spot Instances of the same instance type. Any additional Spot Instance requests that are specified with the same Availability Zone group name are launched in that same Availability Zone, as long as at least one instance from the group is still active.</p>
    /// <p>If there is no active instance running in the Availability Zone group that you specify for a new Spot Instance request (all instances are terminated, the request is expired, or the maximum price you specified falls below current Spot price), then Amazon EC2 launches the instance in any Availability Zone where the constraint can be met. Consequently, the subsequent set of Spot Instances could be placed in a different zone from the original request, even if you specified the same Availability Zone group.</p>
    /// <p>Default: Instances are launched in any available Availability Zone.</p>
    pub availability_zone_group: ::std::option::Option<::std::string::String>,
    /// <p>Deprecated.</p>
    pub block_duration_minutes: ::std::option::Option<i32>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The maximum number of Spot Instances to launch.</p>
    /// <p>Default: 1</p>
    pub instance_count: ::std::option::Option<i32>,
    /// <p>The instance launch group. Launch groups are Spot Instances that launch together and terminate together.</p>
    /// <p>Default: Instances are launched and terminated individually</p>
    pub launch_group: ::std::option::Option<::std::string::String>,
    /// <p>The launch specification.</p>
    pub launch_specification: ::std::option::Option<crate::types::RequestSpotLaunchSpecification>,
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub spot_price: ::std::option::Option<::std::string::String>,
    /// <p>The Spot Instance request type.</p>
    /// <p>Default: <code>one-time</code> </p>
    pub r#type: ::std::option::Option<crate::types::SpotInstanceType>,
    /// <p>The start date of the request. If this is a one-time request, the request becomes active at this date and time and remains active until all instances launch, the request expires, or the request is canceled. If the request is persistent, the request becomes active at this date and time and remains active until it expires or is canceled.</p>
    /// <p>The specified start date and time cannot be equal to the current date and time. You must specify a start date and time that occurs after the current date and time.</p>
    pub valid_from: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it. </p> </li>
    /// <li> <p>For a one-time request, the request remains active until all instances launch, the request is canceled, or the <code>ValidUntil</code> date and time is reached. By default, the request is valid for 7 days from the date the request was created.</p> </li>
    /// </ul>
    pub valid_until: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The key-value pair for tagging the Spot Instance request on creation. The value for <code>ResourceType</code> must be <code>spot-instances-request</code>, otherwise the Spot Instance request fails. To tag the Spot Instance request after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub instance_interruption_behavior: ::std::option::Option<crate::types::InstanceInterruptionBehavior>,
}
impl RequestSpotInstancesInput {
    /// <p>The user-specified name for a logical grouping of requests.</p>
    /// <p>When you specify an Availability Zone group in a Spot Instance request, all Spot Instances in the request are launched in the same Availability Zone. Instance proximity is maintained with this parameter, but the choice of Availability Zone is not. The group applies only to requests for Spot Instances of the same instance type. Any additional Spot Instance requests that are specified with the same Availability Zone group name are launched in that same Availability Zone, as long as at least one instance from the group is still active.</p>
    /// <p>If there is no active instance running in the Availability Zone group that you specify for a new Spot Instance request (all instances are terminated, the request is expired, or the maximum price you specified falls below current Spot price), then Amazon EC2 launches the instance in any Availability Zone where the constraint can be met. Consequently, the subsequent set of Spot Instances could be placed in a different zone from the original request, even if you specified the same Availability Zone group.</p>
    /// <p>Default: Instances are launched in any available Availability Zone.</p>
    pub fn availability_zone_group(&self) -> ::std::option::Option<&str> {
        self.availability_zone_group.as_deref()
    }
    /// <p>Deprecated.</p>
    pub fn block_duration_minutes(&self) -> ::std::option::Option<i32> {
        self.block_duration_minutes
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The maximum number of Spot Instances to launch.</p>
    /// <p>Default: 1</p>
    pub fn instance_count(&self) -> ::std::option::Option<i32> {
        self.instance_count
    }
    /// <p>The instance launch group. Launch groups are Spot Instances that launch together and terminate together.</p>
    /// <p>Default: Instances are launched and terminated individually</p>
    pub fn launch_group(&self) -> ::std::option::Option<&str> {
        self.launch_group.as_deref()
    }
    /// <p>The launch specification.</p>
    pub fn launch_specification(&self) -> ::std::option::Option<&crate::types::RequestSpotLaunchSpecification> {
        self.launch_specification.as_ref()
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn spot_price(&self) -> ::std::option::Option<&str> {
        self.spot_price.as_deref()
    }
    /// <p>The Spot Instance request type.</p>
    /// <p>Default: <code>one-time</code> </p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::SpotInstanceType> {
        self.r#type.as_ref()
    }
    /// <p>The start date of the request. If this is a one-time request, the request becomes active at this date and time and remains active until all instances launch, the request expires, or the request is canceled. If the request is persistent, the request becomes active at this date and time and remains active until it expires or is canceled.</p>
    /// <p>The specified start date and time cannot be equal to the current date and time. You must specify a start date and time that occurs after the current date and time.</p>
    pub fn valid_from(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.valid_from.as_ref()
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it. </p> </li>
    /// <li> <p>For a one-time request, the request remains active until all instances launch, the request is canceled, or the <code>ValidUntil</code> date and time is reached. By default, the request is valid for 7 days from the date the request was created.</p> </li>
    /// </ul>
    pub fn valid_until(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.valid_until.as_ref()
    }
    /// <p>The key-value pair for tagging the Spot Instance request on creation. The value for <code>ResourceType</code> must be <code>spot-instances-request</code>, otherwise the Spot Instance request fails. To tag the Spot Instance request after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn instance_interruption_behavior(&self) -> ::std::option::Option<&crate::types::InstanceInterruptionBehavior> {
        self.instance_interruption_behavior.as_ref()
    }
}
impl RequestSpotInstancesInput {
    /// Creates a new builder-style object to manufacture [`RequestSpotInstancesInput`](crate::operation::request_spot_instances::RequestSpotInstancesInput).
    pub fn builder() -> crate::operation::request_spot_instances::builders::RequestSpotInstancesInputBuilder {
        crate::operation::request_spot_instances::builders::RequestSpotInstancesInputBuilder::default()
    }
}

/// A builder for [`RequestSpotInstancesInput`](crate::operation::request_spot_instances::RequestSpotInstancesInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RequestSpotInstancesInputBuilder {
    pub(crate) availability_zone_group: ::std::option::Option<::std::string::String>,
    pub(crate) block_duration_minutes: ::std::option::Option<i32>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) instance_count: ::std::option::Option<i32>,
    pub(crate) launch_group: ::std::option::Option<::std::string::String>,
    pub(crate) launch_specification: ::std::option::Option<crate::types::RequestSpotLaunchSpecification>,
    pub(crate) spot_price: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::SpotInstanceType>,
    pub(crate) valid_from: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) valid_until: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) instance_interruption_behavior: ::std::option::Option<crate::types::InstanceInterruptionBehavior>,
}
impl RequestSpotInstancesInputBuilder {
    /// <p>The user-specified name for a logical grouping of requests.</p>
    /// <p>When you specify an Availability Zone group in a Spot Instance request, all Spot Instances in the request are launched in the same Availability Zone. Instance proximity is maintained with this parameter, but the choice of Availability Zone is not. The group applies only to requests for Spot Instances of the same instance type. Any additional Spot Instance requests that are specified with the same Availability Zone group name are launched in that same Availability Zone, as long as at least one instance from the group is still active.</p>
    /// <p>If there is no active instance running in the Availability Zone group that you specify for a new Spot Instance request (all instances are terminated, the request is expired, or the maximum price you specified falls below current Spot price), then Amazon EC2 launches the instance in any Availability Zone where the constraint can be met. Consequently, the subsequent set of Spot Instances could be placed in a different zone from the original request, even if you specified the same Availability Zone group.</p>
    /// <p>Default: Instances are launched in any available Availability Zone.</p>
    pub fn availability_zone_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.availability_zone_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The user-specified name for a logical grouping of requests.</p>
    /// <p>When you specify an Availability Zone group in a Spot Instance request, all Spot Instances in the request are launched in the same Availability Zone. Instance proximity is maintained with this parameter, but the choice of Availability Zone is not. The group applies only to requests for Spot Instances of the same instance type. Any additional Spot Instance requests that are specified with the same Availability Zone group name are launched in that same Availability Zone, as long as at least one instance from the group is still active.</p>
    /// <p>If there is no active instance running in the Availability Zone group that you specify for a new Spot Instance request (all instances are terminated, the request is expired, or the maximum price you specified falls below current Spot price), then Amazon EC2 launches the instance in any Availability Zone where the constraint can be met. Consequently, the subsequent set of Spot Instances could be placed in a different zone from the original request, even if you specified the same Availability Zone group.</p>
    /// <p>Default: Instances are launched in any available Availability Zone.</p>
    pub fn set_availability_zone_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.availability_zone_group = input;
        self
    }
    /// <p>The user-specified name for a logical grouping of requests.</p>
    /// <p>When you specify an Availability Zone group in a Spot Instance request, all Spot Instances in the request are launched in the same Availability Zone. Instance proximity is maintained with this parameter, but the choice of Availability Zone is not. The group applies only to requests for Spot Instances of the same instance type. Any additional Spot Instance requests that are specified with the same Availability Zone group name are launched in that same Availability Zone, as long as at least one instance from the group is still active.</p>
    /// <p>If there is no active instance running in the Availability Zone group that you specify for a new Spot Instance request (all instances are terminated, the request is expired, or the maximum price you specified falls below current Spot price), then Amazon EC2 launches the instance in any Availability Zone where the constraint can be met. Consequently, the subsequent set of Spot Instances could be placed in a different zone from the original request, even if you specified the same Availability Zone group.</p>
    /// <p>Default: Instances are launched in any available Availability Zone.</p>
    pub fn get_availability_zone_group(&self) -> &::std::option::Option<::std::string::String> {
        &self.availability_zone_group
    }
    /// <p>Deprecated.</p>
    pub fn block_duration_minutes(mut self, input: i32) -> Self {
        self.block_duration_minutes = ::std::option::Option::Some(input);
        self
    }
    /// <p>Deprecated.</p>
    pub fn set_block_duration_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.block_duration_minutes = input;
        self
    }
    /// <p>Deprecated.</p>
    pub fn get_block_duration_minutes(&self) -> &::std::option::Option<i32> {
        &self.block_duration_minutes
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
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
    /// <p>The maximum number of Spot Instances to launch.</p>
    /// <p>Default: 1</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.instance_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of Spot Instances to launch.</p>
    /// <p>Default: 1</p>
    pub fn set_instance_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.instance_count = input;
        self
    }
    /// <p>The maximum number of Spot Instances to launch.</p>
    /// <p>Default: 1</p>
    pub fn get_instance_count(&self) -> &::std::option::Option<i32> {
        &self.instance_count
    }
    /// <p>The instance launch group. Launch groups are Spot Instances that launch together and terminate together.</p>
    /// <p>Default: Instances are launched and terminated individually</p>
    pub fn launch_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.launch_group = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The instance launch group. Launch groups are Spot Instances that launch together and terminate together.</p>
    /// <p>Default: Instances are launched and terminated individually</p>
    pub fn set_launch_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.launch_group = input;
        self
    }
    /// <p>The instance launch group. Launch groups are Spot Instances that launch together and terminate together.</p>
    /// <p>Default: Instances are launched and terminated individually</p>
    pub fn get_launch_group(&self) -> &::std::option::Option<::std::string::String> {
        &self.launch_group
    }
    /// <p>The launch specification.</p>
    pub fn launch_specification(mut self, input: crate::types::RequestSpotLaunchSpecification) -> Self {
        self.launch_specification = ::std::option::Option::Some(input);
        self
    }
    /// <p>The launch specification.</p>
    pub fn set_launch_specification(mut self, input: ::std::option::Option<crate::types::RequestSpotLaunchSpecification>) -> Self {
        self.launch_specification = input;
        self
    }
    /// <p>The launch specification.</p>
    pub fn get_launch_specification(&self) -> &::std::option::Option<crate::types::RequestSpotLaunchSpecification> {
        &self.launch_specification
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn spot_price(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.spot_price = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn set_spot_price(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.spot_price = input;
        self
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn get_spot_price(&self) -> &::std::option::Option<::std::string::String> {
        &self.spot_price
    }
    /// <p>The Spot Instance request type.</p>
    /// <p>Default: <code>one-time</code> </p>
    pub fn r#type(mut self, input: crate::types::SpotInstanceType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Spot Instance request type.</p>
    /// <p>Default: <code>one-time</code> </p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::SpotInstanceType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The Spot Instance request type.</p>
    /// <p>Default: <code>one-time</code> </p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::SpotInstanceType> {
        &self.r#type
    }
    /// <p>The start date of the request. If this is a one-time request, the request becomes active at this date and time and remains active until all instances launch, the request expires, or the request is canceled. If the request is persistent, the request becomes active at this date and time and remains active until it expires or is canceled.</p>
    /// <p>The specified start date and time cannot be equal to the current date and time. You must specify a start date and time that occurs after the current date and time.</p>
    pub fn valid_from(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.valid_from = ::std::option::Option::Some(input);
        self
    }
    /// <p>The start date of the request. If this is a one-time request, the request becomes active at this date and time and remains active until all instances launch, the request expires, or the request is canceled. If the request is persistent, the request becomes active at this date and time and remains active until it expires or is canceled.</p>
    /// <p>The specified start date and time cannot be equal to the current date and time. You must specify a start date and time that occurs after the current date and time.</p>
    pub fn set_valid_from(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.valid_from = input;
        self
    }
    /// <p>The start date of the request. If this is a one-time request, the request becomes active at this date and time and remains active until all instances launch, the request expires, or the request is canceled. If the request is persistent, the request becomes active at this date and time and remains active until it expires or is canceled.</p>
    /// <p>The specified start date and time cannot be equal to the current date and time. You must specify a start date and time that occurs after the current date and time.</p>
    pub fn get_valid_from(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.valid_from
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it. </p> </li>
    /// <li> <p>For a one-time request, the request remains active until all instances launch, the request is canceled, or the <code>ValidUntil</code> date and time is reached. By default, the request is valid for 7 days from the date the request was created.</p> </li>
    /// </ul>
    pub fn valid_until(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.valid_until = ::std::option::Option::Some(input);
        self
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it. </p> </li>
    /// <li> <p>For a one-time request, the request remains active until all instances launch, the request is canceled, or the <code>ValidUntil</code> date and time is reached. By default, the request is valid for 7 days from the date the request was created.</p> </li>
    /// </ul>
    pub fn set_valid_until(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.valid_until = input;
        self
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it. </p> </li>
    /// <li> <p>For a one-time request, the request remains active until all instances launch, the request is canceled, or the <code>ValidUntil</code> date and time is reached. By default, the request is valid for 7 days from the date the request was created.</p> </li>
    /// </ul>
    pub fn get_valid_until(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.valid_until
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The key-value pair for tagging the Spot Instance request on creation. The value for <code>ResourceType</code> must be <code>spot-instances-request</code>, otherwise the Spot Instance request fails. To tag the Spot Instance request after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The key-value pair for tagging the Spot Instance request on creation. The value for <code>ResourceType</code> must be <code>spot-instances-request</code>, otherwise the Spot Instance request fails. To tag the Spot Instance request after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The key-value pair for tagging the Spot Instance request on creation. The value for <code>ResourceType</code> must be <code>spot-instances-request</code>, otherwise the Spot Instance request fails. To tag the Spot Instance request after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn instance_interruption_behavior(mut self, input: crate::types::InstanceInterruptionBehavior) -> Self {
        self.instance_interruption_behavior = ::std::option::Option::Some(input);
        self
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn set_instance_interruption_behavior(mut self, input: ::std::option::Option<crate::types::InstanceInterruptionBehavior>) -> Self {
        self.instance_interruption_behavior = input;
        self
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn get_instance_interruption_behavior(&self) -> &::std::option::Option<crate::types::InstanceInterruptionBehavior> {
        &self.instance_interruption_behavior
    }
    /// Consumes the builder and constructs a [`RequestSpotInstancesInput`](crate::operation::request_spot_instances::RequestSpotInstancesInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::request_spot_instances::RequestSpotInstancesInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::request_spot_instances::RequestSpotInstancesInput {
            availability_zone_group: self.availability_zone_group,
            block_duration_minutes: self.block_duration_minutes,
            client_token: self.client_token,
            dry_run: self.dry_run,
            instance_count: self.instance_count,
            launch_group: self.launch_group,
            launch_specification: self.launch_specification,
            spot_price: self.spot_price,
            r#type: self.r#type,
            valid_from: self.valid_from,
            valid_until: self.valid_until,
            tag_specifications: self.tag_specifications,
            instance_interruption_behavior: self.instance_interruption_behavior,
        })
    }
}
