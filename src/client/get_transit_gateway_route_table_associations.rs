// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTransitGatewayRouteTableAssociations`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_route_table_id(impl Into<String>)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::transit_gateway_route_table_id) / [`set_transit_gateway_route_table_id(Option<String>)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::set_transit_gateway_route_table_id):<br>required: **true**<br><p>The ID of the transit gateway route table.</p><br>
    ///   - [`filters(Filter)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::set_filters):<br>required: **false**<br><p>One or more filters. The possible values are:</p>  <ul>   <li> <p> <code>resource-id</code> - The ID of the resource.</p> </li>   <li> <p> <code>resource-type</code> - The resource type. Valid values are <code>vpc</code> | <code>vpn</code> | <code>direct-connect-gateway</code> | <code>peering</code> | <code>connect</code>.</p> </li>   <li> <p> <code>transit-gateway-attachment-id</code> - The ID of the attachment.</p> </li>  </ul><br>
    ///   - [`max_results(i32)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`GetTransitGatewayRouteTableAssociationsOutput`](crate::operation::get_transit_gateway_route_table_associations::GetTransitGatewayRouteTableAssociationsOutput) with field(s):
    ///   - [`associations(Option<Vec::<TransitGatewayRouteTableAssociation>>)`](crate::operation::get_transit_gateway_route_table_associations::GetTransitGatewayRouteTableAssociationsOutput::associations): <p>Information about the associations.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_transit_gateway_route_table_associations::GetTransitGatewayRouteTableAssociationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<GetTransitGatewayRouteTableAssociationsError>`](crate::operation::get_transit_gateway_route_table_associations::GetTransitGatewayRouteTableAssociationsError)
    pub fn get_transit_gateway_route_table_associations(
        &self,
    ) -> crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder {
        crate::operation::get_transit_gateway_route_table_associations::builders::GetTransitGatewayRouteTableAssociationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}