// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_ike_versions_request_list_value(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::IkeVersionsRequestListValue,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Value");
    if let Some(var_2) = &input.value {
        scope_1.string(var_2);
    }
    Ok(())
}
