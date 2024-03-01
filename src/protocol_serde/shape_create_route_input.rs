// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_route_input_input_input(
    input: &crate::operation::create_route::CreateRouteInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateRoute", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DestinationCidrBlock");
    if let Some(var_2) = &input.destination_cidr_block {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DestinationIpv6CidrBlock");
    if let Some(var_4) = &input.destination_ipv6_cidr_block {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DestinationPrefixListId");
    if let Some(var_6) = &input.destination_prefix_list_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DryRun");
    if let Some(var_8) = &input.dry_run {
        scope_7.boolean(*var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("VpcEndpointId");
    if let Some(var_10) = &input.vpc_endpoint_id {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("EgressOnlyInternetGatewayId");
    if let Some(var_12) = &input.egress_only_internet_gateway_id {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("GatewayId");
    if let Some(var_14) = &input.gateway_id {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("InstanceId");
    if let Some(var_16) = &input.instance_id {
        scope_15.string(var_16);
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("NatGatewayId");
    if let Some(var_18) = &input.nat_gateway_id {
        scope_17.string(var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("TransitGatewayId");
    if let Some(var_20) = &input.transit_gateway_id {
        scope_19.string(var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("LocalGatewayId");
    if let Some(var_22) = &input.local_gateway_id {
        scope_21.string(var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("CarrierGatewayId");
    if let Some(var_24) = &input.carrier_gateway_id {
        scope_23.string(var_24);
    }
    #[allow(unused_mut)]
    let mut scope_25 = writer.prefix("NetworkInterfaceId");
    if let Some(var_26) = &input.network_interface_id {
        scope_25.string(var_26);
    }
    #[allow(unused_mut)]
    let mut scope_27 = writer.prefix("RouteTableId");
    if let Some(var_28) = &input.route_table_id {
        scope_27.string(var_28);
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("VpcPeeringConnectionId");
    if let Some(var_30) = &input.vpc_peering_connection_id {
        scope_29.string(var_30);
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("CoreNetworkArn");
    if let Some(var_32) = &input.core_network_arn {
        scope_31.string(var_32);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
