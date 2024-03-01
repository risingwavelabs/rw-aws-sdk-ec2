// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_fpga_image_attribute_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeOutput,
    crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_fpga_image_attribute_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeOutput,
    crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_fpga_image_attribute::builders::ModifyFpgaImageAttributeOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_fpga_image_attribute::de_modify_fpga_image_attribute(_response_body, output)
            .map_err(crate::operation::modify_fpga_image_attribute::ModifyFpgaImageAttributeError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_fpga_image_attribute(
    inp: &[u8],
    mut builder: crate::operation::modify_fpga_image_attribute::builders::ModifyFpgaImageAttributeOutputBuilder,
) -> Result<crate::operation::modify_fpga_image_attribute::builders::ModifyFpgaImageAttributeOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyFpgaImageAttributeResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyFpgaImageAttributeResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("fpgaImageAttribute") /* FpgaImageAttribute com.amazonaws.ec2.synthetic#ModifyFpgaImageAttributeOutput$FpgaImageAttribute */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_fpga_image_attribute::de_fpga_image_attribute(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_fpga_image_attribute(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}