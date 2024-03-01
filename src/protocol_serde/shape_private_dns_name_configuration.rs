// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_private_dns_name_configuration(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::PrivateDnsNameConfiguration, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::PrivateDnsNameConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("state") /* State com.amazonaws.ec2#PrivateDnsNameConfiguration$State */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::DnsNameState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DnsNameState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_1);
            }
            ,
            s if s.matches("type") /* Type com.amazonaws.ec2#PrivateDnsNameConfiguration$Type */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_2);
            }
            ,
            s if s.matches("value") /* Value com.amazonaws.ec2#PrivateDnsNameConfiguration$Value */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_value(var_3);
            }
            ,
            s if s.matches("name") /* Name com.amazonaws.ec2#PrivateDnsNameConfiguration$Name */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
