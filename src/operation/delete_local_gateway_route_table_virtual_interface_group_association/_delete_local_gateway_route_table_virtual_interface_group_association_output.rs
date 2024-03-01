// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput {
    /// <p>Information about the association.</p>
    pub local_gateway_route_table_virtual_interface_group_association:
        ::std::option::Option<crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation>,
    _request_id: Option<String>,
}
impl DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput {
    /// <p>Information about the association.</p>
    pub fn local_gateway_route_table_virtual_interface_group_association(
        &self,
    ) -> ::std::option::Option<&crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation> {
        self.local_gateway_route_table_virtual_interface_group_association.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput {
    /// Creates a new builder-style object to manufacture [`DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput).
    pub fn builder() -> crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::builders::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutputBuilder{
        crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::builders::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutputBuilder::default()
    }
}

/// A builder for [`DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutputBuilder {
    pub(crate) local_gateway_route_table_virtual_interface_group_association:
        ::std::option::Option<crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation>,
    _request_id: Option<String>,
}
impl DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutputBuilder {
    /// <p>Information about the association.</p>
    pub fn local_gateway_route_table_virtual_interface_group_association(
        mut self,
        input: crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation,
    ) -> Self {
        self.local_gateway_route_table_virtual_interface_group_association = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the association.</p>
    pub fn set_local_gateway_route_table_virtual_interface_group_association(
        mut self,
        input: ::std::option::Option<crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation>,
    ) -> Self {
        self.local_gateway_route_table_virtual_interface_group_association = input;
        self
    }
    /// <p>Information about the association.</p>
    pub fn get_local_gateway_route_table_virtual_interface_group_association(
        &self,
    ) -> &::std::option::Option<crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation> {
        &self.local_gateway_route_table_virtual_interface_group_association
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput`](crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput).
    pub fn build(self) -> crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput{
        crate::operation::delete_local_gateway_route_table_virtual_interface_group_association::DeleteLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput {
            local_gateway_route_table_virtual_interface_group_association: self.local_gateway_route_table_virtual_interface_group_association
            ,
            _request_id: self._request_id,
        }
    }
}
