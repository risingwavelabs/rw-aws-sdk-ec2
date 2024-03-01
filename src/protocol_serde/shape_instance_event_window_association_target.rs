// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_instance_event_window_association_target(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::InstanceEventWindowAssociationTarget, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::InstanceEventWindowAssociationTarget::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceIdSet") /* InstanceIds com.amazonaws.ec2#InstanceEventWindowAssociationTarget$InstanceIds */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_instance_id_list::de_instance_id_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_ids(var_1);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#InstanceEventWindowAssociationTarget$Tags */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_2);
            }
            ,
            s if s.matches("dedicatedHostIdSet") /* DedicatedHostIds com.amazonaws.ec2#InstanceEventWindowAssociationTarget$DedicatedHostIds */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_dedicated_host_id_list::de_dedicated_host_id_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_dedicated_host_ids(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
