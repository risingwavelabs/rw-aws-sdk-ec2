// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the Classic Load Balancers and target groups to attach to a Spot Fleet request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LoadBalancersConfig {
    /// <p>The Classic Load Balancers.</p>
    pub classic_load_balancers_config: ::std::option::Option<crate::types::ClassicLoadBalancersConfig>,
    /// <p>The target groups.</p>
    pub target_groups_config: ::std::option::Option<crate::types::TargetGroupsConfig>,
}
impl LoadBalancersConfig {
    /// <p>The Classic Load Balancers.</p>
    pub fn classic_load_balancers_config(&self) -> ::std::option::Option<&crate::types::ClassicLoadBalancersConfig> {
        self.classic_load_balancers_config.as_ref()
    }
    /// <p>The target groups.</p>
    pub fn target_groups_config(&self) -> ::std::option::Option<&crate::types::TargetGroupsConfig> {
        self.target_groups_config.as_ref()
    }
}
impl LoadBalancersConfig {
    /// Creates a new builder-style object to manufacture [`LoadBalancersConfig`](crate::types::LoadBalancersConfig).
    pub fn builder() -> crate::types::builders::LoadBalancersConfigBuilder {
        crate::types::builders::LoadBalancersConfigBuilder::default()
    }
}

/// A builder for [`LoadBalancersConfig`](crate::types::LoadBalancersConfig).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LoadBalancersConfigBuilder {
    pub(crate) classic_load_balancers_config: ::std::option::Option<crate::types::ClassicLoadBalancersConfig>,
    pub(crate) target_groups_config: ::std::option::Option<crate::types::TargetGroupsConfig>,
}
impl LoadBalancersConfigBuilder {
    /// <p>The Classic Load Balancers.</p>
    pub fn classic_load_balancers_config(mut self, input: crate::types::ClassicLoadBalancersConfig) -> Self {
        self.classic_load_balancers_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Classic Load Balancers.</p>
    pub fn set_classic_load_balancers_config(mut self, input: ::std::option::Option<crate::types::ClassicLoadBalancersConfig>) -> Self {
        self.classic_load_balancers_config = input;
        self
    }
    /// <p>The Classic Load Balancers.</p>
    pub fn get_classic_load_balancers_config(&self) -> &::std::option::Option<crate::types::ClassicLoadBalancersConfig> {
        &self.classic_load_balancers_config
    }
    /// <p>The target groups.</p>
    pub fn target_groups_config(mut self, input: crate::types::TargetGroupsConfig) -> Self {
        self.target_groups_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>The target groups.</p>
    pub fn set_target_groups_config(mut self, input: ::std::option::Option<crate::types::TargetGroupsConfig>) -> Self {
        self.target_groups_config = input;
        self
    }
    /// <p>The target groups.</p>
    pub fn get_target_groups_config(&self) -> &::std::option::Option<crate::types::TargetGroupsConfig> {
        &self.target_groups_config
    }
    /// Consumes the builder and constructs a [`LoadBalancersConfig`](crate::types::LoadBalancersConfig).
    pub fn build(self) -> crate::types::LoadBalancersConfig {
        crate::types::LoadBalancersConfig {
            classic_load_balancers_config: self.classic_load_balancers_config,
            target_groups_config: self.target_groups_config,
        }
    }
}