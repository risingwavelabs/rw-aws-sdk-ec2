// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLocalGatewayRouteTable`](crate::operation::delete_local_gateway_route_table::builders::DeleteLocalGatewayRouteTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`local_gateway_route_table_id(impl Into<String>)`](crate::operation::delete_local_gateway_route_table::builders::DeleteLocalGatewayRouteTableFluentBuilder::local_gateway_route_table_id) / [`set_local_gateway_route_table_id(Option<String>)`](crate::operation::delete_local_gateway_route_table::builders::DeleteLocalGatewayRouteTableFluentBuilder::set_local_gateway_route_table_id):<br>required: **true**<br><p> The ID of the local gateway route table. </p><br>
    ///   - [`dry_run(bool)`](crate::operation::delete_local_gateway_route_table::builders::DeleteLocalGatewayRouteTableFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_local_gateway_route_table::builders::DeleteLocalGatewayRouteTableFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DeleteLocalGatewayRouteTableOutput`](crate::operation::delete_local_gateway_route_table::DeleteLocalGatewayRouteTableOutput) with field(s):
    ///   - [`local_gateway_route_table(Option<LocalGatewayRouteTable>)`](crate::operation::delete_local_gateway_route_table::DeleteLocalGatewayRouteTableOutput::local_gateway_route_table): <p>Information about the local gateway route table.</p>
    /// - On failure, responds with [`SdkError<DeleteLocalGatewayRouteTableError>`](crate::operation::delete_local_gateway_route_table::DeleteLocalGatewayRouteTableError)
    pub fn delete_local_gateway_route_table(
        &self,
    ) -> crate::operation::delete_local_gateway_route_table::builders::DeleteLocalGatewayRouteTableFluentBuilder {
        crate::operation::delete_local_gateway_route_table::builders::DeleteLocalGatewayRouteTableFluentBuilder::new(self.handle.clone())
    }
}