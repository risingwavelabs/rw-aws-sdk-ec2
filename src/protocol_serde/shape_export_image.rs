// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_export_image_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::export_image::ExportImageOutput, crate::operation::export_image::ExportImageError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::export_image::ExportImageError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::export_image::ExportImageError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_export_image_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::export_image::ExportImageOutput, crate::operation::export_image::ExportImageError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::export_image::builders::ExportImageOutputBuilder::default();
        output = crate::protocol_serde::shape_export_image::de_export_image(_response_body, output)
            .map_err(crate::operation::export_image::ExportImageError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_export_image(
    inp: &[u8],
    mut builder: crate::operation::export_image::builders::ExportImageOutputBuilder,
) -> Result<crate::operation::export_image::builders::ExportImageOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ExportImageResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ExportImageResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("description") /* Description com.amazonaws.ec2.synthetic#ExportImageOutput$Description */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_1);
            }
            ,
            s if s.matches("diskImageFormat") /* DiskImageFormat com.amazonaws.ec2.synthetic#ExportImageOutput$DiskImageFormat */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::DiskImageFormat, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DiskImageFormat::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_disk_image_format(var_2);
            }
            ,
            s if s.matches("exportImageTaskId") /* ExportImageTaskId com.amazonaws.ec2.synthetic#ExportImageOutput$ExportImageTaskId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_export_image_task_id(var_3);
            }
            ,
            s if s.matches("imageId") /* ImageId com.amazonaws.ec2.synthetic#ExportImageOutput$ImageId */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_image_id(var_4);
            }
            ,
            s if s.matches("roleName") /* RoleName com.amazonaws.ec2.synthetic#ExportImageOutput$RoleName */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_role_name(var_5);
            }
            ,
            s if s.matches("progress") /* Progress com.amazonaws.ec2.synthetic#ExportImageOutput$Progress */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_progress(var_6);
            }
            ,
            s if s.matches("s3ExportLocation") /* S3ExportLocation com.amazonaws.ec2.synthetic#ExportImageOutput$S3ExportLocation */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_export_task_s3_location::de_export_task_s3_location(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_s3_export_location(var_7);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2.synthetic#ExportImageOutput$Status */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_8);
            }
            ,
            s if s.matches("statusMessage") /* StatusMessage com.amazonaws.ec2.synthetic#ExportImageOutput$StatusMessage */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_message(var_9);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2.synthetic#ExportImageOutput$Tags */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_10);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}