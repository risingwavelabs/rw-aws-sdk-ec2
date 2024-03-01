// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_transit_gateway_multicast_group(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TransitGatewayMulticastGroup, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TransitGatewayMulticastGroup::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("groupIpAddress") /* GroupIpAddress com.amazonaws.ec2#TransitGatewayMulticastGroup$GroupIpAddress */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_group_ip_address(var_1);
            }
            ,
            s if s.matches("transitGatewayAttachmentId") /* TransitGatewayAttachmentId com.amazonaws.ec2#TransitGatewayMulticastGroup$TransitGatewayAttachmentId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_attachment_id(var_2);
            }
            ,
            s if s.matches("subnetId") /* SubnetId com.amazonaws.ec2#TransitGatewayMulticastGroup$SubnetId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_3);
            }
            ,
            s if s.matches("resourceId") /* ResourceId com.amazonaws.ec2#TransitGatewayMulticastGroup$ResourceId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_id(var_4);
            }
            ,
            s if s.matches("resourceType") /* ResourceType com.amazonaws.ec2#TransitGatewayMulticastGroup$ResourceType */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::TransitGatewayAttachmentResourceType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::TransitGatewayAttachmentResourceType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_type(var_5);
            }
            ,
            s if s.matches("resourceOwnerId") /* ResourceOwnerId com.amazonaws.ec2#TransitGatewayMulticastGroup$ResourceOwnerId */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_owner_id(var_6);
            }
            ,
            s if s.matches("networkInterfaceId") /* NetworkInterfaceId com.amazonaws.ec2#TransitGatewayMulticastGroup$NetworkInterfaceId */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_interface_id(var_7);
            }
            ,
            s if s.matches("groupMember") /* GroupMember com.amazonaws.ec2#TransitGatewayMulticastGroup$GroupMember */ =>  {
                let var_8 =
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
                builder = builder.set_group_member(var_8);
            }
            ,
            s if s.matches("groupSource") /* GroupSource com.amazonaws.ec2#TransitGatewayMulticastGroup$GroupSource */ =>  {
                let var_9 =
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
                builder = builder.set_group_source(var_9);
            }
            ,
            s if s.matches("memberType") /* MemberType com.amazonaws.ec2#TransitGatewayMulticastGroup$MemberType */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::types::MembershipType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::MembershipType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_member_type(var_10);
            }
            ,
            s if s.matches("sourceType") /* SourceType com.amazonaws.ec2#TransitGatewayMulticastGroup$SourceType */ =>  {
                let var_11 =
                    Some(
                        Result::<crate::types::MembershipType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::MembershipType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_source_type(var_11);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
