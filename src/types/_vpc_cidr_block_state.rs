// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the state of a CIDR block.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpcCidrBlockState {
    /// <p>The state of the CIDR block.</p>
    pub state: ::std::option::Option<crate::types::VpcCidrBlockStateCode>,
    /// <p>A message about the status of the CIDR block, if applicable.</p>
    pub status_message: ::std::option::Option<::std::string::String>,
}
impl VpcCidrBlockState {
    /// <p>The state of the CIDR block.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::VpcCidrBlockStateCode> {
        self.state.as_ref()
    }
    /// <p>A message about the status of the CIDR block, if applicable.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
}
impl VpcCidrBlockState {
    /// Creates a new builder-style object to manufacture [`VpcCidrBlockState`](crate::types::VpcCidrBlockState).
    pub fn builder() -> crate::types::builders::VpcCidrBlockStateBuilder {
        crate::types::builders::VpcCidrBlockStateBuilder::default()
    }
}

/// A builder for [`VpcCidrBlockState`](crate::types::VpcCidrBlockState).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VpcCidrBlockStateBuilder {
    pub(crate) state: ::std::option::Option<crate::types::VpcCidrBlockStateCode>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
}
impl VpcCidrBlockStateBuilder {
    /// <p>The state of the CIDR block.</p>
    pub fn state(mut self, input: crate::types::VpcCidrBlockStateCode) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the CIDR block.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::VpcCidrBlockStateCode>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the CIDR block.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::VpcCidrBlockStateCode> {
        &self.state
    }
    /// <p>A message about the status of the CIDR block, if applicable.</p>
    pub fn status_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A message about the status of the CIDR block, if applicable.</p>
    pub fn set_status_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// <p>A message about the status of the CIDR block, if applicable.</p>
    pub fn get_status_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_message
    }
    /// Consumes the builder and constructs a [`VpcCidrBlockState`](crate::types::VpcCidrBlockState).
    pub fn build(self) -> crate::types::VpcCidrBlockState {
        crate::types::VpcCidrBlockState {
            state: self.state,
            status_message: self.status_message,
        }
    }
}