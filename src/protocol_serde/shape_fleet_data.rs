// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_fleet_data(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::FleetData, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::FleetData::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("activityStatus") /* ActivityStatus com.amazonaws.ec2#FleetData$ActivityStatus */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::FleetActivityStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::FleetActivityStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_activity_status(var_1);
            }
            ,
            s if s.matches("createTime") /* CreateTime com.amazonaws.ec2#FleetData$CreateTime */ =>  {
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
                builder = builder.set_create_time(var_2);
            }
            ,
            s if s.matches("fleetId") /* FleetId com.amazonaws.ec2#FleetData$FleetId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_fleet_id(var_3);
            }
            ,
            s if s.matches("fleetState") /* FleetState com.amazonaws.ec2#FleetData$FleetState */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::FleetStateCode, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::FleetStateCode::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_fleet_state(var_4);
            }
            ,
            s if s.matches("clientToken") /* ClientToken com.amazonaws.ec2#FleetData$ClientToken */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_client_token(var_5);
            }
            ,
            s if s.matches("excessCapacityTerminationPolicy") /* ExcessCapacityTerminationPolicy com.amazonaws.ec2#FleetData$ExcessCapacityTerminationPolicy */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::FleetExcessCapacityTerminationPolicy, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::FleetExcessCapacityTerminationPolicy::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_excess_capacity_termination_policy(var_6);
            }
            ,
            s if s.matches("fulfilledCapacity") /* FulfilledCapacity com.amazonaws.ec2#FleetData$FulfilledCapacity */ =>  {
                let var_7 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ec2#Double`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_fulfilled_capacity(var_7);
            }
            ,
            s if s.matches("fulfilledOnDemandCapacity") /* FulfilledOnDemandCapacity com.amazonaws.ec2#FleetData$FulfilledOnDemandCapacity */ =>  {
                let var_8 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ec2#Double`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_fulfilled_on_demand_capacity(var_8);
            }
            ,
            s if s.matches("launchTemplateConfigs") /* LaunchTemplateConfigs com.amazonaws.ec2#FleetData$LaunchTemplateConfigs */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_fleet_launch_template_config_list::de_fleet_launch_template_config_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_launch_template_configs(var_9);
            }
            ,
            s if s.matches("targetCapacitySpecification") /* TargetCapacitySpecification com.amazonaws.ec2#FleetData$TargetCapacitySpecification */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_target_capacity_specification::de_target_capacity_specification(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_target_capacity_specification(var_10);
            }
            ,
            s if s.matches("terminateInstancesWithExpiration") /* TerminateInstancesWithExpiration com.amazonaws.ec2#FleetData$TerminateInstancesWithExpiration */ =>  {
                let var_11 =
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
                builder = builder.set_terminate_instances_with_expiration(var_11);
            }
            ,
            s if s.matches("type") /* Type com.amazonaws.ec2#FleetData$Type */ =>  {
                let var_12 =
                    Some(
                        Result::<crate::types::FleetType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::FleetType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_12);
            }
            ,
            s if s.matches("validFrom") /* ValidFrom com.amazonaws.ec2#FleetData$ValidFrom */ =>  {
                let var_13 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_valid_from(var_13);
            }
            ,
            s if s.matches("validUntil") /* ValidUntil com.amazonaws.ec2#FleetData$ValidUntil */ =>  {
                let var_14 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_valid_until(var_14);
            }
            ,
            s if s.matches("replaceUnhealthyInstances") /* ReplaceUnhealthyInstances com.amazonaws.ec2#FleetData$ReplaceUnhealthyInstances */ =>  {
                let var_15 =
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
                builder = builder.set_replace_unhealthy_instances(var_15);
            }
            ,
            s if s.matches("spotOptions") /* SpotOptions com.amazonaws.ec2#FleetData$SpotOptions */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_spot_options::de_spot_options(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_spot_options(var_16);
            }
            ,
            s if s.matches("onDemandOptions") /* OnDemandOptions com.amazonaws.ec2#FleetData$OnDemandOptions */ =>  {
                let var_17 =
                    Some(
                        crate::protocol_serde::shape_on_demand_options::de_on_demand_options(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_on_demand_options(var_17);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#FleetData$Tags */ =>  {
                let var_18 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_18);
            }
            ,
            s if s.matches("errorSet") /* Errors com.amazonaws.ec2#FleetData$Errors */ =>  {
                let var_19 =
                    Some(
                        crate::protocol_serde::shape_describe_fleets_error_set::de_describe_fleets_error_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_errors(var_19);
            }
            ,
            s if s.matches("fleetInstanceSet") /* Instances com.amazonaws.ec2#FleetData$Instances */ =>  {
                let var_20 =
                    Some(
                        crate::protocol_serde::shape_describe_fleets_instances_set::de_describe_fleets_instances_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instances(var_20);
            }
            ,
            s if s.matches("context") /* Context com.amazonaws.ec2#FleetData$Context */ =>  {
                let var_21 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_context(var_21);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
