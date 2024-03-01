// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_fpga_images_input_input_input(
    input: &crate::operation::describe_fpga_images::DescribeFpgaImagesInput,
) -> Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeFpgaImages", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("FpgaImageId");
    if let Some(var_4) = &input.fpga_image_ids {
        let mut list_6 = scope_3.start_list(true, Some("item"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("Owner");
    if let Some(var_9) = &input.owners {
        let mut list_11 = scope_8.start_list(true, Some("Owner"));
        for item_10 in var_9 {
            #[allow(unused_mut)]
            let mut entry_12 = list_11.entry();
            entry_12.string(item_10);
        }
        list_11.finish();
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("Filter");
    if let Some(var_14) = &input.filters {
        let mut list_16 = scope_13.start_list(true, Some("Filter"));
        for item_15 in var_14 {
            #[allow(unused_mut)]
            let mut entry_17 = list_16.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_17, item_15)?;
        }
        list_16.finish();
    }
    #[allow(unused_mut)]
    let mut scope_18 = writer.prefix("NextToken");
    if let Some(var_19) = &input.next_token {
        scope_18.string(var_19);
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("MaxResults");
    if let Some(var_21) = &input.max_results {
        scope_20.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_21).into()),
        );
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}