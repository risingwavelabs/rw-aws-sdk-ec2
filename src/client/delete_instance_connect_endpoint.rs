// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteInstanceConnectEndpoint`](crate::operation::delete_instance_connect_endpoint::builders::DeleteInstanceConnectEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_instance_connect_endpoint::builders::DeleteInstanceConnectEndpointFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_instance_connect_endpoint::builders::DeleteInstanceConnectEndpointFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`instance_connect_endpoint_id(impl Into<String>)`](crate::operation::delete_instance_connect_endpoint::builders::DeleteInstanceConnectEndpointFluentBuilder::instance_connect_endpoint_id) / [`set_instance_connect_endpoint_id(Option<String>)`](crate::operation::delete_instance_connect_endpoint::builders::DeleteInstanceConnectEndpointFluentBuilder::set_instance_connect_endpoint_id):<br>required: **true**<br><p>The ID of the EC2 Instance Connect Endpoint to delete.</p><br>
    /// - On success, responds with [`DeleteInstanceConnectEndpointOutput`](crate::operation::delete_instance_connect_endpoint::DeleteInstanceConnectEndpointOutput) with field(s):
    ///   - [`instance_connect_endpoint(Option<Ec2InstanceConnectEndpoint>)`](crate::operation::delete_instance_connect_endpoint::DeleteInstanceConnectEndpointOutput::instance_connect_endpoint): <p>Information about the EC2 Instance Connect Endpoint.</p>
    /// - On failure, responds with [`SdkError<DeleteInstanceConnectEndpointError>`](crate::operation::delete_instance_connect_endpoint::DeleteInstanceConnectEndpointError)
    pub fn delete_instance_connect_endpoint(
        &self,
    ) -> crate::operation::delete_instance_connect_endpoint::builders::DeleteInstanceConnectEndpointFluentBuilder {
        crate::operation::delete_instance_connect_endpoint::builders::DeleteInstanceConnectEndpointFluentBuilder::new(self.handle.clone())
    }
}