// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_create_verified_access_endpoint_load_balancer_options(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::CreateVerifiedAccessEndpointLoadBalancerOptions,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Protocol");
    if let Some(var_2) = &input.protocol {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Port");
    if let Some(var_4) = &input.port {
        scope_3.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("LoadBalancerArn");
    if let Some(var_6) = &input.load_balancer_arn {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("SubnetId");
    if let Some(var_8) = &input.subnet_ids {
        let mut list_10 = scope_7.start_list(true, Some("item"));
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            entry_11.string(item_9);
        }
        list_10.finish();
    }
    Ok(())
}
