// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the monitoring for the instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LaunchTemplatesMonitoringRequest {
    /// <p>Specify <code>true</code> to enable detailed monitoring. Otherwise, basic monitoring is enabled.</p>
    pub enabled: ::std::option::Option<bool>,
}
impl LaunchTemplatesMonitoringRequest {
    /// <p>Specify <code>true</code> to enable detailed monitoring. Otherwise, basic monitoring is enabled.</p>
    pub fn enabled(&self) -> ::std::option::Option<bool> {
        self.enabled
    }
}
impl LaunchTemplatesMonitoringRequest {
    /// Creates a new builder-style object to manufacture [`LaunchTemplatesMonitoringRequest`](crate::types::LaunchTemplatesMonitoringRequest).
    pub fn builder() -> crate::types::builders::LaunchTemplatesMonitoringRequestBuilder {
        crate::types::builders::LaunchTemplatesMonitoringRequestBuilder::default()
    }
}

/// A builder for [`LaunchTemplatesMonitoringRequest`](crate::types::LaunchTemplatesMonitoringRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LaunchTemplatesMonitoringRequestBuilder {
    pub(crate) enabled: ::std::option::Option<bool>,
}
impl LaunchTemplatesMonitoringRequestBuilder {
    /// <p>Specify <code>true</code> to enable detailed monitoring. Otherwise, basic monitoring is enabled.</p>
    pub fn enabled(mut self, input: bool) -> Self {
        self.enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specify <code>true</code> to enable detailed monitoring. Otherwise, basic monitoring is enabled.</p>
    pub fn set_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enabled = input;
        self
    }
    /// <p>Specify <code>true</code> to enable detailed monitoring. Otherwise, basic monitoring is enabled.</p>
    pub fn get_enabled(&self) -> &::std::option::Option<bool> {
        &self.enabled
    }
    /// Consumes the builder and constructs a [`LaunchTemplatesMonitoringRequest`](crate::types::LaunchTemplatesMonitoringRequest).
    pub fn build(self) -> crate::types::LaunchTemplatesMonitoringRequest {
        crate::types::LaunchTemplatesMonitoringRequest { enabled: self.enabled }
    }
}