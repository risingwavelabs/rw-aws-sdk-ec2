// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetIpamDiscoveredPublicAddresses`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`ipam_resource_discovery_id(impl Into<String>)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::ipam_resource_discovery_id) / [`set_ipam_resource_discovery_id(Option<String>)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::set_ipam_resource_discovery_id):<br>required: **true**<br><p>An IPAM resource discovery ID.</p><br>
    ///   - [`address_region(impl Into<String>)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::address_region) / [`set_address_region(Option<String>)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::set_address_region):<br>required: **true**<br><p>The Amazon Web Services Region for the IP address.</p><br>
    ///   - [`filters(Filter)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::set_filters):<br>required: **false**<br><p>Filters.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of IPAM discovered public addresses to return in one page of results.</p><br>
    /// - On success, responds with [`GetIpamDiscoveredPublicAddressesOutput`](crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesOutput) with field(s):
    ///   - [`ipam_discovered_public_addresses(Option<Vec::<IpamDiscoveredPublicAddress>>)`](crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesOutput::ipam_discovered_public_addresses): <p>IPAM discovered public addresses.</p>
    ///   - [`oldest_sample_time(Option<DateTime>)`](crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesOutput::oldest_sample_time): <p>The oldest successful resource discovery time.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<GetIpamDiscoveredPublicAddressesError>`](crate::operation::get_ipam_discovered_public_addresses::GetIpamDiscoveredPublicAddressesError)
    pub fn get_ipam_discovered_public_addresses(
        &self,
    ) -> crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder {
        crate::operation::get_ipam_discovered_public_addresses::builders::GetIpamDiscoveredPublicAddressesFluentBuilder::new(self.handle.clone())
    }
}