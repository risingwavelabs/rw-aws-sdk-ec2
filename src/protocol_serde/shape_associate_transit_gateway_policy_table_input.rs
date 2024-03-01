// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_transit_gateway_policy_table_input_input_input(
    input: &crate::operation::associate_transit_gateway_policy_table::AssociateTransitGatewayPolicyTableInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AssociateTransitGatewayPolicyTable", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TransitGatewayPolicyTableId");
    if let Some(var_2) = &input.transit_gateway_policy_table_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TransitGatewayAttachmentId");
    if let Some(var_4) = &input.transit_gateway_attachment_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DryRun");
    if let Some(var_6) = &input.dry_run {
        scope_5.boolean(*var_6);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
