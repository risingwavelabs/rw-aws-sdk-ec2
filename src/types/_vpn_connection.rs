// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a VPN connection.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct VpnConnection {
    /// <p>The configuration information for the VPN connection's customer gateway (in the native XML format). This element is always present in the <code>CreateVpnConnection</code> response; however, it's present in the <code>DescribeVpnConnections</code> response only if the VPN connection is in the <code>pending</code> or <code>available</code> state.</p>
    pub customer_gateway_configuration: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the customer gateway at your end of the VPN connection.</p>
    pub customer_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The category of the VPN connection. A value of <code>VPN</code> indicates an Amazon Web Services VPN connection. A value of <code>VPN-Classic</code> indicates an Amazon Web Services Classic VPN connection.</p>
    pub category: ::std::option::Option<::std::string::String>,
    /// <p>The current state of the VPN connection.</p>
    pub state: ::std::option::Option<crate::types::VpnState>,
    /// <p>The type of VPN connection.</p>
    pub r#type: ::std::option::Option<crate::types::GatewayType>,
    /// <p>The ID of the VPN connection.</p>
    pub vpn_connection_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the virtual private gateway at the Amazon Web Services side of the VPN connection.</p>
    pub vpn_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub transit_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the core network.</p>
    pub core_network_arn: ::std::option::Option<::std::string::String>,
    /// <p>The ARN of the core network attachment.</p>
    pub core_network_attachment_arn: ::std::option::Option<::std::string::String>,
    /// <p>The current state of the gateway association.</p>
    pub gateway_association_state: ::std::option::Option<crate::types::GatewayAssociationState>,
    /// <p>The VPN connection options.</p>
    pub options: ::std::option::Option<crate::types::VpnConnectionOptions>,
    /// <p>The static routes associated with the VPN connection.</p>
    pub routes: ::std::option::Option<::std::vec::Vec<crate::types::VpnStaticRoute>>,
    /// <p>Any tags assigned to the VPN connection.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>Information about the VPN tunnel.</p>
    pub vgw_telemetry: ::std::option::Option<::std::vec::Vec<crate::types::VgwTelemetry>>,
}
impl VpnConnection {
    /// <p>The configuration information for the VPN connection's customer gateway (in the native XML format). This element is always present in the <code>CreateVpnConnection</code> response; however, it's present in the <code>DescribeVpnConnections</code> response only if the VPN connection is in the <code>pending</code> or <code>available</code> state.</p>
    pub fn customer_gateway_configuration(&self) -> ::std::option::Option<&str> {
        self.customer_gateway_configuration.as_deref()
    }
    /// <p>The ID of the customer gateway at your end of the VPN connection.</p>
    pub fn customer_gateway_id(&self) -> ::std::option::Option<&str> {
        self.customer_gateway_id.as_deref()
    }
    /// <p>The category of the VPN connection. A value of <code>VPN</code> indicates an Amazon Web Services VPN connection. A value of <code>VPN-Classic</code> indicates an Amazon Web Services Classic VPN connection.</p>
    pub fn category(&self) -> ::std::option::Option<&str> {
        self.category.as_deref()
    }
    /// <p>The current state of the VPN connection.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::VpnState> {
        self.state.as_ref()
    }
    /// <p>The type of VPN connection.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::GatewayType> {
        self.r#type.as_ref()
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn vpn_connection_id(&self) -> ::std::option::Option<&str> {
        self.vpn_connection_id.as_deref()
    }
    /// <p>The ID of the virtual private gateway at the Amazon Web Services side of the VPN connection.</p>
    pub fn vpn_gateway_id(&self) -> ::std::option::Option<&str> {
        self.vpn_gateway_id.as_deref()
    }
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub fn transit_gateway_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_id.as_deref()
    }
    /// <p>The ARN of the core network.</p>
    pub fn core_network_arn(&self) -> ::std::option::Option<&str> {
        self.core_network_arn.as_deref()
    }
    /// <p>The ARN of the core network attachment.</p>
    pub fn core_network_attachment_arn(&self) -> ::std::option::Option<&str> {
        self.core_network_attachment_arn.as_deref()
    }
    /// <p>The current state of the gateway association.</p>
    pub fn gateway_association_state(&self) -> ::std::option::Option<&crate::types::GatewayAssociationState> {
        self.gateway_association_state.as_ref()
    }
    /// <p>The VPN connection options.</p>
    pub fn options(&self) -> ::std::option::Option<&crate::types::VpnConnectionOptions> {
        self.options.as_ref()
    }
    /// <p>The static routes associated with the VPN connection.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.routes.is_none()`.
    pub fn routes(&self) -> &[crate::types::VpnStaticRoute] {
        self.routes.as_deref().unwrap_or_default()
    }
    /// <p>Any tags assigned to the VPN connection.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
    /// <p>Information about the VPN tunnel.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.vgw_telemetry.is_none()`.
    pub fn vgw_telemetry(&self) -> &[crate::types::VgwTelemetry] {
        self.vgw_telemetry.as_deref().unwrap_or_default()
    }
}
impl ::std::fmt::Debug for VpnConnection {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("VpnConnection");
        formatter.field("customer_gateway_configuration", &"*** Sensitive Data Redacted ***");
        formatter.field("customer_gateway_id", &self.customer_gateway_id);
        formatter.field("category", &self.category);
        formatter.field("state", &self.state);
        formatter.field("r#type", &self.r#type);
        formatter.field("vpn_connection_id", &self.vpn_connection_id);
        formatter.field("vpn_gateway_id", &self.vpn_gateway_id);
        formatter.field("transit_gateway_id", &self.transit_gateway_id);
        formatter.field("core_network_arn", &self.core_network_arn);
        formatter.field("core_network_attachment_arn", &self.core_network_attachment_arn);
        formatter.field("gateway_association_state", &self.gateway_association_state);
        formatter.field("options", &self.options);
        formatter.field("routes", &self.routes);
        formatter.field("tags", &self.tags);
        formatter.field("vgw_telemetry", &self.vgw_telemetry);
        formatter.finish()
    }
}
impl VpnConnection {
    /// Creates a new builder-style object to manufacture [`VpnConnection`](crate::types::VpnConnection).
    pub fn builder() -> crate::types::builders::VpnConnectionBuilder {
        crate::types::builders::VpnConnectionBuilder::default()
    }
}

/// A builder for [`VpnConnection`](crate::types::VpnConnection).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct VpnConnectionBuilder {
    pub(crate) customer_gateway_configuration: ::std::option::Option<::std::string::String>,
    pub(crate) customer_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) category: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::VpnState>,
    pub(crate) r#type: ::std::option::Option<crate::types::GatewayType>,
    pub(crate) vpn_connection_id: ::std::option::Option<::std::string::String>,
    pub(crate) vpn_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) core_network_arn: ::std::option::Option<::std::string::String>,
    pub(crate) core_network_attachment_arn: ::std::option::Option<::std::string::String>,
    pub(crate) gateway_association_state: ::std::option::Option<crate::types::GatewayAssociationState>,
    pub(crate) options: ::std::option::Option<crate::types::VpnConnectionOptions>,
    pub(crate) routes: ::std::option::Option<::std::vec::Vec<crate::types::VpnStaticRoute>>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) vgw_telemetry: ::std::option::Option<::std::vec::Vec<crate::types::VgwTelemetry>>,
}
impl VpnConnectionBuilder {
    /// <p>The configuration information for the VPN connection's customer gateway (in the native XML format). This element is always present in the <code>CreateVpnConnection</code> response; however, it's present in the <code>DescribeVpnConnections</code> response only if the VPN connection is in the <code>pending</code> or <code>available</code> state.</p>
    pub fn customer_gateway_configuration(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.customer_gateway_configuration = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The configuration information for the VPN connection's customer gateway (in the native XML format). This element is always present in the <code>CreateVpnConnection</code> response; however, it's present in the <code>DescribeVpnConnections</code> response only if the VPN connection is in the <code>pending</code> or <code>available</code> state.</p>
    pub fn set_customer_gateway_configuration(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.customer_gateway_configuration = input;
        self
    }
    /// <p>The configuration information for the VPN connection's customer gateway (in the native XML format). This element is always present in the <code>CreateVpnConnection</code> response; however, it's present in the <code>DescribeVpnConnections</code> response only if the VPN connection is in the <code>pending</code> or <code>available</code> state.</p>
    pub fn get_customer_gateway_configuration(&self) -> &::std::option::Option<::std::string::String> {
        &self.customer_gateway_configuration
    }
    /// <p>The ID of the customer gateway at your end of the VPN connection.</p>
    pub fn customer_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.customer_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the customer gateway at your end of the VPN connection.</p>
    pub fn set_customer_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.customer_gateway_id = input;
        self
    }
    /// <p>The ID of the customer gateway at your end of the VPN connection.</p>
    pub fn get_customer_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.customer_gateway_id
    }
    /// <p>The category of the VPN connection. A value of <code>VPN</code> indicates an Amazon Web Services VPN connection. A value of <code>VPN-Classic</code> indicates an Amazon Web Services Classic VPN connection.</p>
    pub fn category(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.category = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The category of the VPN connection. A value of <code>VPN</code> indicates an Amazon Web Services VPN connection. A value of <code>VPN-Classic</code> indicates an Amazon Web Services Classic VPN connection.</p>
    pub fn set_category(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.category = input;
        self
    }
    /// <p>The category of the VPN connection. A value of <code>VPN</code> indicates an Amazon Web Services VPN connection. A value of <code>VPN-Classic</code> indicates an Amazon Web Services Classic VPN connection.</p>
    pub fn get_category(&self) -> &::std::option::Option<::std::string::String> {
        &self.category
    }
    /// <p>The current state of the VPN connection.</p>
    pub fn state(mut self, input: crate::types::VpnState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state of the VPN connection.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::VpnState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The current state of the VPN connection.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::VpnState> {
        &self.state
    }
    /// <p>The type of VPN connection.</p>
    pub fn r#type(mut self, input: crate::types::GatewayType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of VPN connection.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::GatewayType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The type of VPN connection.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::GatewayType> {
        &self.r#type
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn vpn_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpn_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn set_vpn_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpn_connection_id = input;
        self
    }
    /// <p>The ID of the VPN connection.</p>
    pub fn get_vpn_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpn_connection_id
    }
    /// <p>The ID of the virtual private gateway at the Amazon Web Services side of the VPN connection.</p>
    pub fn vpn_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vpn_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the virtual private gateway at the Amazon Web Services side of the VPN connection.</p>
    pub fn set_vpn_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vpn_gateway_id = input;
        self
    }
    /// <p>The ID of the virtual private gateway at the Amazon Web Services side of the VPN connection.</p>
    pub fn get_vpn_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.vpn_gateway_id
    }
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub fn transit_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub fn set_transit_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_id = input;
        self
    }
    /// <p>The ID of the transit gateway associated with the VPN connection.</p>
    pub fn get_transit_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_id
    }
    /// <p>The ARN of the core network.</p>
    pub fn core_network_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.core_network_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the core network.</p>
    pub fn set_core_network_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.core_network_arn = input;
        self
    }
    /// <p>The ARN of the core network.</p>
    pub fn get_core_network_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.core_network_arn
    }
    /// <p>The ARN of the core network attachment.</p>
    pub fn core_network_attachment_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.core_network_attachment_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the core network attachment.</p>
    pub fn set_core_network_attachment_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.core_network_attachment_arn = input;
        self
    }
    /// <p>The ARN of the core network attachment.</p>
    pub fn get_core_network_attachment_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.core_network_attachment_arn
    }
    /// <p>The current state of the gateway association.</p>
    pub fn gateway_association_state(mut self, input: crate::types::GatewayAssociationState) -> Self {
        self.gateway_association_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state of the gateway association.</p>
    pub fn set_gateway_association_state(mut self, input: ::std::option::Option<crate::types::GatewayAssociationState>) -> Self {
        self.gateway_association_state = input;
        self
    }
    /// <p>The current state of the gateway association.</p>
    pub fn get_gateway_association_state(&self) -> &::std::option::Option<crate::types::GatewayAssociationState> {
        &self.gateway_association_state
    }
    /// <p>The VPN connection options.</p>
    pub fn options(mut self, input: crate::types::VpnConnectionOptions) -> Self {
        self.options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The VPN connection options.</p>
    pub fn set_options(mut self, input: ::std::option::Option<crate::types::VpnConnectionOptions>) -> Self {
        self.options = input;
        self
    }
    /// <p>The VPN connection options.</p>
    pub fn get_options(&self) -> &::std::option::Option<crate::types::VpnConnectionOptions> {
        &self.options
    }
    /// Appends an item to `routes`.
    ///
    /// To override the contents of this collection use [`set_routes`](Self::set_routes).
    ///
    /// <p>The static routes associated with the VPN connection.</p>
    pub fn routes(mut self, input: crate::types::VpnStaticRoute) -> Self {
        let mut v = self.routes.unwrap_or_default();
        v.push(input);
        self.routes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The static routes associated with the VPN connection.</p>
    pub fn set_routes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::VpnStaticRoute>>) -> Self {
        self.routes = input;
        self
    }
    /// <p>The static routes associated with the VPN connection.</p>
    pub fn get_routes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::VpnStaticRoute>> {
        &self.routes
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags assigned to the VPN connection.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Any tags assigned to the VPN connection.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Any tags assigned to the VPN connection.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Appends an item to `vgw_telemetry`.
    ///
    /// To override the contents of this collection use [`set_vgw_telemetry`](Self::set_vgw_telemetry).
    ///
    /// <p>Information about the VPN tunnel.</p>
    pub fn vgw_telemetry(mut self, input: crate::types::VgwTelemetry) -> Self {
        let mut v = self.vgw_telemetry.unwrap_or_default();
        v.push(input);
        self.vgw_telemetry = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the VPN tunnel.</p>
    pub fn set_vgw_telemetry(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::VgwTelemetry>>) -> Self {
        self.vgw_telemetry = input;
        self
    }
    /// <p>Information about the VPN tunnel.</p>
    pub fn get_vgw_telemetry(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::VgwTelemetry>> {
        &self.vgw_telemetry
    }
    /// Consumes the builder and constructs a [`VpnConnection`](crate::types::VpnConnection).
    pub fn build(self) -> crate::types::VpnConnection {
        crate::types::VpnConnection {
            customer_gateway_configuration: self.customer_gateway_configuration,
            customer_gateway_id: self.customer_gateway_id,
            category: self.category,
            state: self.state,
            r#type: self.r#type,
            vpn_connection_id: self.vpn_connection_id,
            vpn_gateway_id: self.vpn_gateway_id,
            transit_gateway_id: self.transit_gateway_id,
            core_network_arn: self.core_network_arn,
            core_network_attachment_arn: self.core_network_attachment_arn,
            gateway_association_state: self.gateway_association_state,
            options: self.options,
            routes: self.routes,
            tags: self.tags,
            vgw_telemetry: self.vgw_telemetry,
        }
    }
}
impl ::std::fmt::Debug for VpnConnectionBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("VpnConnectionBuilder");
        formatter.field("customer_gateway_configuration", &"*** Sensitive Data Redacted ***");
        formatter.field("customer_gateway_id", &self.customer_gateway_id);
        formatter.field("category", &self.category);
        formatter.field("state", &self.state);
        formatter.field("r#type", &self.r#type);
        formatter.field("vpn_connection_id", &self.vpn_connection_id);
        formatter.field("vpn_gateway_id", &self.vpn_gateway_id);
        formatter.field("transit_gateway_id", &self.transit_gateway_id);
        formatter.field("core_network_arn", &self.core_network_arn);
        formatter.field("core_network_attachment_arn", &self.core_network_attachment_arn);
        formatter.field("gateway_association_state", &self.gateway_association_state);
        formatter.field("options", &self.options);
        formatter.field("routes", &self.routes);
        formatter.field("tags", &self.tags);
        formatter.field("vgw_telemetry", &self.vgw_telemetry);
        formatter.finish()
    }
}
