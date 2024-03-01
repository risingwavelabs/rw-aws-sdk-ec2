// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disable_fast_snapshot_restores_input_input_input(
    input: &crate::operation::disable_fast_snapshot_restores::DisableFastSnapshotRestoresInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DisableFastSnapshotRestores", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AvailabilityZone");
    if let Some(var_2) = &input.availability_zones {
        let mut list_4 = scope_1.start_list(true, Some("AvailabilityZone"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("SourceSnapshotId");
    if let Some(var_7) = &input.source_snapshot_ids {
        let mut list_9 = scope_6.start_list(true, Some("SnapshotId"));
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            entry_10.string(item_8);
        }
        list_9.finish();
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("DryRun");
    if let Some(var_12) = &input.dry_run {
        scope_11.boolean(*var_12);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
