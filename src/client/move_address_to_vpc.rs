// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`MoveAddressToVpc`](crate::operation::move_address_to_vpc::builders::MoveAddressToVpcFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::move_address_to_vpc::builders::MoveAddressToVpcFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::move_address_to_vpc::builders::MoveAddressToVpcFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`public_ip(impl Into<String>)`](crate::operation::move_address_to_vpc::builders::MoveAddressToVpcFluentBuilder::public_ip) / [`set_public_ip(Option<String>)`](crate::operation::move_address_to_vpc::builders::MoveAddressToVpcFluentBuilder::set_public_ip):<br>required: **true**<br><p>The Elastic IP address.</p><br>
    /// - On success, responds with [`MoveAddressToVpcOutput`](crate::operation::move_address_to_vpc::MoveAddressToVpcOutput) with field(s):
    ///   - [`allocation_id(Option<String>)`](crate::operation::move_address_to_vpc::MoveAddressToVpcOutput::allocation_id): <p>The allocation ID for the Elastic IP address.</p>
    ///   - [`status(Option<Status>)`](crate::operation::move_address_to_vpc::MoveAddressToVpcOutput::status): <p>The status of the move of the IP address.</p>
    /// - On failure, responds with [`SdkError<MoveAddressToVpcError>`](crate::operation::move_address_to_vpc::MoveAddressToVpcError)
    pub fn move_address_to_vpc(&self) -> crate::operation::move_address_to_vpc::builders::MoveAddressToVpcFluentBuilder {
        crate::operation::move_address_to_vpc::builders::MoveAddressToVpcFluentBuilder::new(self.handle.clone())
    }
}
