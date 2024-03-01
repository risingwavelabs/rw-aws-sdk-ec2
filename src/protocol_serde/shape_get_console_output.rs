// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_console_output_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_console_output::GetConsoleOutputOutput, crate::operation::get_console_output::GetConsoleOutputError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_console_output::GetConsoleOutputError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_console_output::GetConsoleOutputError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_console_output_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_console_output::GetConsoleOutputOutput, crate::operation::get_console_output::GetConsoleOutputError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_console_output::builders::GetConsoleOutputOutputBuilder::default();
        output = crate::protocol_serde::shape_get_console_output::de_get_console_output(_response_body, output)
            .map_err(crate::operation::get_console_output::GetConsoleOutputError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_console_output(
    inp: &[u8],
    mut builder: crate::operation::get_console_output::builders::GetConsoleOutputOutputBuilder,
) -> Result<crate::operation::get_console_output::builders::GetConsoleOutputOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetConsoleOutputResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetConsoleOutputResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2.synthetic#GetConsoleOutputOutput$InstanceId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_id(var_1);
            }
            ,
            s if s.matches("output") /* Output com.amazonaws.ec2.synthetic#GetConsoleOutputOutput$Output */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_output(var_2);
            }
            ,
            s if s.matches("timestamp") /* Timestamp com.amazonaws.ec2.synthetic#GetConsoleOutputOutput$Timestamp */ =>  {
                let var_3 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_timestamp(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
