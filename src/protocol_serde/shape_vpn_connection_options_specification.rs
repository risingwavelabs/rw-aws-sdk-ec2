// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_vpn_connection_options_specification(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::VpnConnectionOptionsSpecification,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EnableAcceleration");
    if let Some(var_2) = &input.enable_acceleration {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("StaticRoutesOnly");
    if let Some(var_4) = &input.static_routes_only {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TunnelInsideIpVersion");
    if let Some(var_6) = &input.tunnel_inside_ip_version {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("TunnelOptions");
    if let Some(var_8) = &input.tunnel_options {
        let mut list_10 = scope_7.start_list(true, None);
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            crate::protocol_serde::shape_vpn_tunnel_options_specification::ser_vpn_tunnel_options_specification(entry_11, item_9)?;
        }
        list_10.finish();
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("LocalIpv4NetworkCidr");
    if let Some(var_13) = &input.local_ipv4_network_cidr {
        scope_12.string(var_13);
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("RemoteIpv4NetworkCidr");
    if let Some(var_15) = &input.remote_ipv4_network_cidr {
        scope_14.string(var_15);
    }
    #[allow(unused_mut)]
    let mut scope_16 = writer.prefix("LocalIpv6NetworkCidr");
    if let Some(var_17) = &input.local_ipv6_network_cidr {
        scope_16.string(var_17);
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("RemoteIpv6NetworkCidr");
    if let Some(var_19) = &input.remote_ipv6_network_cidr {
        scope_18.string(var_19);
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("OutsideIpAddressType");
    if let Some(var_21) = &input.outside_ip_address_type {
        scope_20.string(var_21);
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("TransportTransitGatewayAttachmentId");
    if let Some(var_23) = &input.transport_transit_gateway_attachment_id {
        scope_22.string(var_23);
    }
    Ok(())
}