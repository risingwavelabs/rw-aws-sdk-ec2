// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_ebs_default_kms_key_id_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdOutput,
    crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_ebs_default_kms_key_id_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdOutput,
    crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_ebs_default_kms_key_id::builders::GetEbsDefaultKmsKeyIdOutputBuilder::default();
        output = crate::protocol_serde::shape_get_ebs_default_kms_key_id::de_get_ebs_default_kms_key_id(_response_body, output)
            .map_err(crate::operation::get_ebs_default_kms_key_id::GetEbsDefaultKmsKeyIdError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_ebs_default_kms_key_id(
    inp: &[u8],
    mut builder: crate::operation::get_ebs_default_kms_key_id::builders::GetEbsDefaultKmsKeyIdOutputBuilder,
) -> Result<crate::operation::get_ebs_default_kms_key_id::builders::GetEbsDefaultKmsKeyIdOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetEbsDefaultKmsKeyIdResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetEbsDefaultKmsKeyIdResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("kmsKeyId") /* KmsKeyId com.amazonaws.ec2.synthetic#GetEbsDefaultKmsKeyIdOutput$KmsKeyId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kms_key_id(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
