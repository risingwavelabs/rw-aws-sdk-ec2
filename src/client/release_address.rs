// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ReleaseAddress`](crate::operation::release_address::builders::ReleaseAddressFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`allocation_id(impl Into<String>)`](crate::operation::release_address::builders::ReleaseAddressFluentBuilder::allocation_id) / [`set_allocation_id(Option<String>)`](crate::operation::release_address::builders::ReleaseAddressFluentBuilder::set_allocation_id):<br>required: **false**<br><p>The allocation ID. This parameter is required.</p><br>
    ///   - [`public_ip(impl Into<String>)`](crate::operation::release_address::builders::ReleaseAddressFluentBuilder::public_ip) / [`set_public_ip(Option<String>)`](crate::operation::release_address::builders::ReleaseAddressFluentBuilder::set_public_ip):<br>required: **false**<br><p>Deprecated.</p><br>
    ///   - [`network_border_group(impl Into<String>)`](crate::operation::release_address::builders::ReleaseAddressFluentBuilder::network_border_group) / [`set_network_border_group(Option<String>)`](crate::operation::release_address::builders::ReleaseAddressFluentBuilder::set_network_border_group):<br>required: **false**<br><p>The set of Availability Zones, Local Zones, or Wavelength Zones from which Amazon Web Services advertises IP addresses.</p>  <p>If you provide an incorrect network border group, you receive an <code>InvalidAddress.NotFound</code> error.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::release_address::builders::ReleaseAddressFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::release_address::builders::ReleaseAddressFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`ReleaseAddressOutput`](crate::operation::release_address::ReleaseAddressOutput)
    /// - On failure, responds with [`SdkError<ReleaseAddressError>`](crate::operation::release_address::ReleaseAddressError)
    pub fn release_address(&self) -> crate::operation::release_address::builders::ReleaseAddressFluentBuilder {
        crate::operation::release_address::builders::ReleaseAddressFluentBuilder::new(self.handle.clone())
    }
}
