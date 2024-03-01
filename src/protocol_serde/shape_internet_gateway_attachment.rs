// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_internet_gateway_attachment(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InternetGatewayAttachment, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InternetGatewayAttachment::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("state") /* State com.amazonaws.ec2#InternetGatewayAttachment$State */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::AttachmentStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::AttachmentStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_1);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2#InternetGatewayAttachment$VpcId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
