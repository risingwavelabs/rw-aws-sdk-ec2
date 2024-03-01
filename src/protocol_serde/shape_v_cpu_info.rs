// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_v_cpu_info(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::VCpuInfo, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::VCpuInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("defaultVCpus") /* DefaultVCpus com.amazonaws.ec2#VCpuInfo$DefaultVCpus */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#VCpuCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_default_v_cpus(var_1);
            }
            ,
            s if s.matches("defaultCores") /* DefaultCores com.amazonaws.ec2#VCpuInfo$DefaultCores */ =>  {
                let var_2 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#CoreCount`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_default_cores(var_2);
            }
            ,
            s if s.matches("defaultThreadsPerCore") /* DefaultThreadsPerCore com.amazonaws.ec2#VCpuInfo$DefaultThreadsPerCore */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#ThreadsPerCore`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_default_threads_per_core(var_3);
            }
            ,
            s if s.matches("validCores") /* ValidCores com.amazonaws.ec2#VCpuInfo$ValidCores */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_core_count_list::de_core_count_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_valid_cores(var_4);
            }
            ,
            s if s.matches("validThreadsPerCore") /* ValidThreadsPerCore com.amazonaws.ec2#VCpuInfo$ValidThreadsPerCore */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_threads_per_core_list::de_threads_per_core_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_valid_threads_per_core(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
