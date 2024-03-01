// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Spot Instance replacement strategy to use when Amazon EC2 emits a rebalance notification signal that your Spot Instance is at an elevated risk of being interrupted. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-fleet-capacity-rebalance.html">Capacity rebalancing</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FleetSpotCapacityRebalanceRequest {
    /// <p>The replacement strategy to use. Only available for fleets of type <code>maintain</code>.</p>
    /// <p> <code>launch</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet. EC2 Fleet does not terminate the instances that receive a rebalance notification. You can terminate the old instances, or you can leave them running. You are charged for all instances while they are running. </p>
    /// <p> <code>launch-before-terminate</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet, and then, after a delay that you specify (in <code>TerminationDelay</code>), terminates the instances that received a rebalance notification.</p>
    pub replacement_strategy: ::std::option::Option<crate::types::FleetReplacementStrategy>,
    /// <p>The amount of time (in seconds) that Amazon EC2 waits before terminating the old Spot Instance after launching a new replacement Spot Instance.</p>
    /// <p>Required when <code>ReplacementStrategy</code> is set to <code>launch-before-terminate</code>.</p>
    /// <p>Not valid when <code>ReplacementStrategy</code> is set to <code>launch</code>.</p>
    /// <p>Valid values: Minimum value of <code>120</code> seconds. Maximum value of <code>7200</code> seconds.</p>
    pub termination_delay: ::std::option::Option<i32>,
}
impl FleetSpotCapacityRebalanceRequest {
    /// <p>The replacement strategy to use. Only available for fleets of type <code>maintain</code>.</p>
    /// <p> <code>launch</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet. EC2 Fleet does not terminate the instances that receive a rebalance notification. You can terminate the old instances, or you can leave them running. You are charged for all instances while they are running. </p>
    /// <p> <code>launch-before-terminate</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet, and then, after a delay that you specify (in <code>TerminationDelay</code>), terminates the instances that received a rebalance notification.</p>
    pub fn replacement_strategy(&self) -> ::std::option::Option<&crate::types::FleetReplacementStrategy> {
        self.replacement_strategy.as_ref()
    }
    /// <p>The amount of time (in seconds) that Amazon EC2 waits before terminating the old Spot Instance after launching a new replacement Spot Instance.</p>
    /// <p>Required when <code>ReplacementStrategy</code> is set to <code>launch-before-terminate</code>.</p>
    /// <p>Not valid when <code>ReplacementStrategy</code> is set to <code>launch</code>.</p>
    /// <p>Valid values: Minimum value of <code>120</code> seconds. Maximum value of <code>7200</code> seconds.</p>
    pub fn termination_delay(&self) -> ::std::option::Option<i32> {
        self.termination_delay
    }
}
impl FleetSpotCapacityRebalanceRequest {
    /// Creates a new builder-style object to manufacture [`FleetSpotCapacityRebalanceRequest`](crate::types::FleetSpotCapacityRebalanceRequest).
    pub fn builder() -> crate::types::builders::FleetSpotCapacityRebalanceRequestBuilder {
        crate::types::builders::FleetSpotCapacityRebalanceRequestBuilder::default()
    }
}

/// A builder for [`FleetSpotCapacityRebalanceRequest`](crate::types::FleetSpotCapacityRebalanceRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct FleetSpotCapacityRebalanceRequestBuilder {
    pub(crate) replacement_strategy: ::std::option::Option<crate::types::FleetReplacementStrategy>,
    pub(crate) termination_delay: ::std::option::Option<i32>,
}
impl FleetSpotCapacityRebalanceRequestBuilder {
    /// <p>The replacement strategy to use. Only available for fleets of type <code>maintain</code>.</p>
    /// <p> <code>launch</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet. EC2 Fleet does not terminate the instances that receive a rebalance notification. You can terminate the old instances, or you can leave them running. You are charged for all instances while they are running. </p>
    /// <p> <code>launch-before-terminate</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet, and then, after a delay that you specify (in <code>TerminationDelay</code>), terminates the instances that received a rebalance notification.</p>
    pub fn replacement_strategy(mut self, input: crate::types::FleetReplacementStrategy) -> Self {
        self.replacement_strategy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The replacement strategy to use. Only available for fleets of type <code>maintain</code>.</p>
    /// <p> <code>launch</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet. EC2 Fleet does not terminate the instances that receive a rebalance notification. You can terminate the old instances, or you can leave them running. You are charged for all instances while they are running. </p>
    /// <p> <code>launch-before-terminate</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet, and then, after a delay that you specify (in <code>TerminationDelay</code>), terminates the instances that received a rebalance notification.</p>
    pub fn set_replacement_strategy(mut self, input: ::std::option::Option<crate::types::FleetReplacementStrategy>) -> Self {
        self.replacement_strategy = input;
        self
    }
    /// <p>The replacement strategy to use. Only available for fleets of type <code>maintain</code>.</p>
    /// <p> <code>launch</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet. EC2 Fleet does not terminate the instances that receive a rebalance notification. You can terminate the old instances, or you can leave them running. You are charged for all instances while they are running. </p>
    /// <p> <code>launch-before-terminate</code> - EC2 Fleet launches a replacement Spot Instance when a rebalance notification is emitted for an existing Spot Instance in the fleet, and then, after a delay that you specify (in <code>TerminationDelay</code>), terminates the instances that received a rebalance notification.</p>
    pub fn get_replacement_strategy(&self) -> &::std::option::Option<crate::types::FleetReplacementStrategy> {
        &self.replacement_strategy
    }
    /// <p>The amount of time (in seconds) that Amazon EC2 waits before terminating the old Spot Instance after launching a new replacement Spot Instance.</p>
    /// <p>Required when <code>ReplacementStrategy</code> is set to <code>launch-before-terminate</code>.</p>
    /// <p>Not valid when <code>ReplacementStrategy</code> is set to <code>launch</code>.</p>
    /// <p>Valid values: Minimum value of <code>120</code> seconds. Maximum value of <code>7200</code> seconds.</p>
    pub fn termination_delay(mut self, input: i32) -> Self {
        self.termination_delay = ::std::option::Option::Some(input);
        self
    }
    /// <p>The amount of time (in seconds) that Amazon EC2 waits before terminating the old Spot Instance after launching a new replacement Spot Instance.</p>
    /// <p>Required when <code>ReplacementStrategy</code> is set to <code>launch-before-terminate</code>.</p>
    /// <p>Not valid when <code>ReplacementStrategy</code> is set to <code>launch</code>.</p>
    /// <p>Valid values: Minimum value of <code>120</code> seconds. Maximum value of <code>7200</code> seconds.</p>
    pub fn set_termination_delay(mut self, input: ::std::option::Option<i32>) -> Self {
        self.termination_delay = input;
        self
    }
    /// <p>The amount of time (in seconds) that Amazon EC2 waits before terminating the old Spot Instance after launching a new replacement Spot Instance.</p>
    /// <p>Required when <code>ReplacementStrategy</code> is set to <code>launch-before-terminate</code>.</p>
    /// <p>Not valid when <code>ReplacementStrategy</code> is set to <code>launch</code>.</p>
    /// <p>Valid values: Minimum value of <code>120</code> seconds. Maximum value of <code>7200</code> seconds.</p>
    pub fn get_termination_delay(&self) -> &::std::option::Option<i32> {
        &self.termination_delay
    }
    /// Consumes the builder and constructs a [`FleetSpotCapacityRebalanceRequest`](crate::types::FleetSpotCapacityRebalanceRequest).
    pub fn build(self) -> crate::types::FleetSpotCapacityRebalanceRequest {
        crate::types::FleetSpotCapacityRebalanceRequest {
            replacement_strategy: self.replacement_strategy,
            termination_delay: self.termination_delay,
        }
    }
}
