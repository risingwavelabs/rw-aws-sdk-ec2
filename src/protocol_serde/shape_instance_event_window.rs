// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_instance_event_window(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InstanceEventWindow, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InstanceEventWindow::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceEventWindowId") /* InstanceEventWindowId com.amazonaws.ec2#InstanceEventWindow$InstanceEventWindowId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_event_window_id(var_1);
            }
            ,
            s if s.matches("timeRangeSet") /* TimeRanges com.amazonaws.ec2#InstanceEventWindow$TimeRanges */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_instance_event_window_time_range_list::de_instance_event_window_time_range_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_time_ranges(var_2);
            }
            ,
            s if s.matches("name") /* Name com.amazonaws.ec2#InstanceEventWindow$Name */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_3);
            }
            ,
            s if s.matches("cronExpression") /* CronExpression com.amazonaws.ec2#InstanceEventWindow$CronExpression */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cron_expression(var_4);
            }
            ,
            s if s.matches("associationTarget") /* AssociationTarget com.amazonaws.ec2#InstanceEventWindow$AssociationTarget */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_instance_event_window_association_target::de_instance_event_window_association_target(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_association_target(var_5);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#InstanceEventWindow$State */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::InstanceEventWindowState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InstanceEventWindowState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_6);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#InstanceEventWindow$Tags */ =>  {
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