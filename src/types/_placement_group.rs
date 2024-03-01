// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a placement group.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PlacementGroup {
    /// <p>The name of the placement group.</p>
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The state of the placement group.</p>
    pub state: ::std::option::Option<crate::types::PlacementGroupState>,
    /// <p>The placement strategy.</p>
    pub strategy: ::std::option::Option<crate::types::PlacementStrategy>,
    /// <p>The number of partitions. Valid only if <b>strategy</b> is set to <code>partition</code>.</p>
    pub partition_count: ::std::option::Option<i32>,
    /// <p>The ID of the placement group.</p>
    pub group_id: ::std::option::Option<::std::string::String>,
    /// <p>Any tags applied to the placement group.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The Amazon Resource Name (ARN) of the placement group.</p>
    pub group_arn: ::std::option::Option<::std::string::String>,
    /// <p>The spread level for the placement group. <i>Only</i> Outpost placement groups can be spread across hosts.</p>
    pub spread_level: ::std::option::Option<crate::types::SpreadLevel>,
}
impl PlacementGroup {
    /// <p>The name of the placement group.</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The state of the placement group.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::PlacementGroupState> {
        self.state.as_ref()
    }
    /// <p>The placement strategy.</p>
    pub fn strategy(&self) -> ::std::option::Option<&crate::types::PlacementStrategy> {
        self.strategy.as_ref()
    }
    /// <p>The number of partitions. Valid only if <b>strategy</b> is set to <code>partition</code>.</p>
    pub fn partition_count(&self) -> ::std::option::Option<i32> {
        self.partition_count
    }
    /// <p>The ID of the placement group.</p>
    pub fn group_id(&self) -> ::std::option::Option<&str> {
        self.group_id.as_deref()
    }
    /// <p>Any tags applied to the placement group.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
    /// <p>The Amazon Resource Name (ARN) of the placement group.</p>
    pub fn group_arn(&self) -> ::std::option::Option<&str> {
        self.group_arn.as_deref()
    }
    /// <p>The spread level for the placement group. <i>Only</i> Outpost placement groups can be spread across hosts.</p>
    pub fn spread_level(&self) -> ::std::option::Option<&crate::types::SpreadLevel> {
        self.spread_level.as_ref()
    }
}
impl PlacementGroup {
    /// Creates a new builder-style object to manufacture [`PlacementGroup`](crate::types::PlacementGroup).
    pub fn builder() -> crate::types::builders::PlacementGroupBuilder {
        crate::types::builders::PlacementGroupBuilder::default()
    }
}

/// A builder for [`PlacementGroup`](crate::types::PlacementGroup).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct PlacementGroupBuilder {
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::PlacementGroupState>,
    pub(crate) strategy: ::std::option::Option<crate::types::PlacementStrategy>,
    pub(crate) partition_count: ::std::option::Option<i32>,
    pub(crate) group_id: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) group_arn: ::std::option::Option<::std::string::String>,
    pub(crate) spread_level: ::std::option::Option<crate::types::SpreadLevel>,
}
impl PlacementGroupBuilder {
    /// <p>The name of the placement group.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the placement group.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>The name of the placement group.</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_name
    }
    /// <p>The state of the placement group.</p>
    pub fn state(mut self, input: crate::types::PlacementGroupState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the placement group.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::PlacementGroupState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the placement group.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::PlacementGroupState> {
        &self.state
    }
    /// <p>The placement strategy.</p>
    pub fn strategy(mut self, input: crate::types::PlacementStrategy) -> Self {
        self.strategy = ::std::option::Option::Some(input);
        self
    }
    /// <p>The placement strategy.</p>
    pub fn set_strategy(mut self, input: ::std::option::Option<crate::types::PlacementStrategy>) -> Self {
        self.strategy = input;
        self
    }
    /// <p>The placement strategy.</p>
    pub fn get_strategy(&self) -> &::std::option::Option<crate::types::PlacementStrategy> {
        &self.strategy
    }
    /// <p>The number of partitions. Valid only if <b>strategy</b> is set to <code>partition</code>.</p>
    pub fn partition_count(mut self, input: i32) -> Self {
        self.partition_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of partitions. Valid only if <b>strategy</b> is set to <code>partition</code>.</p>
    pub fn set_partition_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.partition_count = input;
        self
    }
    /// <p>The number of partitions. Valid only if <b>strategy</b> is set to <code>partition</code>.</p>
    pub fn get_partition_count(&self) -> &::std::option::Option<i32> {
        &self.partition_count
    }
    /// <p>The ID of the placement group.</p>
    pub fn group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the placement group.</p>
    pub fn set_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_id = input;
        self
    }
    /// <p>The ID of the placement group.</p>
    pub fn get_group_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_id
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags applied to the placement group.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Any tags applied to the placement group.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Any tags applied to the placement group.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// <p>The Amazon Resource Name (ARN) of the placement group.</p>
    pub fn group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the placement group.</p>
    pub fn set_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the placement group.</p>
    pub fn get_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_arn
    }
    /// <p>The spread level for the placement group. <i>Only</i> Outpost placement groups can be spread across hosts.</p>
    pub fn spread_level(mut self, input: crate::types::SpreadLevel) -> Self {
        self.spread_level = ::std::option::Option::Some(input);
        self
    }
    /// <p>The spread level for the placement group. <i>Only</i> Outpost placement groups can be spread across hosts.</p>
    pub fn set_spread_level(mut self, input: ::std::option::Option<crate::types::SpreadLevel>) -> Self {
        self.spread_level = input;
        self
    }
    /// <p>The spread level for the placement group. <i>Only</i> Outpost placement groups can be spread across hosts.</p>
    pub fn get_spread_level(&self) -> &::std::option::Option<crate::types::SpreadLevel> {
        &self.spread_level
    }
    /// Consumes the builder and constructs a [`PlacementGroup`](crate::types::PlacementGroup).
    pub fn build(self) -> crate::types::PlacementGroup {
        crate::types::PlacementGroup {
            group_name: self.group_name,
            state: self.state,
            strategy: self.strategy,
            partition_count: self.partition_count,
            group_id: self.group_id,
            tags: self.tags,
            group_arn: self.group_arn,
            spread_level: self.spread_level,
        }
    }
}
