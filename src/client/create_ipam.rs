// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateIpam`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::set_dry_run):<br>required: **false**<br><p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::set_description):<br>required: **false**<br><p>A description for the IPAM.</p><br>
    ///   - [`operating_regions(AddIpamOperatingRegion)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::operating_regions) / [`set_operating_regions(Option<Vec::<AddIpamOperatingRegion>>)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::set_operating_regions):<br>required: **false**<br><p>The operating Regions for the IPAM. Operating Regions are Amazon Web Services Regions where the IPAM is allowed to manage IP address CIDRs. IPAM only discovers and monitors resources in the Amazon Web Services Regions you select as operating Regions. </p>  <p>For more information about operating Regions, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/create-ipam.html">Create an IPAM</a> in the <i>Amazon VPC IPAM User Guide</i>. </p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    ///   - [`tier(IpamTier)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::tier) / [`set_tier(Option<IpamTier>)`](crate::operation::create_ipam::builders::CreateIpamFluentBuilder::set_tier):<br>required: **false**<br><p>IPAM is offered in a Free Tier and an Advanced Tier. For more information about the features available in each tier and the costs associated with the tiers, see <a href="http://aws.amazon.com/vpc/pricing/">Amazon VPC pricing &gt; IPAM tab</a>.</p><br>
    /// - On success, responds with [`CreateIpamOutput`](crate::operation::create_ipam::CreateIpamOutput) with field(s):
    ///   - [`ipam(Option<Ipam>)`](crate::operation::create_ipam::CreateIpamOutput::ipam): <p>Information about the IPAM created.</p>
    /// - On failure, responds with [`SdkError<CreateIpamError>`](crate::operation::create_ipam::CreateIpamError)
    pub fn create_ipam(&self) -> crate::operation::create_ipam::builders::CreateIpamFluentBuilder {
        crate::operation::create_ipam::builders::CreateIpamFluentBuilder::new(self.handle.clone())
    }
}