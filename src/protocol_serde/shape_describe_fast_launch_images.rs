// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_fast_launch_images_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesOutput,
    crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_fast_launch_images_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesOutput,
    crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_fast_launch_images::de_describe_fast_launch_images(_response_body, output)
            .map_err(crate::operation::describe_fast_launch_images::DescribeFastLaunchImagesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_fast_launch_images(
    inp: &[u8],
    mut builder: crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesOutputBuilder,
) -> Result<crate::operation::describe_fast_launch_images::builders::DescribeFastLaunchImagesOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError>
{
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DescribeFastLaunchImagesResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DescribeFastLaunchImagesResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("fastLaunchImageSet") /* FastLaunchImages com.amazonaws.ec2.synthetic#DescribeFastLaunchImagesOutput$FastLaunchImages */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_describe_fast_launch_images_success_set::de_describe_fast_launch_images_success_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_fast_launch_images(var_1);
            }
            ,
            s if s.matches("nextToken") /* NextToken com.amazonaws.ec2.synthetic#DescribeFastLaunchImagesOutput$NextToken */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
