// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_network_interface_attribute::_modify_network_interface_attribute_output::ModifyNetworkInterfaceAttributeOutputBuilder;

pub use crate::operation::modify_network_interface_attribute::_modify_network_interface_attribute_input::ModifyNetworkInterfaceAttributeInputBuilder;

impl ModifyNetworkInterfaceAttributeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_network_interface_attribute();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyNetworkInterfaceAttribute`.
///
/// <p>Modifies the specified network interface attribute. You can specify only one attribute at a time. You can use this action to attach and detach security groups from an existing EC2 instance.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyNetworkInterfaceAttributeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeOutput,
        crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError,
    > for ModifyNetworkInterfaceAttributeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeOutput,
            crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyNetworkInterfaceAttributeFluentBuilder {
    /// Creates a new `ModifyNetworkInterfaceAttribute`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyNetworkInterfaceAttribute as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeInputBuilder {
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
        crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttribute::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttribute::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeOutput,
        crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeError,
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
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn attachment(mut self, input: crate::types::NetworkInterfaceAttachmentChanges) -> Self {
        self.inner = self.inner.attachment(input);
        self
    }
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn set_attachment(mut self, input: ::std::option::Option<crate::types::NetworkInterfaceAttachmentChanges>) -> Self {
        self.inner = self.inner.set_attachment(input);
        self
    }
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn get_attachment(&self) -> &::std::option::Option<crate::types::NetworkInterfaceAttachmentChanges> {
        self.inner.get_attachment()
    }
    /// <p>A description for the network interface.</p>
    pub fn description(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.description(input);
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn set_description(mut self, input: ::std::option::Option<crate::types::AttributeValue>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn get_description(&self) -> &::std::option::Option<crate::types::AttributeValue> {
        self.inner.get_description()
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
    /// Appends an item to `Groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn groups(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.groups(input.into());
        self
    }
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn set_groups(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_groups(input);
        self
    }
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn get_groups(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_groups()
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.network_interface_id(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_network_interface_id(input);
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn get_network_interface_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_network_interface_id()
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn source_dest_check(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.inner = self.inner.source_dest_check(input);
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn set_source_dest_check(mut self, input: ::std::option::Option<crate::types::AttributeBooleanValue>) -> Self {
        self.inner = self.inner.set_source_dest_check(input);
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn get_source_dest_check(&self) -> &::std::option::Option<crate::types::AttributeBooleanValue> {
        self.inner.get_source_dest_check()
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn ena_srd_specification(mut self, input: crate::types::EnaSrdSpecification) -> Self {
        self.inner = self.inner.ena_srd_specification(input);
        self
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn set_ena_srd_specification(mut self, input: ::std::option::Option<crate::types::EnaSrdSpecification>) -> Self {
        self.inner = self.inner.set_ena_srd_specification(input);
        self
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn get_ena_srd_specification(&self) -> &::std::option::Option<crate::types::EnaSrdSpecification> {
        self.inner.get_ena_srd_specification()
    }
    /// <p>If you’re modifying a network interface in a dual-stack or IPv6-only subnet, you have the option to assign a primary IPv6 IP address. A primary IPv6 address is an IPv6 GUA address associated with an ENI that you have enabled to use a primary IPv6 address. Use this option if the instance that this ENI will be attached to relies on its IPv6 address not changing. Amazon Web Services will automatically assign an IPv6 address associated with the ENI attached to your instance to be the primary IPv6 address. Once you enable an IPv6 GUA address to be a primary IPv6, you cannot disable it. When you enable an IPv6 GUA address to be a primary IPv6, the first IPv6 GUA will be made the primary IPv6 address until the instance is terminated or the network interface is detached. If you have multiple IPv6 addresses associated with an ENI attached to your instance and you enable a primary IPv6 address, the first IPv6 GUA address associated with the ENI becomes the primary IPv6 address.</p>
    pub fn enable_primary_ipv6(mut self, input: bool) -> Self {
        self.inner = self.inner.enable_primary_ipv6(input);
        self
    }
    /// <p>If you’re modifying a network interface in a dual-stack or IPv6-only subnet, you have the option to assign a primary IPv6 IP address. A primary IPv6 address is an IPv6 GUA address associated with an ENI that you have enabled to use a primary IPv6 address. Use this option if the instance that this ENI will be attached to relies on its IPv6 address not changing. Amazon Web Services will automatically assign an IPv6 address associated with the ENI attached to your instance to be the primary IPv6 address. Once you enable an IPv6 GUA address to be a primary IPv6, you cannot disable it. When you enable an IPv6 GUA address to be a primary IPv6, the first IPv6 GUA will be made the primary IPv6 address until the instance is terminated or the network interface is detached. If you have multiple IPv6 addresses associated with an ENI attached to your instance and you enable a primary IPv6 address, the first IPv6 GUA address associated with the ENI becomes the primary IPv6 address.</p>
    pub fn set_enable_primary_ipv6(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_enable_primary_ipv6(input);
        self
    }
    /// <p>If you’re modifying a network interface in a dual-stack or IPv6-only subnet, you have the option to assign a primary IPv6 IP address. A primary IPv6 address is an IPv6 GUA address associated with an ENI that you have enabled to use a primary IPv6 address. Use this option if the instance that this ENI will be attached to relies on its IPv6 address not changing. Amazon Web Services will automatically assign an IPv6 address associated with the ENI attached to your instance to be the primary IPv6 address. Once you enable an IPv6 GUA address to be a primary IPv6, you cannot disable it. When you enable an IPv6 GUA address to be a primary IPv6, the first IPv6 GUA will be made the primary IPv6 address until the instance is terminated or the network interface is detached. If you have multiple IPv6 addresses associated with an ENI attached to your instance and you enable a primary IPv6 address, the first IPv6 GUA address associated with the ENI becomes the primary IPv6 address.</p>
    pub fn get_enable_primary_ipv6(&self) -> &::std::option::Option<bool> {
        self.inner.get_enable_primary_ipv6()
    }
    /// <p>A connection tracking specification.</p>
    pub fn connection_tracking_specification(mut self, input: crate::types::ConnectionTrackingSpecificationRequest) -> Self {
        self.inner = self.inner.connection_tracking_specification(input);
        self
    }
    /// <p>A connection tracking specification.</p>
    pub fn set_connection_tracking_specification(
        mut self,
        input: ::std::option::Option<crate::types::ConnectionTrackingSpecificationRequest>,
    ) -> Self {
        self.inner = self.inner.set_connection_tracking_specification(input);
        self
    }
    /// <p>A connection tracking specification.</p>
    pub fn get_connection_tracking_specification(&self) -> &::std::option::Option<crate::types::ConnectionTrackingSpecificationRequest> {
        self.inner.get_connection_tracking_specification()
    }
}
