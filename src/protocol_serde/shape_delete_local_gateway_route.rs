// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_local_gateway_route_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteOutput,
    crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_local_gateway_route_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteOutput,
    crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_local_gateway_route::de_delete_local_gateway_route(_response_body, output)
            .map_err(crate::operation::delete_local_gateway_route::DeleteLocalGatewayRouteError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_local_gateway_route(
    inp: &[u8],
    mut builder: crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteOutputBuilder,
) -> Result<crate::operation::delete_local_gateway_route::builders::DeleteLocalGatewayRouteOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteLocalGatewayRouteResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteLocalGatewayRouteResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("route") /* Route com.amazonaws.ec2.synthetic#DeleteLocalGatewayRouteOutput$Route */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_local_gateway_route::de_local_gateway_route(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_route(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}