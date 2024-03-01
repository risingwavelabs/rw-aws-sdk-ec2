// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_conversion_task(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ConversionTask, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ConversionTask::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("conversionTaskId") /* ConversionTaskId com.amazonaws.ec2#ConversionTask$ConversionTaskId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_conversion_task_id(var_1);
            }
            ,
            s if s.matches("expirationTime") /* ExpirationTime com.amazonaws.ec2#ConversionTask$ExpirationTime */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_expiration_time(var_2);
            }
            ,
            s if s.matches("importInstance") /* ImportInstance com.amazonaws.ec2#ConversionTask$ImportInstance */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_import_instance_task_details::de_import_instance_task_details(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_import_instance(var_3);
            }
            ,
            s if s.matches("importVolume") /* ImportVolume com.amazonaws.ec2#ConversionTask$ImportVolume */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_import_volume_task_details::de_import_volume_task_details(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_import_volume(var_4);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#ConversionTask$State */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::ConversionTaskState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ConversionTaskState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_5);
            }
            ,
            s if s.matches("statusMessage") /* StatusMessage com.amazonaws.ec2#ConversionTask$StatusMessage */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_message(var_6);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#ConversionTask$Tags */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
