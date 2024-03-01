// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_vpc_peering_connection::_create_vpc_peering_connection_output::CreateVpcPeeringConnectionOutputBuilder;

pub use crate::operation::create_vpc_peering_connection::_create_vpc_peering_connection_input::CreateVpcPeeringConnectionInputBuilder;

impl CreateVpcPeeringConnectionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_vpc_peering_connection();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateVpcPeeringConnection`.
///
/// <p>Requests a VPC peering connection between two VPCs: a requester VPC that you own and an accepter VPC with which to create the connection. The accepter VPC can belong to another Amazon Web Services account and can be in a different Region to the requester VPC. The requester VPC and accepter VPC cannot have overlapping CIDR blocks.</p> <note>
/// <p>Limitations and rules apply to a VPC peering connection. For more information, see the <a href="https://docs.aws.amazon.com/vpc/latest/peering/vpc-peering-basics.html#vpc-peering-limitations">limitations</a> section in the <i>VPC Peering Guide</i>.</p>
/// </note>
/// <p>The owner of the accepter VPC must accept the peering request to activate the peering connection. The VPC peering connection request expires after 7 days, after which it cannot be accepted or rejected.</p>
/// <p>If you create a VPC peering connection request between VPCs with overlapping CIDR blocks, the VPC peering connection has a status of <code>failed</code>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateVpcPeeringConnectionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionOutput,
        crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionError,
    > for CreateVpcPeeringConnectionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionOutput,
            crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateVpcPeeringConnectionFluentBuilder {
    /// Creates a new `CreateVpcPeeringConnection`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateVpcPeeringConnection as a reference.
    pub fn as_input(&self) -> &crate::operation::create_vpc_peering_connection::builders::CreateVpcPeeringConnectionInputBuilder {
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
        crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnection::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnection::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionOutput,
        crate::operation::create_vpc_peering_connection::CreateVpcPeeringConnectionError,
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
    /// <p>The Amazon Web Services account ID of the owner of the accepter VPC.</p>
    /// <p>Default: Your Amazon Web Services account ID</p>
    pub fn peer_owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.peer_owner_id(input.into());
        self
    }
    /// <p>The Amazon Web Services account ID of the owner of the accepter VPC.</p>
    /// <p>Default: Your Amazon Web Services account ID</p>
    pub fn set_peer_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_peer_owner_id(input);
        self
    }
    /// <p>The Amazon Web Services account ID of the owner of the accepter VPC.</p>
    /// <p>Default: Your Amazon Web Services account ID</p>
    pub fn get_peer_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_peer_owner_id()
    }
    /// <p>The ID of the VPC with which you are creating the VPC peering connection. You must specify this parameter in the request.</p>
    pub fn peer_vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.peer_vpc_id(input.into());
        self
    }
    /// <p>The ID of the VPC with which you are creating the VPC peering connection. You must specify this parameter in the request.</p>
    pub fn set_peer_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_peer_vpc_id(input);
        self
    }
    /// <p>The ID of the VPC with which you are creating the VPC peering connection. You must specify this parameter in the request.</p>
    pub fn get_peer_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_peer_vpc_id()
    }
    /// <p>The ID of the requester VPC. You must specify this parameter in the request.</p>
    pub fn vpc_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.vpc_id(input.into());
        self
    }
    /// <p>The ID of the requester VPC. You must specify this parameter in the request.</p>
    pub fn set_vpc_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_vpc_id(input);
        self
    }
    /// <p>The ID of the requester VPC. You must specify this parameter in the request.</p>
    pub fn get_vpc_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_vpc_id()
    }
    /// <p>The Region code for the accepter VPC, if the accepter VPC is located in a Region other than the Region in which you make the request.</p>
    /// <p>Default: The Region in which you make the request.</p>
    pub fn peer_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.peer_region(input.into());
        self
    }
    /// <p>The Region code for the accepter VPC, if the accepter VPC is located in a Region other than the Region in which you make the request.</p>
    /// <p>Default: The Region in which you make the request.</p>
    pub fn set_peer_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_peer_region(input);
        self
    }
    /// <p>The Region code for the accepter VPC, if the accepter VPC is located in a Region other than the Region in which you make the request.</p>
    /// <p>Default: The Region in which you make the request.</p>
    pub fn get_peer_region(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_peer_region()
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the peering connection.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to assign to the peering connection.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The tags to assign to the peering connection.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
    }
}
