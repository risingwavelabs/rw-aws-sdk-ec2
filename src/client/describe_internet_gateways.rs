// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeInternetGateways`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Filter)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::set_filters):<br>required: **false**<br><p>The filters.</p>  <ul>   <li> <p> <code>attachment.state</code> - The current state of the attachment between the gateway and the VPC (<code>available</code>). Present only if a VPC is attached.</p> </li>   <li> <p> <code>attachment.vpc-id</code> - The ID of an attached VPC.</p> </li>   <li> <p> <code>internet-gateway-id</code> - The ID of the Internet gateway.</p> </li>   <li> <p> <code>owner-id</code> - The ID of the Amazon Web Services account that owns the internet gateway.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>  </ul><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`internet_gateway_ids(impl Into<String>)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::internet_gateway_ids) / [`set_internet_gateway_ids(Option<Vec::<String>>)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::set_internet_gateway_ids):<br>required: **false**<br><p>The IDs of the internet gateways.</p>  <p>Default: Describes all your internet gateways.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::set_next_token):<br>required: **false**<br><p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p><br>
    /// - On success, responds with [`DescribeInternetGatewaysOutput`](crate::operation::describe_internet_gateways::DescribeInternetGatewaysOutput) with field(s):
    ///   - [`internet_gateways(Option<Vec::<InternetGateway>>)`](crate::operation::describe_internet_gateways::DescribeInternetGatewaysOutput::internet_gateways): <p>Information about one or more internet gateways.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_internet_gateways::DescribeInternetGatewaysOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<DescribeInternetGatewaysError>`](crate::operation::describe_internet_gateways::DescribeInternetGatewaysError)
    pub fn describe_internet_gateways(&self) -> crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder {
        crate::operation::describe_internet_gateways::builders::DescribeInternetGatewaysFluentBuilder::new(self.handle.clone())
    }
}