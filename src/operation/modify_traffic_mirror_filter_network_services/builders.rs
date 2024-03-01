// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_traffic_mirror_filter_network_services::_modify_traffic_mirror_filter_network_services_output::ModifyTrafficMirrorFilterNetworkServicesOutputBuilder;

pub use crate::operation::modify_traffic_mirror_filter_network_services::_modify_traffic_mirror_filter_network_services_input::ModifyTrafficMirrorFilterNetworkServicesInputBuilder;

impl ModifyTrafficMirrorFilterNetworkServicesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_traffic_mirror_filter_network_services();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyTrafficMirrorFilterNetworkServices`.
///
/// <p>Allows or restricts mirroring network services.</p>
/// <p> By default, Amazon DNS network services are not eligible for Traffic Mirror. Use <code>AddNetworkServices</code> to add network services to a Traffic Mirror filter. When a network service is added to the Traffic Mirror filter, all traffic related to that network service will be mirrored. When you no longer want to mirror network services, use <code>RemoveNetworkServices</code> to remove the network services from the Traffic Mirror filter. </p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyTrafficMirrorFilterNetworkServicesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput,
        crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError,
    > for ModifyTrafficMirrorFilterNetworkServicesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput,
            crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyTrafficMirrorFilterNetworkServicesFluentBuilder {
    /// Creates a new `ModifyTrafficMirrorFilterNetworkServices`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyTrafficMirrorFilterNetworkServices as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::modify_traffic_mirror_filter_network_services::builders::ModifyTrafficMirrorFilterNetworkServicesInputBuilder {
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
        crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServices::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServices::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesOutput,
        crate::operation::modify_traffic_mirror_filter_network_services::ModifyTrafficMirrorFilterNetworkServicesError,
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
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn traffic_mirror_filter_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.traffic_mirror_filter_id(input.into());
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn set_traffic_mirror_filter_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_traffic_mirror_filter_id(input);
        self
    }
    /// <p>The ID of the Traffic Mirror filter.</p>
    pub fn get_traffic_mirror_filter_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_traffic_mirror_filter_id()
    }
    /// Appends an item to `AddNetworkServices`.
    ///
    /// To override the contents of this collection use [`set_add_network_services`](Self::set_add_network_services).
    ///
    /// <p>The network service, for example Amazon DNS, that you want to mirror.</p>
    pub fn add_network_services(mut self, input: crate::types::TrafficMirrorNetworkService) -> Self {
        self.inner = self.inner.add_network_services(input);
        self
    }
    /// <p>The network service, for example Amazon DNS, that you want to mirror.</p>
    pub fn set_add_network_services(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorNetworkService>>) -> Self {
        self.inner = self.inner.set_add_network_services(input);
        self
    }
    /// <p>The network service, for example Amazon DNS, that you want to mirror.</p>
    pub fn get_add_network_services(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorNetworkService>> {
        self.inner.get_add_network_services()
    }
    /// Appends an item to `RemoveNetworkServices`.
    ///
    /// To override the contents of this collection use [`set_remove_network_services`](Self::set_remove_network_services).
    ///
    /// <p>The network service, for example Amazon DNS, that you no longer want to mirror.</p>
    pub fn remove_network_services(mut self, input: crate::types::TrafficMirrorNetworkService) -> Self {
        self.inner = self.inner.remove_network_services(input);
        self
    }
    /// <p>The network service, for example Amazon DNS, that you no longer want to mirror.</p>
    pub fn set_remove_network_services(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorNetworkService>>) -> Self {
        self.inner = self.inner.set_remove_network_services(input);
        self
    }
    /// <p>The network service, for example Amazon DNS, that you no longer want to mirror.</p>
    pub fn get_remove_network_services(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorNetworkService>> {
        self.inner.get_remove_network_services()
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
