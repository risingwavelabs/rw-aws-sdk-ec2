// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_local_gateway_route_table_virtual_interface_group_association(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LocalGatewayRouteTableVirtualInterfaceGroupAssociation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("localGatewayRouteTableVirtualInterfaceGroupAssociationId") /* LocalGatewayRouteTableVirtualInterfaceGroupAssociationId com.amazonaws.ec2#LocalGatewayRouteTableVirtualInterfaceGroupAssociation$LocalGatewayRouteTableVirtualInterfaceGroupAssociationId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_route_table_virtual_interface_group_association_id(var_1);
            }
            ,
            s if s.matches("localGatewayVirtualInterfaceGroupId") /* LocalGatewayVirtualInterfaceGroupId com.amazonaws.ec2#LocalGatewayRouteTableVirtualInterfaceGroupAssociation$LocalGatewayVirtualInterfaceGroupId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_virtual_interface_group_id(var_2);
            }
            ,
            s if s.matches("localGatewayId") /* LocalGatewayId com.amazonaws.ec2#LocalGatewayRouteTableVirtualInterfaceGroupAssociation$LocalGatewayId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_id(var_3);
            }
            ,
            s if s.matches("localGatewayRouteTableId") /* LocalGatewayRouteTableId com.amazonaws.ec2#LocalGatewayRouteTableVirtualInterfaceGroupAssociation$LocalGatewayRouteTableId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_route_table_id(var_4);
            }
            ,
            s if s.matches("localGatewayRouteTableArn") /* LocalGatewayRouteTableArn com.amazonaws.ec2#LocalGatewayRouteTableVirtualInterfaceGroupAssociation$LocalGatewayRouteTableArn */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_local_gateway_route_table_arn(var_5);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#LocalGatewayRouteTableVirtualInterfaceGroupAssociation$OwnerId */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_6);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#LocalGatewayRouteTableVirtualInterfaceGroupAssociation$State */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_7);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#LocalGatewayRouteTableVirtualInterfaceGroupAssociation$Tags */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
