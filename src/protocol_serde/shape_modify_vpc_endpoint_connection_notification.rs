// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_vpc_endpoint_connection_notification_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationOutput,
    crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_vpc_endpoint_connection_notification_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationOutput,
    crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_vpc_endpoint_connection_notification::de_modify_vpc_endpoint_connection_notification(
            _response_body,
            output,
        )
        .map_err(crate::operation::modify_vpc_endpoint_connection_notification::ModifyVpcEndpointConnectionNotificationError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_vpc_endpoint_connection_notification(
    inp: &[u8],
    mut builder: crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationOutputBuilder,
) -> Result<
    crate::operation::modify_vpc_endpoint_connection_notification::builders::ModifyVpcEndpointConnectionNotificationOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyVpcEndpointConnectionNotificationResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyVpcEndpointConnectionNotificationResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("return") /* ReturnValue com.amazonaws.ec2.synthetic#ModifyVpcEndpointConnectionNotificationOutput$ReturnValue */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_return_value(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
