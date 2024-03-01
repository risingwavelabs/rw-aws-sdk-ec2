// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_fast_launch_launch_template_specification_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::FastLaunchLaunchTemplateSpecificationRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LaunchTemplateId");
    if let Some(var_2) = &input.launch_template_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("LaunchTemplateName");
    if let Some(var_4) = &input.launch_template_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Version");
    if let Some(var_6) = &input.version {
        scope_5.string(var_6);
    }
    Ok(())
}
