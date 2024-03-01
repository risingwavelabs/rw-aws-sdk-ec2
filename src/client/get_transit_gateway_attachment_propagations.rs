// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetTransitGatewayAttachmentPropagations`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`transit_gateway_attachment_id(impl Into<String>)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::transit_gateway_attachment_id) / [`set_transit_gateway_attachment_id(Option<String>)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::set_transit_gateway_attachment_id):<br>required: **true**<br><p>The ID of the attachment.</p><br>
    ///   - [`filters(Filter)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::set_filters):<br>required: **false**<br><p>One or more filters. The possible values are:</p>  <ul>   <li> <p> <code>transit-gateway-route-table-id</code> - The ID of the transit gateway route table.</p> </li>  </ul><br>
    ///   - [`max_results(i32)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`GetTransitGatewayAttachmentPropagationsOutput`](crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsOutput) with field(s):
    ///   - [`transit_gateway_attachment_propagations(Option<Vec::<TransitGatewayAttachmentPropagation>>)`](crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsOutput::transit_gateway_attachment_propagations): <p>Information about the propagation route tables.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<GetTransitGatewayAttachmentPropagationsError>`](crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsError)
    pub fn get_transit_gateway_attachment_propagations(
        &self,
    ) -> crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder {
        crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}