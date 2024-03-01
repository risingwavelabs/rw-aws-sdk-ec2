// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_recurring_charge(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::RecurringCharge, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::RecurringCharge::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("amount") /* Amount com.amazonaws.ec2#RecurringCharge$Amount */ =>  {
                let var_1 =
                    Some(
                         {
                            <f64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.ec2#Double`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_amount(var_1);
            }
            ,
            s if s.matches("frequency") /* Frequency com.amazonaws.ec2#RecurringCharge$Frequency */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::RecurringChargeFrequency, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::RecurringChargeFrequency::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_frequency(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}