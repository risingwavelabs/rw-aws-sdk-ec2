// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_placement_group_input_input_input(
    input: &crate::operation::create_placement_group::CreatePlacementGroupInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreatePlacementGroup", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("GroupName");
    if let Some(var_4) = &input.group_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Strategy");
    if let Some(var_6) = &input.strategy {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("PartitionCount");
    if let Some(var_8) = &input.partition_count {
        scope_7.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
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
    let mut scope_14 = writer.prefix("SpreadLevel");
    if let Some(var_15) = &input.spread_level {
        scope_14.string(var_15.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}