// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteIpamPool`](crate::operation::delete_ipam_pool::builders::DeleteIpamPoolFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_ipam_pool::builders::DeleteIpamPoolFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_ipam_pool::builders::DeleteIpamPoolFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`ipam_pool_id(impl Into<String>)`](crate::operation::delete_ipam_pool::builders::DeleteIpamPoolFluentBuilder::ipam_pool_id) / [`set_ipam_pool_id(Option<String>)`](crate::operation::delete_ipam_pool::builders::DeleteIpamPoolFluentBuilder::set_ipam_pool_id):<br>required: **true**<br><p>The ID of the pool to delete.</p><br>
    ///   - [`cascade(bool)`](crate::operation::delete_ipam_pool::builders::DeleteIpamPoolFluentBuilder::cascade) / [`set_cascade(Option<bool>)`](crate::operation::delete_ipam_pool::builders::DeleteIpamPoolFluentBuilder::set_cascade):<br>required: **false**<br><p>Enables you to quickly delete an IPAM pool and all resources within that pool, including provisioned CIDRs, allocations, and other pools.</p> <important>   <p>You can only use this option to delete pools in the private scope or pools in the public scope with a source resource. A source resource is a resource used to provision CIDRs to a resource planning pool.</p>  </important><br>
    /// - On success, responds with [`DeleteIpamPoolOutput`](crate::operation::delete_ipam_pool::DeleteIpamPoolOutput) with field(s):
    ///   - [`ipam_pool(Option<IpamPool>)`](crate::operation::delete_ipam_pool::DeleteIpamPoolOutput::ipam_pool): <p>Information about the results of the deletion.</p>
    /// - On failure, responds with [`SdkError<DeleteIpamPoolError>`](crate::operation::delete_ipam_pool::DeleteIpamPoolError)
    pub fn delete_ipam_pool(&self) -> crate::operation::delete_ipam_pool::builders::DeleteIpamPoolFluentBuilder {
        crate::operation::delete_ipam_pool::builders::DeleteIpamPoolFluentBuilder::new(self.handle.clone())
    }
}