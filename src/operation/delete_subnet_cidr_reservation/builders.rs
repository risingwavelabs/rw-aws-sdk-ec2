// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_subnet_cidr_reservation::_delete_subnet_cidr_reservation_output::DeleteSubnetCidrReservationOutputBuilder;

pub use crate::operation::delete_subnet_cidr_reservation::_delete_subnet_cidr_reservation_input::DeleteSubnetCidrReservationInputBuilder;

impl DeleteSubnetCidrReservationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_subnet_cidr_reservation();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteSubnetCidrReservation`.
///
/// <p>Deletes a subnet CIDR reservation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteSubnetCidrReservationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_subnet_cidr_reservation::builders::DeleteSubnetCidrReservationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput,
        crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationError,
    > for DeleteSubnetCidrReservationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput,
            crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteSubnetCidrReservationFluentBuilder {
    /// Creates a new `DeleteSubnetCidrReservation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteSubnetCidrReservation as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_subnet_cidr_reservation::builders::DeleteSubnetCidrReservationInputBuilder {
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
        crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservation::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationOutput,
        crate::operation::delete_subnet_cidr_reservation::DeleteSubnetCidrReservationError,
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
    /// <p>The ID of the subnet CIDR reservation.</p>
    pub fn subnet_cidr_reservation_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subnet_cidr_reservation_id(input.into());
        self
    }
    /// <p>The ID of the subnet CIDR reservation.</p>
    pub fn set_subnet_cidr_reservation_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subnet_cidr_reservation_id(input);
        self
    }
    /// <p>The ID of the subnet CIDR reservation.</p>
    pub fn get_subnet_cidr_reservation_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subnet_cidr_reservation_id()
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
}
