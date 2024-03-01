// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_ipam_resource_discovery_input_input_input(
    input: &crate::operation::create_ipam_resource_discovery::CreateIpamResourceDiscoveryInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateIpamResourceDiscovery", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Description");
    if let Some(var_4) = &input.description {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("OperatingRegion");
    if let Some(var_6) = &input.operating_regions {
        let mut list_8 = scope_5.start_list(true, None);
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_add_ipam_operating_region::ser_add_ipam_operating_region(entry_9, item_7)?;
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("TagSpecification");
    if let Some(var_11) = &input.tag_specifications {
        let mut list_13 = scope_10.start_list(true, Some("item"));
        for item_12 in var_11 {
            #[allow(unused_mut)]
            let mut entry_14 = list_13.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_14, item_12)?;
        }
        list_13.finish();
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("ClientToken");
    if let Some(var_16) = &input.client_token {
        scope_15.string(var_16);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
