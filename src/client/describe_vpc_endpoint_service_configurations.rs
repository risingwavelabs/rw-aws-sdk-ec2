// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeVpcEndpointServiceConfigurations`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`service_ids(impl Into<String>)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::service_ids) / [`set_service_ids(Option<Vec::<String>>)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::set_service_ids):<br>required: **false**<br><p>The IDs of the endpoint services.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::set_filters):<br>required: **false**<br><p>The filters.</p>  <ul>   <li> <p> <code>service-name</code> - The name of the service.</p> </li>   <li> <p> <code>service-id</code> - The ID of the service.</p> </li>   <li> <p> <code>service-state</code> - The state of the service (<code>Pending</code> | <code>Available</code> | <code>Deleting</code> | <code>Deleted</code> | <code>Failed</code>). </p> </li>   <li> <p> <code>supported-ip-address-types</code> - The IP address type (<code>ipv4</code> | <code>ipv6</code>).</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>  </ul><br>
    ///   - [`max_results(i32)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return for the request in a single page. The remaining results of the initial request can be seen by sending another request with the returned <code>NextToken</code> value. This value can be between 5 and 1,000; if <code>MaxResults</code> is given a value larger than 1,000, only 1,000 results are returned.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token to retrieve the next page of results.</p><br>
    /// - On success, responds with [`DescribeVpcEndpointServiceConfigurationsOutput`](crate::operation::describe_vpc_endpoint_service_configurations::DescribeVpcEndpointServiceConfigurationsOutput) with field(s):
    ///   - [`service_configurations(Option<Vec::<ServiceConfiguration>>)`](crate::operation::describe_vpc_endpoint_service_configurations::DescribeVpcEndpointServiceConfigurationsOutput::service_configurations): <p>Information about the services.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_vpc_endpoint_service_configurations::DescribeVpcEndpointServiceConfigurationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeVpcEndpointServiceConfigurationsError>`](crate::operation::describe_vpc_endpoint_service_configurations::DescribeVpcEndpointServiceConfigurationsError)
    pub fn describe_vpc_endpoint_service_configurations(
        &self,
    ) -> crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder {
        crate::operation::describe_vpc_endpoint_service_configurations::builders::DescribeVpcEndpointServiceConfigurationsFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
