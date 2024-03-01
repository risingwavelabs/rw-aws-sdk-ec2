// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_spot_fleet_requests_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput,
    crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_spot_fleet_requests_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput,
    crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsOutputBuilder::default();
        output = crate::protocol_serde::shape_cancel_spot_fleet_requests::de_cancel_spot_fleet_requests(_response_body, output)
            .map_err(crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_cancel_spot_fleet_requests(
    inp: &[u8],
    mut builder: crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsOutputBuilder,
) -> Result<crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CancelSpotFleetRequestsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CancelSpotFleetRequestsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("successfulFleetRequestSet") /* SuccessfulFleetRequests com.amazonaws.ec2.synthetic#CancelSpotFleetRequestsOutput$SuccessfulFleetRequests */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cancel_spot_fleet_requests_success_set::de_cancel_spot_fleet_requests_success_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_successful_fleet_requests(var_1);
            }
            ,
            s if s.matches("unsuccessfulFleetRequestSet") /* UnsuccessfulFleetRequests com.amazonaws.ec2.synthetic#CancelSpotFleetRequestsOutput$UnsuccessfulFleetRequests */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_cancel_spot_fleet_requests_error_set::de_cancel_spot_fleet_requests_error_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_unsuccessful_fleet_requests(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
