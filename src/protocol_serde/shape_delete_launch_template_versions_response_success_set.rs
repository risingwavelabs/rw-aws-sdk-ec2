// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_delete_launch_template_versions_response_success_set(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<::std::vec::Vec<crate::types::DeleteLaunchTemplateVersionsResponseSuccessItem>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#DeleteLaunchTemplateVersionsResponseSuccessSet$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_delete_launch_template_versions_response_success_item::de_delete_launch_template_versions_response_success_item(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
