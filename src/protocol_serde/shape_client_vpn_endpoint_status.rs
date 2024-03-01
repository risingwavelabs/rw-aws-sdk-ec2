// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_client_vpn_endpoint_status(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ClientVpnEndpointStatus, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ClientVpnEndpointStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("code") /* Code com.amazonaws.ec2#ClientVpnEndpointStatus$Code */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::ClientVpnEndpointStatusCode, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ClientVpnEndpointStatusCode::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_code(var_1);
            }
            ,
            s if s.matches("message") /* Message com.amazonaws.ec2#ClientVpnEndpointStatus$Message */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_message(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
