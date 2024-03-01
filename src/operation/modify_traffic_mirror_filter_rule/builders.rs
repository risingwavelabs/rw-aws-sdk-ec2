// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_traffic_mirror_filter_rule::_modify_traffic_mirror_filter_rule_output::ModifyTrafficMirrorFilterRuleOutputBuilder;

pub use crate::operation::modify_traffic_mirror_filter_rule::_modify_traffic_mirror_filter_rule_input::ModifyTrafficMirrorFilterRuleInputBuilder;

impl ModifyTrafficMirrorFilterRuleInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_traffic_mirror_filter_rule();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyTrafficMirrorFilterRule`.
///
/// <p>Modifies the specified Traffic Mirror rule.</p>
/// <p> <code>DestinationCidrBlock</code> and <code>SourceCidrBlock</code> must both be an IPv4 range or an IPv6 range.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyTrafficMirrorFilterRuleFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleOutput,
        crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleError,
    > for ModifyTrafficMirrorFilterRuleFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleOutput,
            crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyTrafficMirrorFilterRuleFluentBuilder {
    /// Creates a new `ModifyTrafficMirrorFilterRule`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyTrafficMirrorFilterRule as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_traffic_mirror_filter_rule::builders::ModifyTrafficMirrorFilterRuleInputBuilder {
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
        crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRule::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRule::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleOutput,
        crate::operation::modify_traffic_mirror_filter_rule::ModifyTrafficMirrorFilterRuleError,
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
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn traffic_mirror_filter_rule_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.traffic_mirror_filter_rule_id(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn set_traffic_mirror_filter_rule_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_traffic_mirror_filter_rule_id(input);
        self
    }
    /// <p>The ID of the Traffic Mirror rule.</p>
    pub fn get_traffic_mirror_filter_rule_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_traffic_mirror_filter_rule_id()
    }
    /// <p>The type of traffic to assign to the rule.</p>
    pub fn traffic_direction(mut self, input: crate::types::TrafficDirection) -> Self {
        self.inner = self.inner.traffic_direction(input);
        self
    }
    /// <p>The type of traffic to assign to the rule.</p>
    pub fn set_traffic_direction(mut self, input: ::std::option::Option<crate::types::TrafficDirection>) -> Self {
        self.inner = self.inner.set_traffic_direction(input);
        self
    }
    /// <p>The type of traffic to assign to the rule.</p>
    pub fn get_traffic_direction(&self) -> &::std::option::Option<crate::types::TrafficDirection> {
        self.inner.get_traffic_direction()
    }
    /// <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.</p>
    pub fn rule_number(mut self, input: i32) -> Self {
        self.inner = self.inner.rule_number(input);
        self
    }
    /// <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.</p>
    pub fn set_rule_number(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_rule_number(input);
        self
    }
    /// <p>The number of the Traffic Mirror rule. This number must be unique for each Traffic Mirror rule in a given direction. The rules are processed in ascending order by rule number.</p>
    pub fn get_rule_number(&self) -> &::std::option::Option<i32> {
        self.inner.get_rule_number()
    }
    /// <p>The action to assign to the rule.</p>
    pub fn rule_action(mut self, input: crate::types::TrafficMirrorRuleAction) -> Self {
        self.inner = self.inner.rule_action(input);
        self
    }
    /// <p>The action to assign to the rule.</p>
    pub fn set_rule_action(mut self, input: ::std::option::Option<crate::types::TrafficMirrorRuleAction>) -> Self {
        self.inner = self.inner.set_rule_action(input);
        self
    }
    /// <p>The action to assign to the rule.</p>
    pub fn get_rule_action(&self) -> &::std::option::Option<crate::types::TrafficMirrorRuleAction> {
        self.inner.get_rule_action()
    }
    /// <p>The destination ports that are associated with the Traffic Mirror rule.</p>
    pub fn destination_port_range(mut self, input: crate::types::TrafficMirrorPortRangeRequest) -> Self {
        self.inner = self.inner.destination_port_range(input);
        self
    }
    /// <p>The destination ports that are associated with the Traffic Mirror rule.</p>
    pub fn set_destination_port_range(mut self, input: ::std::option::Option<crate::types::TrafficMirrorPortRangeRequest>) -> Self {
        self.inner = self.inner.set_destination_port_range(input);
        self
    }
    /// <p>The destination ports that are associated with the Traffic Mirror rule.</p>
    pub fn get_destination_port_range(&self) -> &::std::option::Option<crate::types::TrafficMirrorPortRangeRequest> {
        self.inner.get_destination_port_range()
    }
    /// <p>The port range to assign to the Traffic Mirror rule.</p>
    pub fn source_port_range(mut self, input: crate::types::TrafficMirrorPortRangeRequest) -> Self {
        self.inner = self.inner.source_port_range(input);
        self
    }
    /// <p>The port range to assign to the Traffic Mirror rule.</p>
    pub fn set_source_port_range(mut self, input: ::std::option::Option<crate::types::TrafficMirrorPortRangeRequest>) -> Self {
        self.inner = self.inner.set_source_port_range(input);
        self
    }
    /// <p>The port range to assign to the Traffic Mirror rule.</p>
    pub fn get_source_port_range(&self) -> &::std::option::Option<crate::types::TrafficMirrorPortRangeRequest> {
        self.inner.get_source_port_range()
    }
    /// <p>The protocol, for example TCP, to assign to the Traffic Mirror rule.</p>
    pub fn protocol(mut self, input: i32) -> Self {
        self.inner = self.inner.protocol(input);
        self
    }
    /// <p>The protocol, for example TCP, to assign to the Traffic Mirror rule.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_protocol(input);
        self
    }
    /// <p>The protocol, for example TCP, to assign to the Traffic Mirror rule.</p>
    pub fn get_protocol(&self) -> &::std::option::Option<i32> {
        self.inner.get_protocol()
    }
    /// <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn destination_cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.destination_cidr_block(input.into());
        self
    }
    /// <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn set_destination_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_destination_cidr_block(input);
        self
    }
    /// <p>The destination CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn get_destination_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_destination_cidr_block()
    }
    /// <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn source_cidr_block(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_cidr_block(input.into());
        self
    }
    /// <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn set_source_cidr_block(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_cidr_block(input);
        self
    }
    /// <p>The source CIDR block to assign to the Traffic Mirror rule.</p>
    pub fn get_source_cidr_block(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_cidr_block()
    }
    /// <p>The description to assign to the Traffic Mirror rule.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description to assign to the Traffic Mirror rule.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description to assign to the Traffic Mirror rule.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// Appends an item to `RemoveFields`.
    ///
    /// To override the contents of this collection use [`set_remove_fields`](Self::set_remove_fields).
    ///
    /// <p>The properties that you want to remove from the Traffic Mirror filter rule.</p>
    /// <p>When you remove a property from a Traffic Mirror filter rule, the property is set to the default.</p>
    pub fn remove_fields(mut self, input: crate::types::TrafficMirrorFilterRuleField) -> Self {
        self.inner = self.inner.remove_fields(input);
        self
    }
    /// <p>The properties that you want to remove from the Traffic Mirror filter rule.</p>
    /// <p>When you remove a property from a Traffic Mirror filter rule, the property is set to the default.</p>
    pub fn set_remove_fields(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorFilterRuleField>>) -> Self {
        self.inner = self.inner.set_remove_fields(input);
        self
    }
    /// <p>The properties that you want to remove from the Traffic Mirror filter rule.</p>
    /// <p>When you remove a property from a Traffic Mirror filter rule, the property is set to the default.</p>
    pub fn get_remove_fields(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorFilterRuleField>> {
        self.inner.get_remove_fields()
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
}
