// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::request_spot_instances::_request_spot_instances_output::RequestSpotInstancesOutputBuilder;

pub use crate::operation::request_spot_instances::_request_spot_instances_input::RequestSpotInstancesInputBuilder;

impl RequestSpotInstancesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::request_spot_instances::RequestSpotInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::request_spot_instances::RequestSpotInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.request_spot_instances();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `RequestSpotInstances`.
///
/// <p>Creates a Spot Instance request.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-requests.html">Spot Instance requests</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p> <important>
/// <p>We strongly discourage using the RequestSpotInstances API because it is a legacy API with no planned investment. For options for requesting Spot Instances, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/spot-best-practices.html#which-spot-request-method-to-use">Which is the best Spot request method to use?</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
/// </important>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct RequestSpotInstancesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::request_spot_instances::builders::RequestSpotInstancesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::request_spot_instances::RequestSpotInstancesOutput,
        crate::operation::request_spot_instances::RequestSpotInstancesError,
    > for RequestSpotInstancesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::request_spot_instances::RequestSpotInstancesOutput,
            crate::operation::request_spot_instances::RequestSpotInstancesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl RequestSpotInstancesFluentBuilder {
    /// Creates a new `RequestSpotInstances`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the RequestSpotInstances as a reference.
    pub fn as_input(&self) -> &crate::operation::request_spot_instances::builders::RequestSpotInstancesInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::request_spot_instances::RequestSpotInstancesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::request_spot_instances::RequestSpotInstancesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::request_spot_instances::RequestSpotInstances::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::request_spot_instances::RequestSpotInstances::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::request_spot_instances::RequestSpotInstancesOutput,
        crate::operation::request_spot_instances::RequestSpotInstancesError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The user-specified name for a logical grouping of requests.</p>
    /// <p>When you specify an Availability Zone group in a Spot Instance request, all Spot Instances in the request are launched in the same Availability Zone. Instance proximity is maintained with this parameter, but the choice of Availability Zone is not. The group applies only to requests for Spot Instances of the same instance type. Any additional Spot Instance requests that are specified with the same Availability Zone group name are launched in that same Availability Zone, as long as at least one instance from the group is still active.</p>
    /// <p>If there is no active instance running in the Availability Zone group that you specify for a new Spot Instance request (all instances are terminated, the request is expired, or the maximum price you specified falls below current Spot price), then Amazon EC2 launches the instance in any Availability Zone where the constraint can be met. Consequently, the subsequent set of Spot Instances could be placed in a different zone from the original request, even if you specified the same Availability Zone group.</p>
    /// <p>Default: Instances are launched in any available Availability Zone.</p>
    pub fn availability_zone_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone_group(input.into());
        self
    }
    /// <p>The user-specified name for a logical grouping of requests.</p>
    /// <p>When you specify an Availability Zone group in a Spot Instance request, all Spot Instances in the request are launched in the same Availability Zone. Instance proximity is maintained with this parameter, but the choice of Availability Zone is not. The group applies only to requests for Spot Instances of the same instance type. Any additional Spot Instance requests that are specified with the same Availability Zone group name are launched in that same Availability Zone, as long as at least one instance from the group is still active.</p>
    /// <p>If there is no active instance running in the Availability Zone group that you specify for a new Spot Instance request (all instances are terminated, the request is expired, or the maximum price you specified falls below current Spot price), then Amazon EC2 launches the instance in any Availability Zone where the constraint can be met. Consequently, the subsequent set of Spot Instances could be placed in a different zone from the original request, even if you specified the same Availability Zone group.</p>
    /// <p>Default: Instances are launched in any available Availability Zone.</p>
    pub fn set_availability_zone_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone_group(input);
        self
    }
    /// <p>The user-specified name for a logical grouping of requests.</p>
    /// <p>When you specify an Availability Zone group in a Spot Instance request, all Spot Instances in the request are launched in the same Availability Zone. Instance proximity is maintained with this parameter, but the choice of Availability Zone is not. The group applies only to requests for Spot Instances of the same instance type. Any additional Spot Instance requests that are specified with the same Availability Zone group name are launched in that same Availability Zone, as long as at least one instance from the group is still active.</p>
    /// <p>If there is no active instance running in the Availability Zone group that you specify for a new Spot Instance request (all instances are terminated, the request is expired, or the maximum price you specified falls below current Spot price), then Amazon EC2 launches the instance in any Availability Zone where the constraint can be met. Consequently, the subsequent set of Spot Instances could be placed in a different zone from the original request, even if you specified the same Availability Zone group.</p>
    /// <p>Default: Instances are launched in any available Availability Zone.</p>
    pub fn get_availability_zone_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone_group()
    }
    /// <p>Deprecated.</p>
    pub fn block_duration_minutes(mut self, input: i32) -> Self {
        self.inner = self.inner.block_duration_minutes(input);
        self
    }
    /// <p>Deprecated.</p>
    pub fn set_block_duration_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_block_duration_minutes(input);
        self
    }
    /// <p>Deprecated.</p>
    pub fn get_block_duration_minutes(&self) -> &::std::option::Option<i32> {
        self.inner.get_block_duration_minutes()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a> in the <i>Amazon EC2 User Guide for Linux Instances</i>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>The maximum number of Spot Instances to launch.</p>
    /// <p>Default: 1</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.inner = self.inner.instance_count(input);
        self
    }
    /// <p>The maximum number of Spot Instances to launch.</p>
    /// <p>Default: 1</p>
    pub fn set_instance_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_instance_count(input);
        self
    }
    /// <p>The maximum number of Spot Instances to launch.</p>
    /// <p>Default: 1</p>
    pub fn get_instance_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_instance_count()
    }
    /// <p>The instance launch group. Launch groups are Spot Instances that launch together and terminate together.</p>
    /// <p>Default: Instances are launched and terminated individually</p>
    pub fn launch_group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.launch_group(input.into());
        self
    }
    /// <p>The instance launch group. Launch groups are Spot Instances that launch together and terminate together.</p>
    /// <p>Default: Instances are launched and terminated individually</p>
    pub fn set_launch_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_launch_group(input);
        self
    }
    /// <p>The instance launch group. Launch groups are Spot Instances that launch together and terminate together.</p>
    /// <p>Default: Instances are launched and terminated individually</p>
    pub fn get_launch_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_launch_group()
    }
    /// <p>The launch specification.</p>
    pub fn launch_specification(mut self, input: crate::types::RequestSpotLaunchSpecification) -> Self {
        self.inner = self.inner.launch_specification(input);
        self
    }
    /// <p>The launch specification.</p>
    pub fn set_launch_specification(mut self, input: ::std::option::Option<crate::types::RequestSpotLaunchSpecification>) -> Self {
        self.inner = self.inner.set_launch_specification(input);
        self
    }
    /// <p>The launch specification.</p>
    pub fn get_launch_specification(&self) -> &::std::option::Option<crate::types::RequestSpotLaunchSpecification> {
        self.inner.get_launch_specification()
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn spot_price(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.spot_price(input.into());
        self
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn set_spot_price(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_spot_price(input);
        self
    }
    /// <p>The maximum price per unit hour that you are willing to pay for a Spot Instance. We do not recommend using this parameter because it can lead to increased interruptions. If you do not specify this parameter, you will pay the current Spot price.</p> <important>
    /// <p>If you specify a maximum price, your instances will be interrupted more frequently than if you do not specify this parameter.</p>
    /// </important>
    pub fn get_spot_price(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_spot_price()
    }
    /// <p>The Spot Instance request type.</p>
    /// <p>Default: <code>one-time</code> </p>
    pub fn r#type(mut self, input: crate::types::SpotInstanceType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The Spot Instance request type.</p>
    /// <p>Default: <code>one-time</code> </p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::SpotInstanceType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The Spot Instance request type.</p>
    /// <p>Default: <code>one-time</code> </p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::SpotInstanceType> {
        self.inner.get_type()
    }
    /// <p>The start date of the request. If this is a one-time request, the request becomes active at this date and time and remains active until all instances launch, the request expires, or the request is canceled. If the request is persistent, the request becomes active at this date and time and remains active until it expires or is canceled.</p>
    /// <p>The specified start date and time cannot be equal to the current date and time. You must specify a start date and time that occurs after the current date and time.</p>
    pub fn valid_from(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.valid_from(input);
        self
    }
    /// <p>The start date of the request. If this is a one-time request, the request becomes active at this date and time and remains active until all instances launch, the request expires, or the request is canceled. If the request is persistent, the request becomes active at this date and time and remains active until it expires or is canceled.</p>
    /// <p>The specified start date and time cannot be equal to the current date and time. You must specify a start date and time that occurs after the current date and time.</p>
    pub fn set_valid_from(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_valid_from(input);
        self
    }
    /// <p>The start date of the request. If this is a one-time request, the request becomes active at this date and time and remains active until all instances launch, the request expires, or the request is canceled. If the request is persistent, the request becomes active at this date and time and remains active until it expires or is canceled.</p>
    /// <p>The specified start date and time cannot be equal to the current date and time. You must specify a start date and time that occurs after the current date and time.</p>
    pub fn get_valid_from(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_valid_from()
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it. </p> </li>
    /// <li> <p>For a one-time request, the request remains active until all instances launch, the request is canceled, or the <code>ValidUntil</code> date and time is reached. By default, the request is valid for 7 days from the date the request was created.</p> </li>
    /// </ul>
    pub fn valid_until(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.valid_until(input);
        self
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it. </p> </li>
    /// <li> <p>For a one-time request, the request remains active until all instances launch, the request is canceled, or the <code>ValidUntil</code> date and time is reached. By default, the request is valid for 7 days from the date the request was created.</p> </li>
    /// </ul>
    pub fn set_valid_until(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_valid_until(input);
        self
    }
    /// <p>The end date of the request, in UTC format (<i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    /// <ul>
    /// <li> <p>For a persistent request, the request remains active until the <code>ValidUntil</code> date and time is reached. Otherwise, the request remains active until you cancel it. </p> </li>
    /// <li> <p>For a one-time request, the request remains active until all instances launch, the request is canceled, or the <code>ValidUntil</code> date and time is reached. By default, the request is valid for 7 days from the date the request was created.</p> </li>
    /// </ul>
    pub fn get_valid_until(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_valid_until()
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The key-value pair for tagging the Spot Instance request on creation. The value for <code>ResourceType</code> must be <code>spot-instances-request</code>, otherwise the Spot Instance request fails. To tag the Spot Instance request after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The key-value pair for tagging the Spot Instance request on creation. The value for <code>ResourceType</code> must be <code>spot-instances-request</code>, otherwise the Spot Instance request fails. To tag the Spot Instance request after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The key-value pair for tagging the Spot Instance request on creation. The value for <code>ResourceType</code> must be <code>spot-instances-request</code>, otherwise the Spot Instance request fails. To tag the Spot Instance request after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn instance_interruption_behavior(mut self, input: crate::types::InstanceInterruptionBehavior) -> Self {
        self.inner = self.inner.instance_interruption_behavior(input);
        self
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn set_instance_interruption_behavior(mut self, input: ::std::option::Option<crate::types::InstanceInterruptionBehavior>) -> Self {
        self.inner = self.inner.set_instance_interruption_behavior(input);
        self
    }
    /// <p>The behavior when a Spot Instance is interrupted. The default is <code>terminate</code>.</p>
    pub fn get_instance_interruption_behavior(&self) -> &::std::option::Option<crate::types::InstanceInterruptionBehavior> {
        self.inner.get_instance_interruption_behavior()
    }
}