// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateSubnetCidrReservation`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`subnet_id(impl Into<String>)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::subnet_id) / [`set_subnet_id(Option<String>)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::set_subnet_id):<br>required: **true**<br><p>The ID of the subnet.</p><br>
    ///   - [`cidr(impl Into<String>)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::cidr) / [`set_cidr(Option<String>)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::set_cidr):<br>required: **true**<br><p>The IPv4 or IPV6 CIDR range to reserve.</p><br>
    ///   - [`reservation_type(SubnetCidrReservationType)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::reservation_type) / [`set_reservation_type(Option<SubnetCidrReservationType>)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::set_reservation_type):<br>required: **true**<br><p>The type of reservation. The reservation type determines how the reserved IP addresses are assigned to resources.</p>  <ul>   <li> <p> <code>prefix</code> - Amazon Web Services assigns the reserved IP addresses to network interfaces.</p> </li>   <li> <p> <code>explicit</code> - You assign the reserved IP addresses to network interfaces.</p> </li>  </ul><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::set_description):<br>required: **false**<br><p>The description to assign to the subnet CIDR reservation.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to assign to the subnet CIDR reservation.</p><br>
    /// - On success, responds with [`CreateSubnetCidrReservationOutput`](crate::operation::create_subnet_cidr_reservation::CreateSubnetCidrReservationOutput) with field(s):
    ///   - [`subnet_cidr_reservation(Option<SubnetCidrReservation>)`](crate::operation::create_subnet_cidr_reservation::CreateSubnetCidrReservationOutput::subnet_cidr_reservation): <p>Information about the created subnet CIDR reservation.</p>
    /// - On failure, responds with [`SdkError<CreateSubnetCidrReservationError>`](crate::operation::create_subnet_cidr_reservation::CreateSubnetCidrReservationError)
    pub fn create_subnet_cidr_reservation(
        &self,
    ) -> crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder {
        crate::operation::create_subnet_cidr_reservation::builders::CreateSubnetCidrReservationFluentBuilder::new(self.handle.clone())
    }
}