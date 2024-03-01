// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_export_image_input_input_input(
    input: &crate::operation::export_image::ExportImageInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ExportImage", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClientToken");
    if let Some(var_2) = &input.client_token {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Description");
    if let Some(var_4) = &input.description {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DiskImageFormat");
    if let Some(var_6) = &input.disk_image_format {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DryRun");
    if let Some(var_8) = &input.dry_run {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("ImageId");
    if let Some(var_10) = &input.image_id {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("S3ExportLocation");
    if let Some(var_12) = &input.s3_export_location {
        crate::protocol_serde::shape_export_task_s3_location_request::ser_export_task_s3_location_request(scope_11, var_12)?;
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("RoleName");
    if let Some(var_14) = &input.role_name {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("TagSpecification");
    if let Some(var_16) = &input.tag_specifications {
        let mut list_18 = scope_15.start_list(true, Some("item"));
        for item_17 in var_16 {
            #[allow(unused_mut)]
            let mut entry_19 = list_18.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_19, item_17)?;
        }
        list_18.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
