// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_network_interface_input_input_input(
    input: &crate::operation::create_network_interface::CreateNetworkInterfaceInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateNetworkInterface", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Description");
    if let Some(var_2) = &input.description {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DryRun");
    if let Some(var_4) = &input.dry_run {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("SecurityGroupId");
    if let Some(var_6) = &input.groups {
        let mut list_8 = scope_5.start_list(true, Some("SecurityGroupId"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            entry_9.string(item_7);
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("Ipv6AddressCount");
    if let Some(var_11) = &input.ipv6_address_count {
        scope_10.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_11).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("Ipv6Addresses");
    if let Some(var_13) = &input.ipv6_addresses {
        let mut list_15 = scope_12.start_list(true, Some("item"));
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            crate::protocol_serde::shape_instance_ipv6_address::ser_instance_ipv6_address(entry_16, item_14)?;
        }
        list_15.finish();
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("PrivateIpAddress");
    if let Some(var_18) = &input.private_ip_address {
        scope_17.string(var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("PrivateIpAddresses");
    if let Some(var_20) = &input.private_ip_addresses {
        let mut list_22 = scope_19.start_list(true, Some("item"));
        for item_21 in var_20 {
            #[allow(unused_mut)]
            let mut entry_23 = list_22.entry();
            crate::protocol_serde::shape_private_ip_address_specification::ser_private_ip_address_specification(entry_23, item_21)?;
        }
        list_22.finish();
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("SecondaryPrivateIpAddressCount");
    if let Some(var_25) = &input.secondary_private_ip_address_count {
        scope_24.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_25).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("Ipv4Prefix");
    if let Some(var_27) = &input.ipv4_prefixes {
        let mut list_29 = scope_26.start_list(true, Some("item"));
        for item_28 in var_27 {
            #[allow(unused_mut)]
            let mut entry_30 = list_29.entry();
            crate::protocol_serde::shape_ipv4_prefix_specification_request::ser_ipv4_prefix_specification_request(entry_30, item_28)?;
        }
        list_29.finish();
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("Ipv4PrefixCount");
    if let Some(var_32) = &input.ipv4_prefix_count {
        scope_31.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_32).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("Ipv6Prefix");
    if let Some(var_34) = &input.ipv6_prefixes {
        let mut list_36 = scope_33.start_list(true, Some("item"));
        for item_35 in var_34 {
            #[allow(unused_mut)]
            let mut entry_37 = list_36.entry();
            crate::protocol_serde::shape_ipv6_prefix_specification_request::ser_ipv6_prefix_specification_request(entry_37, item_35)?;
        }
        list_36.finish();
    }
    #[allow(unused_mut)]
    let mut scope_38 = writer.prefix("Ipv6PrefixCount");
    if let Some(var_39) = &input.ipv6_prefix_count {
        scope_38.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_39).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_40 = writer.prefix("InterfaceType");
    if let Some(var_41) = &input.interface_type {
        scope_40.string(var_41.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_42 = writer.prefix("SubnetId");
    if let Some(var_43) = &input.subnet_id {
        scope_42.string(var_43);
    }
    #[allow(unused_mut)]
    let mut scope_44 = writer.prefix("TagSpecification");
    if let Some(var_45) = &input.tag_specifications {
        let mut list_47 = scope_44.start_list(true, Some("item"));
        for item_46 in var_45 {
            #[allow(unused_mut)]
            let mut entry_48 = list_47.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_48, item_46)?;
        }
        list_47.finish();
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("ClientToken");
    if let Some(var_50) = &input.client_token {
        scope_49.string(var_50);
    }
    #[allow(unused_mut)]
    let mut scope_51 = writer.prefix("EnablePrimaryIpv6");
    if let Some(var_52) = &input.enable_primary_ipv6 {
        scope_51.boolean(*var_52);
    }
    #[allow(unused_mut)]
    let mut scope_53 = writer.prefix("ConnectionTrackingSpecification");
    if let Some(var_54) = &input.connection_tracking_specification {
        crate::protocol_serde::shape_connection_tracking_specification_request::ser_connection_tracking_specification_request(scope_53, var_54)?;
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
