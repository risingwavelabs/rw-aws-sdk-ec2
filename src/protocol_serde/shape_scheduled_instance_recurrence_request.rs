// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_scheduled_instance_recurrence_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ScheduledInstanceRecurrenceRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Frequency");
    if let Some(var_2) = &input.frequency {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Interval");
    if let Some(var_4) = &input.interval {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("OccurrenceDay");
    if let Some(var_6) = &input.occurrence_days {
        let mut list_8 = scope_5.start_list(true, Some("OccurenceDay"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            entry_9.number(
                #[allow(clippy::useless_conversion)]
                ::aws_smithy_types::Number::NegInt((*item_7).into()),
            );
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("OccurrenceRelativeToEnd");
    if let Some(var_11) = &input.occurrence_relative_to_end {
        scope_10.boolean(*var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("OccurrenceUnit");
    if let Some(var_13) = &input.occurrence_unit {
        scope_12.string(var_13);
    }
    Ok(())
}
