// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeVpcEndpointConnections`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::set_filters):<br>required: **false**<br><p>The filters.</p>  <ul>   <li> <p> <code>ip-address-type</code> - The IP address type (<code>ipv4</code> | <code>ipv6</code>).</p> </li>   <li> <p> <code>service-id</code> - The ID of the service.</p> </li>   <li> <p> <code>vpc-endpoint-owner</code> - The ID of the Amazon Web Services account ID that owns the endpoint.</p> </li>   <li> <p> <code>vpc-endpoint-state</code> - The state of the endpoint (<code>pendingAcceptance</code> | <code>pending</code> | <code>available</code> | <code>deleting</code> | <code>deleted</code> | <code>rejected</code> | <code>failed</code>).</p> </li>   <li> <p> <code>vpc-endpoint-id</code> - The ID of the endpoint.</p> </li>  </ul><br>
    ///   - [`max_results(i32)`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return for the request in a single page. The remaining results of the initial request can be seen by sending another request with the returned <code>NextToken</code> value. This value can be between 5 and 1,000; if <code>MaxResults</code> is given a value larger than 1,000, only 1,000 results are returned.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token to retrieve the next page of results.</p><br>
    /// - On success, responds with [`DescribeVpcEndpointConnectionsOutput`](crate::operation::describe_vpc_endpoint_connections::DescribeVpcEndpointConnectionsOutput) with field(s):
    ///   - [`vpc_endpoint_connections(Option<Vec::<VpcEndpointConnection>>)`](crate::operation::describe_vpc_endpoint_connections::DescribeVpcEndpointConnectionsOutput::vpc_endpoint_connections): <p>Information about the VPC endpoint connections.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_vpc_endpoint_connections::DescribeVpcEndpointConnectionsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeVpcEndpointConnectionsError>`](crate::operation::describe_vpc_endpoint_connections::DescribeVpcEndpointConnectionsError)
    pub fn describe_vpc_endpoint_connections(
        &self,
    ) -> crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder {
        crate::operation::describe_vpc_endpoint_connections::builders::DescribeVpcEndpointConnectionsFluentBuilder::new(self.handle.clone())
    }
}
