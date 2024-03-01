// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_volume_attribute_input_input_input(
    input: &crate::operation::modify_volume_attribute::ModifyVolumeAttributeInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyVolumeAttribute", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AutoEnableIO");
    if let Some(var_2) = &input.auto_enable_io {
        crate::protocol_serde::shape_attribute_boolean_value::ser_attribute_boolean_value(scope_1, var_2)?;
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VolumeId");
    if let Some(var_4) = &input.volume_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DryRun");
    if let Some(var_6) = &input.dry_run {
        scope_5.boolean(*var_6);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}