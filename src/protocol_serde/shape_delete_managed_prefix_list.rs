// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_managed_prefix_list_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListOutput,
    crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_managed_prefix_list_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListOutput,
    crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_managed_prefix_list::builders::DeleteManagedPrefixListOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_managed_prefix_list::de_delete_managed_prefix_list(_response_body, output)
            .map_err(crate::operation::delete_managed_prefix_list::DeleteManagedPrefixListError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_delete_managed_prefix_list(
    inp: &[u8],
    mut builder: crate::operation::delete_managed_prefix_list::builders::DeleteManagedPrefixListOutputBuilder,
) -> Result<crate::operation::delete_managed_prefix_list::builders::DeleteManagedPrefixListOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DeleteManagedPrefixListResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DeleteManagedPrefixListResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("prefixList") /* PrefixList com.amazonaws.ec2.synthetic#DeleteManagedPrefixListOutput$PrefixList */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_managed_prefix_list::de_managed_prefix_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_prefix_list(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}