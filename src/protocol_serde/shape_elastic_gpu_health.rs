// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_elastic_gpu_health(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::ElasticGpuHealth, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ElasticGpuHealth::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("status") /* Status com.amazonaws.ec2#ElasticGpuHealth$Status */ =>  {
                let var_1 =
                    Some(
                        Result::<crate::types::ElasticGpuStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ElasticGpuStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}