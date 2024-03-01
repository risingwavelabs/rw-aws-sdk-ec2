// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateCoipCidr`](crate::operation::create_coip_cidr::builders::CreateCoipCidrFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`cidr(impl Into<String>)`](crate::operation::create_coip_cidr::builders::CreateCoipCidrFluentBuilder::cidr) / [`set_cidr(Option<String>)`](crate::operation::create_coip_cidr::builders::CreateCoipCidrFluentBuilder::set_cidr):<br>required: **true**<br><p> A customer-owned IP address range to create. </p><br>
    ///   - [`coip_pool_id(impl Into<String>)`](crate::operation::create_coip_cidr::builders::CreateCoipCidrFluentBuilder::coip_pool_id) / [`set_coip_pool_id(Option<String>)`](crate::operation::create_coip_cidr::builders::CreateCoipCidrFluentBuilder::set_coip_pool_id):<br>required: **true**<br><p> The ID of the address pool. </p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_coip_cidr::builders::CreateCoipCidrFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_coip_cidr::builders::CreateCoipCidrFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`CreateCoipCidrOutput`](crate::operation::create_coip_cidr::CreateCoipCidrOutput) with field(s):
    ///   - [`coip_cidr(Option<CoipCidr>)`](crate::operation::create_coip_cidr::CreateCoipCidrOutput::coip_cidr): <p> Information about a range of customer-owned IP addresses. </p>
    /// - On failure, responds with [`SdkError<CreateCoipCidrError>`](crate::operation::create_coip_cidr::CreateCoipCidrError)
    pub fn create_coip_cidr(&self) -> crate::operation::create_coip_cidr::builders::CreateCoipCidrFluentBuilder {
        crate::operation::create_coip_cidr::builders::CreateCoipCidrFluentBuilder::new(self.handle.clone())
    }
}
