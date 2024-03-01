// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_traffic_mirror_filter_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterOutput,
    crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_traffic_mirror_filter_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterOutput,
    crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_traffic_mirror_filter::de_delete_traffic_mirror_filter(_response_body, output)
            .map_err(crate::operation::delete_traffic_mirror_filter::DeleteTrafficMirrorFilterError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_traffic_mirror_filter(
    inp: &[u8],
    mut builder: crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterOutputBuilder,
) -> Result<crate::operation::delete_traffic_mirror_filter::builders::DeleteTrafficMirrorFilterOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteTrafficMirrorFilterResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteTrafficMirrorFilterResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("trafficMirrorFilterId") /* TrafficMirrorFilterId com.amazonaws.ec2.synthetic#DeleteTrafficMirrorFilterOutput$TrafficMirrorFilterId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_traffic_mirror_filter_id(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
