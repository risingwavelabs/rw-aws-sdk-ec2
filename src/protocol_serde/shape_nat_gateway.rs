// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_nat_gateway(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::NatGateway, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::NatGateway::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("createTime") /* CreateTime com.amazonaws.ec2#NatGateway$CreateTime */ =>  {
                let var_1 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_create_time(var_1);
            }
            ,
            s if s.matches("deleteTime") /* DeleteTime com.amazonaws.ec2#NatGateway$DeleteTime */ =>  {
                let var_2 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_delete_time(var_2);
            }
            ,
            s if s.matches("failureCode") /* FailureCode com.amazonaws.ec2#NatGateway$FailureCode */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_failure_code(var_3);
            }
            ,
            s if s.matches("failureMessage") /* FailureMessage com.amazonaws.ec2#NatGateway$FailureMessage */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_failure_message(var_4);
            }
            ,
            s if s.matches("natGatewayAddressSet") /* NatGatewayAddresses com.amazonaws.ec2#NatGateway$NatGatewayAddresses */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_nat_gateway_address_list::de_nat_gateway_address_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_nat_gateway_addresses(var_5);
            }
            ,
            s if s.matches("natGatewayId") /* NatGatewayId com.amazonaws.ec2#NatGateway$NatGatewayId */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_nat_gateway_id(var_6);
            }
            ,
            s if s.matches("provisionedBandwidth") /* ProvisionedBandwidth com.amazonaws.ec2#NatGateway$ProvisionedBandwidth */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_provisioned_bandwidth::de_provisioned_bandwidth(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_provisioned_bandwidth(var_7);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#NatGateway$State */ =>  {
                let var_8 =
                    Some(
                        Result::<crate::types::NatGatewayState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::NatGatewayState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_8);
            }
            ,
            s if s.matches("subnetId") /* SubnetId com.amazonaws.ec2#NatGateway$SubnetId */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_9);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2#NatGateway$VpcId */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_10);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#NatGateway$Tags */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_11);
            }
            ,
            s if s.matches("connectivityType") /* ConnectivityType com.amazonaws.ec2#NatGateway$ConnectivityType */ =>  {
                let var_12 =
                    Some(
                        Result::<crate::types::ConnectivityType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ConnectivityType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_connectivity_type(var_12);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
