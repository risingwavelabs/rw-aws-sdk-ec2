// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_enable_fast_snapshot_restore_state_error_set(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<::std::vec::Vec<crate::types::EnableFastSnapshotRestoreStateErrorItem>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#EnableFastSnapshotRestoreStateErrorSet$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_enable_fast_snapshot_restore_state_error_item::de_enable_fast_snapshot_restore_state_error_item(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
