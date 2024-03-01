// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_client_vpn_connections_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsOutput,
    crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_client_vpn_connections_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsOutput,
    crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_client_vpn_connections::builders::DescribeClientVpnConnectionsOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_client_vpn_connections::de_describe_client_vpn_connections(_response_body, output)
            .map_err(crate::operation::describe_client_vpn_connections::DescribeClientVpnConnectionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_client_vpn_connections(
    inp: &[u8],
    mut builder: crate::operation::describe_client_vpn_connections::builders::DescribeClientVpnConnectionsOutputBuilder,
) -> Result<
    crate::operation::describe_client_vpn_connections::builders::DescribeClientVpnConnectionsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeClientVpnConnectionsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeClientVpnConnectionsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("connections") /* Connections com.amazonaws.ec2.synthetic#DescribeClientVpnConnectionsOutput$Connections */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_client_vpn_connection_set::de_client_vpn_connection_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_connections(var_1);
            }
            ,
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#DescribeClientVpnConnectionsOutput$NextToken */ =>  {
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
