// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_vpn_connections_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_vpn_connections::DescribeVpnConnectionsOutput,
    crate::operation::describe_vpn_connections::DescribeVpnConnectionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_vpn_connections::DescribeVpnConnectionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::describe_vpn_connections::DescribeVpnConnectionsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_vpn_connections_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_vpn_connections::DescribeVpnConnectionsOutput,
    crate::operation::describe_vpn_connections::DescribeVpnConnectionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_vpn_connections::builders::DescribeVpnConnectionsOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_vpn_connections::de_describe_vpn_connections(_response_body, output)
            .map_err(crate::operation::describe_vpn_connections::DescribeVpnConnectionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_vpn_connections(
    inp: &[u8],
    mut builder: crate::operation::describe_vpn_connections::builders::DescribeVpnConnectionsOutputBuilder,
) -> Result<crate::operation::describe_vpn_connections::builders::DescribeVpnConnectionsOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeVpnConnectionsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeVpnConnectionsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("vpnConnectionSet") /* VpnConnections com.amazonaws.ec2.synthetic#DescribeVpnConnectionsOutput$VpnConnections */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_vpn_connection_list::de_vpn_connection_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpn_connections(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
