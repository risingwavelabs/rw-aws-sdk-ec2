// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_peering_connection_options_request(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::PeeringConnectionOptionsRequest,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AllowDnsResolutionFromRemoteVpc");
    if let Some(var_2) = &input.allow_dns_resolution_from_remote_vpc {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AllowEgressFromLocalClassicLinkToRemoteVpc");
    if let Some(var_4) = &input.allow_egress_from_local_classic_link_to_remote_vpc {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AllowEgressFromLocalVpcToRemoteClassicLink");
    if let Some(var_6) = &input.allow_egress_from_local_vpc_to_remote_classic_link {
        scope_5.boolean(*var_6);
    }
    Ok(())
}
