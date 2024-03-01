// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_ebs_encryption_by_default_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultOutput,
    crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_ebs_encryption_by_default_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultOutput,
    crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultOutputBuilder::default();
        output = crate::protocol_serde::shape_get_ebs_encryption_by_default::de_get_ebs_encryption_by_default(_response_body, output)
            .map_err(crate::operation::get_ebs_encryption_by_default::GetEbsEncryptionByDefaultError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_get_ebs_encryption_by_default(
    inp: &[u8],
    mut builder: crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultOutputBuilder,
) -> Result<crate::operation::get_ebs_encryption_by_default::builders::GetEbsEncryptionByDefaultOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetEbsEncryptionByDefaultResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetEbsEncryptionByDefaultResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ebsEncryptionByDefault") /* EbsEncryptionByDefault com.amazonaws.ec2.synthetic#GetEbsEncryptionByDefaultOutput$EbsEncryptionByDefault */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_ebs_encryption_by_default(var_1);
            }
            ,
            s if s.matches("sseType") /* SseType com.amazonaws.ec2.synthetic#GetEbsEncryptionByDefaultOutput$SseType */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::SseType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::SseType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_sse_type(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
