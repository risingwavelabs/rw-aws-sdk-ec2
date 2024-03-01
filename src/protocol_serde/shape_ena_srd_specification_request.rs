// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_ena_srd_specification_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::EnaSrdSpecificationRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EnaSrdEnabled");
    if let Some(var_2) = &input.ena_srd_enabled {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("EnaSrdUdpSpecification");
    if let Some(var_4) = &input.ena_srd_udp_specification {
        crate::protocol_serde::shape_ena_srd_udp_specification_request::ser_ena_srd_udp_specification_request(scope_3, var_4)?;
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_ena_srd_specification_request(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::EnaSrdSpecificationRequest, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::EnaSrdSpecificationRequest::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("EnaSrdEnabled") /* EnaSrdEnabled com.amazonaws.ec2#EnaSrdSpecificationRequest$EnaSrdEnabled */ =>  {
                let var_5 =
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
                builder = builder.set_ena_srd_enabled(var_5);
            }
            ,
            s if s.matches("EnaSrdUdpSpecification") /* EnaSrdUdpSpecification com.amazonaws.ec2#EnaSrdSpecificationRequest$EnaSrdUdpSpecification */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_ena_srd_udp_specification_request::de_ena_srd_udp_specification_request(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ena_srd_udp_specification(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
