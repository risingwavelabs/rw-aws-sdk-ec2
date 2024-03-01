// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_target_network(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TargetNetwork, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TargetNetwork::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("associationId") /* AssociationId com.amazonaws.ec2#TargetNetwork$AssociationId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_association_id(var_1);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2#TargetNetwork$VpcId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_2);
            }
            ,
            s if s.matches("targetNetworkId") /* TargetNetworkId com.amazonaws.ec2#TargetNetwork$TargetNetworkId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target_network_id(var_3);
            }
            ,
            s if s.matches("clientVpnEndpointId") /* ClientVpnEndpointId com.amazonaws.ec2#TargetNetwork$ClientVpnEndpointId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_client_vpn_endpoint_id(var_4);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2#TargetNetwork$Status */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_association_status::de_association_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_5);
            }
            ,
            s if s.matches("securityGroups") /* SecurityGroups com.amazonaws.ec2#TargetNetwork$SecurityGroups */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_groups(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
