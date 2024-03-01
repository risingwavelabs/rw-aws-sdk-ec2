// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Options for logging VPN tunnel activity.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VpnTunnelLogOptionsSpecification {
    /// <p>Options for sending VPN tunnel logs to CloudWatch.</p>
    pub cloud_watch_log_options: ::std::option::Option<crate::types::CloudWatchLogOptionsSpecification>,
}
impl VpnTunnelLogOptionsSpecification {
    /// <p>Options for sending VPN tunnel logs to CloudWatch.</p>
    pub fn cloud_watch_log_options(&self) -> ::std::option::Option<&crate::types::CloudWatchLogOptionsSpecification> {
        self.cloud_watch_log_options.as_ref()
    }
}
impl VpnTunnelLogOptionsSpecification {
    /// Creates a new builder-style object to manufacture [`VpnTunnelLogOptionsSpecification`](crate::types::VpnTunnelLogOptionsSpecification).
    pub fn builder() -> crate::types::builders::VpnTunnelLogOptionsSpecificationBuilder {
        crate::types::builders::VpnTunnelLogOptionsSpecificationBuilder::default()
    }
}

/// A builder for [`VpnTunnelLogOptionsSpecification`](crate::types::VpnTunnelLogOptionsSpecification).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VpnTunnelLogOptionsSpecificationBuilder {
    pub(crate) cloud_watch_log_options: ::std::option::Option<crate::types::CloudWatchLogOptionsSpecification>,
}
impl VpnTunnelLogOptionsSpecificationBuilder {
    /// <p>Options for sending VPN tunnel logs to CloudWatch.</p>
    pub fn cloud_watch_log_options(mut self, input: crate::types::CloudWatchLogOptionsSpecification) -> Self {
        self.cloud_watch_log_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Options for sending VPN tunnel logs to CloudWatch.</p>
    pub fn set_cloud_watch_log_options(mut self, input: ::std::option::Option<crate::types::CloudWatchLogOptionsSpecification>) -> Self {
        self.cloud_watch_log_options = input;
        self
    }
    /// <p>Options for sending VPN tunnel logs to CloudWatch.</p>
    pub fn get_cloud_watch_log_options(&self) -> &::std::option::Option<crate::types::CloudWatchLogOptionsSpecification> {
        &self.cloud_watch_log_options
    }
    /// Consumes the builder and constructs a [`VpnTunnelLogOptionsSpecification`](crate::types::VpnTunnelLogOptionsSpecification).
    pub fn build(self) -> crate::types::VpnTunnelLogOptionsSpecification {
        crate::types::VpnTunnelLogOptionsSpecification {
            cloud_watch_log_options: self.cloud_watch_log_options,
        }
    }
}