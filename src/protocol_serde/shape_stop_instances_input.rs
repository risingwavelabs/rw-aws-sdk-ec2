// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_instances_input_input_input(
    input: &crate::operation::stop_instances::StopInstancesInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "StopInstances", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("InstanceId");
    if let Some(var_2) = &input.instance_ids {
        let mut list_4 = scope_1.start_list(true, Some("InstanceId"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("Hibernate");
    if let Some(var_7) = &input.hibernate {
        scope_6.boolean(*var_7);
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("DryRun");
    if let Some(var_9) = &input.dry_run {
        scope_8.boolean(*var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("Force");
    if let Some(var_11) = &input.force {
        scope_10.boolean(*var_11);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
