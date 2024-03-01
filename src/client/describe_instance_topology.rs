// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeInstanceTopology`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::set_next_token):<br>required: **false**<br><p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>  <p>You can't specify this parameter and the instance IDs parameter in the same request.</p>  <p>Default: <code>20</code> </p><br>
    ///   - [`instance_ids(impl Into<String>)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::instance_ids) / [`set_instance_ids(Option<Vec::<String>>)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::set_instance_ids):<br>required: **false**<br><p>The instance IDs.</p>  <p>Default: Describes all your instances.</p>  <p>Constraints: Maximum 100 explicitly specified instance IDs.</p><br>
    ///   - [`group_names(impl Into<String>)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::group_names) / [`set_group_names(Option<Vec::<String>>)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::set_group_names):<br>required: **false**<br><p>The name of the placement group that each instance is in.</p>  <p>Constraints: Maximum 100 explicitly specified placement group names.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::set_filters):<br>required: **false**<br><p>The filters.</p>  <ul>   <li> <p> <code>availability-zone</code> - The name of the Availability Zone (for example, <code>us-west-2a</code>) or Local Zone (for example, <code>us-west-2-lax-1b</code>) that the instance is in.</p> </li>   <li> <p> <code>instance-type</code> - The instance type (for example, <code>p4d.24xlarge</code>) or instance family (for example, <code>p4d*</code>). You can use the <code>*</code> wildcard to match zero or more characters, or the <code>?</code> wildcard to match zero or one character.</p> </li>   <li> <p> <code>zone-id</code> - The ID of the Availability Zone (for example, <code>usw2-az2</code>) or Local Zone (for example, <code>usw2-lax1-az1</code>) that the instance is in.</p> </li>  </ul><br>
    /// - On success, responds with [`DescribeInstanceTopologyOutput`](crate::operation::describe_instance_topology::DescribeInstanceTopologyOutput) with field(s):
    ///   - [`instances(Option<Vec::<InstanceTopology>>)`](crate::operation::describe_instance_topology::DescribeInstanceTopologyOutput::instances): <p>Information about the topology of each instance.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_instance_topology::DescribeInstanceTopologyOutput::next_token): <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    /// - On failure, responds with [`SdkError<DescribeInstanceTopologyError>`](crate::operation::describe_instance_topology::DescribeInstanceTopologyError)
    pub fn describe_instance_topology(&self) -> crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder {
        crate::operation::describe_instance_topology::builders::DescribeInstanceTopologyFluentBuilder::new(self.handle.clone())
    }
}
