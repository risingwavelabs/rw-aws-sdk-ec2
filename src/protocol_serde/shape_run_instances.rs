// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_run_instances_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::run_instances::RunInstancesOutput, crate::operation::run_instances::RunInstancesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::run_instances::RunInstancesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::run_instances::RunInstancesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_run_instances_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::run_instances::RunInstancesOutput, crate::operation::run_instances::RunInstancesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::run_instances::builders::RunInstancesOutputBuilder::default();
        output = crate::protocol_serde::shape_run_instances::de_run_instances(_response_body, output)
            .map_err(crate::operation::run_instances::RunInstancesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_run_instances(
    inp: &[u8],
    mut builder: crate::operation::run_instances::builders::RunInstancesOutputBuilder,
) -> Result<crate::operation::run_instances::builders::RunInstancesOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("RunInstancesResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected RunInstancesResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("groupSet") /* Groups com.amazonaws.ec2.synthetic#RunInstancesOutput$Groups */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_group_identifier_list::de_group_identifier_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_groups(var_1);
            }
            ,
            s if s.matches("instancesSet") /* Instances com.amazonaws.ec2.synthetic#RunInstancesOutput$Instances */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_instance_list::de_instance_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instances(var_2);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2.synthetic#RunInstancesOutput$OwnerId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_3);
            }
            ,
            s if s.matches("requesterId") /* RequesterId com.amazonaws.ec2.synthetic#RunInstancesOutput$RequesterId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_requester_id(var_4);
            }
            ,
            s if s.matches("reservationId") /* ReservationId com.amazonaws.ec2.synthetic#RunInstancesOutput$ReservationId */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_reservation_id(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
