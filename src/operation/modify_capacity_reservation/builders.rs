// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_capacity_reservation::_modify_capacity_reservation_output::ModifyCapacityReservationOutputBuilder;

pub use crate::operation::modify_capacity_reservation::_modify_capacity_reservation_input::ModifyCapacityReservationInputBuilder;

impl ModifyCapacityReservationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_capacity_reservation::ModifyCapacityReservationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_capacity_reservation::ModifyCapacityReservationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_capacity_reservation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyCapacityReservation`.
///
/// <p>Modifies a Capacity Reservation's capacity and the conditions under which it is to be released. You cannot change a Capacity Reservation's instance type, EBS optimization, instance store settings, platform, Availability Zone, or instance eligibility. If you need to modify any of these attributes, we recommend that you cancel the Capacity Reservation, and then create a new one with the required attributes.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyCapacityReservationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_capacity_reservation::builders::ModifyCapacityReservationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_capacity_reservation::ModifyCapacityReservationOutput,
        crate::operation::modify_capacity_reservation::ModifyCapacityReservationError,
    > for ModifyCapacityReservationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_capacity_reservation::ModifyCapacityReservationOutput,
            crate::operation::modify_capacity_reservation::ModifyCapacityReservationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyCapacityReservationFluentBuilder {
    /// Creates a new `ModifyCapacityReservation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyCapacityReservation as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_capacity_reservation::builders::ModifyCapacityReservationInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_capacity_reservation::ModifyCapacityReservationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_capacity_reservation::ModifyCapacityReservationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_capacity_reservation::ModifyCapacityReservation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_capacity_reservation::ModifyCapacityReservation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_capacity_reservation::ModifyCapacityReservationOutput,
        crate::operation::modify_capacity_reservation::ModifyCapacityReservationError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn capacity_reservation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.capacity_reservation_id(input.into());
        self
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn set_capacity_reservation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_capacity_reservation_id(input);
        self
    }
    /// <p>The ID of the Capacity Reservation.</p>
    pub fn get_capacity_reservation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_capacity_reservation_id()
    }
    /// <p>The number of instances for which to reserve capacity. The number of instances can't be increased or decreased by more than <code>1000</code> in a single request.</p>
    pub fn instance_count(mut self, input: i32) -> Self {
        self.inner = self.inner.instance_count(input);
        self
    }
    /// <p>The number of instances for which to reserve capacity. The number of instances can't be increased or decreased by more than <code>1000</code> in a single request.</p>
    pub fn set_instance_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_instance_count(input);
        self
    }
    /// <p>The number of instances for which to reserve capacity. The number of instances can't be increased or decreased by more than <code>1000</code> in a single request.</p>
    pub fn get_instance_count(&self) -> &::std::option::Option<i32> {
        self.inner.get_instance_count()
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>The Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    pub fn end_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_date(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>The Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    pub fn set_end_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_date(input);
        self
    }
    /// <p>The date and time at which the Capacity Reservation expires. When a Capacity Reservation expires, the reserved capacity is released and you can no longer launch instances into it. The Capacity Reservation's state changes to <code>expired</code> when it reaches its end date and time.</p>
    /// <p>The Capacity Reservation is cancelled within an hour from the specified time. For example, if you specify 5/31/2019, 13:30:55, the Capacity Reservation is guaranteed to end between 13:30:55 and 14:30:55 on 5/31/2019.</p>
    /// <p>You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>. Omit <code>EndDate</code> if <code>EndDateType</code> is <code>unlimited</code>.</p>
    pub fn get_end_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_date()
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn end_date_type(mut self, input: crate::types::EndDateType) -> Self {
        self.inner = self.inner.end_date_type(input);
        self
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn set_end_date_type(mut self, input: ::std::option::Option<crate::types::EndDateType>) -> Self {
        self.inner = self.inner.set_end_date_type(input);
        self
    }
    /// <p>Indicates the way in which the Capacity Reservation ends. A Capacity Reservation can have one of the following end types:</p>
    /// <ul>
    /// <li> <p> <code>unlimited</code> - The Capacity Reservation remains active until you explicitly cancel it. Do not provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>unlimited</code>.</p> </li>
    /// <li> <p> <code>limited</code> - The Capacity Reservation expires automatically at a specified date and time. You must provide an <code>EndDate</code> value if <code>EndDateType</code> is <code>limited</code>.</p> </li>
    /// </ul>
    pub fn get_end_date_type(&self) -> &::std::option::Option<crate::types::EndDateType> {
        self.inner.get_end_date_type()
    }
    /// <p>Reserved. Capacity Reservations you have created are accepted by default.</p>
    pub fn accept(mut self, input: bool) -> Self {
        self.inner = self.inner.accept(input);
        self
    }
    /// <p>Reserved. Capacity Reservations you have created are accepted by default.</p>
    pub fn set_accept(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_accept(input);
        self
    }
    /// <p>Reserved. Capacity Reservations you have created are accepted by default.</p>
    pub fn get_accept(&self) -> &::std::option::Option<bool> {
        self.inner.get_accept()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>Reserved for future use.</p>
    pub fn additional_info(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.additional_info(input.into());
        self
    }
    /// <p>Reserved for future use.</p>
    pub fn set_additional_info(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_additional_info(input);
        self
    }
    /// <p>Reserved for future use.</p>
    pub fn get_additional_info(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_additional_info()
    }
}
