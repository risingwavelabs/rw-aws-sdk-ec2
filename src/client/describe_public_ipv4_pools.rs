// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribePublicIpv4Pools`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`pool_ids(impl Into<String>)`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::pool_ids) / [`set_pool_ids(Option<Vec::<String>>)`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::set_pool_ids):<br>required: **false**<br><p>The IDs of the address pools.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::set_filters):<br>required: **false**<br><p>One or more filters.</p>  <ul>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>  </ul><br>
    /// - On success, responds with [`DescribePublicIpv4PoolsOutput`](crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput) with field(s):
    ///   - [`public_ipv4_pools(Option<Vec::<PublicIpv4Pool>>)`](crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput::public_ipv4_pools): <p>Information about the address pools.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribePublicIpv4PoolsError>`](crate::operation::describe_public_ipv4_pools::DescribePublicIpv4PoolsError)
    pub fn describe_public_ipv4_pools(&self) -> crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder {
        crate::operation::describe_public_ipv4_pools::builders::DescribePublicIpv4PoolsFluentBuilder::new(self.handle.clone())
    }
}
