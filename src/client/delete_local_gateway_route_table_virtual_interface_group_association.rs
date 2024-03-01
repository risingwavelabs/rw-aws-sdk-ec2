// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociation`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::builders::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`local_gateway_route_table_virtual_interface_group_association_id(impl Into<String>)`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::builders::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder::local_gateway_route_table_virtual_interface_group_association_id) / [`set_local_gateway_route_table_virtual_interface_group_association_id(Option<String>)`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::builders::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder::set_local_gateway_route_table_virtual_interface_group_association_id):<br>required: **true**<br><p> The ID of the local gateway route table virtual interface group association. </p><br>
    ///   - [`dry_run(bool)`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::builders::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::builders::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput) with field(s):
    ///   - [`local_gateway_route_table_virtual_interface_group_association(Option<LocalGatewayRouteTableVirtualInterfaceGroupAssociation>)`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput::local_gateway_route_table_virtual_interface_group_association): <p>Information about the association.</p>
    /// - On failure, responds with [`SdkError<DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationError>`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationError)
    pub fn delete_local_gateway_route_table_virtual_interface_group_association(&self) -> crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::builders::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder{
        crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::builders::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder::new(self.handle.clone())
    }
}
