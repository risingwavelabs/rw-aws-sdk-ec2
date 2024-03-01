// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_transit_gateway_connect(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TransitGatewayConnect, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TransitGatewayConnect::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("transitGatewayAttachmentId") /* TransitGatewayAttachmentId com.amazonaws.ec2#TransitGatewayConnect$TransitGatewayAttachmentId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_attachment_id(var_1);
            }
            ,
            s if s.matches("transportTransitGatewayAttachmentId") /* TransportTransitGatewayAttachmentId com.amazonaws.ec2#TransitGatewayConnect$TransportTransitGatewayAttachmentId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transport_transit_gateway_attachment_id(var_2);
            }
            ,
            s if s.matches("transitGatewayId") /* TransitGatewayId com.amazonaws.ec2#TransitGatewayConnect$TransitGatewayId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_id(var_3);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#TransitGatewayConnect$State */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::TransitGatewayAttachmentState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::TransitGatewayAttachmentState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_4);
            }
            ,
            s if s.matches("creationTime") /* CreationTime com.amazonaws.ec2#TransitGatewayConnect$CreationTime */ =>  {
                let var_5 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_creation_time(var_5);
            }
            ,
            s if s.matches("options") /* Options com.amazonaws.ec2#TransitGatewayConnect$Options */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_transit_gateway_connect_options::de_transit_gateway_connect_options(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_options(var_6);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#TransitGatewayConnect$Tags */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
