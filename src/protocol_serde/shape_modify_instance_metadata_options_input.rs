// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_instance_metadata_options_input_input_input(
    input: &crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyInstanceMetadataOptions", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("InstanceId");
    if let Some(var_2) = &input.instance_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("HttpTokens");
    if let Some(var_4) = &input.http_tokens {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("HttpPutResponseHopLimit");
    if let Some(var_6) = &input.http_put_response_hop_limit {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("HttpEndpoint");
    if let Some(var_8) = &input.http_endpoint {
        scope_7.string(var_8.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DryRun");
    if let Some(var_10) = &input.dry_run {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("HttpProtocolIpv6");
    if let Some(var_12) = &input.http_protocol_ipv6 {
        scope_11.string(var_12.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("InstanceMetadataTags");
    if let Some(var_14) = &input.instance_metadata_tags {
        scope_13.string(var_14.as_str());
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
