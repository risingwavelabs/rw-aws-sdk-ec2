// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_provision_ipam_byoasn_input_input_input(
    input: &crate::operation::provision_ipam_byoasn::ProvisionIpamByoasnInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ProvisionIpamByoasn", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("IpamId");
    if let Some(var_4) = &input.ipam_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Asn");
    if let Some(var_6) = &input.asn {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("AsnAuthorizationContext");
    if let Some(var_8) = &input.asn_authorization_context {
        crate::protocol_serde::shape_asn_authorization_context::ser_asn_authorization_context(scope_7, var_8)?;
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
