// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AdvertiseByoipCidr`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cidr(impl Into<String>)`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::cidr) / [`set_cidr(Option<String>)`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::set_cidr):<br>required: **true**<br><p>The address range, in CIDR notation. This must be the exact range that you provisioned. You can't advertise only a portion of the provisioned range.</p><br>
    ///   - [`asn(impl Into<String>)`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::asn) / [`set_asn(Option<String>)`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::set_asn):<br>required: **false**<br><p>The public 2-byte or 4-byte ASN that you want to advertise.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`AdvertiseByoipCidrOutput`](crate::operation::advertise_byoip_cidr::AdvertiseByoipCidrOutput) with field(s):
    ///   - [`byoip_cidr(Option<ByoipCidr>)`](crate::operation::advertise_byoip_cidr::AdvertiseByoipCidrOutput::byoip_cidr): <p>Information about the address range.</p>
    /// - On failure, responds with [`SdkError<AdvertiseByoipCidrError>`](crate::operation::advertise_byoip_cidr::AdvertiseByoipCidrError)
    pub fn advertise_byoip_cidr(&self) -> crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder {
        crate::operation::advertise_byoip_cidr::builders::AdvertiseByoipCidrFluentBuilder::new(self.handle.clone())
    }
}