// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_baseline_ebs_bandwidth_mbps_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::BaselineEbsBandwidthMbpsRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Min");
    if let Some(var_2) = &input.min {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Max");
    if let Some(var_4) = &input.max {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    Ok(())
}
