// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_launch_template_instance_metadata_options_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::LaunchTemplateInstanceMetadataOptionsRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("HttpTokens");
    if let Some(var_2) = &input.http_tokens {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("HttpPutResponseHopLimit");
    if let Some(var_4) = &input.http_put_response_hop_limit {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("HttpEndpoint");
    if let Some(var_6) = &input.http_endpoint {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("HttpProtocolIpv6");
    if let Some(var_8) = &input.http_protocol_ipv6 {
        scope_7.string(var_8.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("InstanceMetadataTags");
    if let Some(var_10) = &input.instance_metadata_tags {
        scope_9.string(var_10.as_str());
    }
    Ok(())
}
