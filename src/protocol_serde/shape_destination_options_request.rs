// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_destination_options_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::DestinationOptionsRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("FileFormat");
    if let Some(var_2) = &input.file_format {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("HiveCompatiblePartitions");
    if let Some(var_4) = &input.hive_compatible_partitions {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("PerHourPartition");
    if let Some(var_6) = &input.per_hour_partition {
        scope_5.boolean(*var_6);
    }
    Ok(())
}
