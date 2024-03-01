// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_copy_snapshot_input_input_input(
    input: &crate::operation::copy_snapshot::CopySnapshotInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CopySnapshot", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Description");
    if let Some(var_2) = &input.description {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DestinationOutpostArn");
    if let Some(var_4) = &input.destination_outpost_arn {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DestinationRegion");
    if let Some(var_6) = &input.destination_region {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Encrypted");
    if let Some(var_8) = &input.encrypted {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("KmsKeyId");
    if let Some(var_10) = &input.kms_key_id {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("PresignedUrl");
    if let Some(var_12) = &input.presigned_url {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("SourceRegion");
    if let Some(var_14) = &input.source_region {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("SourceSnapshotId");
    if let Some(var_16) = &input.source_snapshot_id {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("TagSpecification");
    if let Some(var_18) = &input.tag_specifications {
        let mut list_20 = scope_17.start_list(true, Some("item"));
        for item_19 in var_18 {
            #[allow(unused_mut)]
            let mut entry_21 = list_20.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_21, item_19)?;
        }
        list_20.finish();
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("DryRun");
    if let Some(var_23) = &input.dry_run {
        scope_22.boolean(*var_23);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}