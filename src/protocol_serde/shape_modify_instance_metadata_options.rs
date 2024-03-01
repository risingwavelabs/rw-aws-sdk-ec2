// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_instance_metadata_options_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsOutput,
    crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_instance_metadata_options_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsOutput,
    crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_instance_metadata_options::builders::ModifyInstanceMetadataOptionsOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_instance_metadata_options::de_modify_instance_metadata_options(_response_body, output)
            .map_err(crate::operation::modify_instance_metadata_options::ModifyInstanceMetadataOptionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_instance_metadata_options(
    inp: &[u8],
    mut builder: crate::operation::modify_instance_metadata_options::builders::ModifyInstanceMetadataOptionsOutputBuilder,
) -> Result<
    crate::operation::modify_instance_metadata_options::builders::ModifyInstanceMetadataOptionsOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyInstanceMetadataOptionsResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyInstanceMetadataOptionsResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2.synthetic#ModifyInstanceMetadataOptionsOutput$InstanceId */ =>  {
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
            s if s.matches("instanceMetadataOptions") /* InstanceMetadataOptions com.amazonaws.ec2.synthetic#ModifyInstanceMetadataOptionsOutput$InstanceMetadataOptions */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_instance_metadata_options_response::de_instance_metadata_options_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_metadata_options(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
