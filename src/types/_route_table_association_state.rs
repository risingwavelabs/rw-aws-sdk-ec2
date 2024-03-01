// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the state of an association between a route table and a subnet or gateway.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RouteTableAssociationState {
    /// <p>The state of the association.</p>
    pub state: ::std::option::Option<crate::types::RouteTableAssociationStateCode>,
    /// <p>The status message, if applicable.</p>
    pub status_message: ::std::option::Option<::std::string::String>,
}
impl RouteTableAssociationState {
    /// <p>The state of the association.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::RouteTableAssociationStateCode> {
        self.state.as_ref()
    }
    /// <p>The status message, if applicable.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
}
impl RouteTableAssociationState {
    /// Creates a new builder-style object to manufacture [`RouteTableAssociationState`](crate::types::RouteTableAssociationState).
    pub fn builder() -> crate::types::builders::RouteTableAssociationStateBuilder {
        crate::types::builders::RouteTableAssociationStateBuilder::default()
    }
}

/// A builder for [`RouteTableAssociationState`](crate::types::RouteTableAssociationState).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RouteTableAssociationStateBuilder {
    pub(crate) state: ::std::option::Option<crate::types::RouteTableAssociationStateCode>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
}
impl RouteTableAssociationStateBuilder {
    /// <p>The state of the association.</p>
    pub fn state(mut self, input: crate::types::RouteTableAssociationStateCode) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the association.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::RouteTableAssociationStateCode>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the association.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::RouteTableAssociationStateCode> {
        &self.state
    }
    /// <p>The status message, if applicable.</p>
    pub fn status_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The status message, if applicable.</p>
    pub fn set_status_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// <p>The status message, if applicable.</p>
    pub fn get_status_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_message
    }
    /// Consumes the builder and constructs a [`RouteTableAssociationState`](crate::types::RouteTableAssociationState).
    pub fn build(self) -> crate::types::RouteTableAssociationState {
        crate::types::RouteTableAssociationState {
            state: self.state,
            status_message: self.status_message,
        }
    }
}