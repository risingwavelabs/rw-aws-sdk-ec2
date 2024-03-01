// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_detach_network_interface_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::detach_network_interface::DetachNetworkInterfaceOutput,
    crate::operation::detach_network_interface::DetachNetworkInterfaceError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::detach_network_interface::DetachNetworkInterfaceError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::detach_network_interface::DetachNetworkInterfaceError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_detach_network_interface_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::detach_network_interface::DetachNetworkInterfaceOutput,
    crate::operation::detach_network_interface::DetachNetworkInterfaceError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::detach_network_interface::builders::DetachNetworkInterfaceOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
