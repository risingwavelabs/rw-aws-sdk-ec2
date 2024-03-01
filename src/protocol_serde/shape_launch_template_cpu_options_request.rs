// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_launch_template_cpu_options_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::LaunchTemplateCpuOptionsRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CoreCount");
    if let Some(var_2) = &input.core_count {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ThreadsPerCore");
    if let Some(var_4) = &input.threads_per_core {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AmdSevSnp");
    if let Some(var_6) = &input.amd_sev_snp {
        scope_5.string(var_6.as_str());
    }
    Ok(())
}
