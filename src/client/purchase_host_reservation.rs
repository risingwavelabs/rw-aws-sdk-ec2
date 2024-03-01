// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PurchaseHostReservation`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`client_token(impl Into<String>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::set_client_token):<br>required: **false**<br><p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p><br>
    ///   - [`currency_code(CurrencyCodeValues)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::currency_code) / [`set_currency_code(Option<CurrencyCodeValues>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::set_currency_code):<br>required: **false**<br><p>The currency in which the <code>totalUpfrontPrice</code>, <code>LimitPrice</code>, and <code>totalHourlyPrice</code> amounts are specified. At this time, the only supported currency is <code>USD</code>.</p><br>
    ///   - [`host_id_set(impl Into<String>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::host_id_set) / [`set_host_id_set(Option<Vec::<String>>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::set_host_id_set):<br>required: **true**<br><p>The IDs of the Dedicated Hosts with which the reservation will be associated.</p><br>
    ///   - [`limit_price(impl Into<String>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::limit_price) / [`set_limit_price(Option<String>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::set_limit_price):<br>required: **false**<br><p>The specified limit is checked against the total upfront cost of the reservation (calculated as the offering's upfront cost multiplied by the host count). If the total upfront cost is greater than the specified price limit, the request fails. This is used to ensure that the purchase does not exceed the expected upfront cost of the purchase. At this time, the only supported currency is <code>USD</code>. For example, to indicate a limit price of USD 100, specify 100.00.</p><br>
    ///   - [`offering_id(impl Into<String>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::offering_id) / [`set_offering_id(Option<String>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::set_offering_id):<br>required: **true**<br><p>The ID of the offering.</p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to apply to the Dedicated Host Reservation during purchase.</p><br>
    /// - On success, responds with [`PurchaseHostReservationOutput`](crate::operation::purchase_host_reservation::PurchaseHostReservationOutput) with field(s):
    ///   - [`client_token(Option<String>)`](crate::operation::purchase_host_reservation::PurchaseHostReservationOutput::client_token): <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    ///   - [`currency_code(Option<CurrencyCodeValues>)`](crate::operation::purchase_host_reservation::PurchaseHostReservationOutput::currency_code): <p>The currency in which the <code>totalUpfrontPrice</code> and <code>totalHourlyPrice</code> amounts are specified. At this time, the only supported currency is <code>USD</code>.</p>
    ///   - [`purchase(Option<Vec::<Purchase>>)`](crate::operation::purchase_host_reservation::PurchaseHostReservationOutput::purchase): <p>Describes the details of the purchase.</p>
    ///   - [`total_hourly_price(Option<String>)`](crate::operation::purchase_host_reservation::PurchaseHostReservationOutput::total_hourly_price): <p>The total hourly price of the reservation calculated per hour.</p>
    ///   - [`total_upfront_price(Option<String>)`](crate::operation::purchase_host_reservation::PurchaseHostReservationOutput::total_upfront_price): <p>The total amount charged to your account when you purchase the reservation.</p>
    /// - On failure, responds with [`SdkError<PurchaseHostReservationError>`](crate::operation::purchase_host_reservation::PurchaseHostReservationError)
    pub fn purchase_host_reservation(&self) -> crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder {
        crate::operation::purchase_host_reservation::builders::PurchaseHostReservationFluentBuilder::new(self.handle.clone())
    }
}