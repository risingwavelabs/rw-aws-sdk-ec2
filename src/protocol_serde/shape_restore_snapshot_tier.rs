// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_restore_snapshot_tier_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::restore_snapshot_tier::RestoreSnapshotTierOutput,
    crate::operation::restore_snapshot_tier::RestoreSnapshotTierError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::restore_snapshot_tier::RestoreSnapshotTierError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::restore_snapshot_tier::RestoreSnapshotTierError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_restore_snapshot_tier_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::restore_snapshot_tier::RestoreSnapshotTierOutput,
    crate::operation::restore_snapshot_tier::RestoreSnapshotTierError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::restore_snapshot_tier::builders::RestoreSnapshotTierOutputBuilder::default();
        output = crate::protocol_serde::shape_restore_snapshot_tier::de_restore_snapshot_tier(_response_body, output)
            .map_err(crate::operation::restore_snapshot_tier::RestoreSnapshotTierError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_restore_snapshot_tier(
    inp: &[u8],
    mut builder: crate::operation::restore_snapshot_tier::builders::RestoreSnapshotTierOutputBuilder,
) -> Result<crate::operation::restore_snapshot_tier::builders::RestoreSnapshotTierOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("RestoreSnapshotTierResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected RestoreSnapshotTierResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("snapshotId") /* SnapshotId com.amazonaws.ec2.synthetic#RestoreSnapshotTierOutput$SnapshotId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_snapshot_id(var_1);
            }
            ,
            s if s.matches("restoreStartTime") /* RestoreStartTime com.amazonaws.ec2.synthetic#RestoreSnapshotTierOutput$RestoreStartTime */ =>  {
                let var_2 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_restore_start_time(var_2);
            }
            ,
            s if s.matches("restoreDuration") /* RestoreDuration com.amazonaws.ec2.synthetic#RestoreSnapshotTierOutput$RestoreDuration */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_restore_duration(var_3);
            }
            ,
            s if s.matches("isPermanentRestore") /* IsPermanentRestore com.amazonaws.ec2.synthetic#RestoreSnapshotTierOutput$IsPermanentRestore */ =>  {
                let var_4 =
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
                builder = builder.set_is_permanent_restore(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}