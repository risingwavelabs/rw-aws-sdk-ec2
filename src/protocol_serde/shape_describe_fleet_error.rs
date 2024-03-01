// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_describe_fleet_error(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::DescribeFleetError, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DescribeFleetError::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("launchTemplateAndOverrides") /* LaunchTemplateAndOverrides com.amazonaws.ec2#DescribeFleetError$LaunchTemplateAndOverrides */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_launch_template_and_overrides_response::de_launch_template_and_overrides_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_launch_template_and_overrides(var_1);
            }
            ,
            s if s.matches("lifecycle") /* Lifecycle com.amazonaws.ec2#DescribeFleetError$Lifecycle */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::InstanceLifecycle, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InstanceLifecycle::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_lifecycle(var_2);
            }
            ,
            s if s.matches("errorCode") /* ErrorCode com.amazonaws.ec2#DescribeFleetError$ErrorCode */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_error_code(var_3);
            }
            ,
            s if s.matches("errorMessage") /* ErrorMessage com.amazonaws.ec2#DescribeFleetError$ErrorMessage */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_error_message(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}