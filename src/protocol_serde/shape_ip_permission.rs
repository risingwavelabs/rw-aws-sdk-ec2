// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_ip_permission(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::IpPermission,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("FromPort");
    if let Some(var_2) = &input.from_port {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("IpProtocol");
    if let Some(var_4) = &input.ip_protocol {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("IpRanges");
    if let Some(var_6) = &input.ip_ranges {
        let mut list_8 = scope_5.start_list(true, Some("item"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_ip_range::ser_ip_range(entry_9, item_7)?;
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("Ipv6Ranges");
    if let Some(var_11) = &input.ipv6_ranges {
        let mut list_13 = scope_10.start_list(true, Some("item"));
        for item_12 in var_11 {
            #[allow(unused_mut)]
            let mut entry_14 = list_13.entry();
            crate::protocol_serde::shape_ipv6_range::ser_ipv6_range(entry_14, item_12)?;
        }
        list_13.finish();
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("PrefixListIds");
    if let Some(var_16) = &input.prefix_list_ids {
        let mut list_18 = scope_15.start_list(true, Some("item"));
        for item_17 in var_16 {
            #[allow(unused_mut)]
            let mut entry_19 = list_18.entry();
            crate::protocol_serde::shape_prefix_list_id::ser_prefix_list_id(entry_19, item_17)?;
        }
        list_18.finish();
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("ToPort");
    if let Some(var_21) = &input.to_port {
        scope_20.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_21).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("Groups");
    if let Some(var_23) = &input.user_id_group_pairs {
        let mut list_25 = scope_22.start_list(true, Some("item"));
        for item_24 in var_23 {
            #[allow(unused_mut)]
            let mut entry_26 = list_25.entry();
            crate::protocol_serde::shape_user_id_group_pair::ser_user_id_group_pair(entry_26, item_24)?;
        }
        list_25.finish();
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_ip_permission(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::IpPermission, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::IpPermission::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("fromPort") /* FromPort com.amazonaws.ec2#IpPermission$FromPort */ =>  {
                let var_27 =
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
                builder = builder.set_from_port(var_27);
            }
            ,
            s if s.matches("ipProtocol") /* IpProtocol com.amazonaws.ec2#IpPermission$IpProtocol */ =>  {
                let var_28 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ip_protocol(var_28);
            }
            ,
            s if s.matches("ipRanges") /* IpRanges com.amazonaws.ec2#IpPermission$IpRanges */ =>  {
                let var_29 =
                    Some(
                        crate::protocol_serde::shape_ip_range_list::de_ip_range_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ip_ranges(var_29);
            }
            ,
            s if s.matches("ipv6Ranges") /* Ipv6Ranges com.amazonaws.ec2#IpPermission$Ipv6Ranges */ =>  {
                let var_30 =
                    Some(
                        crate::protocol_serde::shape_ipv6_range_list::de_ipv6_range_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv6_ranges(var_30);
            }
            ,
            s if s.matches("prefixListIds") /* PrefixListIds com.amazonaws.ec2#IpPermission$PrefixListIds */ =>  {
                let var_31 =
                    Some(
                        crate::protocol_serde::shape_prefix_list_id_list::de_prefix_list_id_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_prefix_list_ids(var_31);
            }
            ,
            s if s.matches("toPort") /* ToPort com.amazonaws.ec2#IpPermission$ToPort */ =>  {
                let var_32 =
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
                builder = builder.set_to_port(var_32);
            }
            ,
            s if s.matches("groups") /* UserIdGroupPairs com.amazonaws.ec2#IpPermission$UserIdGroupPairs */ =>  {
                let var_33 =
                    Some(
                        crate::protocol_serde::shape_user_id_group_pair_list::de_user_id_group_pair_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_user_id_group_pairs(var_33);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
