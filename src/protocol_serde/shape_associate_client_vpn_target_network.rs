// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_client_vpn_target_network_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_client_vpn_target_network::AssociateClientVpnTargetNetworkOutput,
    crate::operation::associate_client_vpn_target_network::AssociateClientVpnTargetNetworkError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::associate_client_vpn_target_network::AssociateClientVpnTargetNetworkError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::associate_client_vpn_target_network::AssociateClientVpnTargetNetworkError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_client_vpn_target_network_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_client_vpn_target_network::AssociateClientVpnTargetNetworkOutput,
    crate::operation::associate_client_vpn_target_network::AssociateClientVpnTargetNetworkError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::associate_client_vpn_target_network::builders::AssociateClientVpnTargetNetworkOutputBuilder::default();
        output = crate::protocol_serde::shape_associate_client_vpn_target_network::de_associate_client_vpn_target_network(_response_body, output)
            .map_err(crate::operation::associate_client_vpn_target_network::AssociateClientVpnTargetNetworkError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_associate_client_vpn_target_network(
    inp: &[u8],
    mut builder: crate::operation::associate_client_vpn_target_network::builders::AssociateClientVpnTargetNetworkOutputBuilder,
) -> Result<
    crate::operation::associate_client_vpn_target_network::builders::AssociateClientVpnTargetNetworkOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("AssociateClientVpnTargetNetworkResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected AssociateClientVpnTargetNetworkResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("associationId") /* AssociationId com.amazonaws.ec2.synthetic#AssociateClientVpnTargetNetworkOutput$AssociationId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_association_id(var_1);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2.synthetic#AssociateClientVpnTargetNetworkOutput$Status */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_association_status::de_association_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}