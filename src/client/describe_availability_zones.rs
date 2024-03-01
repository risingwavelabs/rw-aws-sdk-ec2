// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAvailabilityZones`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Filter)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::set_filters):<br>required: **false**<br><p>The filters.</p>  <ul>   <li> <p> <code>group-name</code> - For Availability Zones, use the Region name. For Local Zones, use the name of the group associated with the Local Zone (for example, <code>us-west-2-lax-1</code>) For Wavelength Zones, use the name of the group associated with the Wavelength Zone (for example, <code>us-east-1-wl1-bos-wlz-1</code>).</p> </li>   <li> <p> <code>message</code> - The Zone message.</p> </li>   <li> <p> <code>opt-in-status</code> - The opt-in status (<code>opted-in</code> | <code>not-opted-in</code> | <code>opt-in-not-required</code>).</p> </li>   <li> <p> <code>parent-zoneID</code> - The ID of the zone that handles some of the Local Zone and Wavelength Zone control plane operations, such as API calls.</p> </li>   <li> <p> <code>parent-zoneName</code> - The ID of the zone that handles some of the Local Zone and Wavelength Zone control plane operations, such as API calls.</p> </li>   <li> <p> <code>region-name</code> - The name of the Region for the Zone (for example, <code>us-east-1</code>).</p> </li>   <li> <p> <code>state</code> - The state of the Availability Zone, the Local Zone, or the Wavelength Zone (<code>available</code>).</p> </li>   <li> <p> <code>zone-id</code> - The ID of the Availability Zone (for example, <code>use1-az1</code>), the Local Zone (for example, <code>usw2-lax1-az1</code>), or the Wavelength Zone (for example, <code>us-east-1-wl1-bos-wlz-1</code>).</p> </li>   <li> <p> <code>zone-name</code> - The name of the Availability Zone (for example, <code>us-east-1a</code>), the Local Zone (for example, <code>us-west-2-lax-1a</code>), or the Wavelength Zone (for example, <code>us-east-1-wl1-bos-wlz-1</code>).</p> </li>   <li> <p> <code>zone-type</code> - The type of zone (<code>availability-zone</code> | <code>local-zone</code> | <code>wavelength-zone</code>).</p> </li>  </ul><br>
    ///   - [`zone_names(impl Into<String>)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::zone_names) / [`set_zone_names(Option<Vec::<String>>)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::set_zone_names):<br>required: **false**<br><p>The names of the Availability Zones, Local Zones, and Wavelength Zones.</p><br>
    ///   - [`zone_ids(impl Into<String>)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::zone_ids) / [`set_zone_ids(Option<Vec::<String>>)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::set_zone_ids):<br>required: **false**<br><p>The IDs of the Availability Zones, Local Zones, and Wavelength Zones.</p><br>
    ///   - [`all_availability_zones(bool)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::all_availability_zones) / [`set_all_availability_zones(Option<bool>)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::set_all_availability_zones):<br>required: **false**<br><p>Include all Availability Zones, Local Zones, and Wavelength Zones regardless of your opt-in status.</p>  <p>If you do not use this parameter, the results include only the zones for the Regions where you have chosen the option to opt in.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DescribeAvailabilityZonesOutput`](crate::operation::describe_availability_zones::DescribeAvailabilityZonesOutput) with field(s):
    ///   - [`availability_zones(Option<Vec::<AvailabilityZone>>)`](crate::operation::describe_availability_zones::DescribeAvailabilityZonesOutput::availability_zones): <p>Information about the Availability Zones, Local Zones, and Wavelength Zones.</p>
    /// - On failure, responds with [`SdkError<DescribeAvailabilityZonesError>`](crate::operation::describe_availability_zones::DescribeAvailabilityZonesError)
    pub fn describe_availability_zones(&self) -> crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder {
        crate::operation::describe_availability_zones::builders::DescribeAvailabilityZonesFluentBuilder::new(self.handle.clone())
    }
}
