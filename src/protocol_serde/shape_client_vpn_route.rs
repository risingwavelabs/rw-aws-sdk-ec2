// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_client_vpn_route(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ClientVpnRoute, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ClientVpnRoute::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("clientVpnEndpointId") /* ClientVpnEndpointId com.amazonaws.ec2#ClientVpnRoute$ClientVpnEndpointId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_client_vpn_endpoint_id(var_1);
            }
            ,
            s if s.matches("destinationCidr") /* DestinationCidr com.amazonaws.ec2#ClientVpnRoute$DestinationCidr */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_destination_cidr(var_2);
            }
            ,
            s if s.matches("targetSubnet") /* TargetSubnet com.amazonaws.ec2#ClientVpnRoute$TargetSubnet */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target_subnet(var_3);
            }
            ,
            s if s.matches("type") /* Type com.amazonaws.ec2#ClientVpnRoute$Type */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_4);
            }
            ,
            s if s.matches("origin") /* Origin com.amazonaws.ec2#ClientVpnRoute$Origin */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_origin(var_5);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2#ClientVpnRoute$Status */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_client_vpn_route_status::de_client_vpn_route_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_6);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#ClientVpnRoute$Description */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
