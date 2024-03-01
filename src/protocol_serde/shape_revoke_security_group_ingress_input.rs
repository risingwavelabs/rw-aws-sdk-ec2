// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_revoke_security_group_ingress_input_input_input(
    input: &crate::operation::revoke_security_group_ingress::RevokeSecurityGroupIngressInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "RevokeSecurityGroupIngress", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CidrIp");
    if let Some(var_2) = &input.cidr_ip {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("FromPort");
    if let Some(var_4) = &input.from_port {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("GroupId");
    if let Some(var_6) = &input.group_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("GroupName");
    if let Some(var_8) = &input.group_name {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("IpPermissions");
    if let Some(var_10) = &input.ip_permissions {
        let mut list_12 = scope_9.start_list(true, Some("item"));
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            crate::protocol_serde::shape_ip_permission::ser_ip_permission(entry_13, item_11)?;
        }
        list_12.finish();
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("IpProtocol");
    if let Some(var_15) = &input.ip_protocol {
        scope_14.string(var_15);
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("SourceSecurityGroupName");
    if let Some(var_17) = &input.source_security_group_name {
        scope_16.string(var_17);
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("SourceSecurityGroupOwnerId");
    if let Some(var_19) = &input.source_security_group_owner_id {
        scope_18.string(var_19);
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
    let mut scope_22 = writer.prefix("DryRun");
    if let Some(var_23) = &input.dry_run {
        scope_22.boolean(*var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("SecurityGroupRuleId");
    if let Some(var_25) = &input.security_group_rule_ids {
        let mut list_27 = scope_24.start_list(true, Some("item"));
        for item_26 in var_25 {
            #[allow(unused_mut)]
            let mut entry_28 = list_27.entry();
            entry_28.string(item_26);
        }
        list_27.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
