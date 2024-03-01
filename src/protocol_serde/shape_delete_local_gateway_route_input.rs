// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_local_gateway_route_input_input_input(
    input: &crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DeleteLocalGatewayRoute", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DestinationCidrBlock");
    if let Some(var_2) = &input.destination_cidr_block {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("LocalGatewayRouteTableId");
    if let Some(var_4) = &input.local_gateway_route_table_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DryRun");
    if let Some(var_6) = &input.dry_run {
        scope_5.boolean(*var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DestinationPrefixListId");
    if let Some(var_8) = &input.destination_prefix_list_id {
        scope_7.string(var_8);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}