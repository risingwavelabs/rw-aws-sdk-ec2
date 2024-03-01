// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_ipam_resource_discovery_input_input_input(
    input: &crate::operation::delete_ipam_resource_discovery::DeleteIpamResourceDiscoveryInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DeleteIpamResourceDiscovery", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("IpamResourceDiscoveryId");
    if let Some(var_4) = &input.ipam_resource_discovery_id {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
