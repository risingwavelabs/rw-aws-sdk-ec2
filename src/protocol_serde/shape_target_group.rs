// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_target_group(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::TargetGroup,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Arn");
    if let Some(var_2) = &input.arn {
        scope_1.string(var_2);
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_target_group(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::TargetGroup, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::TargetGroup::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("arn") /* Arn com.amazonaws.ec2#TargetGroup$Arn */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_arn(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}