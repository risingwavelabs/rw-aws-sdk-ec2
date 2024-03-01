// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_network_acl(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::NetworkAcl, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::NetworkAcl::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("associationSet") /* Associations com.amazonaws.ec2#NetworkAcl$Associations */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_network_acl_association_list::de_network_acl_association_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_associations(var_1);
            }
            ,
            s if s.matches("entrySet") /* Entries com.amazonaws.ec2#NetworkAcl$Entries */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_network_acl_entry_list::de_network_acl_entry_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_entries(var_2);
            }
            ,
            s if s.matches("default") /* IsDefault com.amazonaws.ec2#NetworkAcl$IsDefault */ =>  {
                let var_3 =
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
                builder = builder.set_is_default(var_3);
            }
            ,
            s if s.matches("networkAclId") /* NetworkAclId com.amazonaws.ec2#NetworkAcl$NetworkAclId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_acl_id(var_4);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#NetworkAcl$Tags */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_5);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2#NetworkAcl$VpcId */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_6);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#NetworkAcl$OwnerId */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
