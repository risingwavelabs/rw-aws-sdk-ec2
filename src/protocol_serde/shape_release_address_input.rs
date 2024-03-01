// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_release_address_input_input_input(
    input: &crate::operation::release_address::ReleaseAddressInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ReleaseAddress", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AllocationId");
    if let Some(var_2) = &input.allocation_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PublicIp");
    if let Some(var_4) = &input.public_ip {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("NetworkBorderGroup");
    if let Some(var_6) = &input.network_border_group {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DryRun");
    if let Some(var_8) = &input.dry_run {
        scope_7.boolean(*var_8);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
