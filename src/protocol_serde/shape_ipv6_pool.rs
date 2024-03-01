// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_ipv6_pool(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> Result<crate::types::Ipv6Pool, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Ipv6Pool::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("poolId") /* PoolId com.amazonaws.ec2#Ipv6Pool$PoolId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_pool_id(var_1);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#Ipv6Pool$Description */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_2);
            }
            ,
            s if s.matches("poolCidrBlockSet") /* PoolCidrBlocks com.amazonaws.ec2#Ipv6Pool$PoolCidrBlocks */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_pool_cidr_blocks_set::de_pool_cidr_blocks_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_pool_cidr_blocks(var_3);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#Ipv6Pool$Tags */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
