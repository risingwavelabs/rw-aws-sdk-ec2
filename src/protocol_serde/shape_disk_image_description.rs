// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_disk_image_description(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::DiskImageDescription, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::DiskImageDescription::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("checksum") /* Checksum com.amazonaws.ec2#DiskImageDescription$Checksum */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_checksum(var_1);
            }
            ,
            s if s.matches("format") /* Format com.amazonaws.ec2#DiskImageDescription$Format */ =>  {
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
                builder = builder.set_format(var_2);
            }
            ,
            s if s.matches("importManifestUrl") /* ImportManifestUrl com.amazonaws.ec2#DiskImageDescription$ImportManifestUrl */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_import_manifest_url(var_3);
            }
            ,
            s if s.matches("size") /* Size com.amazonaws.ec2#DiskImageDescription$Size */ =>  {
                let var_4 =
                    Some(
                         {
                            <i64 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (long: `com.amazonaws.ec2#Long`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_size(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}