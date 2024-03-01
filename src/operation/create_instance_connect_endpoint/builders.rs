// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_instance_connect_endpoint::_create_instance_connect_endpoint_output::CreateInstanceConnectEndpointOutputBuilder;

pub use crate::operation::create_instance_connect_endpoint::_create_instance_connect_endpoint_input::CreateInstanceConnectEndpointInputBuilder;

impl CreateInstanceConnectEndpointInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_instance_connect_endpoint();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateInstanceConnectEndpoint`.
///
/// <p>Creates an EC2 Instance Connect Endpoint.</p>
/// <p>An EC2 Instance Connect Endpoint allows you to connect to an instance, without requiring the instance to have a public IPv4 address. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Connect-using-EC2-Instance-Connect-Endpoint.html">Connect to your instances without requiring a public IPv4 address using EC2 Instance Connect Endpoint</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateInstanceConnectEndpointFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_instance_connect_endpoint::builders::CreateInstanceConnectEndpointInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput,
        crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointError,
    > for CreateInstanceConnectEndpointFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput,
            crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateInstanceConnectEndpointFluentBuilder {
    /// Creates a new `CreateInstanceConnectEndpoint`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateInstanceConnectEndpoint as a reference.
    pub fn as_input(&self) -> &crate::operation::create_instance_connect_endpoint::builders::CreateInstanceConnectEndpointInputBuilder {
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
        crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpoint::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpoint::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput,
        crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointError,
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
    /// <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p>
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subnet_id(input.into());
        self
    }
    /// <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subnet_id(input);
        self
    }
    /// <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p>
    pub fn get_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subnet_id()
    }
    /// Appends an item to `SecurityGroupIds`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for your VPC will be associated with the endpoint.</p>
    pub fn security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.security_group_ids(input.into());
        self
    }
    /// <p>One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for your VPC will be associated with the endpoint.</p>
    pub fn set_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_security_group_ids(input);
        self
    }
    /// <p>One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for your VPC will be associated with the endpoint.</p>
    pub fn get_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_security_group_ids()
    }
    /// <p>Indicates whether your client's IP address is preserved as the source. The value is <code>true</code> or <code>false</code>.</p>
    /// <ul>
    /// <li> <p>If <code>true</code>, your client's IP address is used when you connect to a resource.</p> </li>
    /// <li> <p>If <code>false</code>, the elastic network interface IP address is used when you connect to a resource.</p> </li>
    /// </ul>
    /// <p>Default: <code>true</code> </p>
    pub fn preserve_client_ip(mut self, input: bool) -> Self {
        self.inner = self.inner.preserve_client_ip(input);
        self
    }
    /// <p>Indicates whether your client's IP address is preserved as the source. The value is <code>true</code> or <code>false</code>.</p>
    /// <ul>
    /// <li> <p>If <code>true</code>, your client's IP address is used when you connect to a resource.</p> </li>
    /// <li> <p>If <code>false</code>, the elastic network interface IP address is used when you connect to a resource.</p> </li>
    /// </ul>
    /// <p>Default: <code>true</code> </p>
    pub fn set_preserve_client_ip(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_preserve_client_ip(input);
        self
    }
    /// <p>Indicates whether your client's IP address is preserved as the source. The value is <code>true</code> or <code>false</code>.</p>
    /// <ul>
    /// <li> <p>If <code>true</code>, your client's IP address is used when you connect to a resource.</p> </li>
    /// <li> <p>If <code>false</code>, the elastic network interface IP address is used when you connect to a resource.</p> </li>
    /// </ul>
    /// <p>Default: <code>true</code> </p>
    pub fn get_preserve_client_ip(&self) -> &::std::option::Option<bool> {
        self.inner.get_preserve_client_ip()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the EC2 Instance Connect Endpoint during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the EC2 Instance Connect Endpoint during creation.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the EC2 Instance Connect Endpoint during creation.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
    }
}