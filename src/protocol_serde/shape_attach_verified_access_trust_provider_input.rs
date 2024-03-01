// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attach_verified_access_trust_provider_input_input_input(
    input: &crate::operation::attach_verified_access_trust_provider::AttachVerifiedAccessTrustProviderInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "AttachVerifiedAccessTrustProvider", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("VerifiedAccessInstanceId");
    if let Some(var_2) = &input.verified_access_instance_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VerifiedAccessTrustProviderId");
    if let Some(var_4) = &input.verified_access_trust_provider_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ClientToken");
    if let Some(var_6) = &input.client_token {
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