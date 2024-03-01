// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_instance_maintenance_options_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsOutput,
    crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_instance_maintenance_options_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsOutput,
    crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_instance_maintenance_options::builders::ModifyInstanceMaintenanceOptionsOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_instance_maintenance_options::de_modify_instance_maintenance_options(_response_body, output)
            .map_err(crate::operation::modify_instance_maintenance_options::ModifyInstanceMaintenanceOptionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_instance_maintenance_options(
    inp: &[u8],
    mut builder: crate::operation::modify_instance_maintenance_options::builders::ModifyInstanceMaintenanceOptionsOutputBuilder,
) -> Result<
    crate::operation::modify_instance_maintenance_options::builders::ModifyInstanceMaintenanceOptionsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyInstanceMaintenanceOptionsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyInstanceMaintenanceOptionsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2.synthetic#ModifyInstanceMaintenanceOptionsOutput$InstanceId */ =>  {
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
            s if s.matches("autoRecovery") /* AutoRecovery com.amazonaws.ec2.synthetic#ModifyInstanceMaintenanceOptionsOutput$AutoRecovery */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::InstanceAutoRecoveryState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::InstanceAutoRecoveryState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_auto_recovery(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
