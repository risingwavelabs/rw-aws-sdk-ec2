// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateVpcPeeringConnection`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`peer_owner_id(impl Into<String>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::peer_owner_id) / [`set_peer_owner_id(Option<String>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::set_peer_owner_id):<br>required: **false**<br><p>The Amazon Web Services account ID of the owner of the accepter VPC.</p>  <p>Default: Your Amazon Web Services account ID</p><br>
    ///   - [`peer_vpc_id(impl Into<String>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::peer_vpc_id) / [`set_peer_vpc_id(Option<String>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::set_peer_vpc_id):<br>required: **false**<br><p>The ID of the VPC with which you are creating the VPC peering connection. You must specify this parameter in the request.</p><br>
    ///   - [`vpc_id(impl Into<String>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::vpc_id) / [`set_vpc_id(Option<String>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::set_vpc_id):<br>required: **true**<br><p>The ID of the requester VPC. You must specify this parameter in the request.</p><br>
    ///   - [`peer_region(impl Into<String>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::peer_region) / [`set_peer_region(Option<String>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::set_peer_region):<br>required: **false**<br><p>The Region code for the accepter VPC, if the accepter VPC is located in a Region other than the Region in which you make the request.</p>  <p>Default: The Region in which you make the request.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to assign to the peering connection.</p><br>
    /// - On success, responds with [`CreateVpcPeeringConnectionOutput`](crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionOutput) with field(s):
    ///   - [`vpc_peering_connection(Option<VpcPeeringConnection>)`](crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionOutput::vpc_peering_connection): <p>Information about the VPC peering connection.</p>
    /// - On failure, responds with [`SdkError<CreateVpcPeeringConnectionError>`](crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionError)
    pub fn create_vpc_peering_connection(
        &self,
    ) -> crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder {
        crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionFluentBuilder::new(self.handle.clone())
    }
}
