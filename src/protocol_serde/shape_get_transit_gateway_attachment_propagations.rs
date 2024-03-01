// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_transit_gateway_attachment_propagations_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsOutput,
    crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_transit_gateway_attachment_propagations_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsOutput,
    crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsOutputBuilder::default();
        output = crate::protocol_serde::shape_get_transit_gateway_attachment_propagations::de_get_transit_gateway_attachment_propagations(
            _response_body,
            output,
        )
        .map_err(crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_transit_gateway_attachment_propagations(
    inp: &[u8],
    mut builder: crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsOutputBuilder,
) -> Result<
    crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetTransitGatewayAttachmentPropagationsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetTransitGatewayAttachmentPropagationsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("transitGatewayAttachmentPropagations") /* TransitGatewayAttachmentPropagations com.amazonaws.ec2.synthetic#GetTransitGatewayAttachmentPropagationsOutput$TransitGatewayAttachmentPropagations */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_transit_gateway_attachment_propagation_list::de_transit_gateway_attachment_propagation_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_attachment_propagations(var_1);
            }
            ,
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#GetTransitGatewayAttachmentPropagationsOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
