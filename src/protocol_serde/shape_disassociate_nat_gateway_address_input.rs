// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_nat_gateway_address_input_input_input(
    input: &crate::operation::disassociate_nat_gateway_address::DisassociateNatGatewayAddressInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DisassociateNatGatewayAddress", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("NatGatewayId");
    if let Some(var_2) = &input.nat_gateway_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AssociationId");
    if let Some(var_4) = &input.association_ids {
        let mut list_6 = scope_3.start_list(true, Some("item"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("MaxDrainDurationSeconds");
    if let Some(var_9) = &input.max_drain_duration_seconds {
        scope_8.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("DryRun");
    if let Some(var_11) = &input.dry_run {
        scope_10.boolean(*var_11);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
