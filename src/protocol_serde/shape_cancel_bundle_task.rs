// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_bundle_task_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::cancel_bundle_task::CancelBundleTaskOutput, crate::operation::cancel_bundle_task::CancelBundleTaskError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::cancel_bundle_task::CancelBundleTaskError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::cancel_bundle_task::CancelBundleTaskError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_bundle_task_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::cancel_bundle_task::CancelBundleTaskOutput, crate::operation::cancel_bundle_task::CancelBundleTaskError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::cancel_bundle_task::builders::CancelBundleTaskOutputBuilder::default();
        output = crate::protocol_serde::shape_cancel_bundle_task::de_cancel_bundle_task(_response_body, output)
            .map_err(crate::operation::cancel_bundle_task::CancelBundleTaskError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_cancel_bundle_task(
    inp: &[u8],
    mut builder: crate::operation::cancel_bundle_task::builders::CancelBundleTaskOutputBuilder,
) -> Result<crate::operation::cancel_bundle_task::builders::CancelBundleTaskOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("CancelBundleTaskResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected CancelBundleTaskResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("bundleInstanceTask") /* BundleTask com.amazonaws.ec2.synthetic#CancelBundleTaskOutput$BundleTask */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_bundle_task::de_bundle_task(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_bundle_task(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
