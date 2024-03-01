// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_nat_gateway_address_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput,
    crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_nat_gateway_address_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressOutput,
    crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::associate_nat_gateway_address::builders::AssociateNatGatewayAddressOutputBuilder::default();
        output = crate::protocol_serde::shape_associate_nat_gateway_address::de_associate_nat_gateway_address(_response_body, output)
            .map_err(crate::operation::associate_nat_gateway_address::AssociateNatGatewayAddressError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_associate_nat_gateway_address(
    inp: &[u8],
    mut builder: crate::operation::associate_nat_gateway_address::builders::AssociateNatGatewayAddressOutputBuilder,
) -> Result<
    crate::operation::associate_nat_gateway_address::builders::AssociateNatGatewayAddressOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("AssociateNatGatewayAddressResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected AssociateNatGatewayAddressResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("natGatewayId") /* NatGatewayId com.amazonaws.ec2.synthetic#AssociateNatGatewayAddressOutput$NatGatewayId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_nat_gateway_id(var_1);
            }
            ,
            s if s.matches("natGatewayAddressSet") /* NatGatewayAddresses com.amazonaws.ec2.synthetic#AssociateNatGatewayAddressOutput$NatGatewayAddresses */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_nat_gateway_address_list::de_nat_gateway_address_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_nat_gateway_addresses(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}