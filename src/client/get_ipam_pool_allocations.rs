// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetIpamPoolAllocations`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`ipam_pool_id(impl Into<String>)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::ipam_pool_id) / [`set_ipam_pool_id(Option<String>)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::set_ipam_pool_id):<br>required: **true**<br><p>The ID of the IPAM pool you want to see the allocations for.</p><br>
    ///   - [`ipam_pool_allocation_id(impl Into<String>)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::ipam_pool_allocation_id) / [`set_ipam_pool_allocation_id(Option<String>)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::set_ipam_pool_allocation_id):<br>required: **false**<br><p>The ID of the allocation.</p><br>
    ///   - [`filters(Filter)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::set_filters):<br>required: **false**<br><p>One or more filters for the request. For more information about filtering, see <a href="https://docs.aws.amazon.com/cli/latest/userguide/cli-usage-filter.html">Filtering CLI output</a>.</p><br>
    ///   - [`max_results(i32)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results you would like returned per page.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    /// - On success, responds with [`GetIpamPoolAllocationsOutput`](crate::operation::get_ipam_pool_allocations::GetIpamPoolAllocationsOutput) with field(s):
    ///   - [`ipam_pool_allocations(Option<Vec::<IpamPoolAllocation>>)`](crate::operation::get_ipam_pool_allocations::GetIpamPoolAllocationsOutput::ipam_pool_allocations): <p>The IPAM pool allocations you want information on.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_ipam_pool_allocations::GetIpamPoolAllocationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<GetIpamPoolAllocationsError>`](crate::operation::get_ipam_pool_allocations::GetIpamPoolAllocationsError)
    pub fn get_ipam_pool_allocations(&self) -> crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder {
        crate::operation::get_ipam_pool_allocations::builders::GetIpamPoolAllocationsFluentBuilder::new(self.handle.clone())
    }
}
