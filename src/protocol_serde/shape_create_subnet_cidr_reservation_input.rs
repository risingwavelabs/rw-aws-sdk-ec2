// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_subnet_cidr_reservation_input_input_input(
    input: &crate::operation::create_subnet_cidr_reservation::CreateSubnetCidrReservationInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateSubnetCidrReservation", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SubnetId");
    if let Some(var_2) = &input.subnet_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Cidr");
    if let Some(var_4) = &input.cidr {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ReservationType");
    if let Some(var_6) = &input.reservation_type {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Description");
    if let Some(var_8) = &input.description {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DryRun");
    if let Some(var_10) = &input.dry_run {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("TagSpecification");
    if let Some(var_12) = &input.tag_specifications {
        let mut list_14 = scope_11.start_list(true, Some("item"));
        for item_13 in var_12 {
            #[allow(unused_mut)]
            let mut entry_15 = list_14.entry();
            crate::protocol_serde::shape_tag_specification::ser_tag_specification(entry_15, item_13)?;
        }
        list_14.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}