// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_request_launch_template_data(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::RequestLaunchTemplateData,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("KernelId");
    if let Some(var_2) = &input.kernel_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("EbsOptimized");
    if let Some(var_4) = &input.ebs_optimized {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("IamInstanceProfile");
    if let Some(var_6) = &input.iam_instance_profile {
        crate::protocol_serde::shape_launch_template_iam_instance_profile_specification_request::ser_launch_template_iam_instance_profile_specification_request(scope_5, var_6)?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("BlockDeviceMapping");
    if let Some(var_8) = &input.block_device_mappings {
        let mut list_10 = scope_7.start_list(true, Some("BlockDeviceMapping"));
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            crate::protocol_serde::shape_launch_template_block_device_mapping_request::ser_launch_template_block_device_mapping_request(
                entry_11, item_9,
            )?;
        }
        list_10.finish();
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("NetworkInterface");
    if let Some(var_13) = &input.network_interfaces {
        let mut list_15 = scope_12.start_list(true, Some("InstanceNetworkInterfaceSpecification"));
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            crate::protocol_serde::shape_launch_template_instance_network_interface_specification_request::ser_launch_template_instance_network_interface_specification_request(entry_16, item_14)?;
        }
        list_15.finish();
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
    let mut scope_21 = writer.prefix("KeyName");
    if let Some(var_22) = &input.key_name {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("Monitoring");
    if let Some(var_24) = &input.monitoring {
        crate::protocol_serde::shape_launch_templates_monitoring_request::ser_launch_templates_monitoring_request(scope_23, var_24)?;
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("Placement");
    if let Some(var_26) = &input.placement {
        crate::protocol_serde::shape_launch_template_placement_request::ser_launch_template_placement_request(scope_25, var_26)?;
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("RamDiskId");
    if let Some(var_28) = &input.ram_disk_id {
        scope_27.string(var_28);
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("DisableApiTermination");
    if let Some(var_30) = &input.disable_api_termination {
        scope_29.boolean(*var_30);
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("InstanceInitiatedShutdownBehavior");
    if let Some(var_32) = &input.instance_initiated_shutdown_behavior {
        scope_31.string(var_32.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("UserData");
    if let Some(var_34) = &input.user_data {
        scope_33.string(var_34);
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("TagSpecification");
    if let Some(var_36) = &input.tag_specifications {
        let mut list_38 = scope_35.start_list(true, Some("LaunchTemplateTagSpecificationRequest"));
        for item_37 in var_36 {
            #[allow(unused_mut)]
            let mut entry_39 = list_38.entry();
            crate::protocol_serde::shape_launch_template_tag_specification_request::ser_launch_template_tag_specification_request(entry_39, item_37)?;
        }
        list_38.finish();
    }
    #[allow(unused_mut)]
    let mut scope_40 = writer.prefix("ElasticGpuSpecification");
    if let Some(var_41) = &input.elastic_gpu_specifications {
        let mut list_43 = scope_40.start_list(true, Some("ElasticGpuSpecification"));
        for item_42 in var_41 {
            #[allow(unused_mut)]
            let mut entry_44 = list_43.entry();
            crate::protocol_serde::shape_elastic_gpu_specification::ser_elastic_gpu_specification(entry_44, item_42)?;
        }
        list_43.finish();
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("ElasticInferenceAccelerator");
    if let Some(var_46) = &input.elastic_inference_accelerators {
        let mut list_48 = scope_45.start_list(true, Some("item"));
        for item_47 in var_46 {
            #[allow(unused_mut)]
            let mut entry_49 = list_48.entry();
            crate::protocol_serde::shape_launch_template_elastic_inference_accelerator::ser_launch_template_elastic_inference_accelerator(
                entry_49, item_47,
            )?;
        }
        list_48.finish();
    }
    #[allow(unused_mut)]
    let mut scope_50 = writer.prefix("SecurityGroupId");
    if let Some(var_51) = &input.security_group_ids {
        let mut list_53 = scope_50.start_list(true, Some("SecurityGroupId"));
        for item_52 in var_51 {
            #[allow(unused_mut)]
            let mut entry_54 = list_53.entry();
            entry_54.string(item_52);
        }
        list_53.finish();
    }
    #[allow(unused_mut)]
    let mut scope_55 = writer.prefix("SecurityGroup");
    if let Some(var_56) = &input.security_groups {
        let mut list_58 = scope_55.start_list(true, Some("SecurityGroup"));
        for item_57 in var_56 {
            #[allow(unused_mut)]
            let mut entry_59 = list_58.entry();
            entry_59.string(item_57);
        }
        list_58.finish();
    }
    #[allow(unused_mut)]
    let mut scope_60 = writer.prefix("InstanceMarketOptions");
    if let Some(var_61) = &input.instance_market_options {
        crate::protocol_serde::shape_launch_template_instance_market_options_request::ser_launch_template_instance_market_options_request(
            scope_60, var_61,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_62 = writer.prefix("CreditSpecification");
    if let Some(var_63) = &input.credit_specification {
        crate::protocol_serde::shape_credit_specification_request::ser_credit_specification_request(scope_62, var_63)?;
    }
    #[allow(unused_mut)]
    let mut scope_64 = writer.prefix("CpuOptions");
    if let Some(var_65) = &input.cpu_options {
        crate::protocol_serde::shape_launch_template_cpu_options_request::ser_launch_template_cpu_options_request(scope_64, var_65)?;
    }
    #[allow(unused_mut)]
    let mut scope_66 = writer.prefix("CapacityReservationSpecification");
    if let Some(var_67) = &input.capacity_reservation_specification {
        crate::protocol_serde::shape_launch_template_capacity_reservation_specification_request::ser_launch_template_capacity_reservation_specification_request(scope_66, var_67)?;
    }
    #[allow(unused_mut)]
    let mut scope_68 = writer.prefix("LicenseSpecification");
    if let Some(var_69) = &input.license_specifications {
        let mut list_71 = scope_68.start_list(true, Some("item"));
        for item_70 in var_69 {
            #[allow(unused_mut)]
            let mut entry_72 = list_71.entry();
            crate::protocol_serde::shape_launch_template_license_configuration_request::ser_launch_template_license_configuration_request(
                entry_72, item_70,
            )?;
        }
        list_71.finish();
    }
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("HibernationOptions");
    if let Some(var_74) = &input.hibernation_options {
        crate::protocol_serde::shape_launch_template_hibernation_options_request::ser_launch_template_hibernation_options_request(scope_73, var_74)?;
    }
    #[allow(unused_mut)]
    let mut scope_75 = writer.prefix("MetadataOptions");
    if let Some(var_76) = &input.metadata_options {
        crate::protocol_serde::shape_launch_template_instance_metadata_options_request::ser_launch_template_instance_metadata_options_request(
            scope_75, var_76,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("EnclaveOptions");
    if let Some(var_78) = &input.enclave_options {
        crate::protocol_serde::shape_launch_template_enclave_options_request::ser_launch_template_enclave_options_request(scope_77, var_78)?;
    }
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("InstanceRequirements");
    if let Some(var_80) = &input.instance_requirements {
        crate::protocol_serde::shape_instance_requirements_request::ser_instance_requirements_request(scope_79, var_80)?;
    }
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("PrivateDnsNameOptions");
    if let Some(var_82) = &input.private_dns_name_options {
        crate::protocol_serde::shape_launch_template_private_dns_name_options_request::ser_launch_template_private_dns_name_options_request(
            scope_81, var_82,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("MaintenanceOptions");
    if let Some(var_84) = &input.maintenance_options {
        crate::protocol_serde::shape_launch_template_instance_maintenance_options_request::ser_launch_template_instance_maintenance_options_request(
            scope_83, var_84,
        )?;
    }
    #[allow(unused_mut)]
    let mut scope_85 = writer.prefix("DisableApiStop");
    if let Some(var_86) = &input.disable_api_stop {
        scope_85.boolean(*var_86);
    }
    Ok(())
}