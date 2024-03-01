// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_federated_authentication_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::FederatedAuthenticationRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SAMLProviderArn");
    if let Some(var_2) = &input.saml_provider_arn {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SelfServiceSAMLProviderArn");
    if let Some(var_4) = &input.self_service_saml_provider_arn {
        scope_3.string(var_4);
    }
    Ok(())
}