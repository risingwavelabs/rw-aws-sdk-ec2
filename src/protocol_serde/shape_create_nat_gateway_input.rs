// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_nat_gateway_input_input_input(
    input: &crate::operation::create_nat_gateway::CreateNatGatewayInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateNatGateway", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AllocationId");
    if let Some(var_2) = &input.allocation_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ClientToken");
    if let Some(var_4) = &input.client_token {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DryRun");
    if let Some(var_6) = &input.dry_run {
        scope_5.boolean(*var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("SubnetId");
    if let Some(var_8) = &input.subnet_id {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("TagSpecification");
    if let Some(var_10) = &input.tag_specifications {
        let mut list_12 = scope_9.start_list(true, Some("item"));
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_13, item_11)?;
        }
        list_12.finish();
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("ConnectivityType");
    if let Some(var_15) = &input.connectivity_type {
        scope_14.string(var_15.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("PrivateIpAddress");
    if let Some(var_17) = &input.private_ip_address {
        scope_16.string(var_17);
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("SecondaryAllocationId");
    if let Some(var_19) = &input.secondary_allocation_ids {
        let mut list_21 = scope_18.start_list(true, Some("AllocationId"));
        for item_20 in var_19 {
            #[allow(unused_mut)]
            let mut entry_22 = list_21.entry();
            entry_22.string(item_20);
        }
        list_21.finish();
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("SecondaryPrivateIpAddress");
    if let Some(var_24) = &input.secondary_private_ip_addresses {
        let mut list_26 = scope_23.start_list(true, Some("item"));
        for item_25 in var_24 {
            #[allow(unused_mut)]
            let mut entry_27 = list_26.entry();
            entry_27.string(item_25);
        }
        list_26.finish();
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("SecondaryPrivateIpAddressCount");
    if let Some(var_29) = &input.secondary_private_ip_address_count {
        scope_28.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_29).into()),
        );
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
