// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeprovisionIpamPoolCidr`](crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`ipam_pool_id(impl Into<String>)`](crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrFluentBuilder::ipam_pool_id) / [`set_ipam_pool_id(Option<String>)`](crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrFluentBuilder::set_ipam_pool_id):<br>required: **true**<br><p>The ID of the pool that has the CIDR you want to deprovision.</p><br>
    ///   - [`cidr(impl Into<String>)`](crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrFluentBuilder::cidr) / [`set_cidr(Option<String>)`](crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrFluentBuilder::set_cidr):<br>required: **false**<br><p>The CIDR which you want to deprovision from the pool.</p><br>
    /// - On success, responds with [`DeprovisionIpamPoolCidrOutput`](crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrOutput) with field(s):
    ///   - [`ipam_pool_cidr(Option<IpamPoolCidr>)`](crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrOutput::ipam_pool_cidr): <p>The deprovisioned pool CIDR.</p>
    /// - On failure, responds with [`SdkError<DeprovisionIpamPoolCidrError>`](crate::operation::deprovision_ipam_pool_cidr::DeprovisionIpamPoolCidrError)
    pub fn deprovision_ipam_pool_cidr(&self) -> crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrFluentBuilder {
        crate::operation::deprovision_ipam_pool_cidr::builders::DeprovisionIpamPoolCidrFluentBuilder::new(self.handle.clone())
    }
}
