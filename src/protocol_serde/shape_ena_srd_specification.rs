// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_ena_srd_specification(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::EnaSrdSpecification,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EnaSrdEnabled");
    if let Some(var_2) = &input.ena_srd_enabled {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("EnaSrdUdpSpecification");
    if let Some(var_4) = &input.ena_srd_udp_specification {
        crate::protocol_serde::shape_ena_srd_udp_specification::ser_ena_srd_udp_specification(scope_3, var_4)?;
    }
    Ok(())
}
