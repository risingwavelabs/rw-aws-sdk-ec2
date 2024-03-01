// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_dns_options(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::DnsOptions, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DnsOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("dnsRecordIpType") /* DnsRecordIpType com.amazonaws.ec2#DnsOptions$DnsRecordIpType */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::DnsRecordIpType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DnsRecordIpType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_dns_record_ip_type(var_1);
            }
            ,
            s if s.matches("privateDnsOnlyForInboundResolverEndpoint") /* PrivateDnsOnlyForInboundResolverEndpoint com.amazonaws.ec2#DnsOptions$PrivateDnsOnlyForInboundResolverEndpoint */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_private_dns_only_for_inbound_resolver_endpoint(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}