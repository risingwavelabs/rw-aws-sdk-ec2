// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_block_device_mapping(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::BlockDeviceMapping,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DeviceName");
    if let Some(var_2) = &input.device_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VirtualName");
    if let Some(var_4) = &input.virtual_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Ebs");
    if let Some(var_6) = &input.ebs {
        crate::protocol_serde::shape_ebs_block_device::ser_ebs_block_device(scope_5, var_6)?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("NoDevice");
    if let Some(var_8) = &input.no_device {
        scope_7.string(var_8);
    }
    Ok(())
}

#[allow(clippy::needless_question_mark)]
pub fn de_block_device_mapping(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::BlockDeviceMapping, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::BlockDeviceMapping::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("deviceName") /* DeviceName com.amazonaws.ec2#BlockDeviceMapping$DeviceName */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_device_name(var_9);
            }
            ,
            s if s.matches("virtualName") /* VirtualName com.amazonaws.ec2#BlockDeviceMapping$VirtualName */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_virtual_name(var_10);
            }
            ,
            s if s.matches("ebs") /* Ebs com.amazonaws.ec2#BlockDeviceMapping$Ebs */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_ebs_block_device::de_ebs_block_device(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ebs(var_11);
            }
            ,
            s if s.matches("noDevice") /* NoDevice com.amazonaws.ec2#BlockDeviceMapping$NoDevice */ =>  {
                let var_12 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_no_device(var_12);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
