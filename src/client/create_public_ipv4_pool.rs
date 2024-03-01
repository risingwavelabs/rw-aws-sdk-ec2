// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreatePublicIpv4Pool`](crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p><br>
    /// - On success, responds with [`CreatePublicIpv4PoolOutput`](crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolOutput) with field(s):
    ///   - [`pool_id(Option<String>)`](crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolOutput::pool_id): <p>The ID of the public IPv4 pool.</p>
    /// - On failure, responds with [`SdkError<CreatePublicIpv4PoolError>`](crate::operation::create_public_ipv4_pool::CreatePublicIpv4PoolError)
    pub fn create_public_ipv4_pool(&self) -> crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolFluentBuilder {
        crate::operation::create_public_ipv4_pool::builders::CreatePublicIpv4PoolFluentBuilder::new(self.handle.clone())
    }
}
