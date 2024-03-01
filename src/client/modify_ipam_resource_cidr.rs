// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyIpamResourceCidr`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`resource_id(impl Into<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::resource_id) / [`set_resource_id(Option<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::set_resource_id):<br>required: **true**<br><p>The ID of the resource you want to modify.</p><br>
    ///   - [`resource_cidr(impl Into<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::resource_cidr) / [`set_resource_cidr(Option<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::set_resource_cidr):<br>required: **true**<br><p>The CIDR of the resource you want to modify.</p><br>
    ///   - [`resource_region(impl Into<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::resource_region) / [`set_resource_region(Option<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::set_resource_region):<br>required: **true**<br><p>The Amazon Web Services Region of the resource you want to modify.</p><br>
    ///   - [`current_ipam_scope_id(impl Into<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::current_ipam_scope_id) / [`set_current_ipam_scope_id(Option<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::set_current_ipam_scope_id):<br>required: **true**<br><p>The ID of the current scope that the resource CIDR is in.</p><br>
    ///   - [`destination_ipam_scope_id(impl Into<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::destination_ipam_scope_id) / [`set_destination_ipam_scope_id(Option<String>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::set_destination_ipam_scope_id):<br>required: **false**<br><p>The ID of the scope you want to transfer the resource CIDR to.</p><br>
    ///   - [`monitored(bool)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::monitored) / [`set_monitored(Option<bool>)`](crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::set_monitored):<br>required: **true**<br><p>Determines if the resource is monitored by IPAM. If a resource is monitored, the resource is discovered by IPAM and you can view details about the resource’s CIDR.</p><br>
    /// - On success, responds with [`ModifyIpamResourceCidrOutput`](crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput) with field(s):
    ///   - [`ipam_resource_cidr(Option<IpamResourceCidr>)`](crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrOutput::ipam_resource_cidr): <p>The CIDR of the resource.</p>
    /// - On failure, responds with [`SdkError<ModifyIpamResourceCidrError>`](crate::operation::modify_ipam_resource_cidr::ModifyIpamResourceCidrError)
    pub fn modify_ipam_resource_cidr(&self) -> crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder {
        crate::operation::modify_ipam_resource_cidr::builders::ModifyIpamResourceCidrFluentBuilder::new(self.handle.clone())
    }
}