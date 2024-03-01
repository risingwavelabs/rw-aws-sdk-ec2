// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_snapshot_recycle_bin_info(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::SnapshotRecycleBinInfo, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::SnapshotRecycleBinInfo::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("snapshotId") /* SnapshotId com.amazonaws.ec2#SnapshotRecycleBinInfo$SnapshotId */ =>  {
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
            s if s.matches("recycleBinEnterTime") /* RecycleBinEnterTime com.amazonaws.ec2#SnapshotRecycleBinInfo$RecycleBinEnterTime */ =>  {
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
                builder = builder.set_recycle_bin_enter_time(var_2);
            }
            ,
            s if s.matches("recycleBinExitTime") /* RecycleBinExitTime com.amazonaws.ec2#SnapshotRecycleBinInfo$RecycleBinExitTime */ =>  {
                let var_3 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#MillisecondDateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_recycle_bin_exit_time(var_3);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#SnapshotRecycleBinInfo$Description */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_4);
            }
            ,
            s if s.matches("volumeId") /* VolumeId com.amazonaws.ec2#SnapshotRecycleBinInfo$VolumeId */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_volume_id(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}