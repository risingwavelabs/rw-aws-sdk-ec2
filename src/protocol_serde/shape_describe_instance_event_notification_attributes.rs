// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_instance_event_notification_attributes_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesOutput,
    crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_instance_event_notification_attributes_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesOutput,
    crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_instance_event_notification_attributes::builders::DescribeInstanceEventNotificationAttributesOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_instance_event_notification_attributes::de_describe_instance_event_notification_attributes(
            _response_body,
            output,
        )
        .map_err(crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_instance_event_notification_attributes(
    inp: &[u8],
    mut builder: crate::operation::describe_instance_event_notification_attributes::builders::DescribeInstanceEventNotificationAttributesOutputBuilder,
) -> Result<
    crate::operation::describe_instance_event_notification_attributes::builders::DescribeInstanceEventNotificationAttributesOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeInstanceEventNotificationAttributesResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeInstanceEventNotificationAttributesResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceTagAttribute") /* InstanceTagAttribute com.amazonaws.ec2.synthetic#DescribeInstanceEventNotificationAttributesOutput$InstanceTagAttribute */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_instance_tag_notification_attribute::de_instance_tag_notification_attribute(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_tag_attribute(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
