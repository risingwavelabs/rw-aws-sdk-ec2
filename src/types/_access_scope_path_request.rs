// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a path.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AccessScopePathRequest {
    /// <p>The source.</p>
    pub source: ::std::option::Option<crate::types::PathStatementRequest>,
    /// <p>The destination.</p>
    pub destination: ::std::option::Option<crate::types::PathStatementRequest>,
    /// <p>The through resources.</p>
    pub through_resources: ::std::option::Option<::std::vec::Vec<crate::types::ThroughResourcesStatementRequest>>,
}
impl AccessScopePathRequest {
    /// <p>The source.</p>
    pub fn source(&self) -> ::std::option::Option<&crate::types::PathStatementRequest> {
        self.source.as_ref()
    }
    /// <p>The destination.</p>
    pub fn destination(&self) -> ::std::option::Option<&crate::types::PathStatementRequest> {
        self.destination.as_ref()
    }
    /// <p>The through resources.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.through_resources.is_none()`.
    pub fn through_resources(&self) -> &[crate::types::ThroughResourcesStatementRequest] {
        self.through_resources.as_deref().unwrap_or_default()
    }
}
impl AccessScopePathRequest {
    /// Creates a new builder-style object to manufacture [`AccessScopePathRequest`](crate::types::AccessScopePathRequest).
    pub fn builder() -> crate::types::builders::AccessScopePathRequestBuilder {
        crate::types::builders::AccessScopePathRequestBuilder::default()
    }
}

/// A builder for [`AccessScopePathRequest`](crate::types::AccessScopePathRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AccessScopePathRequestBuilder {
    pub(crate) source: ::std::option::Option<crate::types::PathStatementRequest>,
    pub(crate) destination: ::std::option::Option<crate::types::PathStatementRequest>,
    pub(crate) through_resources: ::std::option::Option<::std::vec::Vec<crate::types::ThroughResourcesStatementRequest>>,
}
impl AccessScopePathRequestBuilder {
    /// <p>The source.</p>
    pub fn source(mut self, input: crate::types::PathStatementRequest) -> Self {
        self.source = ::std::option::Option::Some(input);
        self
    }
    /// <p>The source.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::PathStatementRequest>) -> Self {
        self.source = input;
        self
    }
    /// <p>The source.</p>
    pub fn get_source(&self) -> &::std::option::Option<crate::types::PathStatementRequest> {
        &self.source
    }
    /// <p>The destination.</p>
    pub fn destination(mut self, input: crate::types::PathStatementRequest) -> Self {
        self.destination = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<crate::types::PathStatementRequest>) -> Self {
        self.destination = input;
        self
    }
    /// <p>The destination.</p>
    pub fn get_destination(&self) -> &::std::option::Option<crate::types::PathStatementRequest> {
        &self.destination
    }
    /// Appends an item to `through_resources`.
    ///
    /// To override the contents of this collection use [`set_through_resources`](Self::set_through_resources).
    ///
    /// <p>The through resources.</p>
    pub fn through_resources(mut self, input: crate::types::ThroughResourcesStatementRequest) -> Self {
        let mut v = self.through_resources.unwrap_or_default();
        v.push(input);
        self.through_resources = ::std::option::Option::Some(v);
        self
    }
    /// <p>The through resources.</p>
    pub fn set_through_resources(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ThroughResourcesStatementRequest>>) -> Self {
        self.through_resources = input;
        self
    }
    /// <p>The through resources.</p>
    pub fn get_through_resources(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ThroughResourcesStatementRequest>> {
        &self.through_resources
    }
    /// Consumes the builder and constructs a [`AccessScopePathRequest`](crate::types::AccessScopePathRequest).
    pub fn build(self) -> crate::types::AccessScopePathRequest {
        crate::types::AccessScopePathRequest {
            source: self.source,
            destination: self.destination,
            through_resources: self.through_resources,
        }
    }
}
