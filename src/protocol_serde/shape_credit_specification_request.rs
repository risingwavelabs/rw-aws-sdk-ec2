// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_credit_specification_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::CreditSpecificationRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CpuCredits");
    if let Some(var_2) = &input.cpu_credits {
        scope_1.string(var_2);
    }
    Ok(())
}
