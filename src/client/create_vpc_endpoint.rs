// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVpcEndpoint`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`vpc_endpoint_type(VpcEndpointType)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::vpc_endpoint_type) / [`set_vpc_endpoint_type(Option<VpcEndpointType>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_vpc_endpoint_type):<br>required: **false**<br><p>The type of endpoint.</p>  <p>Default: Gateway</p><br>
    ///   - [`vpc_id(impl Into<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::vpc_id) / [`set_vpc_id(Option<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_vpc_id):<br>required: **true**<br><p>The ID of the VPC.</p><br>
    ///   - [`service_name(impl Into<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::service_name) / [`set_service_name(Option<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_service_name):<br>required: **true**<br><p>The name of the endpoint service.</p><br>
    ///   - [`policy_document(impl Into<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::policy_document) / [`set_policy_document(Option<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_policy_document):<br>required: **false**<br><p>(Interface and gateway endpoints) A policy to attach to the endpoint that controls access to the service. The policy must be in valid JSON format. If this parameter is not specified, we attach a default policy that allows full access to the service.</p><br>
    ///   - [`route_table_ids(impl Into<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::route_table_ids) / [`set_route_table_ids(Option<Vec::<String>>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_route_table_ids):<br>required: **false**<br><p>(Gateway endpoint) The route table IDs.</p><br>
    ///   - [`subnet_ids(impl Into<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::subnet_ids) / [`set_subnet_ids(Option<Vec::<String>>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_subnet_ids):<br>required: **false**<br><p>(Interface and Gateway Load Balancer endpoints) The IDs of the subnets in which to create endpoint network interfaces. For a Gateway Load Balancer endpoint, you can specify only one subnet.</p><br>
    ///   - [`security_group_ids(impl Into<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::security_group_ids) / [`set_security_group_ids(Option<Vec::<String>>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_security_group_ids):<br>required: **false**<br><p>(Interface endpoint) The IDs of the security groups to associate with the endpoint network interfaces. If this parameter is not specified, we use the default security group for the VPC.</p><br>
    ///   - [`ip_address_type(IpAddressType)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::ip_address_type) / [`set_ip_address_type(Option<IpAddressType>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_ip_address_type):<br>required: **false**<br><p>The IP address type for the endpoint.</p><br>
    ///   - [`dns_options(DnsOptionsSpecification)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::dns_options) / [`set_dns_options(Option<DnsOptionsSpecification>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_dns_options):<br>required: **false**<br><p>The DNS options for the endpoint.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_client_token):<br>required: **false**<br><p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p><br>
    ///   - [`private_dns_enabled(bool)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::private_dns_enabled) / [`set_private_dns_enabled(Option<bool>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_private_dns_enabled):<br>required: **false**<br><p>(Interface endpoint) Indicates whether to associate a private hosted zone with the specified VPC. The private hosted zone contains a record set for the default public DNS name for the service for the Region (for example, <code>kinesis.us-east-1.amazonaws.com</code>), which resolves to the private IP addresses of the endpoint network interfaces in the VPC. This enables you to make requests to the default public DNS name for the service instead of the public DNS names that are automatically generated by the VPC endpoint service.</p>  <p>To use a private hosted zone, you must set the following VPC attributes to <code>true</code>: <code>enableDnsHostnames</code> and <code>enableDnsSupport</code>. Use <code>ModifyVpcAttribute</code> to set the VPC attributes.</p>  <p>Default: <code>true</code> </p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to associate with the endpoint.</p><br>
    ///   - [`subnet_configurations(SubnetConfiguration)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::subnet_configurations) / [`set_subnet_configurations(Option<Vec::<SubnetConfiguration>>)`](crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::set_subnet_configurations):<br>required: **false**<br><p>The subnet configurations for the endpoint.</p><br>
    /// - On success, responds with [`CreateVpcEndpointOutput`](crate::operation::create_vpc_endpoint::CreateVpcEndpointOutput) with field(s):
    ///   - [`vpc_endpoint(Option<VpcEndpoint>)`](crate::operation::create_vpc_endpoint::CreateVpcEndpointOutput::vpc_endpoint): <p>Information about the endpoint.</p>
    ///   - [`client_token(Option<String>)`](crate::operation::create_vpc_endpoint::CreateVpcEndpointOutput::client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    /// - On failure, responds with [`SdkError<CreateVpcEndpointError>`](crate::operation::create_vpc_endpoint::CreateVpcEndpointError)
    pub fn create_vpc_endpoint(&self) -> crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder {
        crate::operation::create_vpc_endpoint::builders::CreateVpcEndpointFluentBuilder::new(self.handle.clone())
    }
}