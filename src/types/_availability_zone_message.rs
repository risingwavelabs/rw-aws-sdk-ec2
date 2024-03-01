// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a message about an Availability Zone, Local Zone, or Wavelength Zone.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AvailabilityZoneMessage {
    /// <p>The message about the Availability Zone, Local Zone, or Wavelength Zone.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl AvailabilityZoneMessage {
    /// <p>The message about the Availability Zone, Local Zone, or Wavelength Zone.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl AvailabilityZoneMessage {
    /// Creates a new builder-style object to manufacture [`AvailabilityZoneMessage`](crate::types::AvailabilityZoneMessage).
    pub fn builder() -> crate::types::builders::AvailabilityZoneMessageBuilder {
        crate::types::builders::AvailabilityZoneMessageBuilder::default()
    }
}

/// A builder for [`AvailabilityZoneMessage`](crate::types::AvailabilityZoneMessage).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AvailabilityZoneMessageBuilder {
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl AvailabilityZoneMessageBuilder {
    /// <p>The message about the Availability Zone, Local Zone, or Wavelength Zone.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The message about the Availability Zone, Local Zone, or Wavelength Zone.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The message about the Availability Zone, Local Zone, or Wavelength Zone.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Consumes the builder and constructs a [`AvailabilityZoneMessage`](crate::types::AvailabilityZoneMessage).
    pub fn build(self) -> crate::types::AvailabilityZoneMessage {
        crate::types::AvailabilityZoneMessage { message: self.message }
    }
}
