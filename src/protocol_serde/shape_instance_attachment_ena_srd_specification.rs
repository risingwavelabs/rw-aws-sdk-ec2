// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_instance_attachment_ena_srd_specification(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InstanceAttachmentEnaSrdSpecification, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InstanceAttachmentEnaSrdSpecification::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("enaSrdEnabled") /* EnaSrdEnabled com.amazonaws.ec2#InstanceAttachmentEnaSrdSpecification$EnaSrdEnabled */ =>  {
                let var_1 =
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
                builder = builder.set_ena_srd_enabled(var_1);
            }
            ,
            s if s.matches("enaSrdUdpSpecification") /* EnaSrdUdpSpecification com.amazonaws.ec2#InstanceAttachmentEnaSrdSpecification$EnaSrdUdpSpecification */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_instance_attachment_ena_srd_udp_specification::de_instance_attachment_ena_srd_udp_specification(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ena_srd_udp_specification(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
