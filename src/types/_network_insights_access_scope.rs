// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Network Access Scope.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NetworkInsightsAccessScope {
    /// <p>The ID of the Network Access Scope.</p>
    pub network_insights_access_scope_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Network Access Scope.</p>
    pub network_insights_access_scope_arn: ::std::option::Option<::std::string::String>,
    /// <p>The creation date.</p>
    pub created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The last updated date.</p>
    pub updated_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The tags.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl NetworkInsightsAccessScope {
    /// <p>The ID of the Network Access Scope.</p>
    pub fn network_insights_access_scope_id(&self) -> ::std::option::Option<&str> {
        self.network_insights_access_scope_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Access Scope.</p>
    pub fn network_insights_access_scope_arn(&self) -> ::std::option::Option<&str> {
        self.network_insights_access_scope_arn.as_deref()
    }
    /// <p>The creation date.</p>
    pub fn created_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_date.as_ref()
    }
    /// <p>The last updated date.</p>
    pub fn updated_date(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.updated_date.as_ref()
    }
    /// <p>The tags.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl NetworkInsightsAccessScope {
    /// Creates a new builder-style object to manufacture [`NetworkInsightsAccessScope`](crate::types::NetworkInsightsAccessScope).
    pub fn builder() -> crate::types::builders::NetworkInsightsAccessScopeBuilder {
        crate::types::builders::NetworkInsightsAccessScopeBuilder::default()
    }
}

/// A builder for [`NetworkInsightsAccessScope`](crate::types::NetworkInsightsAccessScope).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct NetworkInsightsAccessScopeBuilder {
    pub(crate) network_insights_access_scope_id: ::std::option::Option<::std::string::String>,
    pub(crate) network_insights_access_scope_arn: ::std::option::Option<::std::string::String>,
    pub(crate) created_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) updated_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl NetworkInsightsAccessScopeBuilder {
    /// <p>The ID of the Network Access Scope.</p>
    pub fn network_insights_access_scope_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_insights_access_scope_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Network Access Scope.</p>
    pub fn set_network_insights_access_scope_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_insights_access_scope_id = input;
        self
    }
    /// <p>The ID of the Network Access Scope.</p>
    pub fn get_network_insights_access_scope_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.network_insights_access_scope_id
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Access Scope.</p>
    pub fn network_insights_access_scope_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_insights_access_scope_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Access Scope.</p>
    pub fn set_network_insights_access_scope_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_insights_access_scope_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Access Scope.</p>
    pub fn get_network_insights_access_scope_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.network_insights_access_scope_arn
    }
    /// <p>The creation date.</p>
    pub fn created_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The creation date.</p>
    pub fn set_created_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_date = input;
        self
    }
    /// <p>The creation date.</p>
    pub fn get_created_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_date
    }
    /// <p>The last updated date.</p>
    pub fn updated_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.updated_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The last updated date.</p>
    pub fn set_updated_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.updated_date = input;
        self
    }
    /// <p>The last updated date.</p>
    pub fn get_updated_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.updated_date
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`NetworkInsightsAccessScope`](crate::types::NetworkInsightsAccessScope).
    pub fn build(self) -> crate::types::NetworkInsightsAccessScope {
        crate::types::NetworkInsightsAccessScope {
            network_insights_access_scope_id: self.network_insights_access_scope_id,
            network_insights_access_scope_arn: self.network_insights_access_scope_arn,
            created_date: self.created_date,
            updated_date: self.updated_date,
            tags: self.tags,
        }
    }
}
