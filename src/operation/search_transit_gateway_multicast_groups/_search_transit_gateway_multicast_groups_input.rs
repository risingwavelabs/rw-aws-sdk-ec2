// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SearchTransitGatewayMulticastGroupsInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub transit_gateway_multicast_domain_id: ::std::option::Option<::std::string::String>,
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>group-ip-address</code> - The IP address of the transit gateway multicast group.</p> </li>
    /// <li> <p> <code>is-group-member</code> - The resource is a group member. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>is-group-source</code> - The resource is a group source. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>member-type</code> - The member type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>resource-id</code> - The ID of the resource.</p> </li>
    /// <li> <p> <code>resource-type</code> - The type of resource. Valid values are <code>vpc</code> | <code>vpn</code> | <code>direct-connect-gateway</code> | <code>tgw-peering</code>.</p> </li>
    /// <li> <p> <code>source-type</code> - The source type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>transit-gateway-attachment-id</code> - The id of the transit gateway attachment.</p> </li>
    /// </ul>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token for the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl SearchTransitGatewayMulticastGroupsInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_multicast_domain_id.as_deref()
    }
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>group-ip-address</code> - The IP address of the transit gateway multicast group.</p> </li>
    /// <li> <p> <code>is-group-member</code> - The resource is a group member. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>is-group-source</code> - The resource is a group source. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>member-type</code> - The member type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>resource-id</code> - The ID of the resource.</p> </li>
    /// <li> <p> <code>resource-type</code> - The type of resource. Valid values are <code>vpc</code> | <code>vpn</code> | <code>direct-connect-gateway</code> | <code>tgw-peering</code>.</p> </li>
    /// <li> <p> <code>source-type</code> - The source type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>transit-gateway-attachment-id</code> - The id of the transit gateway attachment.</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl SearchTransitGatewayMulticastGroupsInput {
    /// Creates a new builder-style object to manufacture [`SearchTransitGatewayMulticastGroupsInput`](crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsInput).
    pub fn builder() -> crate::operation::search_transit_gateway_multicast_groups::builders::SearchTransitGatewayMulticastGroupsInputBuilder {
        crate::operation::search_transit_gateway_multicast_groups::builders::SearchTransitGatewayMulticastGroupsInputBuilder::default()
    }
}

/// A builder for [`SearchTransitGatewayMulticastGroupsInput`](crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct SearchTransitGatewayMulticastGroupsInputBuilder {
    pub(crate) transit_gateway_multicast_domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl SearchTransitGatewayMulticastGroupsInputBuilder {
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
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>group-ip-address</code> - The IP address of the transit gateway multicast group.</p> </li>
    /// <li> <p> <code>is-group-member</code> - The resource is a group member. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>is-group-source</code> - The resource is a group source. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>member-type</code> - The member type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>resource-id</code> - The ID of the resource.</p> </li>
    /// <li> <p> <code>resource-type</code> - The type of resource. Valid values are <code>vpc</code> | <code>vpn</code> | <code>direct-connect-gateway</code> | <code>tgw-peering</code>.</p> </li>
    /// <li> <p> <code>source-type</code> - The source type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>transit-gateway-attachment-id</code> - The id of the transit gateway attachment.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>group-ip-address</code> - The IP address of the transit gateway multicast group.</p> </li>
    /// <li> <p> <code>is-group-member</code> - The resource is a group member. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>is-group-source</code> - The resource is a group source. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>member-type</code> - The member type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>resource-id</code> - The ID of the resource.</p> </li>
    /// <li> <p> <code>resource-type</code> - The type of resource. Valid values are <code>vpc</code> | <code>vpn</code> | <code>direct-connect-gateway</code> | <code>tgw-peering</code>.</p> </li>
    /// <li> <p> <code>source-type</code> - The source type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>transit-gateway-attachment-id</code> - The id of the transit gateway attachment.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>One or more filters. The possible values are:</p>
    /// <ul>
    /// <li> <p> <code>group-ip-address</code> - The IP address of the transit gateway multicast group.</p> </li>
    /// <li> <p> <code>is-group-member</code> - The resource is a group member. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>is-group-source</code> - The resource is a group source. Valid values are <code>true</code> | <code>false</code>.</p> </li>
    /// <li> <p> <code>member-type</code> - The member type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>resource-id</code> - The ID of the resource.</p> </li>
    /// <li> <p> <code>resource-type</code> - The type of resource. Valid values are <code>vpc</code> | <code>vpn</code> | <code>direct-connect-gateway</code> | <code>tgw-peering</code>.</p> </li>
    /// <li> <p> <code>source-type</code> - The source type. Valid values are <code>igmp</code> | <code>static</code>.</p> </li>
    /// <li> <p> <code>subnet-id</code> - The ID of the subnet.</p> </li>
    /// <li> <p> <code>transit-gateway-attachment-id</code> - The id of the transit gateway attachment.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
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
    /// Consumes the builder and constructs a [`SearchTransitGatewayMulticastGroupsInput`](crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::search_transit_gateway_multicast_groups::SearchTransitGatewayMulticastGroupsInput {
                transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id,
                filters: self.filters,
                max_results: self.max_results,
                next_token: self.next_token,
                dry_run: self.dry_run,
            },
        )
    }
}
