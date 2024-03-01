// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeLocalGatewayRouteTables`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`local_gateway_route_table_ids(impl Into<String>)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::local_gateway_route_table_ids) / [`set_local_gateway_route_table_ids(Option<Vec::<String>>)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::set_local_gateway_route_table_ids):<br>required: **false**<br><p>The IDs of the local gateway route tables.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::set_filters):<br>required: **false**<br><p>One or more filters.</p>  <ul>   <li> <p> <code>local-gateway-id</code> - The ID of a local gateway.</p> </li>   <li> <p> <code>local-gateway-route-table-arn</code> - The Amazon Resource Name (ARN) of the local gateway route table.</p> </li>   <li> <p> <code>local-gateway-route-table-id</code> - The ID of a local gateway route table.</p> </li>   <li> <p> <code>outpost-arn</code> - The Amazon Resource Name (ARN) of the Outpost.</p> </li>   <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the local gateway route table.</p> </li>   <li> <p> <code>state</code> - The state of the local gateway route table.</p> </li>  </ul><br>
    ///   - [`max_results(i32)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DescribeLocalGatewayRouteTablesOutput`](crate::operation::describe_local_gateway_route_tables::DescribeLocalGatewayRouteTablesOutput) with field(s):
    ///   - [`local_gateway_route_tables(Option<Vec::<LocalGatewayRouteTable>>)`](crate::operation::describe_local_gateway_route_tables::DescribeLocalGatewayRouteTablesOutput::local_gateway_route_tables): <p>Information about the local gateway route tables.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_local_gateway_route_tables::DescribeLocalGatewayRouteTablesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeLocalGatewayRouteTablesError>`](crate::operation::describe_local_gateway_route_tables::DescribeLocalGatewayRouteTablesError)
    pub fn describe_local_gateway_route_tables(
        &self,
    ) -> crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder {
        crate::operation::describe_local_gateway_route_tables::builders::DescribeLocalGatewayRouteTablesFluentBuilder::new(self.handle.clone())
    }
}
