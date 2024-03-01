// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_spot_fleet_request_history_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_spot_fleet_request_history::DescribeSpotFleetRequestHistoryOutput,
    crate::operation::describe_spot_fleet_request_history::DescribeSpotFleetRequestHistoryError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_spot_fleet_request_history::DescribeSpotFleetRequestHistoryError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::describe_spot_fleet_request_history::DescribeSpotFleetRequestHistoryError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_spot_fleet_request_history_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_spot_fleet_request_history::DescribeSpotFleetRequestHistoryOutput,
    crate::operation::describe_spot_fleet_request_history::DescribeSpotFleetRequestHistoryError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_spot_fleet_request_history::builders::DescribeSpotFleetRequestHistoryOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_spot_fleet_request_history::de_describe_spot_fleet_request_history(_response_body, output)
            .map_err(crate::operation::describe_spot_fleet_request_history::DescribeSpotFleetRequestHistoryError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_spot_fleet_request_history(
    inp: &[u8],
    mut builder: crate::operation::describe_spot_fleet_request_history::builders::DescribeSpotFleetRequestHistoryOutputBuilder,
) -> Result<
    crate::operation::describe_spot_fleet_request_history::builders::DescribeSpotFleetRequestHistoryOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeSpotFleetRequestHistoryResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeSpotFleetRequestHistoryResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("historyRecordSet") /* HistoryRecords com.amazonaws.ec2.synthetic#DescribeSpotFleetRequestHistoryOutput$HistoryRecords */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_history_records::de_history_records(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_history_records(var_1);
            }
            ,
            s if s.matches("lastEvaluatedTime") /* LastEvaluatedTime com.amazonaws.ec2.synthetic#DescribeSpotFleetRequestHistoryOutput$LastEvaluatedTime */ =>  {
                let var_2 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_last_evaluated_time(var_2);
            }
            ,
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#DescribeSpotFleetRequestHistoryOutput$NextToken */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_3);
            }
            ,
            s if s.matches("spotFleetRequestId") /* SpotFleetRequestId com.amazonaws.ec2.synthetic#DescribeSpotFleetRequestHistoryOutput$SpotFleetRequestId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_spot_fleet_request_id(var_4);
            }
            ,
            s if s.matches("startTime") /* StartTime com.amazonaws.ec2.synthetic#DescribeSpotFleetRequestHistoryOutput$StartTime */ =>  {
                let var_5 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_start_time(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
