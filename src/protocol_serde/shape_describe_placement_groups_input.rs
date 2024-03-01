// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_placement_groups_input_input_input(
    input: &crate::operation::describe_placement_groups::DescribePlacementGroupsInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribePlacementGroups", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Filter");
    if let Some(var_2) = &input.filters {
        let mut list_4 = scope_1.start_list(true, Some("Filter"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_5, item_3)?;
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("DryRun");
    if let Some(var_7) = &input.dry_run {
        scope_6.boolean(*var_7);
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("GroupName");
    if let Some(var_9) = &input.group_names {
        let mut list_11 = scope_8.start_list(true, None);
        for item_10 in var_9 {
            #[allow(unused_mut)]
            let mut entry_12 = list_11.entry();
            entry_12.string(item_10);
        }
        list_11.finish();
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("GroupId");
    if let Some(var_14) = &input.group_ids {
        let mut list_16 = scope_13.start_list(true, Some("GroupId"));
        for item_15 in var_14 {
            #[allow(unused_mut)]
            let mut entry_17 = list_16.entry();
            entry_17.string(item_15);
        }
        list_16.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}