// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyCapacityReservationFleet`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`capacity_reservation_fleet_id(impl Into<String>)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::capacity_reservation_fleet_id) / [`set_capacity_reservation_fleet_id(Option<String>)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::set_capacity_reservation_fleet_id):<br>required: **true**<br><p>The ID of the Capacity Reservation Fleet to modify.</p><br>
    ///   - [`total_target_capacity(i32)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::total_target_capacity) / [`set_total_target_capacity(Option<i32>)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::set_total_target_capacity):<br>required: **false**<br><p>The total number of capacity units to be reserved by the Capacity Reservation Fleet. This value, together with the instance type weights that you assign to each instance type used by the Fleet determine the number of instances for which the Fleet reserves capacity. Both values are based on units that make sense for your workload. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/crfleet-concepts.html#target-capacity">Total target capacity</a> in the Amazon EC2 User Guide.</p><br>
    ///   - [`end_date(DateTime)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::end_date) / [`set_end_date(Option<DateTime>)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::set_end_date):<br>required: **false**<br><p>The date and time at which the Capacity Reservation Fleet expires. When the Capacity Reservation Fleet expires, its state changes to <code>expired</code> and all of the Capacity Reservations in the Fleet expire.</p>  <p>The Capacity Reservation Fleet expires within an hour after the specified time. For example, if you specify <code>5/31/2019</code>, <code>13:30:55</code>, the Capacity Reservation Fleet is guaranteed to expire between <code>13:30:55</code> and <code>14:30:55</code> on <code>5/31/2019</code>.</p>  <p>You can't specify <b>EndDate</b> and <b> RemoveEndDate</b> in the same request.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`remove_end_date(bool)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::remove_end_date) / [`set_remove_end_date(Option<bool>)`](crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::set_remove_end_date):<br>required: **false**<br><p>Indicates whether to remove the end date from the Capacity Reservation Fleet. If you remove the end date, the Capacity Reservation Fleet does not expire and it remains active until you explicitly cancel it using the <b>CancelCapacityReservationFleet</b> action.</p>  <p>You can't specify <b>RemoveEndDate</b> and <b> EndDate</b> in the same request.</p><br>
    /// - On success, responds with [`ModifyCapacityReservationFleetOutput`](crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetOutput::return): <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    /// - On failure, responds with [`SdkError<ModifyCapacityReservationFleetError>`](crate::operation::modify_capacity_reservation_fleet::ModifyCapacityReservationFleetError)
    pub fn modify_capacity_reservation_fleet(
        &self,
    ) -> crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder {
        crate::operation::modify_capacity_reservation_fleet::builders::ModifyCapacityReservationFleetFluentBuilder::new(self.handle.clone())
    }
}