// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_launch_template_instance_metadata_options(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LaunchTemplateInstanceMetadataOptions, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LaunchTemplateInstanceMetadataOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("state") /* State com.amazonaws.ec2#LaunchTemplateInstanceMetadataOptions$State */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::LaunchTemplateInstanceMetadataOptionsState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LaunchTemplateInstanceMetadataOptionsState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_1);
            }
            ,
            s if s.matches("httpTokens") /* HttpTokens com.amazonaws.ec2#LaunchTemplateInstanceMetadataOptions$HttpTokens */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::LaunchTemplateHttpTokensState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LaunchTemplateHttpTokensState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_http_tokens(var_2);
            }
            ,
            s if s.matches("httpPutResponseHopLimit") /* HttpPutResponseHopLimit com.amazonaws.ec2#LaunchTemplateInstanceMetadataOptions$HttpPutResponseHopLimit */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_http_put_response_hop_limit(var_3);
            }
            ,
            s if s.matches("httpEndpoint") /* HttpEndpoint com.amazonaws.ec2#LaunchTemplateInstanceMetadataOptions$HttpEndpoint */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::LaunchTemplateInstanceMetadataEndpointState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LaunchTemplateInstanceMetadataEndpointState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_http_endpoint(var_4);
            }
            ,
            s if s.matches("httpProtocolIpv6") /* HttpProtocolIpv6 com.amazonaws.ec2#LaunchTemplateInstanceMetadataOptions$HttpProtocolIpv6 */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::LaunchTemplateInstanceMetadataProtocolIpv6, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LaunchTemplateInstanceMetadataProtocolIpv6::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_http_protocol_ipv6(var_5);
            }
            ,
            s if s.matches("instanceMetadataTags") /* InstanceMetadataTags com.amazonaws.ec2#LaunchTemplateInstanceMetadataOptions$InstanceMetadataTags */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::LaunchTemplateInstanceMetadataTagsState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LaunchTemplateInstanceMetadataTagsState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_metadata_tags(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
