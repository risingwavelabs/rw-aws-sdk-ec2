// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_connection_notification(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ConnectionNotification, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ConnectionNotification::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("connectionNotificationId") /* ConnectionNotificationId com.amazonaws.ec2#ConnectionNotification$ConnectionNotificationId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_connection_notification_id(var_1);
            }
            ,
            s if s.matches("serviceId") /* ServiceId com.amazonaws.ec2#ConnectionNotification$ServiceId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_service_id(var_2);
            }
            ,
            s if s.matches("vpcEndpointId") /* VpcEndpointId com.amazonaws.ec2#ConnectionNotification$VpcEndpointId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_endpoint_id(var_3);
            }
            ,
            s if s.matches("connectionNotificationType") /* ConnectionNotificationType com.amazonaws.ec2#ConnectionNotification$ConnectionNotificationType */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::ConnectionNotificationType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ConnectionNotificationType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_connection_notification_type(var_4);
            }
            ,
            s if s.matches("connectionNotificationArn") /* ConnectionNotificationArn com.amazonaws.ec2#ConnectionNotification$ConnectionNotificationArn */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_connection_notification_arn(var_5);
            }
            ,
            s if s.matches("connectionEvents") /* ConnectionEvents com.amazonaws.ec2#ConnectionNotification$ConnectionEvents */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_connection_events(var_6);
            }
            ,
            s if s.matches("connectionNotificationState") /* ConnectionNotificationState com.amazonaws.ec2#ConnectionNotification$ConnectionNotificationState */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::types::ConnectionNotificationState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ConnectionNotificationState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_connection_notification_state(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
