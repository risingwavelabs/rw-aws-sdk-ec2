// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_ipam_resource_discovery_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryOutput,
    crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_ipam_resource_discovery_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryOutput,
    crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::associate_ipam_resource_discovery::builders::AssociateIpamResourceDiscoveryOutputBuilder::default();
        output = crate::protocol_serde::shape_associate_ipam_resource_discovery::de_associate_ipam_resource_discovery(_response_body, output)
            .map_err(crate::operation::associate_ipam_resource_discovery::AssociateIpamResourceDiscoveryError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_associate_ipam_resource_discovery(
    inp: &[u8],
    mut builder: crate::operation::associate_ipam_resource_discovery::builders::AssociateIpamResourceDiscoveryOutputBuilder,
) -> Result<
    crate::operation::associate_ipam_resource_discovery::builders::AssociateIpamResourceDiscoveryOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("AssociateIpamResourceDiscoveryResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected AssociateIpamResourceDiscoveryResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ipamResourceDiscoveryAssociation") /* IpamResourceDiscoveryAssociation com.amazonaws.ec2.synthetic#AssociateIpamResourceDiscoveryOutput$IpamResourceDiscoveryAssociation */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_ipam_resource_discovery_association::de_ipam_resource_discovery_association(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipam_resource_discovery_association(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
