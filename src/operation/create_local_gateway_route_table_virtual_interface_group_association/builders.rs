// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_local_gateway_route_table_virtual_interface_group_association::_create_local_gateway_route_table_virtual_interface_group_association_output::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutputBuilder;

pub use crate::operation::create_local_gateway_route_table_virtual_interface_group_association::_create_local_gateway_route_table_virtual_interface_group_association_input::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder;

impl CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder {
    /// Sends a request with this input using the given client.
                    pub async fn send_with(self, client: &crate::Client) -> ::std::result::Result<
                        crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput,
                        ::aws_smithy_runtime_api::client::result::SdkError<
                            crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError,
                            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse
                        >
    >{
        let mut fluent_builder = client.create_local_gateway_route_table_virtual_interface_group_association();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation`.
///
/// <p> Creates a local gateway route table virtual interface group association. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
                    inner: crate::operation::create_local_gateway_route_table_virtual_interface_group_association::builders::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
                crate::client::customize::internal::CustomizableSend<
                    crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput,
                    crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError,
                > for CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder
            {
                fn send(
                    self,
                    config_override: crate::config::Builder,
                ) -> crate::client::customize::internal::BoxFuture<
                    crate::client::customize::internal::SendResult<
                        crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput,
                        crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError,
                    >,
                > {
                    ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
                }
            }
impl CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationFluentBuilder {
    /// Creates a new `CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation as a reference.
    pub fn as_input(&self) -> &crate::operation::create_local_gateway_route_table_virtual_interface_group_association::builders::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationInputBuilder{
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
    pub async fn send(self) -> ::std::result::Result<crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError, ::aws_smithy_runtime_api::client::orchestrator::HttpResponse>>{
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociation::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
                    pub fn customize(
                        self,
    ) -> crate::client::customize::CustomizableOperation<crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationOutput, crate::operation::create_local_gateway_route_table_virtual_interface_group_association::CreateLocalGatewayRouteTableVirtualInterfaceGroupAssociationError, Self>{
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
    /// <p> The ID of the local gateway route table. </p>
    pub fn local_gateway_route_table_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.local_gateway_route_table_id(input.into());
        self
    }
    /// <p> The ID of the local gateway route table. </p>
    pub fn set_local_gateway_route_table_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_local_gateway_route_table_id(input);
        self
    }
    /// <p> The ID of the local gateway route table. </p>
    pub fn get_local_gateway_route_table_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_local_gateway_route_table_id()
    }
    /// <p> The ID of the local gateway route table virtual interface group association. </p>
    pub fn local_gateway_virtual_interface_group_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.local_gateway_virtual_interface_group_id(input.into());
        self
    }
    /// <p> The ID of the local gateway route table virtual interface group association. </p>
    pub fn set_local_gateway_virtual_interface_group_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_local_gateway_virtual_interface_group_id(input);
        self
    }
    /// <p> The ID of the local gateway route table virtual interface group association. </p>
    pub fn get_local_gateway_virtual_interface_group_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_local_gateway_virtual_interface_group_id()
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p> The tags assigned to the local gateway route table virtual interface group association. </p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p> The tags assigned to the local gateway route table virtual interface group association. </p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p> The tags assigned to the local gateway route table virtual interface group association. </p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
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