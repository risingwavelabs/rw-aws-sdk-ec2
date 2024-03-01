// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyInstanceEventStartTimeOutput {
    /// <p>Information about the event.</p>
    pub event: ::std::option::Option<crate::types::InstanceStatusEvent>,
    _request_id: Option<String>,
}
impl ModifyInstanceEventStartTimeOutput {
    /// <p>Information about the event.</p>
    pub fn event(&self) -> ::std::option::Option<&crate::types::InstanceStatusEvent> {
        self.event.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ModifyInstanceEventStartTimeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyInstanceEventStartTimeOutput {
    /// Creates a new builder-style object to manufacture [`ModifyInstanceEventStartTimeOutput`](crate::operation::modify_instance_event_start_time::ModifyInstanceEventStartTimeOutput).
    pub fn builder() -> crate::operation::modify_instance_event_start_time::builders::ModifyInstanceEventStartTimeOutputBuilder {
        crate::operation::modify_instance_event_start_time::builders::ModifyInstanceEventStartTimeOutputBuilder::default()
    }
}

/// A builder for [`ModifyInstanceEventStartTimeOutput`](crate::operation::modify_instance_event_start_time::ModifyInstanceEventStartTimeOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyInstanceEventStartTimeOutputBuilder {
    pub(crate) event: ::std::option::Option<crate::types::InstanceStatusEvent>,
    _request_id: Option<String>,
}
impl ModifyInstanceEventStartTimeOutputBuilder {
    /// <p>Information about the event.</p>
    pub fn event(mut self, input: crate::types::InstanceStatusEvent) -> Self {
        self.event = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the event.</p>
    pub fn set_event(mut self, input: ::std::option::Option<crate::types::InstanceStatusEvent>) -> Self {
        self.event = input;
        self
    }
    /// <p>Information about the event.</p>
    pub fn get_event(&self) -> &::std::option::Option<crate::types::InstanceStatusEvent> {
        &self.event
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifyInstanceEventStartTimeOutput`](crate::operation::modify_instance_event_start_time::ModifyInstanceEventStartTimeOutput).
    pub fn build(self) -> crate::operation::modify_instance_event_start_time::ModifyInstanceEventStartTimeOutput {
        crate::operation::modify_instance_event_start_time::ModifyInstanceEventStartTimeOutput {
            event: self.event,
            _request_id: self._request_id,
        }
    }
}
