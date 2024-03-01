// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_local_gateway_route_table_vpc_association_input_input_input(
    input: &crate::operation::delete_local_gateway_route_table_vpc_association::DeleteLocalGatewayRouteTableVpcAssociationInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DeleteLocalGatewayRouteTableVpcAssociation", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LocalGatewayRouteTableVpcAssociationId");
    if let Some(var_2) = &input.local_gateway_route_table_vpc_association_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DryRun");
    if let Some(var_4) = &input.dry_run {
        scope_3.boolean(*var_4);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
