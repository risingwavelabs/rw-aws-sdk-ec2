// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_register_image_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::register_image::RegisterImageOutput, crate::operation::register_image::RegisterImageError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::register_image::RegisterImageError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::register_image::RegisterImageError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_register_image_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::register_image::RegisterImageOutput, crate::operation::register_image::RegisterImageError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::register_image::builders::RegisterImageOutputBuilder::default();
        output = crate::protocol_serde::shape_register_image::de_register_image(_response_body, output)
            .map_err(crate::operation::register_image::RegisterImageError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_register_image(
    inp: &[u8],
    mut builder: crate::operation::register_image::builders::RegisterImageOutputBuilder,
) -> Result<crate::operation::register_image::builders::RegisterImageOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("RegisterImageResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected RegisterImageResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("imageId") /* ImageId com.amazonaws.ec2.synthetic#RegisterImageOutput$ImageId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_image_id(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
