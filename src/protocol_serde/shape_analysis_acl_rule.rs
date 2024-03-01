// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_analysis_acl_rule(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AnalysisAclRule, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AnalysisAclRule::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("cidr") /* Cidr com.amazonaws.ec2#AnalysisAclRule$Cidr */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cidr(var_1);
            }
            ,
            s if s.matches("egress") /* Egress com.amazonaws.ec2#AnalysisAclRule$Egress */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_egress(var_2);
            }
            ,
            s if s.matches("portRange") /* PortRange com.amazonaws.ec2#AnalysisAclRule$PortRange */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_port_range::de_port_range(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_port_range(var_3);
            }
            ,
            s if s.matches("protocol") /* Protocol com.amazonaws.ec2#AnalysisAclRule$Protocol */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_protocol(var_4);
            }
            ,
            s if s.matches("ruleAction") /* RuleAction com.amazonaws.ec2#AnalysisAclRule$RuleAction */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_rule_action(var_5);
            }
            ,
            s if s.matches("ruleNumber") /* RuleNumber com.amazonaws.ec2#AnalysisAclRule$RuleNumber */ =>  {
                let var_6 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_rule_number(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
