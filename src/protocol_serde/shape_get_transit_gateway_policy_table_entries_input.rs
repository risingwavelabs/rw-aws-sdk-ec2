// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_transit_gateway_policy_table_entries_input_input_input(
    input: &crate::operation::get_transit_gateway_policy_table_entries::GetTransitGatewayPolicyTableEntriesInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetTransitGatewayPolicyTableEntries", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TransitGatewayPolicyTableId");
    if let Some(var_2) = &input.transit_gateway_policy_table_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Filter");
    if let Some(var_4) = &input.filters {
        let mut list_6 = scope_3.start_list(true, Some("Filter"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_7, item_5)?;
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("MaxResults");
    if let Some(var_9) = &input.max_results {
        scope_8.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_9).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("NextToken");
    if let Some(var_11) = &input.next_token {
        scope_10.string(var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("DryRun");
    if let Some(var_13) = &input.dry_run {
        scope_12.boolean(*var_13);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
