// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_lock_snapshot_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::lock_snapshot::LockSnapshotOutput, crate::operation::lock_snapshot::LockSnapshotError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::lock_snapshot::LockSnapshotError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::lock_snapshot::LockSnapshotError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_lock_snapshot_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::lock_snapshot::LockSnapshotOutput, crate::operation::lock_snapshot::LockSnapshotError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::lock_snapshot::builders::LockSnapshotOutputBuilder::default();
        output = crate::protocol_serde::shape_lock_snapshot::de_lock_snapshot(_response_body, output)
            .map_err(crate::operation::lock_snapshot::LockSnapshotError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_lock_snapshot(
    inp: &[u8],
    mut builder: crate::operation::lock_snapshot::builders::LockSnapshotOutputBuilder,
) -> Result<crate::operation::lock_snapshot::builders::LockSnapshotOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("LockSnapshotResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected LockSnapshotResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("snapshotId") /* SnapshotId com.amazonaws.ec2.synthetic#LockSnapshotOutput$SnapshotId */ =>  {
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
            s if s.matches("lockState") /* LockState com.amazonaws.ec2.synthetic#LockSnapshotOutput$LockState */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::LockState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LockState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_lock_state(var_2);
            }
            ,
            s if s.matches("lockDuration") /* LockDuration com.amazonaws.ec2.synthetic#LockSnapshotOutput$LockDuration */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#RetentionPeriodResponseDays`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_lock_duration(var_3);
            }
            ,
            s if s.matches("coolOffPeriod") /* CoolOffPeriod com.amazonaws.ec2.synthetic#LockSnapshotOutput$CoolOffPeriod */ =>  {
                let var_4 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#CoolOffPeriodResponseHours`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_cool_off_period(var_4);
            }
            ,
            s if s.matches("coolOffPeriodExpiresOn") /* CoolOffPeriodExpiresOn com.amazonaws.ec2.synthetic#LockSnapshotOutput$CoolOffPeriodExpiresOn */ =>  {
                let var_5 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_cool_off_period_expires_on(var_5);
            }
            ,
            s if s.matches("lockCreatedOn") /* LockCreatedOn com.amazonaws.ec2.synthetic#LockSnapshotOutput$LockCreatedOn */ =>  {
                let var_6 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_lock_created_on(var_6);
            }
            ,
            s if s.matches("lockExpiresOn") /* LockExpiresOn com.amazonaws.ec2.synthetic#LockSnapshotOutput$LockExpiresOn */ =>  {
                let var_7 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_lock_expires_on(var_7);
            }
            ,
            s if s.matches("lockDurationStartTime") /* LockDurationStartTime com.amazonaws.ec2.synthetic#LockSnapshotOutput$LockDurationStartTime */ =>  {
                let var_8 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_lock_duration_start_time(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
