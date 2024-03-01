// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyAvailabilityZoneGroup`](crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`group_name(impl Into<String>)`](crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupFluentBuilder::group_name) / [`set_group_name(Option<String>)`](crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupFluentBuilder::set_group_name):<br>required: **true**<br><p>The name of the Availability Zone group, Local Zone group, or Wavelength Zone group.</p><br>
    ///   - [`opt_in_status(ModifyAvailabilityZoneOptInStatus)`](crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupFluentBuilder::opt_in_status) / [`set_opt_in_status(Option<ModifyAvailabilityZoneOptInStatus>)`](crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupFluentBuilder::set_opt_in_status):<br>required: **true**<br><p>Indicates whether you are opted in to the Local Zone group or Wavelength Zone group. The only valid value is <code>opted-in</code>. You must contact <a href="https://console.aws.amazon.com/support/home#/case/create%3FissueType=customer-service%26serviceCode=general-info%26getting-started%26categoryCode=using-aws%26services">Amazon Web Services Support</a> to opt out of a Local Zone or Wavelength Zone group.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`ModifyAvailabilityZoneGroupOutput`](crate::operation::modify_availability_zone_group::ModifyAvailabilityZoneGroupOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::modify_availability_zone_group::ModifyAvailabilityZoneGroupOutput::return): <p>Is <code>true</code> if the request succeeds, and an error otherwise.</p>
    /// - On failure, responds with [`SdkError<ModifyAvailabilityZoneGroupError>`](crate::operation::modify_availability_zone_group::ModifyAvailabilityZoneGroupError)
    pub fn modify_availability_zone_group(
        &self,
    ) -> crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupFluentBuilder {
        crate::operation::modify_availability_zone_group::builders::ModifyAvailabilityZoneGroupFluentBuilder::new(self.handle.clone())
    }
}