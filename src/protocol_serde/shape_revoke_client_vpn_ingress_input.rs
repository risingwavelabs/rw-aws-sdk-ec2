// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_revoke_client_vpn_ingress_input_input_input(
    input: &crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "RevokeClientVpnIngress", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClientVpnEndpointId");
    if let Some(var_2) = &input.client_vpn_endpoint_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TargetNetworkCidr");
    if let Some(var_4) = &input.target_network_cidr {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AccessGroupId");
    if let Some(var_6) = &input.access_group_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("RevokeAllGroups");
    if let Some(var_8) = &input.revoke_all_groups {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DryRun");
    if let Some(var_10) = &input.dry_run {
        scope_9.boolean(*var_10);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
