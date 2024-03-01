// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_vpn_gateway::_create_vpn_gateway_output::CreateVpnGatewayOutputBuilder;

pub use crate::operation::create_vpn_gateway::_create_vpn_gateway_input::CreateVpnGatewayInputBuilder;

impl CreateVpnGatewayInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_vpn_gateway::CreateVpnGatewayOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_vpn_gateway::CreateVpnGatewayError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_vpn_gateway();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateVpnGateway`.
///
/// <p>Creates a virtual private gateway. A virtual private gateway is the endpoint on the VPC side of your VPN connection. You can create a virtual private gateway before creating the VPC itself.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpn/latest/s2svpn/VPC_VPN.html">Amazon Web Services Site-to-Site VPN</a> in the <i>Amazon Web Services Site-to-Site VPN User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateVpnGatewayFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_vpn_gateway::builders::CreateVpnGatewayInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_vpn_gateway::CreateVpnGatewayOutput,
        crate::operation::create_vpn_gateway::CreateVpnGatewayError,
    > for CreateVpnGatewayFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_vpn_gateway::CreateVpnGatewayOutput,
            crate::operation::create_vpn_gateway::CreateVpnGatewayError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateVpnGatewayFluentBuilder {
    /// Creates a new `CreateVpnGateway`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateVpnGateway as a reference.
    pub fn as_input(&self) -> &crate::operation::create_vpn_gateway::builders::CreateVpnGatewayInputBuilder {
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
        crate::operation::create_vpn_gateway::CreateVpnGatewayOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_vpn_gateway::CreateVpnGatewayError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_vpn_gateway::CreateVpnGateway::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_vpn_gateway::CreateVpnGateway::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_vpn_gateway::CreateVpnGatewayOutput,
        crate::operation::create_vpn_gateway::CreateVpnGatewayError,
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
    /// <p>The Availability Zone for the virtual private gateway.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone for the virtual private gateway.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The Availability Zone for the virtual private gateway.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone()
    }
    /// <p>The type of VPN connection this virtual private gateway supports.</p>
    pub fn r#type(mut self, input: crate::types::GatewayType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The type of VPN connection this virtual private gateway supports.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::GatewayType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The type of VPN connection this virtual private gateway supports.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::GatewayType> {
        self.inner.get_type()
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the virtual private gateway.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the virtual private gateway.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the virtual private gateway.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
    }
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. If you're using a 16-bit ASN, it must be in the 64512 to 65534 range. If you're using a 32-bit ASN, it must be in the 4200000000 to 4294967294 range.</p>
    /// <p>Default: 64512</p>
    pub fn amazon_side_asn(mut self, input: i64) -> Self {
        self.inner = self.inner.amazon_side_asn(input);
        self
    }
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. If you're using a 16-bit ASN, it must be in the 64512 to 65534 range. If you're using a 32-bit ASN, it must be in the 4200000000 to 4294967294 range.</p>
    /// <p>Default: 64512</p>
    pub fn set_amazon_side_asn(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_amazon_side_asn(input);
        self
    }
    /// <p>A private Autonomous System Number (ASN) for the Amazon side of a BGP session. If you're using a 16-bit ASN, it must be in the 64512 to 65534 range. If you're using a 32-bit ASN, it must be in the 4200000000 to 4294967294 range.</p>
    /// <p>Default: 64512</p>
    pub fn get_amazon_side_asn(&self) -> &::std::option::Option<i64> {
        self.inner.get_amazon_side_asn()
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