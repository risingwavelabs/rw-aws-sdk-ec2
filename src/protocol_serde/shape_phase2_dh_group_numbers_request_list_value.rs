// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_phase2_dh_group_numbers_request_list_value(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::Phase2DhGroupNumbersRequestListValue,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Value");
    if let Some(var_2) = &input.value {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    Ok(())
}
