// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_accelerator_manufacturer_set(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<::std::vec::Vec<crate::types::AcceleratorManufacturer>, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#AcceleratorManufacturerSet$member */ =>  {
                out.push(
                    Result::<crate::types::AcceleratorManufacturer, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                        crate::types::AcceleratorManufacturer::from(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        )
                    )
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}
