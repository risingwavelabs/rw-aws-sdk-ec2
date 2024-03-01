// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an additional detail for a path analysis. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/reachability/additional-detail-codes.html">Reachability Analyzer additional detail codes</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AdditionalDetail {
    /// <p>The additional detail code.</p>
    pub additional_detail_type: ::std::option::Option<::std::string::String>,
    /// <p>The path component.</p>
    pub component: ::std::option::Option<crate::types::AnalysisComponent>,
    /// <p>The VPC endpoint service.</p>
    pub vpc_endpoint_service: ::std::option::Option<crate::types::AnalysisComponent>,
    /// <p>The rule options.</p>
    pub rule_options: ::std::option::Option<::std::vec::Vec<crate::types::RuleOption>>,
    /// <p>The rule group type.</p>
    pub rule_group_type_pairs: ::std::option::Option<::std::vec::Vec<crate::types::RuleGroupTypePair>>,
    /// <p>The rule options.</p>
    pub rule_group_rule_options_pairs: ::std::option::Option<::std::vec::Vec<crate::types::RuleGroupRuleOptionsPair>>,
    /// <p>The name of the VPC endpoint service.</p>
    pub service_name: ::std::option::Option<::std::string::String>,
    /// <p>The load balancers.</p>
    pub load_balancers: ::std::option::Option<::std::vec::Vec<crate::types::AnalysisComponent>>,
}
impl AdditionalDetail {
    /// <p>The additional detail code.</p>
    pub fn additional_detail_type(&self) -> ::std::option::Option<&str> {
        self.additional_detail_type.as_deref()
    }
    /// <p>The path component.</p>
    pub fn component(&self) -> ::std::option::Option<&crate::types::AnalysisComponent> {
        self.component.as_ref()
    }
    /// <p>The VPC endpoint service.</p>
    pub fn vpc_endpoint_service(&self) -> ::std::option::Option<&crate::types::AnalysisComponent> {
        self.vpc_endpoint_service.as_ref()
    }
    /// <p>The rule options.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.rule_options.is_none()`.
    pub fn rule_options(&self) -> &[crate::types::RuleOption] {
        self.rule_options.as_deref().unwrap_or_default()
    }
    /// <p>The rule group type.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.rule_group_type_pairs.is_none()`.
    pub fn rule_group_type_pairs(&self) -> &[crate::types::RuleGroupTypePair] {
        self.rule_group_type_pairs.as_deref().unwrap_or_default()
    }
    /// <p>The rule options.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.rule_group_rule_options_pairs.is_none()`.
    pub fn rule_group_rule_options_pairs(&self) -> &[crate::types::RuleGroupRuleOptionsPair] {
        self.rule_group_rule_options_pairs.as_deref().unwrap_or_default()
    }
    /// <p>The name of the VPC endpoint service.</p>
    pub fn service_name(&self) -> ::std::option::Option<&str> {
        self.service_name.as_deref()
    }
    /// <p>The load balancers.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.load_balancers.is_none()`.
    pub fn load_balancers(&self) -> &[crate::types::AnalysisComponent] {
        self.load_balancers.as_deref().unwrap_or_default()
    }
}
impl AdditionalDetail {
    /// Creates a new builder-style object to manufacture [`AdditionalDetail`](crate::types::AdditionalDetail).
    pub fn builder() -> crate::types::builders::AdditionalDetailBuilder {
        crate::types::builders::AdditionalDetailBuilder::default()
    }
}

/// A builder for [`AdditionalDetail`](crate::types::AdditionalDetail).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AdditionalDetailBuilder {
    pub(crate) additional_detail_type: ::std::option::Option<::std::string::String>,
    pub(crate) component: ::std::option::Option<crate::types::AnalysisComponent>,
    pub(crate) vpc_endpoint_service: ::std::option::Option<crate::types::AnalysisComponent>,
    pub(crate) rule_options: ::std::option::Option<::std::vec::Vec<crate::types::RuleOption>>,
    pub(crate) rule_group_type_pairs: ::std::option::Option<::std::vec::Vec<crate::types::RuleGroupTypePair>>,
    pub(crate) rule_group_rule_options_pairs: ::std::option::Option<::std::vec::Vec<crate::types::RuleGroupRuleOptionsPair>>,
    pub(crate) service_name: ::std::option::Option<::std::string::String>,
    pub(crate) load_balancers: ::std::option::Option<::std::vec::Vec<crate::types::AnalysisComponent>>,
}
impl AdditionalDetailBuilder {
    /// <p>The additional detail code.</p>
    pub fn additional_detail_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.additional_detail_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The additional detail code.</p>
    pub fn set_additional_detail_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.additional_detail_type = input;
        self
    }
    /// <p>The additional detail code.</p>
    pub fn get_additional_detail_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.additional_detail_type
    }
    /// <p>The path component.</p>
    pub fn component(mut self, input: crate::types::AnalysisComponent) -> Self {
        self.component = ::std::option::Option::Some(input);
        self
    }
    /// <p>The path component.</p>
    pub fn set_component(mut self, input: ::std::option::Option<crate::types::AnalysisComponent>) -> Self {
        self.component = input;
        self
    }
    /// <p>The path component.</p>
    pub fn get_component(&self) -> &::std::option::Option<crate::types::AnalysisComponent> {
        &self.component
    }
    /// <p>The VPC endpoint service.</p>
    pub fn vpc_endpoint_service(mut self, input: crate::types::AnalysisComponent) -> Self {
        self.vpc_endpoint_service = ::std::option::Option::Some(input);
        self
    }
    /// <p>The VPC endpoint service.</p>
    pub fn set_vpc_endpoint_service(mut self, input: ::std::option::Option<crate::types::AnalysisComponent>) -> Self {
        self.vpc_endpoint_service = input;
        self
    }
    /// <p>The VPC endpoint service.</p>
    pub fn get_vpc_endpoint_service(&self) -> &::std::option::Option<crate::types::AnalysisComponent> {
        &self.vpc_endpoint_service
    }
    /// Appends an item to `rule_options`.
    ///
    /// To override the contents of this collection use [`set_rule_options`](Self::set_rule_options).
    ///
    /// <p>The rule options.</p>
    pub fn rule_options(mut self, input: crate::types::RuleOption) -> Self {
        let mut v = self.rule_options.unwrap_or_default();
        v.push(input);
        self.rule_options = ::std::option::Option::Some(v);
        self
    }
    /// <p>The rule options.</p>
    pub fn set_rule_options(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RuleOption>>) -> Self {
        self.rule_options = input;
        self
    }
    /// <p>The rule options.</p>
    pub fn get_rule_options(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RuleOption>> {
        &self.rule_options
    }
    /// Appends an item to `rule_group_type_pairs`.
    ///
    /// To override the contents of this collection use [`set_rule_group_type_pairs`](Self::set_rule_group_type_pairs).
    ///
    /// <p>The rule group type.</p>
    pub fn rule_group_type_pairs(mut self, input: crate::types::RuleGroupTypePair) -> Self {
        let mut v = self.rule_group_type_pairs.unwrap_or_default();
        v.push(input);
        self.rule_group_type_pairs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The rule group type.</p>
    pub fn set_rule_group_type_pairs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::RuleGroupTypePair>>) -> Self {
        self.rule_group_type_pairs = input;
        self
    }
    /// <p>The rule group type.</p>
    pub fn get_rule_group_type_pairs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RuleGroupTypePair>> {
        &self.rule_group_type_pairs
    }
    /// Appends an item to `rule_group_rule_options_pairs`.
    ///
    /// To override the contents of this collection use [`set_rule_group_rule_options_pairs`](Self::set_rule_group_rule_options_pairs).
    ///
    /// <p>The rule options.</p>
    pub fn rule_group_rule_options_pairs(mut self, input: crate::types::RuleGroupRuleOptionsPair) -> Self {
        let mut v = self.rule_group_rule_options_pairs.unwrap_or_default();
        v.push(input);
        self.rule_group_rule_options_pairs = ::std::option::Option::Some(v);
        self
    }
    /// <p>The rule options.</p>
    pub fn set_rule_group_rule_options_pairs(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::RuleGroupRuleOptionsPair>>,
    ) -> Self {
        self.rule_group_rule_options_pairs = input;
        self
    }
    /// <p>The rule options.</p>
    pub fn get_rule_group_rule_options_pairs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::RuleGroupRuleOptionsPair>> {
        &self.rule_group_rule_options_pairs
    }
    /// <p>The name of the VPC endpoint service.</p>
    pub fn service_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the VPC endpoint service.</p>
    pub fn set_service_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_name = input;
        self
    }
    /// <p>The name of the VPC endpoint service.</p>
    pub fn get_service_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.service_name
    }
    /// Appends an item to `load_balancers`.
    ///
    /// To override the contents of this collection use [`set_load_balancers`](Self::set_load_balancers).
    ///
    /// <p>The load balancers.</p>
    pub fn load_balancers(mut self, input: crate::types::AnalysisComponent) -> Self {
        let mut v = self.load_balancers.unwrap_or_default();
        v.push(input);
        self.load_balancers = ::std::option::Option::Some(v);
        self
    }
    /// <p>The load balancers.</p>
    pub fn set_load_balancers(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AnalysisComponent>>) -> Self {
        self.load_balancers = input;
        self
    }
    /// <p>The load balancers.</p>
    pub fn get_load_balancers(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AnalysisComponent>> {
        &self.load_balancers
    }
    /// Consumes the builder and constructs a [`AdditionalDetail`](crate::types::AdditionalDetail).
    pub fn build(self) -> crate::types::AdditionalDetail {
        crate::types::AdditionalDetail {
            additional_detail_type: self.additional_detail_type,
            component: self.component,
            vpc_endpoint_service: self.vpc_endpoint_service,
            rule_options: self.rule_options,
            rule_group_type_pairs: self.rule_group_type_pairs,
            rule_group_rule_options_pairs: self.rule_group_rule_options_pairs,
            service_name: self.service_name,
            load_balancers: self.load_balancers,
        }
    }
}