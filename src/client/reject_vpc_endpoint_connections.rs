// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RejectVpcEndpointConnections`](crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`service_id(impl Into<String>)`](crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsFluentBuilder::service_id) / [`set_service_id(Option<String>)`](crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsFluentBuilder::set_service_id):<br>required: **true**<br><p>The ID of the service.</p><br>
    ///   - [`vpc_endpoint_ids(impl Into<String>)`](crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsFluentBuilder::vpc_endpoint_ids) / [`set_vpc_endpoint_ids(Option<Vec::<String>>)`](crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsFluentBuilder::set_vpc_endpoint_ids):<br>required: **true**<br><p>The IDs of the VPC endpoints.</p><br>
    /// - On success, responds with [`RejectVpcEndpointConnectionsOutput`](crate::operation::reject_vpc_endpoint_connections::RejectVpcEndpointConnectionsOutput) with field(s):
    ///   - [`unsuccessful(Option<Vec::<UnsuccessfulItem>>)`](crate::operation::reject_vpc_endpoint_connections::RejectVpcEndpointConnectionsOutput::unsuccessful): <p>Information about the endpoints that were not rejected, if applicable.</p>
    /// - On failure, responds with [`SdkError<RejectVpcEndpointConnectionsError>`](crate::operation::reject_vpc_endpoint_connections::RejectVpcEndpointConnectionsError)
    pub fn reject_vpc_endpoint_connections(
        &self,
    ) -> crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsFluentBuilder {
        crate::operation::reject_vpc_endpoint_connections::builders::RejectVpcEndpointConnectionsFluentBuilder::new(self.handle.clone())
    }
}
