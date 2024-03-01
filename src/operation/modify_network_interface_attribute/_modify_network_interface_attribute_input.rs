// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for ModifyNetworkInterfaceAttribute.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyNetworkInterfaceAttributeInput {
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub attachment: ::std::option::Option<crate::types::NetworkInterfaceAttachmentChanges>,
    /// <p>A description for the network interface.</p>
    pub description: ::std::option::Option<crate::types::AttributeValue>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The ID of the network interface.</p>
    pub network_interface_id: ::std::option::Option<::std::string::String>,
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub source_dest_check: ::std::option::Option<crate::types::AttributeBooleanValue>,
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub ena_srd_specification: ::std::option::Option<crate::types::EnaSrdSpecification>,
    /// <p>If you’re modifying a network interface in a dual-stack or IPv6-only subnet, you have the option to assign a primary IPv6 IP address. A primary IPv6 address is an IPv6 GUA address associated with an ENI that you have enabled to use a primary IPv6 address. Use this option if the instance that this ENI will be attached to relies on its IPv6 address not changing. Amazon Web Services will automatically assign an IPv6 address associated with the ENI attached to your instance to be the primary IPv6 address. Once you enable an IPv6 GUA address to be a primary IPv6, you cannot disable it. When you enable an IPv6 GUA address to be a primary IPv6, the first IPv6 GUA will be made the primary IPv6 address until the instance is terminated or the network interface is detached. If you have multiple IPv6 addresses associated with an ENI attached to your instance and you enable a primary IPv6 address, the first IPv6 GUA address associated with the ENI becomes the primary IPv6 address.</p>
    pub enable_primary_ipv6: ::std::option::Option<bool>,
    /// <p>A connection tracking specification.</p>
    pub connection_tracking_specification: ::std::option::Option<crate::types::ConnectionTrackingSpecificationRequest>,
}
impl ModifyNetworkInterfaceAttributeInput {
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn attachment(&self) -> ::std::option::Option<&crate::types::NetworkInterfaceAttachmentChanges> {
        self.attachment.as_ref()
    }
    /// <p>A description for the network interface.</p>
    pub fn description(&self) -> ::std::option::Option<&crate::types::AttributeValue> {
        self.description.as_ref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.groups.is_none()`.
    pub fn groups(&self) -> &[::std::string::String] {
        self.groups.as_deref().unwrap_or_default()
    }
    /// <p>The ID of the network interface.</p>
    pub fn network_interface_id(&self) -> ::std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn source_dest_check(&self) -> ::std::option::Option<&crate::types::AttributeBooleanValue> {
        self.source_dest_check.as_ref()
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn ena_srd_specification(&self) -> ::std::option::Option<&crate::types::EnaSrdSpecification> {
        self.ena_srd_specification.as_ref()
    }
    /// <p>If you’re modifying a network interface in a dual-stack or IPv6-only subnet, you have the option to assign a primary IPv6 IP address. A primary IPv6 address is an IPv6 GUA address associated with an ENI that you have enabled to use a primary IPv6 address. Use this option if the instance that this ENI will be attached to relies on its IPv6 address not changing. Amazon Web Services will automatically assign an IPv6 address associated with the ENI attached to your instance to be the primary IPv6 address. Once you enable an IPv6 GUA address to be a primary IPv6, you cannot disable it. When you enable an IPv6 GUA address to be a primary IPv6, the first IPv6 GUA will be made the primary IPv6 address until the instance is terminated or the network interface is detached. If you have multiple IPv6 addresses associated with an ENI attached to your instance and you enable a primary IPv6 address, the first IPv6 GUA address associated with the ENI becomes the primary IPv6 address.</p>
    pub fn enable_primary_ipv6(&self) -> ::std::option::Option<bool> {
        self.enable_primary_ipv6
    }
    /// <p>A connection tracking specification.</p>
    pub fn connection_tracking_specification(&self) -> ::std::option::Option<&crate::types::ConnectionTrackingSpecificationRequest> {
        self.connection_tracking_specification.as_ref()
    }
}
impl ModifyNetworkInterfaceAttributeInput {
    /// Creates a new builder-style object to manufacture [`ModifyNetworkInterfaceAttributeInput`](crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput).
    pub fn builder() -> crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeInputBuilder {
        crate::operation::modify_network_interface_attribute::builders::ModifyNetworkInterfaceAttributeInputBuilder::default()
    }
}

/// A builder for [`ModifyNetworkInterfaceAttributeInput`](crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyNetworkInterfaceAttributeInputBuilder {
    pub(crate) attachment: ::std::option::Option<crate::types::NetworkInterfaceAttachmentChanges>,
    pub(crate) description: ::std::option::Option<crate::types::AttributeValue>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) network_interface_id: ::std::option::Option<::std::string::String>,
    pub(crate) source_dest_check: ::std::option::Option<crate::types::AttributeBooleanValue>,
    pub(crate) ena_srd_specification: ::std::option::Option<crate::types::EnaSrdSpecification>,
    pub(crate) enable_primary_ipv6: ::std::option::Option<bool>,
    pub(crate) connection_tracking_specification: ::std::option::Option<crate::types::ConnectionTrackingSpecificationRequest>,
}
impl ModifyNetworkInterfaceAttributeInputBuilder {
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn attachment(mut self, input: crate::types::NetworkInterfaceAttachmentChanges) -> Self {
        self.attachment = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn set_attachment(mut self, input: ::std::option::Option<crate::types::NetworkInterfaceAttachmentChanges>) -> Self {
        self.attachment = input;
        self
    }
    /// <p>Information about the interface attachment. If modifying the <code>delete on termination</code> attribute, you must specify the ID of the interface attachment.</p>
    pub fn get_attachment(&self) -> &::std::option::Option<crate::types::NetworkInterfaceAttachmentChanges> {
        &self.attachment
    }
    /// <p>A description for the network interface.</p>
    pub fn description(mut self, input: crate::types::AttributeValue) -> Self {
        self.description = ::std::option::Option::Some(input);
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn set_description(mut self, input: ::std::option::Option<crate::types::AttributeValue>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description for the network interface.</p>
    pub fn get_description(&self) -> &::std::option::Option<crate::types::AttributeValue> {
        &self.description
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Appends an item to `groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn groups(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.groups.unwrap_or_default();
        v.push(input.into());
        self.groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn set_groups(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.groups = input;
        self
    }
    /// <p>Changes the security groups for the network interface. The new set of groups you specify replaces the current set. You must specify at least one group, even if it's just the default security group in the VPC. You must specify the ID of the security group, not the name.</p>
    pub fn get_groups(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.groups
    }
    /// <p>The ID of the network interface.</p>
    /// This field is required.
    pub fn network_interface_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_interface_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn set_network_interface_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>The ID of the network interface.</p>
    pub fn get_network_interface_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.network_interface_id
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn source_dest_check(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.source_dest_check = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn set_source_dest_check(mut self, input: ::std::option::Option<crate::types::AttributeBooleanValue>) -> Self {
        self.source_dest_check = input;
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn get_source_dest_check(&self) -> &::std::option::Option<crate::types::AttributeBooleanValue> {
        &self.source_dest_check
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn ena_srd_specification(mut self, input: crate::types::EnaSrdSpecification) -> Self {
        self.ena_srd_specification = ::std::option::Option::Some(input);
        self
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn set_ena_srd_specification(mut self, input: ::std::option::Option<crate::types::EnaSrdSpecification>) -> Self {
        self.ena_srd_specification = input;
        self
    }
    /// <p>Updates the ENA Express configuration for the network interface that’s attached to the instance.</p>
    pub fn get_ena_srd_specification(&self) -> &::std::option::Option<crate::types::EnaSrdSpecification> {
        &self.ena_srd_specification
    }
    /// <p>If you’re modifying a network interface in a dual-stack or IPv6-only subnet, you have the option to assign a primary IPv6 IP address. A primary IPv6 address is an IPv6 GUA address associated with an ENI that you have enabled to use a primary IPv6 address. Use this option if the instance that this ENI will be attached to relies on its IPv6 address not changing. Amazon Web Services will automatically assign an IPv6 address associated with the ENI attached to your instance to be the primary IPv6 address. Once you enable an IPv6 GUA address to be a primary IPv6, you cannot disable it. When you enable an IPv6 GUA address to be a primary IPv6, the first IPv6 GUA will be made the primary IPv6 address until the instance is terminated or the network interface is detached. If you have multiple IPv6 addresses associated with an ENI attached to your instance and you enable a primary IPv6 address, the first IPv6 GUA address associated with the ENI becomes the primary IPv6 address.</p>
    pub fn enable_primary_ipv6(mut self, input: bool) -> Self {
        self.enable_primary_ipv6 = ::std::option::Option::Some(input);
        self
    }
    /// <p>If you’re modifying a network interface in a dual-stack or IPv6-only subnet, you have the option to assign a primary IPv6 IP address. A primary IPv6 address is an IPv6 GUA address associated with an ENI that you have enabled to use a primary IPv6 address. Use this option if the instance that this ENI will be attached to relies on its IPv6 address not changing. Amazon Web Services will automatically assign an IPv6 address associated with the ENI attached to your instance to be the primary IPv6 address. Once you enable an IPv6 GUA address to be a primary IPv6, you cannot disable it. When you enable an IPv6 GUA address to be a primary IPv6, the first IPv6 GUA will be made the primary IPv6 address until the instance is terminated or the network interface is detached. If you have multiple IPv6 addresses associated with an ENI attached to your instance and you enable a primary IPv6 address, the first IPv6 GUA address associated with the ENI becomes the primary IPv6 address.</p>
    pub fn set_enable_primary_ipv6(mut self, input: ::std::option::Option<bool>) -> Self {
        self.enable_primary_ipv6 = input;
        self
    }
    /// <p>If you’re modifying a network interface in a dual-stack or IPv6-only subnet, you have the option to assign a primary IPv6 IP address. A primary IPv6 address is an IPv6 GUA address associated with an ENI that you have enabled to use a primary IPv6 address. Use this option if the instance that this ENI will be attached to relies on its IPv6 address not changing. Amazon Web Services will automatically assign an IPv6 address associated with the ENI attached to your instance to be the primary IPv6 address. Once you enable an IPv6 GUA address to be a primary IPv6, you cannot disable it. When you enable an IPv6 GUA address to be a primary IPv6, the first IPv6 GUA will be made the primary IPv6 address until the instance is terminated or the network interface is detached. If you have multiple IPv6 addresses associated with an ENI attached to your instance and you enable a primary IPv6 address, the first IPv6 GUA address associated with the ENI becomes the primary IPv6 address.</p>
    pub fn get_enable_primary_ipv6(&self) -> &::std::option::Option<bool> {
        &self.enable_primary_ipv6
    }
    /// <p>A connection tracking specification.</p>
    pub fn connection_tracking_specification(mut self, input: crate::types::ConnectionTrackingSpecificationRequest) -> Self {
        self.connection_tracking_specification = ::std::option::Option::Some(input);
        self
    }
    /// <p>A connection tracking specification.</p>
    pub fn set_connection_tracking_specification(
        mut self,
        input: ::std::option::Option<crate::types::ConnectionTrackingSpecificationRequest>,
    ) -> Self {
        self.connection_tracking_specification = input;
        self
    }
    /// <p>A connection tracking specification.</p>
    pub fn get_connection_tracking_specification(&self) -> &::std::option::Option<crate::types::ConnectionTrackingSpecificationRequest> {
        &self.connection_tracking_specification
    }
    /// Consumes the builder and constructs a [`ModifyNetworkInterfaceAttributeInput`](crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::modify_network_interface_attribute::ModifyNetworkInterfaceAttributeInput {
                attachment: self.attachment,
                description: self.description,
                dry_run: self.dry_run,
                groups: self.groups,
                network_interface_id: self.network_interface_id,
                source_dest_check: self.source_dest_check,
                ena_srd_specification: self.ena_srd_specification,
                enable_primary_ipv6: self.enable_primary_ipv6,
                connection_tracking_specification: self.connection_tracking_specification,
            },
        )
    }
}
