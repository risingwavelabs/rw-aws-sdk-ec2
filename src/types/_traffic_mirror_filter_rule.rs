// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the Traffic Mirror rule.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TrafficMirrorFilterRule {
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub traffic_mirror_filter_rule_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Traffic Mirror filter that the rule is associated with.</p>
    pub traffic_mirror_filter_id: ::std::option::Option<::std::string::String>,
    /// <p>The traffic direction assigned to the Traffic Mirror rule.</p>
    pub traffic_direction: ::std::option::Option<crate::types::TrafficDirection>,
    /// <p>The rule number of the Traffic Mirror rule.</p>
    pub rule_number: ::std::option::Option<i32>,
    /// <p>The action assigned to the Traffic Mirror rule.</p>
    pub rule_action: ::std::option::Option<crate::types::TrafficMirrorRuleAction>,
    /// <p>The protocol assigned to the Traffic Mirror rule.</p>
    pub protocol: ::std::option::Option<i32>,
    /// <p>The destination port range assigned to the Traffic Mirror rule.</p>
    pub destination_port_range: ::std::option::Option<crate::types::TrafficMirrorPortRange>,
    /// <p>The source port range assigned to the Traffic Mirror rule.</p>
    pub source_port_range: ::std::option::Option<crate::types::TrafficMirrorPortRange>,
    /// <p>The destination CIDR block assigned to the Traffic Mirror rule.</p>
    pub destination_cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>The source CIDR block assigned to the Traffic Mirror rule.</p>
    pub source_cidr_block: ::std::option::Option<::std::string::String>,
    /// <p>The description of the Traffic Mirror rule.</p>
    pub description: ::std::option::Option<::std::string::String>,
}
impl TrafficMirrorFilterRule {
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn traffic_mirror_filter_rule_id(&self) -> ::std::option::Option<&str> {
        self.traffic_mirror_filter_rule_id.as_deref()
    }
    /// <p>The ID of the Traffic Mirror filter that the rule is associated with.</p>
    pub fn traffic_mirror_filter_id(&self) -> ::std::option::Option<&str> {
        self.traffic_mirror_filter_id.as_deref()
    }
    /// <p>The traffic direction assigned to the Traffic Mirror rule.</p>
    pub fn traffic_direction(&self) -> ::std::option::Option<&crate::types::TrafficDirection> {
        self.traffic_direction.as_ref()
    }
    /// <p>The rule number of the Traffic Mirror rule.</p>
    pub fn rule_number(&self) -> ::std::option::Option<i32> {
        self.rule_number
    }
    /// <p>The action assigned to the Traffic Mirror rule.</p>
    pub fn rule_action(&self) -> ::std::option::Option<&crate::types::TrafficMirrorRuleAction> {
        self.rule_action.as_ref()
    }
    /// <p>The protocol assigned to the Traffic Mirror rule.</p>
    pub fn protocol(&self) -> ::std::option::Option<i32> {
        self.protocol
    }
    /// <p>The destination port range assigned to the Traffic Mirror rule.</p>
    pub fn destination_port_range(&self) -> ::std::option::Option<&crate::types::TrafficMirrorPortRange> {
        self.destination_port_range.as_ref()
    }
    /// <p>The source port range assigned to the Traffic Mirror rule.</p>
    pub fn source_port_range(&self) -> ::std::option::Option<&crate::types::TrafficMirrorPortRange> {
        self.source_port_range.as_ref()
    }
    /// <p>The destination CIDR block assigned to the Traffic Mirror rule.</p>
    pub fn destination_cidr_block(&self) -> ::std::option::Option<&str> {
        self.destination_cidr_block.as_deref()
    }
    /// <p>The source CIDR block assigned to the Traffic Mirror rule.</p>
    pub fn source_cidr_block(&self) -> ::std::option::Option<&str> {
        self.source_cidr_block.as_deref()
    }
    /// <p>The description of the Traffic Mirror rule.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
}
impl TrafficMirrorFilterRule {
    /// Creates a new builder-style object to manufacture [`TrafficMirrorFilterRule`](crate::types::TrafficMirrorFilterRule).
    pub fn builder() -> crate::types::builders::TrafficMirrorFilterRuleBuilder {
        crate::types::builders::TrafficMirrorFilterRuleBuilder::default()
    }
}

/// A builder for [`TrafficMirrorFilterRule`](crate::types::TrafficMirrorFilterRule).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TrafficMirrorFilterRuleBuilder {
    pub(crate) traffic_mirror_filter_rule_id: ::std::option::Option<::std::string::String>,
    pub(crate) traffic_mirror_filter_id: ::std::option::Option<::std::string::String>,
    pub(crate) traffic_direction: ::std::option::Option<crate::types::TrafficDirection>,
    pub(crate) rule_number: ::std::option::Option<i32>,
    pub(crate) rule_action: ::std::option::Option<crate::types::TrafficMirrorRuleAction>,
    pub(crate) protocol: ::std::option::Option<i32>,
    pub(crate) destination_port_range: ::std::option::Option<crate::types::TrafficMirrorPortRange>,
    pub(crate) source_port_range: ::std::option::Option<crate::types::TrafficMirrorPortRange>,
    pub(crate) destination_cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) source_cidr_block: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
}
impl TrafficMirrorFilterRuleBuilder {
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn traffic_mirror_filter_rule_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.traffic_mirror_filter_rule_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn set_traffic_mirror_filter_rule_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.traffic_mirror_filter_rule_id = input;
        self
    }
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn get_traffic_mirror_filter_rule_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.traffic_mirror_filter_rule_id
    }
    /// <p>The ID of the Traffic Mirror filter that the rule is associated with.</p>
    pub fn traffic_mirror_filter_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.traffic_mirror_filter_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror filter that the rule is associated with.</p>
    pub fn set_traffic_mirror_filter_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.traffic_mirror_filter_id = input;
        self
    }
    /// <p>The ID of the Traffic Mirror filter that the rule is associated with.</p>
    pub fn get_traffic_mirror_filter_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.traffic_mirror_filter_id
    }
    /// <p>The traffic direction assigned to the Traffic Mirror rule.</p>
    pub fn traffic_direction(mut self, input: crate::types::TrafficDirection) -> Self {
        self.traffic_direction = ::std::option::Option::Some(input);
        self
    }
    /// <p>The traffic direction assigned to the Traffic Mirror rule.</p>
    pub fn set_traffic_direction(mut self, input: ::std::option::Option<crate::types::TrafficDirection>) -> Self {
        self.traffic_direction = input;
        self
    }
    /// <p>The traffic direction assigned to the Traffic Mirror rule.</p>
    pub fn get_traffic_direction(&self) -> &::std::option::Option<crate::types::TrafficDirection> {
        &self.traffic_direction
    }
    /// <p>The rule number of the Traffic Mirror rule.</p>
    pub fn rule_number(mut self, input: i32) -> Self {
        self.rule_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The rule number of the Traffic Mirror rule.</p>
    pub fn set_rule_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.rule_number = input;
        self
    }
    /// <p>The rule number of the Traffic Mirror rule.</p>
    pub fn get_rule_number(&self) -> &::std::option::Option<i32> {
        &self.rule_number
    }
    /// <p>The action assigned to the Traffic Mirror rule.</p>
    pub fn rule_action(mut self, input: crate::types::TrafficMirrorRuleAction) -> Self {
        self.rule_action = ::std::option::Option::Some(input);
        self
    }
    /// <p>The action assigned to the Traffic Mirror rule.</p>
    pub fn set_rule_action(mut self, input: ::std::option::Option<crate::types::TrafficMirrorRuleAction>) -> Self {
        self.rule_action = input;
        self
    }
    /// <p>The action assigned to the Traffic Mirror rule.</p>
    pub fn get_rule_action(&self) -> &::std::option::Option<crate::types::TrafficMirrorRuleAction> {
        &self.rule_action
    }
    /// <p>The protocol assigned to the Traffic Mirror rule.</p>
    pub fn protocol(mut self, input: i32) -> Self {
        self.protocol = ::std::option::Option::Some(input);
        self
    }
    /// <p>The protocol assigned to the Traffic Mirror rule.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<i32>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The protocol assigned to the Traffic Mirror rule.</p>
    pub fn get_protocol(&self) -> &::std::option::Option<i32> {
        &self.protocol
    }
    /// <p>The destination port range assigned to the Traffic Mirror rule.</p>
    pub fn destination_port_range(mut self, input: crate::types::TrafficMirrorPortRange) -> Self {
        self.destination_port_range = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination port range assigned to the Traffic Mirror rule.</p>
    pub fn set_destination_port_range(mut self, input: ::std::option::Option<crate::types::TrafficMirrorPortRange>) -> Self {
        self.destination_port_range = input;
        self
    }
    /// <p>The destination port range assigned to the Traffic Mirror rule.</p>
    pub fn get_destination_port_range(&self) -> &::std::option::Option<crate::types::TrafficMirrorPortRange> {
        &self.destination_port_range
    }
    /// <p>The source port range assigned to the Traffic Mirror rule.</p>
    pub fn source_port_range(mut self, input: crate::types::TrafficMirrorPortRange) -> Self {
        self.source_port_range = ::std::option::Option::Some(input);
        self
    }
    /// <p>The source port range assigned to the Traffic Mirror rule.</p>
    pub fn set_source_port_range(mut self, input: ::std::option::Option<crate::types::TrafficMirrorPortRange>) -> Self {
        self.source_port_range = input;
        self
    }
    /// <p>The source port range assigned to the Traffic Mirror rule.</p>
    pub fn get_source_port_range(&self) -> &::std::option::Option<crate::types::TrafficMirrorPortRange> {
        &self.source_port_range
    }
    /// <p>The destination CIDR block assigned to the Traffic Mirror rule.</p>
    pub fn destination_cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination_cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The destination CIDR block assigned to the Traffic Mirror rule.</p>
    pub fn set_destination_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination_cidr_block = input;
        self
    }
    /// <p>The destination CIDR block assigned to the Traffic Mirror rule.</p>
    pub fn get_destination_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination_cidr_block
    }
    /// <p>The source CIDR block assigned to the Traffic Mirror rule.</p>
    pub fn source_cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_cidr_block = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The source CIDR block assigned to the Traffic Mirror rule.</p>
    pub fn set_source_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_cidr_block = input;
        self
    }
    /// <p>The source CIDR block assigned to the Traffic Mirror rule.</p>
    pub fn get_source_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_cidr_block
    }
    /// <p>The description of the Traffic Mirror rule.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the Traffic Mirror rule.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the Traffic Mirror rule.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Consumes the builder and constructs a [`TrafficMirrorFilterRule`](crate::types::TrafficMirrorFilterRule).
    pub fn build(self) -> crate::types::TrafficMirrorFilterRule {
        crate::types::TrafficMirrorFilterRule {
            traffic_mirror_filter_rule_id: self.traffic_mirror_filter_rule_id,
            traffic_mirror_filter_id: self.traffic_mirror_filter_id,
            traffic_direction: self.traffic_direction,
            rule_number: self.rule_number,
            rule_action: self.rule_action,
            protocol: self.protocol,
            destination_port_range: self.destination_port_range,
            source_port_range: self.source_port_range,
            destination_cidr_block: self.destination_cidr_block,
            source_cidr_block: self.source_cidr_block,
            description: self.description,
        }
    }
}
