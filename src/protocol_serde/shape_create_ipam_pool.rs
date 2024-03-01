// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_ipam_pool_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_ipam_pool::CreateIpamPoolOutput, crate::operation::create_ipam_pool::CreateIpamPoolError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_ipam_pool::CreateIpamPoolError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::create_ipam_pool::CreateIpamPoolError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_ipam_pool_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::create_ipam_pool::CreateIpamPoolOutput, crate::operation::create_ipam_pool::CreateIpamPoolError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_ipam_pool::builders::CreateIpamPoolOutputBuilder::default();
        output = crate::protocol_serde::shape_create_ipam_pool::de_create_ipam_pool(_response_body, output)
            .map_err(crate::operation::create_ipam_pool::CreateIpamPoolError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_ipam_pool(
    inp: &[u8],
    mut builder: crate::operation::create_ipam_pool::builders::CreateIpamPoolOutputBuilder,
) -> Result<crate::operation::create_ipam_pool::builders::CreateIpamPoolOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CreateIpamPoolResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CreateIpamPoolResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ipamPool") /* IpamPool com.amazonaws.ec2.synthetic#CreateIpamPoolOutput$IpamPool */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_ipam_pool::de_ipam_pool(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipam_pool(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
