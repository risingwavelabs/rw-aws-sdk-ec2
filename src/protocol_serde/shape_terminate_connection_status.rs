// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_terminate_connection_status(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TerminateConnectionStatus, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TerminateConnectionStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("connectionId") /* ConnectionId com.amazonaws.ec2#TerminateConnectionStatus$ConnectionId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_connection_id(var_1);
            }
            ,
            s if s.matches("previousStatus") /* PreviousStatus com.amazonaws.ec2#TerminateConnectionStatus$PreviousStatus */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_client_vpn_connection_status::de_client_vpn_connection_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_previous_status(var_2);
            }
            ,
            s if s.matches("currentStatus") /* CurrentStatus com.amazonaws.ec2#TerminateConnectionStatus$CurrentStatus */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_client_vpn_connection_status::de_client_vpn_connection_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_current_status(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}