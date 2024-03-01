// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_instance_attribute_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_instance_attribute::ModifyInstanceAttributeOutput,
    crate::operation::modify_instance_attribute::ModifyInstanceAttributeError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_instance_attribute::ModifyInstanceAttributeError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::modify_instance_attribute::ModifyInstanceAttributeError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_instance_attribute_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_instance_attribute::ModifyInstanceAttributeOutput,
    crate::operation::modify_instance_attribute::ModifyInstanceAttributeError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_instance_attribute::builders::ModifyInstanceAttributeOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
