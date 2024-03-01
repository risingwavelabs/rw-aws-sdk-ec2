// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_spot_fleet_launch_specification(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::SpotFleetLaunchSpecification,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("GroupSet");
    if let Some(var_2) = &input.security_groups {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_group_identifier::ser_group_identifier(entry_5, item_3)?;
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("AddressingType");
    if let Some(var_7) = &input.addressing_type {
        scope_6.string(var_7);
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("BlockDeviceMapping");
    if let Some(var_9) = &input.block_device_mappings {
        let mut list_11 = scope_8.start_list(true, Some("item"));
        for item_10 in var_9 {
            #[allow(unused_mut)]
            let mut entry_12 = list_11.entry();
            crate::protocol_serde::shape_block_device_mapping::ser_block_device_mapping(entry_12, item_10)?;
        }
        list_11.finish();
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("EbsOptimized");
    if let Some(var_14) = &input.ebs_optimized {
        scope_13.boolean(*var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("IamInstanceProfile");
    if let Some(var_16) = &input.iam_instance_profile {
        crate::protocol_serde::shape_iam_instance_profile_specification::ser_iam_instance_profile_specification(scope_15, var_16)?;
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("ImageId");
    if let Some(var_18) = &input.image_id {
        scope_17.string(var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("InstanceType");
    if let Some(var_20) = &input.instance_type {
        scope_19.string(var_20.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("KernelId");
    if let Some(var_22) = &input.kernel_id {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("KeyName");
    if let Some(var_24) = &input.key_name {
        scope_23.string(var_24);
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("Monitoring");
    if let Some(var_26) = &input.monitoring {
        crate::protocol_serde::shape_spot_fleet_monitoring::ser_spot_fleet_monitoring(scope_25, var_26)?;
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("NetworkInterfaceSet");
    if let Some(var_28) = &input.network_interfaces {
        let mut list_30 = scope_27.start_list(true, Some("item"));
        for item_29 in var_28 {
            #[allow(unused_mut)]
            let mut entry_31 = list_30.entry();
            crate::protocol_serde::shape_instance_network_interface_specification::ser_instance_network_interface_specification(entry_31, item_29)?;
        }
        list_30.finish();
    }
    #[allow(unused_mut)]
    let mut scope_32 = writer.prefix("Placement");
    if let Some(var_33) = &input.placement {
        crate::protocol_serde::shape_spot_placement::ser_spot_placement(scope_32, var_33)?;
    }
    #[allow(unused_mut)]
    let mut scope_34 = writer.prefix("RamdiskId");
    if let Some(var_35) = &input.ramdisk_id {
        scope_34.string(var_35);
    }
    #[allow(unused_mut)]
    let mut scope_36 = writer.prefix("SpotPrice");
    if let Some(var_37) = &input.spot_price {
        scope_36.string(var_37);
    }
    #[allow(unused_mut)]
    let mut scope_38 = writer.prefix("SubnetId");
    if let Some(var_39) = &input.subnet_id {
        scope_38.string(var_39);
    }
    #[allow(unused_mut)]
    let mut scope_40 = writer.prefix("UserData");
    if let Some(var_41) = &input.user_data {
        scope_40.string(var_41);
    }
    #[allow(unused_mut)]
    let mut scope_42 = writer.prefix("WeightedCapacity");
    if let Some(var_43) = &input.weighted_capacity {
        scope_42.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_43).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_44 = writer.prefix("TagSpecificationSet");
    if let Some(var_45) = &input.tag_specifications {
        let mut list_47 = scope_44.start_list(true, Some("item"));
        for item_46 in var_45 {
            #[allow(unused_mut)]
            let mut entry_48 = list_47.entry();
            crate::protocol_serde::shape_spot_fleet_tag_specification::ser_spot_fleet_tag_specification(entry_48, item_46)?;
        }
        list_47.finish();
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("InstanceRequirements");
    if let Some(var_50) = &input.instance_requirements {
        crate::protocol_serde::shape_instance_requirements::ser_instance_requirements(scope_49, var_50)?;
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_spot_fleet_launch_specification(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::SpotFleetLaunchSpecification, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SpotFleetLaunchSpecification::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("groupSet") /* SecurityGroups com.amazonaws.ec2#SpotFleetLaunchSpecification$SecurityGroups */ =>  {
                let var_51 =
                    Some(
                        crate::protocol_serde::shape_group_identifier_list::de_group_identifier_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_groups(var_51);
            }
            ,
            s if s.matches("addressingType") /* AddressingType com.amazonaws.ec2#SpotFleetLaunchSpecification$AddressingType */ =>  {
                let var_52 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_addressing_type(var_52);
            }
            ,
            s if s.matches("blockDeviceMapping") /* BlockDeviceMappings com.amazonaws.ec2#SpotFleetLaunchSpecification$BlockDeviceMappings */ =>  {
                let var_53 =
                    Some(
                        crate::protocol_serde::shape_block_device_mapping_list::de_block_device_mapping_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_block_device_mappings(var_53);
            }
            ,
            s if s.matches("ebsOptimized") /* EbsOptimized com.amazonaws.ec2#SpotFleetLaunchSpecification$EbsOptimized */ =>  {
                let var_54 =
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
                builder = builder.set_ebs_optimized(var_54);
            }
            ,
            s if s.matches("iamInstanceProfile") /* IamInstanceProfile com.amazonaws.ec2#SpotFleetLaunchSpecification$IamInstanceProfile */ =>  {
                let var_55 =
                    Some(
                        crate::protocol_serde::shape_iam_instance_profile_specification::de_iam_instance_profile_specification(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_iam_instance_profile(var_55);
            }
            ,
            s if s.matches("imageId") /* ImageId com.amazonaws.ec2#SpotFleetLaunchSpecification$ImageId */ =>  {
                let var_56 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_image_id(var_56);
            }
            ,
            s if s.matches("instanceType") /* InstanceType com.amazonaws.ec2#SpotFleetLaunchSpecification$InstanceType */ =>  {
                let var_57 =
                    Some(
                        Result::<crate::types::InstanceType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InstanceType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_type(var_57);
            }
            ,
            s if s.matches("kernelId") /* KernelId com.amazonaws.ec2#SpotFleetLaunchSpecification$KernelId */ =>  {
                let var_58 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kernel_id(var_58);
            }
            ,
            s if s.matches("keyName") /* KeyName com.amazonaws.ec2#SpotFleetLaunchSpecification$KeyName */ =>  {
                let var_59 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key_name(var_59);
            }
            ,
            s if s.matches("monitoring") /* Monitoring com.amazonaws.ec2#SpotFleetLaunchSpecification$Monitoring */ =>  {
                let var_60 =
                    Some(
                        crate::protocol_serde::shape_spot_fleet_monitoring::de_spot_fleet_monitoring(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_monitoring(var_60);
            }
            ,
            s if s.matches("networkInterfaceSet") /* NetworkInterfaces com.amazonaws.ec2#SpotFleetLaunchSpecification$NetworkInterfaces */ =>  {
                let var_61 =
                    Some(
                        crate::protocol_serde::shape_instance_network_interface_specification_list::de_instance_network_interface_specification_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_network_interfaces(var_61);
            }
            ,
            s if s.matches("placement") /* Placement com.amazonaws.ec2#SpotFleetLaunchSpecification$Placement */ =>  {
                let var_62 =
                    Some(
                        crate::protocol_serde::shape_spot_placement::de_spot_placement(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_placement(var_62);
            }
            ,
            s if s.matches("ramdiskId") /* RamdiskId com.amazonaws.ec2#SpotFleetLaunchSpecification$RamdiskId */ =>  {
                let var_63 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ramdisk_id(var_63);
            }
            ,
            s if s.matches("spotPrice") /* SpotPrice com.amazonaws.ec2#SpotFleetLaunchSpecification$SpotPrice */ =>  {
                let var_64 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_spot_price(var_64);
            }
            ,
            s if s.matches("subnetId") /* SubnetId com.amazonaws.ec2#SpotFleetLaunchSpecification$SubnetId */ =>  {
                let var_65 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_65);
            }
            ,
            s if s.matches("userData") /* UserData com.amazonaws.ec2#SpotFleetLaunchSpecification$UserData */ =>  {
                let var_66 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_user_data(var_66);
            }
            ,
            s if s.matches("weightedCapacity") /* WeightedCapacity com.amazonaws.ec2#SpotFleetLaunchSpecification$WeightedCapacity */ =>  {
                let var_67 =
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
                builder = builder.set_weighted_capacity(var_67);
            }
            ,
            s if s.matches("tagSpecificationSet") /* TagSpecifications com.amazonaws.ec2#SpotFleetLaunchSpecification$TagSpecifications */ =>  {
                let var_68 =
                    Some(
                        crate::protocol_serde::shape_spot_fleet_tag_specification_list::de_spot_fleet_tag_specification_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tag_specifications(var_68);
            }
            ,
            s if s.matches("instanceRequirements") /* InstanceRequirements com.amazonaws.ec2#SpotFleetLaunchSpecification$InstanceRequirements */ =>  {
                let var_69 =
                    Some(
                        crate::protocol_serde::shape_instance_requirements::de_instance_requirements(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_requirements(var_69);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
