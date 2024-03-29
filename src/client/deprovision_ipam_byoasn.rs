// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeprovisionIpamByoasn`](crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`ipam_id(impl Into<String>)`](crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnFluentBuilder::ipam_id) / [`set_ipam_id(Option<String>)`](crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnFluentBuilder::set_ipam_id):<br>required: **true**<br><p>The IPAM ID.</p><br>
    ///   - [`asn(impl Into<String>)`](crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnFluentBuilder::asn) / [`set_asn(Option<String>)`](crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnFluentBuilder::set_asn):<br>required: **true**<br><p>An ASN.</p><br>
    /// - On success, responds with [`DeprovisionIpamByoasnOutput`](crate::operation::deprovision_ipam_byoasn::DeprovisionIpamByoasnOutput) with field(s):
    ///   - [`byoasn(Option<Byoasn>)`](crate::operation::deprovision_ipam_byoasn::DeprovisionIpamByoasnOutput::byoasn): <p>An ASN and BYOIP CIDR association.</p>
    /// - On failure, responds with [`SdkError<DeprovisionIpamByoasnError>`](crate::operation::deprovision_ipam_byoasn::DeprovisionIpamByoasnError)
    pub fn deprovision_ipam_byoasn(&self) -> crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnFluentBuilder {
        crate::operation::deprovision_ipam_byoasn::builders::DeprovisionIpamByoasnFluentBuilder::new(self.handle.clone())
    }
}
