// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_rule_option(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::RuleOption, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::RuleOption::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("keyword") /* Keyword com.amazonaws.ec2#RuleOption$Keyword */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_keyword(var_1);
            }
            ,
            s if s.matches("settingSet") /* Settings com.amazonaws.ec2#RuleOption$Settings */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_string_list::de_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_settings(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}