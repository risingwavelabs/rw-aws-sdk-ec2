// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyVpcEndpointServiceConfiguration`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`service_id(impl Into<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::service_id) / [`set_service_id(Option<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_service_id):<br>required: **true**<br><p>The ID of the service.</p><br>
    ///   - [`private_dns_name(impl Into<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::private_dns_name) / [`set_private_dns_name(Option<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_private_dns_name):<br>required: **false**<br><p>(Interface endpoint configuration) The private DNS name to assign to the endpoint service.</p><br>
    ///   - [`remove_private_dns_name(bool)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::remove_private_dns_name) / [`set_remove_private_dns_name(Option<bool>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_remove_private_dns_name):<br>required: **false**<br><p>(Interface endpoint configuration) Removes the private DNS name of the endpoint service.</p><br>
    ///   - [`acceptance_required(bool)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::acceptance_required) / [`set_acceptance_required(Option<bool>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_acceptance_required):<br>required: **false**<br><p>Indicates whether requests to create an endpoint to your service must be accepted.</p><br>
    ///   - [`add_network_load_balancer_arns(impl Into<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::add_network_load_balancer_arns) / [`set_add_network_load_balancer_arns(Option<Vec::<String>>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_add_network_load_balancer_arns):<br>required: **false**<br><p>The Amazon Resource Names (ARNs) of Network Load Balancers to add to your service configuration.</p><br>
    ///   - [`remove_network_load_balancer_arns(impl Into<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::remove_network_load_balancer_arns) / [`set_remove_network_load_balancer_arns(Option<Vec::<String>>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_remove_network_load_balancer_arns):<br>required: **false**<br><p>The Amazon Resource Names (ARNs) of Network Load Balancers to remove from your service configuration.</p><br>
    ///   - [`add_gateway_load_balancer_arns(impl Into<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::add_gateway_load_balancer_arns) / [`set_add_gateway_load_balancer_arns(Option<Vec::<String>>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_add_gateway_load_balancer_arns):<br>required: **false**<br><p>The Amazon Resource Names (ARNs) of Gateway Load Balancers to add to your service configuration.</p><br>
    ///   - [`remove_gateway_load_balancer_arns(impl Into<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::remove_gateway_load_balancer_arns) / [`set_remove_gateway_load_balancer_arns(Option<Vec::<String>>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_remove_gateway_load_balancer_arns):<br>required: **false**<br><p>The Amazon Resource Names (ARNs) of Gateway Load Balancers to remove from your service configuration.</p><br>
    ///   - [`add_supported_ip_address_types(impl Into<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::add_supported_ip_address_types) / [`set_add_supported_ip_address_types(Option<Vec::<String>>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_add_supported_ip_address_types):<br>required: **false**<br><p>The IP address types to add to your service configuration.</p><br>
    ///   - [`remove_supported_ip_address_types(impl Into<String>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::remove_supported_ip_address_types) / [`set_remove_supported_ip_address_types(Option<Vec::<String>>)`](crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::set_remove_supported_ip_address_types):<br>required: **false**<br><p>The IP address types to remove from your service configuration.</p><br>
    /// - On success, responds with [`ModifyVpcEndpointServiceConfigurationOutput`](crate::operation::modify_vpc_endpoint_service_configuration::ModifyVpcEndpointServiceConfigurationOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::modify_vpc_endpoint_service_configuration::ModifyVpcEndpointServiceConfigurationOutput::return): <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    /// - On failure, responds with [`SdkError<ModifyVpcEndpointServiceConfigurationError>`](crate::operation::modify_vpc_endpoint_service_configuration::ModifyVpcEndpointServiceConfigurationError)
    pub fn modify_vpc_endpoint_service_configuration(
        &self,
    ) -> crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder {
        crate::operation::modify_vpc_endpoint_service_configuration::builders::ModifyVpcEndpointServiceConfigurationFluentBuilder::new(
            self.handle.clone(),
        )
    }
}