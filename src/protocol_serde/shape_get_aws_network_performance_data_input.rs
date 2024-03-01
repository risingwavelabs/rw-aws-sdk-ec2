// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_aws_network_performance_data_input_input_input(
    input: &crate::operation::get_aws_network_performance_data::GetAwsNetworkPerformanceDataInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "GetAwsNetworkPerformanceData", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DataQuery");
    if let Some(var_2) = &input.data_queries {
        let mut list_4 = scope_1.start_list(true, None);
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_data_query::ser_data_query(entry_5, item_3)?;
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("StartTime");
    if let Some(var_7) = &input.start_time {
        scope_6.date_time(var_7, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("EndTime");
    if let Some(var_9) = &input.end_time {
        scope_8.date_time(var_9, ::aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("MaxResults");
    if let Some(var_11) = &input.max_results {
        scope_10.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("NextToken");
    if let Some(var_13) = &input.next_token {
        scope_12.string(var_13);
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("DryRun");
    if let Some(var_15) = &input.dry_run {
        scope_14.boolean(*var_15);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
