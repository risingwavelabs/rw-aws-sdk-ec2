// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::allocate_ipam_pool_cidr::_allocate_ipam_pool_cidr_output::AllocateIpamPoolCidrOutputBuilder;

pub use crate::operation::allocate_ipam_pool_cidr::_allocate_ipam_pool_cidr_input::AllocateIpamPoolCidrInputBuilder;

impl AllocateIpamPoolCidrInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.allocate_ipam_pool_cidr();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AllocateIpamPoolCidr`.
///
/// <p>Allocate a CIDR from an IPAM pool. The Region you use should be the IPAM pool locale. The locale is the Amazon Web Services Region where this IPAM pool is available for allocations.</p>
/// <p>In IPAM, an allocation is a CIDR assignment from an IPAM pool to another IPAM pool or to a resource. For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/ipam/allocate-cidrs-ipam.html">Allocate CIDRs</a> in the <i>Amazon VPC IPAM User Guide</i>.</p> <note>
/// <p>This action creates an allocation with strong consistency. The returned CIDR will not overlap with any other allocations from the same pool.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AllocateIpamPoolCidrFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::allocate_ipam_pool_cidr::builders::AllocateIpamPoolCidrInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrOutput,
        crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrError,
    > for AllocateIpamPoolCidrFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrOutput,
            crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AllocateIpamPoolCidrFluentBuilder {
    /// Creates a new `AllocateIpamPoolCidr`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AllocateIpamPoolCidr as a reference.
    pub fn as_input(&self) -> &crate::operation::allocate_ipam_pool_cidr::builders::AllocateIpamPoolCidrInputBuilder {
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
        crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidr::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidr::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrOutput,
        crate::operation::allocate_ipam_pool_cidr::AllocateIpamPoolCidrError,
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
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>A check for whether you have the required permissions for the action without actually making the request and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>The ID of the IPAM pool from which you would like to allocate a CIDR.</p>
    pub fn ipam_pool_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ipam_pool_id(input.into());
        self
    }
    /// <p>The ID of the IPAM pool from which you would like to allocate a CIDR.</p>
    pub fn set_ipam_pool_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ipam_pool_id(input);
        self
    }
    /// <p>The ID of the IPAM pool from which you would like to allocate a CIDR.</p>
    pub fn get_ipam_pool_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ipam_pool_id()
    }
    /// <p>The CIDR you would like to allocate from the IPAM pool. Note the following:</p>
    /// <ul>
    /// <li> <p>If there is no DefaultNetmaskLength allocation rule set on the pool, you must specify either the NetmaskLength or the CIDR.</p> </li>
    /// <li> <p>If the DefaultNetmaskLength allocation rule is set on the pool, you can specify either the NetmaskLength or the CIDR and the DefaultNetmaskLength allocation rule will be ignored.</p> </li>
    /// </ul>
    /// <p>Possible values: Any available IPv4 or IPv6 CIDR.</p>
    pub fn cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.cidr(input.into());
        self
    }
    /// <p>The CIDR you would like to allocate from the IPAM pool. Note the following:</p>
    /// <ul>
    /// <li> <p>If there is no DefaultNetmaskLength allocation rule set on the pool, you must specify either the NetmaskLength or the CIDR.</p> </li>
    /// <li> <p>If the DefaultNetmaskLength allocation rule is set on the pool, you can specify either the NetmaskLength or the CIDR and the DefaultNetmaskLength allocation rule will be ignored.</p> </li>
    /// </ul>
    /// <p>Possible values: Any available IPv4 or IPv6 CIDR.</p>
    pub fn set_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_cidr(input);
        self
    }
    /// <p>The CIDR you would like to allocate from the IPAM pool. Note the following:</p>
    /// <ul>
    /// <li> <p>If there is no DefaultNetmaskLength allocation rule set on the pool, you must specify either the NetmaskLength or the CIDR.</p> </li>
    /// <li> <p>If the DefaultNetmaskLength allocation rule is set on the pool, you can specify either the NetmaskLength or the CIDR and the DefaultNetmaskLength allocation rule will be ignored.</p> </li>
    /// </ul>
    /// <p>Possible values: Any available IPv4 or IPv6 CIDR.</p>
    pub fn get_cidr(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_cidr()
    }
    /// <p>The netmask length of the CIDR you would like to allocate from the IPAM pool. Note the following:</p>
    /// <ul>
    /// <li> <p>If there is no DefaultNetmaskLength allocation rule set on the pool, you must specify either the NetmaskLength or the CIDR.</p> </li>
    /// <li> <p>If the DefaultNetmaskLength allocation rule is set on the pool, you can specify either the NetmaskLength or the CIDR and the DefaultNetmaskLength allocation rule will be ignored.</p> </li>
    /// </ul>
    /// <p>Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128.</p>
    pub fn netmask_length(mut self, input: i32) -> Self {
        self.inner = self.inner.netmask_length(input);
        self
    }
    /// <p>The netmask length of the CIDR you would like to allocate from the IPAM pool. Note the following:</p>
    /// <ul>
    /// <li> <p>If there is no DefaultNetmaskLength allocation rule set on the pool, you must specify either the NetmaskLength or the CIDR.</p> </li>
    /// <li> <p>If the DefaultNetmaskLength allocation rule is set on the pool, you can specify either the NetmaskLength or the CIDR and the DefaultNetmaskLength allocation rule will be ignored.</p> </li>
    /// </ul>
    /// <p>Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128.</p>
    pub fn set_netmask_length(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_netmask_length(input);
        self
    }
    /// <p>The netmask length of the CIDR you would like to allocate from the IPAM pool. Note the following:</p>
    /// <ul>
    /// <li> <p>If there is no DefaultNetmaskLength allocation rule set on the pool, you must specify either the NetmaskLength or the CIDR.</p> </li>
    /// <li> <p>If the DefaultNetmaskLength allocation rule is set on the pool, you can specify either the NetmaskLength or the CIDR and the DefaultNetmaskLength allocation rule will be ignored.</p> </li>
    /// </ul>
    /// <p>Possible netmask lengths for IPv4 addresses are 0 - 32. Possible netmask lengths for IPv6 addresses are 0 - 128.</p>
    pub fn get_netmask_length(&self) -> &::std::option::Option<i32> {
        self.inner.get_netmask_length()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>A description for the allocation.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description for the allocation.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description for the allocation.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>A preview of the next available CIDR in a pool.</p>
    pub fn preview_next_cidr(mut self, input: bool) -> Self {
        self.inner = self.inner.preview_next_cidr(input);
        self
    }
    /// <p>A preview of the next available CIDR in a pool.</p>
    pub fn set_preview_next_cidr(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_preview_next_cidr(input);
        self
    }
    /// <p>A preview of the next available CIDR in a pool.</p>
    pub fn get_preview_next_cidr(&self) -> &::std::option::Option<bool> {
        self.inner.get_preview_next_cidr()
    }
    /// Appends an item to `AllowedCidrs`.
    ///
    /// To override the contents of this collection use [`set_allowed_cidrs`](Self::set_allowed_cidrs).
    ///
    /// <p>Include a particular CIDR range that can be returned by the pool. Allowed CIDRs are only allowed if using netmask length for allocation.</p>
    pub fn allowed_cidrs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.allowed_cidrs(input.into());
        self
    }
    /// <p>Include a particular CIDR range that can be returned by the pool. Allowed CIDRs are only allowed if using netmask length for allocation.</p>
    pub fn set_allowed_cidrs(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_allowed_cidrs(input);
        self
    }
    /// <p>Include a particular CIDR range that can be returned by the pool. Allowed CIDRs are only allowed if using netmask length for allocation.</p>
    pub fn get_allowed_cidrs(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_allowed_cidrs()
    }
    /// Appends an item to `DisallowedCidrs`.
    ///
    /// To override the contents of this collection use [`set_disallowed_cidrs`](Self::set_disallowed_cidrs).
    ///
    /// <p>Exclude a particular CIDR range from being returned by the pool. Disallowed CIDRs are only allowed if using netmask length for allocation.</p>
    pub fn disallowed_cidrs(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.disallowed_cidrs(input.into());
        self
    }
    /// <p>Exclude a particular CIDR range from being returned by the pool. Disallowed CIDRs are only allowed if using netmask length for allocation.</p>
    pub fn set_disallowed_cidrs(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_disallowed_cidrs(input);
        self
    }
    /// <p>Exclude a particular CIDR range from being returned by the pool. Disallowed CIDRs are only allowed if using netmask length for allocation.</p>
    pub fn get_disallowed_cidrs(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_disallowed_cidrs()
    }
}
