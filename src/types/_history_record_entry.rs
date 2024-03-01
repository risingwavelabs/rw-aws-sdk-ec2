// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an event in the history of an EC2 Fleet.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HistoryRecordEntry {
    /// <p>Information about the event.</p>
    pub event_information: ::std::option::Option<crate::types::EventInformation>,
    /// <p>The event type.</p>
    pub event_type: ::std::option::Option<crate::types::FleetEventType>,
    /// <p>The date and time of the event, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl HistoryRecordEntry {
    /// <p>Information about the event.</p>
    pub fn event_information(&self) -> ::std::option::Option<&crate::types::EventInformation> {
        self.event_information.as_ref()
    }
    /// <p>The event type.</p>
    pub fn event_type(&self) -> ::std::option::Option<&crate::types::FleetEventType> {
        self.event_type.as_ref()
    }
    /// <p>The date and time of the event, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn timestamp(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.timestamp.as_ref()
    }
}
impl HistoryRecordEntry {
    /// Creates a new builder-style object to manufacture [`HistoryRecordEntry`](crate::types::HistoryRecordEntry).
    pub fn builder() -> crate::types::builders::HistoryRecordEntryBuilder {
        crate::types::builders::HistoryRecordEntryBuilder::default()
    }
}

/// A builder for [`HistoryRecordEntry`](crate::types::HistoryRecordEntry).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct HistoryRecordEntryBuilder {
    pub(crate) event_information: ::std::option::Option<crate::types::EventInformation>,
    pub(crate) event_type: ::std::option::Option<crate::types::FleetEventType>,
    pub(crate) timestamp: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl HistoryRecordEntryBuilder {
    /// <p>Information about the event.</p>
    pub fn event_information(mut self, input: crate::types::EventInformation) -> Self {
        self.event_information = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the event.</p>
    pub fn set_event_information(mut self, input: ::std::option::Option<crate::types::EventInformation>) -> Self {
        self.event_information = input;
        self
    }
    /// <p>Information about the event.</p>
    pub fn get_event_information(&self) -> &::std::option::Option<crate::types::EventInformation> {
        &self.event_information
    }
    /// <p>The event type.</p>
    pub fn event_type(mut self, input: crate::types::FleetEventType) -> Self {
        self.event_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The event type.</p>
    pub fn set_event_type(mut self, input: ::std::option::Option<crate::types::FleetEventType>) -> Self {
        self.event_type = input;
        self
    }
    /// <p>The event type.</p>
    pub fn get_event_type(&self) -> &::std::option::Option<crate::types::FleetEventType> {
        &self.event_type
    }
    /// <p>The date and time of the event, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn timestamp(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.timestamp = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time of the event, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn set_timestamp(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.timestamp = input;
        self
    }
    /// <p>The date and time of the event, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn get_timestamp(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.timestamp
    }
    /// Consumes the builder and constructs a [`HistoryRecordEntry`](crate::types::HistoryRecordEntry).
    pub fn build(self) -> crate::types::HistoryRecordEntry {
        crate::types::HistoryRecordEntry {
            event_information: self.event_information,
            event_type: self.event_type,
            timestamp: self.timestamp,
        }
    }
}