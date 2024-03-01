// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_packet_header_statement_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::PacketHeaderStatementRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SourceAddress");
    if let Some(var_2) = &input.source_addresses {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("DestinationAddress");
    if let Some(var_7) = &input.destination_addresses {
        let mut list_9 = scope_6.start_list(true, Some("item"));
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            entry_10.string(item_8);
        }
        list_9.finish();
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("SourcePort");
    if let Some(var_12) = &input.source_ports {
        let mut list_14 = scope_11.start_list(true, Some("item"));
        for item_13 in var_12 {
            #[allow(unused_mut)]
            let mut entry_15 = list_14.entry();
            entry_15.string(item_13);
        }
        list_14.finish();
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("DestinationPort");
    if let Some(var_17) = &input.destination_ports {
        let mut list_19 = scope_16.start_list(true, Some("item"));
        for item_18 in var_17 {
            #[allow(unused_mut)]
            let mut entry_20 = list_19.entry();
            entry_20.string(item_18);
        }
        list_19.finish();
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("SourcePrefixList");
    if let Some(var_22) = &input.source_prefix_lists {
        let mut list_24 = scope_21.start_list(true, Some("item"));
        for item_23 in var_22 {
            #[allow(unused_mut)]
            let mut entry_25 = list_24.entry();
            entry_25.string(item_23);
        }
        list_24.finish();
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("DestinationPrefixList");
    if let Some(var_27) = &input.destination_prefix_lists {
        let mut list_29 = scope_26.start_list(true, Some("item"));
        for item_28 in var_27 {
            #[allow(unused_mut)]
            let mut entry_30 = list_29.entry();
            entry_30.string(item_28);
        }
        list_29.finish();
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("Protocol");
    if let Some(var_32) = &input.protocols {
        let mut list_34 = scope_31.start_list(true, Some("item"));
        for item_33 in var_32 {
            #[allow(unused_mut)]
            let mut entry_35 = list_34.entry();
            entry_35.string(item_33.as_str());
        }
        list_34.finish();
    }
    Ok(())
}