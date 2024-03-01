// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_available_capacity(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::AvailableCapacity, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::AvailableCapacity::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("availableInstanceCapacity") /* AvailableInstanceCapacity com.amazonaws.ec2#AvailableCapacity$AvailableInstanceCapacity */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_available_instance_capacity_list::de_available_instance_capacity_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_available_instance_capacity(var_1);
            }
            ,
            s if s.matches("availableVCpus") /* AvailableVCpus com.amazonaws.ec2#AvailableCapacity$AvailableVCpus */ =>  {
                let var_2 =
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
                builder = builder.set_available_v_cpus(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
