// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_analysis_packet_header(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AnalysisPacketHeader, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AnalysisPacketHeader::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("destinationAddressSet") /* DestinationAddresses com.amazonaws.ec2#AnalysisPacketHeader$DestinationAddresses */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_ip_address_list::de_ip_address_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_destination_addresses(var_1);
            }
            ,
            s if s.matches("destinationPortRangeSet") /* DestinationPortRanges com.amazonaws.ec2#AnalysisPacketHeader$DestinationPortRanges */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_port_range_list::de_port_range_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_destination_port_ranges(var_2);
            }
            ,
            s if s.matches("protocol") /* Protocol com.amazonaws.ec2#AnalysisPacketHeader$Protocol */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_protocol(var_3);
            }
            ,
            s if s.matches("sourceAddressSet") /* SourceAddresses com.amazonaws.ec2#AnalysisPacketHeader$SourceAddresses */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_ip_address_list::de_ip_address_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_source_addresses(var_4);
            }
            ,
            s if s.matches("sourcePortRangeSet") /* SourcePortRanges com.amazonaws.ec2#AnalysisPacketHeader$SourcePortRanges */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_port_range_list::de_port_range_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_source_port_ranges(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
