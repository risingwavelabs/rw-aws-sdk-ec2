// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_reserved_instances_listings::_describe_reserved_instances_listings_output::DescribeReservedInstancesListingsOutputBuilder;

pub use crate::operation::describe_reserved_instances_listings::_describe_reserved_instances_listings_input::DescribeReservedInstancesListingsInputBuilder;

impl DescribeReservedInstancesListingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_reserved_instances_listings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeReservedInstancesListings`.
///
/// <p>Describes your account's Reserved Instance listings in the Reserved Instance Marketplace.</p>
/// <p>The Reserved Instance Marketplace matches sellers who want to resell Reserved Instance capacity that they no longer need with buyers who want to purchase additional capacity. Reserved Instances bought and sold through the Reserved Instance Marketplace work like any other Reserved Instances.</p>
/// <p>As a seller, you choose to list some or all of your Reserved Instances, and you specify the upfront price to receive for them. Your Reserved Instances are then listed in the Reserved Instance Marketplace and are available for purchase.</p>
/// <p>As a buyer, you specify the configuration of the Reserved Instance to purchase, and the Marketplace matches what you're searching for with what's available. The Marketplace first sells the lowest priced Reserved Instances to you, and continues to sell available Reserved Instance listings to you until your demand is met. You are charged based on the total price of all of the listings that you purchase.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ri-market-general.html">Reserved Instance Marketplace</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeReservedInstancesListingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsOutput,
        crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsError,
    > for DescribeReservedInstancesListingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsOutput,
            crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeReservedInstancesListingsFluentBuilder {
    /// Creates a new `DescribeReservedInstancesListings`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeReservedInstancesListings as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_reserved_instances_listings::builders::DescribeReservedInstancesListingsInputBuilder {
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
        crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsOutput,
        crate::operation::describe_reserved_instances_listings::DescribeReservedInstancesListingsError,
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
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-listing-id</code> - The ID of the Reserved Instances listing.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instance listing (<code>pending</code> | <code>active</code> | <code>cancelled</code> | <code>closed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-listing-id</code> - The ID of the Reserved Instances listing.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instance listing (<code>pending</code> | <code>active</code> | <code>cancelled</code> | <code>closed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>One or more filters.</p>
    /// <ul>
    /// <li> <p> <code>reserved-instances-id</code> - The ID of the Reserved Instances.</p> </li>
    /// <li> <p> <code>reserved-instances-listing-id</code> - The ID of the Reserved Instances listing.</p> </li>
    /// <li> <p> <code>status</code> - The status of the Reserved Instance listing (<code>pending</code> | <code>active</code> | <code>cancelled</code> | <code>closed</code>).</p> </li>
    /// <li> <p> <code>status-message</code> - The reason for the status.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
    /// <p>One or more Reserved Instance IDs.</p>
    pub fn reserved_instances_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reserved_instances_id(input.into());
        self
    }
    /// <p>One or more Reserved Instance IDs.</p>
    pub fn set_reserved_instances_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_reserved_instances_id(input);
        self
    }
    /// <p>One or more Reserved Instance IDs.</p>
    pub fn get_reserved_instances_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_reserved_instances_id()
    }
    /// <p>One or more Reserved Instance listing IDs.</p>
    pub fn reserved_instances_listing_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.reserved_instances_listing_id(input.into());
        self
    }
    /// <p>One or more Reserved Instance listing IDs.</p>
    pub fn set_reserved_instances_listing_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_reserved_instances_listing_id(input);
        self
    }
    /// <p>One or more Reserved Instance listing IDs.</p>
    pub fn get_reserved_instances_listing_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_reserved_instances_listing_id()
    }
}
