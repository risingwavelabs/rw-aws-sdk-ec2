// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTransitGatewayConnectPeer`](crate::operation::delete_transit_gateway_connect_peer::builders::DeleteTransitGatewayConnectPeerFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_connect_peer_id(impl Into<String>)`](crate::operation::delete_transit_gateway_connect_peer::builders::DeleteTransitGatewayConnectPeerFluentBuilder::transit_gateway_connect_peer_id) / [`set_transit_gateway_connect_peer_id(Option<String>)`](crate::operation::delete_transit_gateway_connect_peer::builders::DeleteTransitGatewayConnectPeerFluentBuilder::set_transit_gateway_connect_peer_id):<br>required: **true**<br><p>The ID of the Connect peer.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::delete_transit_gateway_connect_peer::builders::DeleteTransitGatewayConnectPeerFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_transit_gateway_connect_peer::builders::DeleteTransitGatewayConnectPeerFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DeleteTransitGatewayConnectPeerOutput`](crate::operation::delete_transit_gateway_connect_peer::DeleteTransitGatewayConnectPeerOutput) with field(s):
    ///   - [`transit_gateway_connect_peer(Option<TransitGatewayConnectPeer>)`](crate::operation::delete_transit_gateway_connect_peer::DeleteTransitGatewayConnectPeerOutput::transit_gateway_connect_peer): <p>Information about the deleted Connect peer.</p>
    /// - On failure, responds with [`SdkError<DeleteTransitGatewayConnectPeerError>`](crate::operation::delete_transit_gateway_connect_peer::DeleteTransitGatewayConnectPeerError)
    pub fn delete_transit_gateway_connect_peer(
        &self,
    ) -> crate::operation::delete_transit_gateway_connect_peer::builders::DeleteTransitGatewayConnectPeerFluentBuilder {
        crate::operation::delete_transit_gateway_connect_peer::builders::DeleteTransitGatewayConnectPeerFluentBuilder::new(self.handle.clone())
    }
}
