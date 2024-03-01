// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cancel_spot_fleet_requests_input_input_input(
    input: &crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CancelSpotFleetRequests", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SpotFleetRequestId");
    if let Some(var_4) = &input.spot_fleet_request_ids {
        let mut list_6 = scope_3.start_list(true, Some("item"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("TerminateInstances");
    if let Some(var_9) = &input.terminate_instances {
        scope_8.boolean(*var_9);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
