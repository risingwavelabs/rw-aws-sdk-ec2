// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_transit_gateway_prefix_list_reference_input_input_input(
    input: &crate::operation::modify_transit_gateway_prefix_list_reference::ModifyTransitGatewayPrefixListReferenceInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyTransitGatewayPrefixListReference", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TransitGatewayRouteTableId");
    if let Some(var_2) = &input.transit_gateway_route_table_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PrefixListId");
    if let Some(var_4) = &input.prefix_list_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TransitGatewayAttachmentId");
    if let Some(var_6) = &input.transit_gateway_attachment_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Blackhole");
    if let Some(var_8) = &input.blackhole {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DryRun");
    if let Some(var_10) = &input.dry_run {
        scope_9.boolean(*var_10);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}