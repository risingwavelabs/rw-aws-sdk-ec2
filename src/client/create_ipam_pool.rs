// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateIpamPool`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`ipam_scope_id(impl Into<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::ipam_scope_id) / [`set_ipam_scope_id(Option<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_ipam_scope_id):<br>required: **true**<br><p>The ID of the scope in which you would like to create the IPAM pool.</p><br>
    ///   - [`locale(impl Into<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::locale) / [`set_locale(Option<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_locale):<br>required: **false**<br><p>In IPAM, the locale is the Amazon Web Services Region where you want to make an IPAM pool available for allocations. Only resources in the same Region as the locale of the pool can get IP address allocations from the pool. You can only allocate a CIDR for a VPC, for example, from an IPAM pool that shares a locale with the VPC’s Region. Note that once you choose a Locale for a pool, you cannot modify it. If you do not choose a locale, resources in Regions others than the IPAM's home region cannot use CIDRs from this pool.</p>  <p>Possible values: Any Amazon Web Services Region, such as us-east-1.</p><br>
    ///   - [`source_ipam_pool_id(impl Into<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::source_ipam_pool_id) / [`set_source_ipam_pool_id(Option<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_source_ipam_pool_id):<br>required: **false**<br><p>The ID of the source IPAM pool. Use this option to create a pool within an existing pool. Note that the CIDR you provision for the pool within the source pool must be available in the source pool's CIDR range.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_description):<br>required: **false**<br><p>A description for the IPAM pool.</p><br>
    ///   - [`address_family(AddressFamily)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::address_family) / [`set_address_family(Option<AddressFamily>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_address_family):<br>required: **true**<br><p>The IP protocol assigned to this IPAM pool. You must choose either IPv4 or IPv6 protocol for a pool.</p><br>
    ///   - [`auto_import(bool)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::auto_import) / [`set_auto_import(Option<bool>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_auto_import):<br>required: **false**<br><p>If selected, IPAM will continuously look for resources within the CIDR range of this pool and automatically import them as allocations into your IPAM. The CIDRs that will be allocated for these resources must not already be allocated to other resources in order for the import to succeed. IPAM will import a CIDR regardless of its compliance with the pool's allocation rules, so a resource might be imported and subsequently marked as noncompliant. If IPAM discovers multiple CIDRs that overlap, IPAM will import the largest CIDR only. If IPAM discovers multiple CIDRs with matching CIDRs, IPAM will randomly import one of them only. </p>  <p>A locale must be set on the pool for this feature to work.</p><br>
    ///   - [`publicly_advertisable(bool)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::publicly_advertisable) / [`set_publicly_advertisable(Option<bool>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_publicly_advertisable):<br>required: **false**<br><p>Determines if the pool is publicly advertisable. This option is not available for pools with AddressFamily set to <code>ipv4</code>.</p><br>
    ///   - [`allocation_min_netmask_length(i32)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::allocation_min_netmask_length) / [`set_allocation_min_netmask_length(Option<i32>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_allocation_min_netmask_length):<br>required: **false**<br><p>The minimum netmask length required for CIDR allocations in this IPAM pool to be compliant. The minimum netmask length must be less than the maximum netmask length. Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128.</p><br>
    ///   - [`allocation_max_netmask_length(i32)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::allocation_max_netmask_length) / [`set_allocation_max_netmask_length(Option<i32>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_allocation_max_netmask_length):<br>required: **false**<br><p>The maximum netmask length possible for CIDR allocations in this IPAM pool to be compliant. The maximum netmask length must be greater than the minimum netmask length. Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128.</p><br>
    ///   - [`allocation_default_netmask_length(i32)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::allocation_default_netmask_length) / [`set_allocation_default_netmask_length(Option<i32>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_allocation_default_netmask_length):<br>required: **false**<br><p>The default netmask length for allocations added to this pool. If, for example, the CIDR assigned to this pool is 10.0.0.0/8 and you enter 16 here, new allocations will default to 10.0.0.0/16.</p><br>
    ///   - [`allocation_resource_tags(RequestIpamResourceTag)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::allocation_resource_tags) / [`set_allocation_resource_tags(Option<Vec::<RequestIpamResourceTag>>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_allocation_resource_tags):<br>required: **false**<br><p>Tags that are required for resources that use CIDRs from this IPAM pool. Resources that do not have these tags will not be allowed to allocate space from the pool. If the resources have their tags changed after they have allocated space or if the allocation tagging requirements are changed on the pool, the resource may be marked as noncompliant.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    ///   - [`aws_service(IpamPoolAwsService)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::aws_service) / [`set_aws_service(Option<IpamPoolAwsService>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_aws_service):<br>required: **false**<br><p>Limits which service in Amazon Web Services that the pool can be used in. "ec2", for example, allows users to use space for Elastic IP addresses and VPCs.</p><br>
    ///   - [`public_ip_source(IpamPoolPublicIpSource)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::public_ip_source) / [`set_public_ip_source(Option<IpamPoolPublicIpSource>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_public_ip_source):<br>required: **false**<br><p>The IP address source for pools in the public scope. Only used for provisioning IP address CIDRs to pools in the public scope. Default is <code>byoip</code>. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/intro-create-ipv6-pools.html">Create IPv6 pools</a> in the <i>Amazon VPC IPAM User Guide</i>. By default, you can add only one Amazon-provided IPv6 CIDR block to a top-level IPv6 pool if PublicIpSource is <code>amazon</code>. For information on increasing the default limit, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/quotas-ipam.html"> Quotas for your IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>.</p><br>
    ///   - [`source_resource(IpamPoolSourceResourceRequest)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::source_resource) / [`set_source_resource(Option<IpamPoolSourceResourceRequest>)`](crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::set_source_resource):<br>required: **false**<br><p>The resource used to provision CIDRs to a resource planning pool.</p><br>
    /// - On success, responds with [`CreateIpamPoolOutput`](crate::operation::create_ipam_pool::CreateIpamPoolOutput) with field(s):
    ///   - [`ipam_pool(Option<IpamPool>)`](crate::operation::create_ipam_pool::CreateIpamPoolOutput::ipam_pool): <p>Information about the IPAM pool created.</p>
    /// - On failure, responds with [`SdkError<CreateIpamPoolError>`](crate::operation::create_ipam_pool::CreateIpamPoolError)
    pub fn create_ipam_pool(&self) -> crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder {
        crate::operation::create_ipam_pool::builders::CreateIpamPoolFluentBuilder::new(self.handle.clone())
    }
}
