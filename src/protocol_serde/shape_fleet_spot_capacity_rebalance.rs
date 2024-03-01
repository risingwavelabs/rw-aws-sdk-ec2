// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_fleet_spot_capacity_rebalance(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::FleetSpotCapacityRebalance, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::FleetSpotCapacityRebalance::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("replacementStrategy") /* ReplacementStrategy com.amazonaws.ec2#FleetSpotCapacityRebalance$ReplacementStrategy */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::FleetReplacementStrategy, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::FleetReplacementStrategy::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_replacement_strategy(var_1);
            }
            ,
            s if s.matches("terminationDelay") /* TerminationDelay com.amazonaws.ec2#FleetSpotCapacityRebalance$TerminationDelay */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_termination_delay(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
