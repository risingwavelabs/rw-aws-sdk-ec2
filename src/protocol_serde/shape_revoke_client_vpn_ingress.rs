// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_revoke_client_vpn_ingress_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressOutput,
    crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_revoke_client_vpn_ingress_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressOutput,
    crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::revoke_client_vpn_ingress::builders::RevokeClientVpnIngressOutputBuilder::default();
        output = crate::protocol_serde::shape_revoke_client_vpn_ingress::de_revoke_client_vpn_ingress(_response_body, output)
            .map_err(crate::operation::revoke_client_vpn_ingress::RevokeClientVpnIngressError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_revoke_client_vpn_ingress(
    inp: &[u8],
    mut builder: crate::operation::revoke_client_vpn_ingress::builders::RevokeClientVpnIngressOutputBuilder,
) -> Result<crate::operation::revoke_client_vpn_ingress::builders::RevokeClientVpnIngressOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("RevokeClientVpnIngressResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected RevokeClientVpnIngressResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("status") /* Status com.amazonaws.ec2.synthetic#RevokeClientVpnIngressOutput$Status */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_client_vpn_authorization_rule_status::de_client_vpn_authorization_rule_status(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}