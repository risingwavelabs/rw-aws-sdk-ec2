// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`MoveByoipCidrToIpam`](crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`cidr(impl Into<String>)`](crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder::cidr) / [`set_cidr(Option<String>)`](crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder::set_cidr):<br>required: **true**<br><p>The BYOIP CIDR.</p><br>
    ///   - [`ipam_pool_id(impl Into<String>)`](crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder::ipam_pool_id) / [`set_ipam_pool_id(Option<String>)`](crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder::set_ipam_pool_id):<br>required: **true**<br><p>The IPAM pool ID.</p><br>
    ///   - [`ipam_pool_owner(impl Into<String>)`](crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder::ipam_pool_owner) / [`set_ipam_pool_owner(Option<String>)`](crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder::set_ipam_pool_owner):<br>required: **true**<br><p>The Amazon Web Services account ID of the owner of the IPAM pool.</p><br>
    /// - On success, responds with [`MoveByoipCidrToIpamOutput`](crate::operation::move_byoip_cidr_to_ipam::MoveByoipCidrToIpamOutput) with field(s):
    ///   - [`byoip_cidr(Option<ByoipCidr>)`](crate::operation::move_byoip_cidr_to_ipam::MoveByoipCidrToIpamOutput::byoip_cidr): <p>The BYOIP CIDR.</p>
    /// - On failure, responds with [`SdkError<MoveByoipCidrToIpamError>`](crate::operation::move_byoip_cidr_to_ipam::MoveByoipCidrToIpamError)
    pub fn move_byoip_cidr_to_ipam(&self) -> crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder {
        crate::operation::move_byoip_cidr_to_ipam::builders::MoveByoipCidrToIpamFluentBuilder::new(self.handle.clone())
    }
}
