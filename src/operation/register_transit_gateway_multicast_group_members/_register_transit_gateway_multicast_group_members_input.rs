// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterTransitGatewayMulticastGroupMembersInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub transit_gateway_multicast_domain_id: ::std::option::Option<::std::string::String>,
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub group_ip_address: ::std::option::Option<::std::string::String>,
    /// <p>The group members' network interface IDs to register with the transit gateway multicast group.</p>
    pub network_interface_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl RegisterTransitGatewayMulticastGroupMembersInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_multicast_domain_id.as_deref()
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn group_ip_address(&self) -> ::std::option::Option<&str> {
        self.group_ip_address.as_deref()
    }
    /// <p>The group members' network interface IDs to register with the transit gateway multicast group.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.network_interface_ids.is_none()`.
    pub fn network_interface_ids(&self) -> &[::std::string::String] {
        self.network_interface_ids.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl RegisterTransitGatewayMulticastGroupMembersInput {
    /// Creates a new builder-style object to manufacture [`RegisterTransitGatewayMulticastGroupMembersInput`](crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersInput).
    pub fn builder(
    ) -> crate::operation::register_transit_gateway_multicast_group_members::builders::RegisterTransitGatewayMulticastGroupMembersInputBuilder {
        crate::operation::register_transit_gateway_multicast_group_members::builders::RegisterTransitGatewayMulticastGroupMembersInputBuilder::default(
        )
    }
}

/// A builder for [`RegisterTransitGatewayMulticastGroupMembersInput`](crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RegisterTransitGatewayMulticastGroupMembersInputBuilder {
    pub(crate) transit_gateway_multicast_domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) group_ip_address: ::std::option::Option<::std::string::String>,
    pub(crate) network_interface_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl RegisterTransitGatewayMulticastGroupMembersInputBuilder {
    /// <p>The ID of the transit gateway multicast domain.</p>
    /// This field is required.
    pub fn transit_gateway_multicast_domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_multicast_domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_multicast_domain_id = input;
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn get_transit_gateway_multicast_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_multicast_domain_id
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn group_ip_address(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_ip_address = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn set_group_ip_address(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_ip_address = input;
        self
    }
    /// <p>The IP address assigned to the transit gateway multicast group.</p>
    pub fn get_group_ip_address(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_ip_address
    }
    /// Appends an item to `network_interface_ids`.
    ///
    /// To override the contents of this collection use [`set_network_interface_ids`](Self::set_network_interface_ids).
    ///
    /// <p>The group members' network interface IDs to register with the transit gateway multicast group.</p>
    pub fn network_interface_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.network_interface_ids.unwrap_or_default();
        v.push(input.into());
        self.network_interface_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The group members' network interface IDs to register with the transit gateway multicast group.</p>
    pub fn set_network_interface_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.network_interface_ids = input;
        self
    }
    /// <p>The group members' network interface IDs to register with the transit gateway multicast group.</p>
    pub fn get_network_interface_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.network_interface_ids
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`RegisterTransitGatewayMulticastGroupMembersInput`](crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::register_transit_gateway_multicast_group_members::RegisterTransitGatewayMulticastGroupMembersInput {
                transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id,
                group_ip_address: self.group_ip_address,
                network_interface_ids: self.network_interface_ids,
                dry_run: self.dry_run,
            },
        )
    }
}
