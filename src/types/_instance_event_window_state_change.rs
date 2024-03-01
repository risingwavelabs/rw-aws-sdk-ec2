// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The state of the event window.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceEventWindowStateChange {
    /// <p>The ID of the event window.</p>
    pub instance_event_window_id: ::std::option::Option<::std::string::String>,
    /// <p>The current state of the event window.</p>
    pub state: ::std::option::Option<crate::types::InstanceEventWindowState>,
}
impl InstanceEventWindowStateChange {
    /// <p>The ID of the event window.</p>
    pub fn instance_event_window_id(&self) -> ::std::option::Option<&str> {
        self.instance_event_window_id.as_deref()
    }
    /// <p>The current state of the event window.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::InstanceEventWindowState> {
        self.state.as_ref()
    }
}
impl InstanceEventWindowStateChange {
    /// Creates a new builder-style object to manufacture [`InstanceEventWindowStateChange`](crate::types::InstanceEventWindowStateChange).
    pub fn builder() -> crate::types::builders::InstanceEventWindowStateChangeBuilder {
        crate::types::builders::InstanceEventWindowStateChangeBuilder::default()
    }
}

/// A builder for [`InstanceEventWindowStateChange`](crate::types::InstanceEventWindowStateChange).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InstanceEventWindowStateChangeBuilder {
    pub(crate) instance_event_window_id: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::InstanceEventWindowState>,
}
impl InstanceEventWindowStateChangeBuilder {
    /// <p>The ID of the event window.</p>
    pub fn instance_event_window_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_event_window_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the event window.</p>
    pub fn set_instance_event_window_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_event_window_id = input;
        self
    }
    /// <p>The ID of the event window.</p>
    pub fn get_instance_event_window_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_event_window_id
    }
    /// <p>The current state of the event window.</p>
    pub fn state(mut self, input: crate::types::InstanceEventWindowState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state of the event window.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::InstanceEventWindowState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The current state of the event window.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::InstanceEventWindowState> {
        &self.state
    }
    /// Consumes the builder and constructs a [`InstanceEventWindowStateChange`](crate::types::InstanceEventWindowStateChange).
    pub fn build(self) -> crate::types::InstanceEventWindowStateChange {
        crate::types::InstanceEventWindowStateChange {
            instance_event_window_id: self.instance_event_window_id,
            state: self.state,
        }
    }
}