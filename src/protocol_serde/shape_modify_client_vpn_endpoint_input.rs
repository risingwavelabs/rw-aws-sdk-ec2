// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_client_vpn_endpoint_input_input_input(
    input: &crate::operation::modify_client_vpn_endpoint::ModifyClientVpnEndpointInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyClientVpnEndpoint", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClientVpnEndpointId");
    if let Some(var_2) = &input.client_vpn_endpoint_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ServerCertificateArn");
    if let Some(var_4) = &input.server_certificate_arn {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ConnectionLogOptions");
    if let Some(var_6) = &input.connection_log_options {
        crate::protocol_serde::shape_connection_log_options::ser_connection_log_options(scope_5, var_6)?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DnsServers");
    if let Some(var_8) = &input.dns_servers {
        crate::protocol_serde::shape_dns_servers_options_modify_structure::ser_dns_servers_options_modify_structure(scope_7, var_8)?;
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("VpnPort");
    if let Some(var_10) = &input.vpn_port {
        scope_9.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_10).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("Description");
    if let Some(var_12) = &input.description {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("SplitTunnel");
    if let Some(var_14) = &input.split_tunnel {
        scope_13.boolean(*var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("DryRun");
    if let Some(var_16) = &input.dry_run {
        scope_15.boolean(*var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("SecurityGroupId");
    if let Some(var_18) = &input.security_group_ids {
        let mut list_20 = scope_17.start_list(true, Some("item"));
        for item_19 in var_18 {
            #[allow(unused_mut)]
            let mut entry_21 = list_20.entry();
            entry_21.string(item_19);
        }
        list_20.finish();
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("VpcId");
    if let Some(var_23) = &input.vpc_id {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("SelfServicePortal");
    if let Some(var_25) = &input.self_service_portal {
        scope_24.string(var_25.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("ClientConnectOptions");
    if let Some(var_27) = &input.client_connect_options {
        crate::protocol_serde::shape_client_connect_options::ser_client_connect_options(scope_26, var_27)?;
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("SessionTimeoutHours");
    if let Some(var_29) = &input.session_timeout_hours {
        scope_28.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_29).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("ClientLoginBannerOptions");
    if let Some(var_31) = &input.client_login_banner_options {
        crate::protocol_serde::shape_client_login_banner_options::ser_client_login_banner_options(scope_30, var_31)?;
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
