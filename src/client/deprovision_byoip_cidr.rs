// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeprovisionByoipCidr`](crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cidr(impl Into<String>)`](crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrFluentBuilder::cidr) / [`set_cidr(Option<String>)`](crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrFluentBuilder::set_cidr):<br>required: **true**<br><p>The address range, in CIDR notation. The prefix must be the same prefix that you specified when you provisioned the address range.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DeprovisionByoipCidrOutput`](crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrOutput) with field(s):
    ///   - [`byoip_cidr(Option<ByoipCidr>)`](crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrOutput::byoip_cidr): <p>Information about the address range.</p>
    /// - On failure, responds with [`SdkError<DeprovisionByoipCidrError>`](crate::operation::deprovision_byoip_cidr::DeprovisionByoipCidrError)
    pub fn deprovision_byoip_cidr(&self) -> crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrFluentBuilder {
        crate::operation::deprovision_byoip_cidr::builders::DeprovisionByoipCidrFluentBuilder::new(self.handle.clone())
    }
}
