// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_identity_id_format::_modify_identity_id_format_output::ModifyIdentityIdFormatOutputBuilder;

pub use crate::operation::modify_identity_id_format::_modify_identity_id_format_input::ModifyIdentityIdFormatInputBuilder;

impl ModifyIdentityIdFormatInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_identity_id_format::ModifyIdentityIdFormatOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_identity_id_format::ModifyIdentityIdFormatError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_identity_id_format();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyIdentityIdFormat`.
///
/// <p>Modifies the ID format of a resource for a specified IAM user, IAM role, or the root user for an account; or all IAM users, IAM roles, and the root user for an account. You can specify that resources should receive longer IDs (17-character IDs) when they are created. </p>
/// <p>This request can only be used to modify longer ID settings for resource types that are within the opt-in period. Resources currently in their opt-in period include: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>. </p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/resource-ids.html">Resource IDs</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>. </p>
/// <p>This setting applies to the principal specified in the request; it does not apply to the principal that makes the request. </p>
/// <p>Resources created with longer IDs are visible to all IAM roles and users, regardless of these settings and provided that they have permission to use the relevant <code>Describe</code> command for the resource type.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyIdentityIdFormatFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_identity_id_format::builders::ModifyIdentityIdFormatInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_identity_id_format::ModifyIdentityIdFormatOutput,
        crate::operation::modify_identity_id_format::ModifyIdentityIdFormatError,
    > for ModifyIdentityIdFormatFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_identity_id_format::ModifyIdentityIdFormatOutput,
            crate::operation::modify_identity_id_format::ModifyIdentityIdFormatError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyIdentityIdFormatFluentBuilder {
    /// Creates a new `ModifyIdentityIdFormat`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyIdentityIdFormat as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_identity_id_format::builders::ModifyIdentityIdFormatInputBuilder {
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
        crate::operation::modify_identity_id_format::ModifyIdentityIdFormatOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_identity_id_format::ModifyIdentityIdFormatError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_identity_id_format::ModifyIdentityIdFormat::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_identity_id_format::ModifyIdentityIdFormat::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_identity_id_format::ModifyIdentityIdFormatOutput,
        crate::operation::modify_identity_id_format::ModifyIdentityIdFormatError,
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
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    pub fn principal_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.principal_arn(input.into());
        self
    }
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    pub fn set_principal_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_principal_arn(input);
        self
    }
    /// <p>The ARN of the principal, which can be an IAM user, IAM role, or the root user. Specify <code>all</code> to modify the ID format for all IAM users, IAM roles, and the root user of the account.</p>
    pub fn get_principal_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_principal_arn()
    }
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    pub fn resource(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource(input.into());
        self
    }
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    pub fn set_resource(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource(input);
        self
    }
    /// <p>The type of resource: <code>bundle</code> | <code>conversion-task</code> | <code>customer-gateway</code> | <code>dhcp-options</code> | <code>elastic-ip-allocation</code> | <code>elastic-ip-association</code> | <code>export-task</code> | <code>flow-log</code> | <code>image</code> | <code>import-task</code> | <code>internet-gateway</code> | <code>network-acl</code> | <code>network-acl-association</code> | <code>network-interface</code> | <code>network-interface-attachment</code> | <code>prefix-list</code> | <code>route-table</code> | <code>route-table-association</code> | <code>security-group</code> | <code>subnet</code> | <code>subnet-cidr-block-association</code> | <code>vpc</code> | <code>vpc-cidr-block-association</code> | <code>vpc-endpoint</code> | <code>vpc-peering-connection</code> | <code>vpn-connection</code> | <code>vpn-gateway</code>.</p>
    /// <p>Alternatively, use the <code>all-current</code> option to include all resource types that are currently within their opt-in period for longer IDs.</p>
    pub fn get_resource(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource()
    }
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    pub fn use_long_ids(mut self, input: bool) -> Self {
        self.inner = self.inner.use_long_ids(input);
        self
    }
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    pub fn set_use_long_ids(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_use_long_ids(input);
        self
    }
    /// <p>Indicates whether the resource should use longer IDs (17-character IDs)</p>
    pub fn get_use_long_ids(&self) -> &::std::option::Option<bool> {
        self.inner.get_use_long_ids()
    }
}