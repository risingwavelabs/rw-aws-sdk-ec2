// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_instance_event_window_association_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::InstanceEventWindowAssociationRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("InstanceId");
    if let Some(var_2) = &input.instance_ids {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("InstanceTag");
    if let Some(var_7) = &input.instance_tags {
        let mut list_9 = scope_6.start_list(true, Some("item"));
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_10, item_8)?;
        }
        list_9.finish();
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("DedicatedHostId");
    if let Some(var_12) = &input.dedicated_host_ids {
        let mut list_14 = scope_11.start_list(true, Some("item"));
        for item_13 in var_12 {
            #[allow(unused_mut)]
            let mut entry_15 = list_14.entry();
            entry_15.string(item_13);
        }
        list_14.finish();
    }
    Ok(())
}
