// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_ipam_pool_input_input_input(
    input: &crate::operation::create_ipam_pool::CreateIpamPoolInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateIpamPool", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("IpamScopeId");
    if let Some(var_4) = &input.ipam_scope_id {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Locale");
    if let Some(var_6) = &input.locale {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("SourceIpamPoolId");
    if let Some(var_8) = &input.source_ipam_pool_id {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Description");
    if let Some(var_10) = &input.description {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("AddressFamily");
    if let Some(var_12) = &input.address_family {
        scope_11.string(var_12.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("AutoImport");
    if let Some(var_14) = &input.auto_import {
        scope_13.boolean(*var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("PubliclyAdvertisable");
    if let Some(var_16) = &input.publicly_advertisable {
        scope_15.boolean(*var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("AllocationMinNetmaskLength");
    if let Some(var_18) = &input.allocation_min_netmask_length {
        scope_17.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_18).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("AllocationMaxNetmaskLength");
    if let Some(var_20) = &input.allocation_max_netmask_length {
        scope_19.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_20).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("AllocationDefaultNetmaskLength");
    if let Some(var_22) = &input.allocation_default_netmask_length {
        scope_21.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_22).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("AllocationResourceTag");
    if let Some(var_24) = &input.allocation_resource_tags {
        let mut list_26 = scope_23.start_list(true, Some("item"));
        for item_25 in var_24 {
            #[allow(unused_mut)]
            let mut entry_27 = list_26.entry();
            crate::protocol_serde::shape_request_ipam_resource_tag::ser_request_ipam_resource_tag(entry_27, item_25)?;
        }
        list_26.finish();
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("TagSpecification");
    if let Some(var_29) = &input.tag_specifications {
        let mut list_31 = scope_28.start_list(true, Some("item"));
        for item_30 in var_29 {
            #[allow(unused_mut)]
            let mut entry_32 = list_31.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_32, item_30)?;
        }
        list_31.finish();
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("ClientToken");
    if let Some(var_34) = &input.client_token {
        scope_33.string(var_34);
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("AwsService");
    if let Some(var_36) = &input.aws_service {
        scope_35.string(var_36.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("PublicIpSource");
    if let Some(var_38) = &input.public_ip_source {
        scope_37.string(var_38.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("SourceResource");
    if let Some(var_40) = &input.source_resource {
        crate::protocol_serde::shape_ipam_pool_source_resource_request::ser_ipam_pool_source_resource_request(scope_39, var_40)?;
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
