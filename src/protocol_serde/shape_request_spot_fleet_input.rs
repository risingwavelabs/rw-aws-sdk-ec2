// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_request_spot_fleet_input_input_input(
    input: &crate::operation::request_spot_fleet::RequestSpotFleetInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "RequestSpotFleet", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SpotFleetRequestConfig");
    if let Some(var_4) = &input.spot_fleet_request_config {
        crate::protocol_serde::shape_spot_fleet_request_config_data::ser_spot_fleet_request_config_data(scope_3, var_4)?;
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}