// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_instance_topology_input_input_input(
    input: &crate::operation::describe_instance_topology::DescribeInstanceTopologyInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeInstanceTopology", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("NextToken");
    if let Some(var_4) = &input.next_token {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MaxResults");
    if let Some(var_6) = &input.max_results {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("InstanceId");
    if let Some(var_8) = &input.instance_ids {
        let mut list_10 = scope_7.start_list(true, None);
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            entry_11.string(item_9);
        }
        list_10.finish();
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("GroupName");
    if let Some(var_13) = &input.group_names {
        let mut list_15 = scope_12.start_list(true, None);
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            entry_16.string(item_14);
        }
        list_15.finish();
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("Filter");
    if let Some(var_18) = &input.filters {
        let mut list_20 = scope_17.start_list(true, Some("Filter"));
        for item_19 in var_18 {
            #[allow(unused_mut)]
            let mut entry_21 = list_20.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_21, item_19)?;
        }
        list_20.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
