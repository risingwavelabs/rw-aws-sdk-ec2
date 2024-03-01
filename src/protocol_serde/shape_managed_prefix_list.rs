// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_managed_prefix_list(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ManagedPrefixList, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ManagedPrefixList::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("prefixListId") /* PrefixListId com.amazonaws.ec2#ManagedPrefixList$PrefixListId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix_list_id(var_1);
            }
            ,
            s if s.matches("addressFamily") /* AddressFamily com.amazonaws.ec2#ManagedPrefixList$AddressFamily */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_address_family(var_2);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#ManagedPrefixList$State */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::PrefixListState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::PrefixListState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_3);
            }
            ,
            s if s.matches("stateMessage") /* StateMessage com.amazonaws.ec2#ManagedPrefixList$StateMessage */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_state_message(var_4);
            }
            ,
            s if s.matches("prefixListArn") /* PrefixListArn com.amazonaws.ec2#ManagedPrefixList$PrefixListArn */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix_list_arn(var_5);
            }
            ,
            s if s.matches("prefixListName") /* PrefixListName com.amazonaws.ec2#ManagedPrefixList$PrefixListName */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_prefix_list_name(var_6);
            }
            ,
            s if s.matches("maxEntries") /* MaxEntries com.amazonaws.ec2#ManagedPrefixList$MaxEntries */ =>  {
                let var_7 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_max_entries(var_7);
            }
            ,
            s if s.matches("version") /* Version com.amazonaws.ec2#ManagedPrefixList$Version */ =>  {
                let var_8 =
                    Some(
                         {
                            <i64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.ec2#Long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_version(var_8);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#ManagedPrefixList$Tags */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_9);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#ManagedPrefixList$OwnerId */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_10);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
