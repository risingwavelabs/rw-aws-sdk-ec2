// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_host_reservation_offerings_input_input_input(
    input: &crate::operation::describe_host_reservation_offerings::DescribeHostReservationOfferingsInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeHostReservationOfferings", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Filter");
    if let Some(var_2) = &input.filter {
        let mut list_4 = scope_1.start_list(true, Some("Filter"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_5, item_3)?;
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("MaxDuration");
    if let Some(var_7) = &input.max_duration {
        scope_6.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
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
    let mut scope_10 = writer.prefix("MinDuration");
    if let Some(var_11) = &input.min_duration {
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
    let mut scope_14 = writer.prefix("OfferingId");
    if let Some(var_15) = &input.offering_id {
        scope_14.string(var_15);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}