// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_placement_group::_create_placement_group_output::CreatePlacementGroupOutputBuilder;

pub use crate::operation::create_placement_group::_create_placement_group_input::CreatePlacementGroupInputBuilder;

impl CreatePlacementGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_placement_group::CreatePlacementGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_placement_group::CreatePlacementGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_placement_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreatePlacementGroup`.
///
/// <p>Creates a placement group in which to launch instances. The strategy of the placement group determines how the instances are organized within the group. </p>
/// <p>A <code>cluster</code> placement group is a logical grouping of instances within a single Availability Zone that benefit from low network latency, high network throughput. A <code>spread</code> placement group places instances on distinct hardware. A <code>partition</code> placement group places groups of instances in different partitions, where instances in one partition do not share the same hardware with instances in another partition.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/placement-groups.html">Placement groups</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreatePlacementGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_placement_group::builders::CreatePlacementGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_placement_group::CreatePlacementGroupOutput,
        crate::operation::create_placement_group::CreatePlacementGroupError,
    > for CreatePlacementGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_placement_group::CreatePlacementGroupOutput,
            crate::operation::create_placement_group::CreatePlacementGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreatePlacementGroupFluentBuilder {
    /// Creates a new `CreatePlacementGroup`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreatePlacementGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::create_placement_group::builders::CreatePlacementGroupInputBuilder {
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
        crate::operation::create_placement_group::CreatePlacementGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_placement_group::CreatePlacementGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_placement_group::CreatePlacementGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_placement_group::CreatePlacementGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_placement_group::CreatePlacementGroupOutput,
        crate::operation::create_placement_group::CreatePlacementGroupError,
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
    /// <p>A name for the placement group. Must be unique within the scope of your account for the Region.</p>
    /// <p>Constraints: Up to 255 ASCII characters</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group_name(input.into());
        self
    }
    /// <p>A name for the placement group. Must be unique within the scope of your account for the Region.</p>
    /// <p>Constraints: Up to 255 ASCII characters</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group_name(input);
        self
    }
    /// <p>A name for the placement group. Must be unique within the scope of your account for the Region.</p>
    /// <p>Constraints: Up to 255 ASCII characters</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group_name()
    }
    /// <p>The placement strategy.</p>
    pub fn strategy(mut self, input: crate::types::PlacementStrategy) -> Self {
        self.inner = self.inner.strategy(input);
        self
    }
    /// <p>The placement strategy.</p>
    pub fn set_strategy(mut self, input: ::std::option::Option<crate::types::PlacementStrategy>) -> Self {
        self.inner = self.inner.set_strategy(input);
        self
    }
    /// <p>The placement strategy.</p>
    pub fn get_strategy(&self) -> &::std::option::Option<crate::types::PlacementStrategy> {
        self.inner.get_strategy()
    }
    /// <p>The number of partitions. Valid only when <b>Strategy</b> is set to <code>partition</code>.</p>
    pub fn partition_count(mut self, input: i32) -> Self {
        self.inner = self.inner.partition_count(input);
        self
    }
    /// <p>The number of partitions. Valid only when <b>Strategy</b> is set to <code>partition</code>.</p>
    pub fn set_partition_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_partition_count(input);
        self
    }
    /// <p>The number of partitions. Valid only when <b>Strategy</b> is set to <code>partition</code>.</p>
    pub fn get_partition_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_partition_count()
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the new placement group.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the new placement group.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the new placement group.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
    }
    /// <p>Determines how placement groups spread instances. </p>
    /// <ul>
    /// <li> <p>Host – You can use <code>host</code> only with Outpost placement groups.</p> </li>
    /// <li> <p>Rack – No usage restrictions.</p> </li>
    /// </ul>
    pub fn spread_level(mut self, input: crate::types::SpreadLevel) -> Self {
        self.inner = self.inner.spread_level(input);
        self
    }
    /// <p>Determines how placement groups spread instances. </p>
    /// <ul>
    /// <li> <p>Host – You can use <code>host</code> only with Outpost placement groups.</p> </li>
    /// <li> <p>Rack – No usage restrictions.</p> </li>
    /// </ul>
    pub fn set_spread_level(mut self, input: ::std::option::Option<crate::types::SpreadLevel>) -> Self {
        self.inner = self.inner.set_spread_level(input);
        self
    }
    /// <p>Determines how placement groups spread instances. </p>
    /// <ul>
    /// <li> <p>Host – You can use <code>host</code> only with Outpost placement groups.</p> </li>
    /// <li> <p>Rack – No usage restrictions.</p> </li>
    /// </ul>
    pub fn get_spread_level(&self) -> &::std::option::Option<crate::types::SpreadLevel> {
        self.inner.get_spread_level()
    }
}
