// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_spot_price(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::SpotPrice, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SpotPrice::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#SpotPrice$AvailabilityZone */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_1);
            }
            ,
            s if s.matches("instanceType") /* InstanceType com.amazonaws.ec2#SpotPrice$InstanceType */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::InstanceType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InstanceType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_type(var_2);
            }
            ,
            s if s.matches("productDescription") /* ProductDescription com.amazonaws.ec2#SpotPrice$ProductDescription */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::types::RiProductDescription, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::RiProductDescription::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_product_description(var_3);
            }
            ,
            s if s.matches("spotPrice") /* SpotPrice com.amazonaws.ec2#SpotPrice$SpotPrice */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_spot_price(var_4);
            }
            ,
            s if s.matches("timestamp") /* Timestamp com.amazonaws.ec2#SpotPrice$Timestamp */ =>  {
                let var_5 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_timestamp(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
