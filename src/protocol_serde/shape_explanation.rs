// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_explanation(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Explanation, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Explanation::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("acl") /* Acl com.amazonaws.ec2#Explanation$Acl */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_acl(var_1);
            }
            ,
            s if s.matches("aclRule") /* AclRule com.amazonaws.ec2#Explanation$AclRule */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_analysis_acl_rule::de_analysis_acl_rule(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_acl_rule(var_2);
            }
            ,
            s if s.matches("address") /* Address com.amazonaws.ec2#Explanation$Address */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_address(var_3);
            }
            ,
            s if s.matches("addressSet") /* Addresses com.amazonaws.ec2#Explanation$Addresses */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_ip_address_list::de_ip_address_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_addresses(var_4);
            }
            ,
            s if s.matches("attachedTo") /* AttachedTo com.amazonaws.ec2#Explanation$AttachedTo */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_attached_to(var_5);
            }
            ,
            s if s.matches("availabilityZoneSet") /* AvailabilityZones com.amazonaws.ec2#Explanation$AvailabilityZones */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zones(var_6);
            }
            ,
            s if s.matches("cidrSet") /* Cidrs com.amazonaws.ec2#Explanation$Cidrs */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cidrs(var_7);
            }
            ,
            s if s.matches("component") /* Component com.amazonaws.ec2#Explanation$Component */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_component(var_8);
            }
            ,
            s if s.matches("customerGateway") /* CustomerGateway com.amazonaws.ec2#Explanation$CustomerGateway */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_customer_gateway(var_9);
            }
            ,
            s if s.matches("destination") /* Destination com.amazonaws.ec2#Explanation$Destination */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_destination(var_10);
            }
            ,
            s if s.matches("destinationVpc") /* DestinationVpc com.amazonaws.ec2#Explanation$DestinationVpc */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_destination_vpc(var_11);
            }
            ,
            s if s.matches("direction") /* Direction com.amazonaws.ec2#Explanation$Direction */ =>  {
                let var_12 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_direction(var_12);
            }
            ,
            s if s.matches("explanationCode") /* ExplanationCode com.amazonaws.ec2#Explanation$ExplanationCode */ =>  {
                let var_13 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_explanation_code(var_13);
            }
            ,
            s if s.matches("ingressRouteTable") /* IngressRouteTable com.amazonaws.ec2#Explanation$IngressRouteTable */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ingress_route_table(var_14);
            }
            ,
            s if s.matches("internetGateway") /* InternetGateway com.amazonaws.ec2#Explanation$InternetGateway */ =>  {
                let var_15 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_internet_gateway(var_15);
            }
            ,
            s if s.matches("loadBalancerArn") /* LoadBalancerArn com.amazonaws.ec2#Explanation$LoadBalancerArn */ =>  {
                let var_16 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_load_balancer_arn(var_16);
            }
            ,
            s if s.matches("classicLoadBalancerListener") /* ClassicLoadBalancerListener com.amazonaws.ec2#Explanation$ClassicLoadBalancerListener */ =>  {
                let var_17 =
                    Some(
                        crate::protocol_serde::shape_analysis_load_balancer_listener::de_analysis_load_balancer_listener(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_classic_load_balancer_listener(var_17);
            }
            ,
            s if s.matches("loadBalancerListenerPort") /* LoadBalancerListenerPort com.amazonaws.ec2#Explanation$LoadBalancerListenerPort */ =>  {
                let var_18 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Port`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_load_balancer_listener_port(var_18);
            }
            ,
            s if s.matches("loadBalancerTarget") /* LoadBalancerTarget com.amazonaws.ec2#Explanation$LoadBalancerTarget */ =>  {
                let var_19 =
                    Some(
                        crate::protocol_serde::shape_analysis_load_balancer_target::de_analysis_load_balancer_target(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_load_balancer_target(var_19);
            }
            ,
            s if s.matches("loadBalancerTargetGroup") /* LoadBalancerTargetGroup com.amazonaws.ec2#Explanation$LoadBalancerTargetGroup */ =>  {
                let var_20 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_load_balancer_target_group(var_20);
            }
            ,
            s if s.matches("loadBalancerTargetGroupSet") /* LoadBalancerTargetGroups com.amazonaws.ec2#Explanation$LoadBalancerTargetGroups */ =>  {
                let var_21 =
                    Some(
                        crate::protocol_serde::shape_analysis_component_list::de_analysis_component_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_load_balancer_target_groups(var_21);
            }
            ,
            s if s.matches("loadBalancerTargetPort") /* LoadBalancerTargetPort com.amazonaws.ec2#Explanation$LoadBalancerTargetPort */ =>  {
                let var_22 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Port`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_load_balancer_target_port(var_22);
            }
            ,
            s if s.matches("elasticLoadBalancerListener") /* ElasticLoadBalancerListener com.amazonaws.ec2#Explanation$ElasticLoadBalancerListener */ =>  {
                let var_23 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_elastic_load_balancer_listener(var_23);
            }
            ,
            s if s.matches("missingComponent") /* MissingComponent com.amazonaws.ec2#Explanation$MissingComponent */ =>  {
                let var_24 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_missing_component(var_24);
            }
            ,
            s if s.matches("natGateway") /* NatGateway com.amazonaws.ec2#Explanation$NatGateway */ =>  {
                let var_25 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_nat_gateway(var_25);
            }
            ,
            s if s.matches("networkInterface") /* NetworkInterface com.amazonaws.ec2#Explanation$NetworkInterface */ =>  {
                let var_26 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_network_interface(var_26);
            }
            ,
            s if s.matches("packetField") /* PacketField com.amazonaws.ec2#Explanation$PacketField */ =>  {
                let var_27 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_packet_field(var_27);
            }
            ,
            s if s.matches("vpcPeeringConnection") /* VpcPeeringConnection com.amazonaws.ec2#Explanation$VpcPeeringConnection */ =>  {
                let var_28 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpc_peering_connection(var_28);
            }
            ,
            s if s.matches("port") /* Port com.amazonaws.ec2#Explanation$Port */ =>  {
                let var_29 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Port`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_port(var_29);
            }
            ,
            s if s.matches("portRangeSet") /* PortRanges com.amazonaws.ec2#Explanation$PortRanges */ =>  {
                let var_30 =
                    Some(
                        crate::protocol_serde::shape_port_range_list::de_port_range_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_port_ranges(var_30);
            }
            ,
            s if s.matches("prefixList") /* PrefixList com.amazonaws.ec2#Explanation$PrefixList */ =>  {
                let var_31 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_prefix_list(var_31);
            }
            ,
            s if s.matches("protocolSet") /* Protocols com.amazonaws.ec2#Explanation$Protocols */ =>  {
                let var_32 =
                    Some(
                        crate::protocol_serde::shape_string_list::de_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_protocols(var_32);
            }
            ,
            s if s.matches("routeTableRoute") /* RouteTableRoute com.amazonaws.ec2#Explanation$RouteTableRoute */ =>  {
                let var_33 =
                    Some(
                        crate::protocol_serde::shape_analysis_route_table_route::de_analysis_route_table_route(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_route_table_route(var_33);
            }
            ,
            s if s.matches("routeTable") /* RouteTable com.amazonaws.ec2#Explanation$RouteTable */ =>  {
                let var_34 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_route_table(var_34);
            }
            ,
            s if s.matches("securityGroup") /* SecurityGroup com.amazonaws.ec2#Explanation$SecurityGroup */ =>  {
                let var_35 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_group(var_35);
            }
            ,
            s if s.matches("securityGroupRule") /* SecurityGroupRule com.amazonaws.ec2#Explanation$SecurityGroupRule */ =>  {
                let var_36 =
                    Some(
                        crate::protocol_serde::shape_analysis_security_group_rule::de_analysis_security_group_rule(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_group_rule(var_36);
            }
            ,
            s if s.matches("securityGroupSet") /* SecurityGroups com.amazonaws.ec2#Explanation$SecurityGroups */ =>  {
                let var_37 =
                    Some(
                        crate::protocol_serde::shape_analysis_component_list::de_analysis_component_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_groups(var_37);
            }
            ,
            s if s.matches("sourceVpc") /* SourceVpc com.amazonaws.ec2#Explanation$SourceVpc */ =>  {
                let var_38 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_source_vpc(var_38);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#Explanation$State */ =>  {
                let var_39 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_39);
            }
            ,
            s if s.matches("subnet") /* Subnet com.amazonaws.ec2#Explanation$Subnet */ =>  {
                let var_40 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_subnet(var_40);
            }
            ,
            s if s.matches("subnetRouteTable") /* SubnetRouteTable com.amazonaws.ec2#Explanation$SubnetRouteTable */ =>  {
                let var_41 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_subnet_route_table(var_41);
            }
            ,
            s if s.matches("vpc") /* Vpc com.amazonaws.ec2#Explanation$Vpc */ =>  {
                let var_42 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpc(var_42);
            }
            ,
            s if s.matches("vpcEndpoint") /* VpcEndpoint com.amazonaws.ec2#Explanation$VpcEndpoint */ =>  {
                let var_43 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpc_endpoint(var_43);
            }
            ,
            s if s.matches("vpnConnection") /* VpnConnection com.amazonaws.ec2#Explanation$VpnConnection */ =>  {
                let var_44 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpn_connection(var_44);
            }
            ,
            s if s.matches("vpnGateway") /* VpnGateway com.amazonaws.ec2#Explanation$VpnGateway */ =>  {
                let var_45 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpn_gateway(var_45);
            }
            ,
            s if s.matches("transitGateway") /* TransitGateway com.amazonaws.ec2#Explanation$TransitGateway */ =>  {
                let var_46 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_transit_gateway(var_46);
            }
            ,
            s if s.matches("transitGatewayRouteTable") /* TransitGatewayRouteTable com.amazonaws.ec2#Explanation$TransitGatewayRouteTable */ =>  {
                let var_47 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_route_table(var_47);
            }
            ,
            s if s.matches("transitGatewayRouteTableRoute") /* TransitGatewayRouteTableRoute com.amazonaws.ec2#Explanation$TransitGatewayRouteTableRoute */ =>  {
                let var_48 =
                    Some(
                        crate::protocol_serde::shape_transit_gateway_route_table_route::de_transit_gateway_route_table_route(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_route_table_route(var_48);
            }
            ,
            s if s.matches("transitGatewayAttachment") /* TransitGatewayAttachment com.amazonaws.ec2#Explanation$TransitGatewayAttachment */ =>  {
                let var_49 =
                    Some(
                        crate::protocol_serde::shape_analysis_component::de_analysis_component(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_attachment(var_49);
            }
            ,
            s if s.matches("componentAccount") /* ComponentAccount com.amazonaws.ec2#Explanation$ComponentAccount */ =>  {
                let var_50 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_component_account(var_50);
            }
            ,
            s if s.matches("componentRegion") /* ComponentRegion com.amazonaws.ec2#Explanation$ComponentRegion */ =>  {
                let var_51 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_component_region(var_51);
            }
            ,
            s if s.matches("firewallStatelessRule") /* FirewallStatelessRule com.amazonaws.ec2#Explanation$FirewallStatelessRule */ =>  {
                let var_52 =
                    Some(
                        crate::protocol_serde::shape_firewall_stateless_rule::de_firewall_stateless_rule(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_firewall_stateless_rule(var_52);
            }
            ,
            s if s.matches("firewallStatefulRule") /* FirewallStatefulRule com.amazonaws.ec2#Explanation$FirewallStatefulRule */ =>  {
                let var_53 =
                    Some(
                        crate::protocol_serde::shape_firewall_stateful_rule::de_firewall_stateful_rule(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_firewall_stateful_rule(var_53);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
