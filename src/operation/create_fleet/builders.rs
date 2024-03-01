// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_fleet::_create_fleet_output::CreateFleetOutputBuilder;

pub use crate::operation::create_fleet::_create_fleet_input::CreateFleetInputBuilder;

impl CreateFleetInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_fleet::CreateFleetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_fleet::CreateFleetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_fleet();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateFleet`.
///
/// <p>Creates an EC2 Fleet that contains the configuration information for On-Demand Instances and Spot Instances. Instances are launched immediately if there is available capacity.</p>
/// <p>A single EC2 Fleet can include multiple launch specifications that vary by instance type, AMI, Availability Zone, or subnet.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet.html">EC2 Fleet</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateFleetFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_fleet::builders::CreateFleetInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_fleet::CreateFleetOutput,
        crate::operation::create_fleet::CreateFleetError,
    > for CreateFleetFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_fleet::CreateFleetOutput,
            crate::operation::create_fleet::CreateFleetError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateFleetFluentBuilder {
    /// Creates a new `CreateFleet`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateFleet as a reference.
    pub fn as_input(&self) -> &crate::operation::create_fleet::builders::CreateFleetInputBuilder {
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
        crate::operation::create_fleet::CreateFleetOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_fleet::CreateFleetError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_fleet::CreateFleet::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_fleet::CreateFleet::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_fleet::CreateFleetOutput,
        crate::operation::create_fleet::CreateFleetError,
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>Describes the configuration of Spot Instances in an EC2 Fleet.</p>
    pub fn spot_options(mut self, input: crate::types::SpotOptionsRequest) -> Self {
        self.inner = self.inner.spot_options(input);
        self
    }
    /// <p>Describes the configuration of Spot Instances in an EC2 Fleet.</p>
    pub fn set_spot_options(mut self, input: ::std::option::Option<crate::types::SpotOptionsRequest>) -> Self {
        self.inner = self.inner.set_spot_options(input);
        self
    }
    /// <p>Describes the configuration of Spot Instances in an EC2 Fleet.</p>
    pub fn get_spot_options(&self) -> &::std::option::Option<crate::types::SpotOptionsRequest> {
        self.inner.get_spot_options()
    }
    /// <p>Describes the configuration of On-Demand Instances in an EC2 Fleet.</p>
    pub fn on_demand_options(mut self, input: crate::types::OnDemandOptionsRequest) -> Self {
        self.inner = self.inner.on_demand_options(input);
        self
    }
    /// <p>Describes the configuration of On-Demand Instances in an EC2 Fleet.</p>
    pub fn set_on_demand_options(mut self, input: ::std::option::Option<crate::types::OnDemandOptionsRequest>) -> Self {
        self.inner = self.inner.set_on_demand_options(input);
        self
    }
    /// <p>Describes the configuration of On-Demand Instances in an EC2 Fleet.</p>
    pub fn get_on_demand_options(&self) -> &::std::option::Option<crate::types::OnDemandOptionsRequest> {
        self.inner.get_on_demand_options()
    }
    /// <p>Indicates whether running instances should be terminated if the total target capacity of the EC2 Fleet is decreased below the current size of the EC2 Fleet.</p>
    /// <p>Supported only for fleets of type <code>maintain</code>.</p>
    pub fn excess_capacity_termination_policy(mut self, input: crate::types::FleetExcessCapacityTerminationPolicy) -> Self {
        self.inner = self.inner.excess_capacity_termination_policy(input);
        self
    }
    /// <p>Indicates whether running instances should be terminated if the total target capacity of the EC2 Fleet is decreased below the current size of the EC2 Fleet.</p>
    /// <p>Supported only for fleets of type <code>maintain</code>.</p>
    pub fn set_excess_capacity_termination_policy(
        mut self,
        input: ::std::option::Option<crate::types::FleetExcessCapacityTerminationPolicy>,
    ) -> Self {
        self.inner = self.inner.set_excess_capacity_termination_policy(input);
        self
    }
    /// <p>Indicates whether running instances should be terminated if the total target capacity of the EC2 Fleet is decreased below the current size of the EC2 Fleet.</p>
    /// <p>Supported only for fleets of type <code>maintain</code>.</p>
    pub fn get_excess_capacity_termination_policy(&self) -> &::std::option::Option<crate::types::FleetExcessCapacityTerminationPolicy> {
        self.inner.get_excess_capacity_termination_policy()
    }
    /// Appends an item to `LaunchTemplateConfigs`.
    ///
    /// To override the contents of this collection use [`set_launch_template_configs`](Self::set_launch_template_configs).
    ///
    /// <p>The configuration for the EC2 Fleet.</p>
    pub fn launch_template_configs(mut self, input: crate::types::FleetLaunchTemplateConfigRequest) -> Self {
        self.inner = self.inner.launch_template_configs(input);
        self
    }
    /// <p>The configuration for the EC2 Fleet.</p>
    pub fn set_launch_template_configs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::FleetLaunchTemplateConfigRequest>>,
    ) -> Self {
        self.inner = self.inner.set_launch_template_configs(input);
        self
    }
    /// <p>The configuration for the EC2 Fleet.</p>
    pub fn get_launch_template_configs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FleetLaunchTemplateConfigRequest>> {
        self.inner.get_launch_template_configs()
    }
    /// <p>The number of units to request.</p>
    pub fn target_capacity_specification(mut self, input: crate::types::TargetCapacitySpecificationRequest) -> Self {
        self.inner = self.inner.target_capacity_specification(input);
        self
    }
    /// <p>The number of units to request.</p>
    pub fn set_target_capacity_specification(mut self, input: ::std::option::Option<crate::types::TargetCapacitySpecificationRequest>) -> Self {
        self.inner = self.inner.set_target_capacity_specification(input);
        self
    }
    /// <p>The number of units to request.</p>
    pub fn get_target_capacity_specification(&self) -> &::std::option::Option<crate::types::TargetCapacitySpecificationRequest> {
        self.inner.get_target_capacity_specification()
    }
    /// <p>Indicates whether running instances should be terminated when the EC2 Fleet expires.</p>
    pub fn terminate_instances_with_expiration(mut self, input: bool) -> Self {
        self.inner = self.inner.terminate_instances_with_expiration(input);
        self
    }
    /// <p>Indicates whether running instances should be terminated when the EC2 Fleet expires.</p>
    pub fn set_terminate_instances_with_expiration(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_terminate_instances_with_expiration(input);
        self
    }
    /// <p>Indicates whether running instances should be terminated when the EC2 Fleet expires.</p>
    pub fn get_terminate_instances_with_expiration(&self) -> &::std::option::Option<bool> {
        self.inner.get_terminate_instances_with_expiration()
    }
    /// <p>The fleet type. The default value is <code>maintain</code>.</p>
    /// <ul>
    /// <li> <p> <code>maintain</code> - The EC2 Fleet places an asynchronous request for your desired capacity, and continues to maintain your desired Spot capacity by replenishing interrupted Spot Instances.</p> </li>
    /// <li> <p> <code>request</code> - The EC2 Fleet places an asynchronous one-time request for your desired capacity, but does submit Spot requests in alternative capacity pools if Spot capacity is unavailable, and does not maintain Spot capacity if Spot Instances are interrupted.</p> </li>
    /// <li> <p> <code>instant</code> - The EC2 Fleet places a synchronous one-time request for your desired capacity, and returns errors for any instances that could not be launched.</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-request-type.html">EC2 Fleet request types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn r#type(mut self, input: crate::types::FleetType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The fleet type. The default value is <code>maintain</code>.</p>
    /// <ul>
    /// <li> <p> <code>maintain</code> - The EC2 Fleet places an asynchronous request for your desired capacity, and continues to maintain your desired Spot capacity by replenishing interrupted Spot Instances.</p> </li>
    /// <li> <p> <code>request</code> - The EC2 Fleet places an asynchronous one-time request for your desired capacity, but does submit Spot requests in alternative capacity pools if Spot capacity is unavailable, and does not maintain Spot capacity if Spot Instances are interrupted.</p> </li>
    /// <li> <p> <code>instant</code> - The EC2 Fleet places a synchronous one-time request for your desired capacity, and returns errors for any instances that could not be launched.</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-request-type.html">EC2 Fleet request types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::FleetType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The fleet type. The default value is <code>maintain</code>.</p>
    /// <ul>
    /// <li> <p> <code>maintain</code> - The EC2 Fleet places an asynchronous request for your desired capacity, and continues to maintain your desired Spot capacity by replenishing interrupted Spot Instances.</p> </li>
    /// <li> <p> <code>request</code> - The EC2 Fleet places an asynchronous one-time request for your desired capacity, but does submit Spot requests in alternative capacity pools if Spot capacity is unavailable, and does not maintain Spot capacity if Spot Instances are interrupted.</p> </li>
    /// <li> <p> <code>instant</code> - The EC2 Fleet places a synchronous one-time request for your desired capacity, and returns errors for any instances that could not be launched.</p> </li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-request-type.html">EC2 Fleet request types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::FleetType> {
        self.inner.get_type()
    }
    /// <p>The start date and time of the request, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). The default is to start fulfilling the request immediately.</p>
    pub fn valid_from(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.valid_from(input);
        self
    }
    /// <p>The start date and time of the request, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). The default is to start fulfilling the request immediately.</p>
    pub fn set_valid_from(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_valid_from(input);
        self
    }
    /// <p>The start date and time of the request, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). The default is to start fulfilling the request immediately.</p>
    pub fn get_valid_from(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_valid_from()
    }
    /// <p>The end date and time of the request, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). At this point, no new EC2 Fleet requests are placed or able to fulfill the request. If no value is specified, the request remains until you cancel it.</p>
    pub fn valid_until(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.valid_until(input);
        self
    }
    /// <p>The end date and time of the request, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). At this point, no new EC2 Fleet requests are placed or able to fulfill the request. If no value is specified, the request remains until you cancel it.</p>
    pub fn set_valid_until(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_valid_until(input);
        self
    }
    /// <p>The end date and time of the request, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z). At this point, no new EC2 Fleet requests are placed or able to fulfill the request. If no value is specified, the request remains until you cancel it.</p>
    pub fn get_valid_until(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_valid_until()
    }
    /// <p>Indicates whether EC2 Fleet should replace unhealthy Spot Instances. Supported only for fleets of type <code>maintain</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/manage-ec2-fleet.html#ec2-fleet-health-checks">EC2 Fleet health checks</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn replace_unhealthy_instances(mut self, input: bool) -> Self {
        self.inner = self.inner.replace_unhealthy_instances(input);
        self
    }
    /// <p>Indicates whether EC2 Fleet should replace unhealthy Spot Instances. Supported only for fleets of type <code>maintain</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/manage-ec2-fleet.html#ec2-fleet-health-checks">EC2 Fleet health checks</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_replace_unhealthy_instances(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_replace_unhealthy_instances(input);
        self
    }
    /// <p>Indicates whether EC2 Fleet should replace unhealthy Spot Instances. Supported only for fleets of type <code>maintain</code>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/manage-ec2-fleet.html#ec2-fleet-health-checks">EC2 Fleet health checks</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn get_replace_unhealthy_instances(&self) -> &::std::option::Option<bool> {
        self.inner.get_replace_unhealthy_instances()
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The key-value pair for tagging the EC2 Fleet request on creation. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-resources">Tag your resources</a>.</p>
    /// <p>If the fleet type is <code>instant</code>, specify a resource type of <code>fleet</code> to tag the fleet or <code>instance</code> to tag the instances at launch.</p>
    /// <p>If the fleet type is <code>maintain</code> or <code>request</code>, specify a resource type of <code>fleet</code> to tag the fleet. You cannot specify a resource type of <code>instance</code>. To tag instances at launch, specify the tags in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html#create-launch-template">launch template</a>.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The key-value pair for tagging the EC2 Fleet request on creation. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-resources">Tag your resources</a>.</p>
    /// <p>If the fleet type is <code>instant</code>, specify a resource type of <code>fleet</code> to tag the fleet or <code>instance</code> to tag the instances at launch.</p>
    /// <p>If the fleet type is <code>maintain</code> or <code>request</code>, specify a resource type of <code>fleet</code> to tag the fleet. You cannot specify a resource type of <code>instance</code>. To tag instances at launch, specify the tags in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html#create-launch-template">launch template</a>.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The key-value pair for tagging the EC2 Fleet request on creation. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html#tag-resources">Tag your resources</a>.</p>
    /// <p>If the fleet type is <code>instant</code>, specify a resource type of <code>fleet</code> to tag the fleet or <code>instance</code> to tag the instances at launch.</p>
    /// <p>If the fleet type is <code>maintain</code> or <code>request</code>, specify a resource type of <code>fleet</code> to tag the fleet. You cannot specify a resource type of <code>instance</code>. To tag instances at launch, specify the tags in a <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html#create-launch-template">launch template</a>.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
    }
    /// <p>Reserved.</p>
    pub fn context(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.context(input.into());
        self
    }
    /// <p>Reserved.</p>
    pub fn set_context(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_context(input);
        self
    }
    /// <p>Reserved.</p>
    pub fn get_context(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_context()
    }
}
