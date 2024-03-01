// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_transit_gateway_multicast_domain_options(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TransitGatewayMulticastDomainOptions, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TransitGatewayMulticastDomainOptions::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("igmpv2Support") /* Igmpv2Support com.amazonaws.ec2#TransitGatewayMulticastDomainOptions$Igmpv2Support */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::Igmpv2SupportValue, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::Igmpv2SupportValue::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_igmpv2_support(var_1);
            }
            ,
            s if s.matches("staticSourcesSupport") /* StaticSourcesSupport com.amazonaws.ec2#TransitGatewayMulticastDomainOptions$StaticSourcesSupport */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::StaticSourcesSupportValue, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::StaticSourcesSupportValue::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_static_sources_support(var_2);
            }
            ,
            s if s.matches("autoAcceptSharedAssociations") /* AutoAcceptSharedAssociations com.amazonaws.ec2#TransitGatewayMulticastDomainOptions$AutoAcceptSharedAssociations */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::AutoAcceptSharedAssociationsValue, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::AutoAcceptSharedAssociationsValue::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_auto_accept_shared_associations(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}