// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetFlowLogsIntegrationTemplateInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the flow log.</p>
    pub flow_log_id: ::std::option::Option<::std::string::String>,
    /// <p>To store the CloudFormation template in Amazon S3, specify the location in Amazon S3.</p>
    pub config_delivery_s3_destination_arn: ::std::option::Option<::std::string::String>,
    /// <p>Information about the service integration.</p>
    pub integrate_services: ::std::option::Option<crate::types::IntegrateServices>,
}
impl GetFlowLogsIntegrationTemplateInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the flow log.</p>
    pub fn flow_log_id(&self) -> ::std::option::Option<&str> {
        self.flow_log_id.as_deref()
    }
    /// <p>To store the CloudFormation template in Amazon S3, specify the location in Amazon S3.</p>
    pub fn config_delivery_s3_destination_arn(&self) -> ::std::option::Option<&str> {
        self.config_delivery_s3_destination_arn.as_deref()
    }
    /// <p>Information about the service integration.</p>
    pub fn integrate_services(&self) -> ::std::option::Option<&crate::types::IntegrateServices> {
        self.integrate_services.as_ref()
    }
}
impl GetFlowLogsIntegrationTemplateInput {
    /// Creates a new builder-style object to manufacture [`GetFlowLogsIntegrationTemplateInput`](crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateInput).
    pub fn builder() -> crate::operation::get_flow_logs_integration_template::builders::GetFlowLogsIntegrationTemplateInputBuilder {
        crate::operation::get_flow_logs_integration_template::builders::GetFlowLogsIntegrationTemplateInputBuilder::default()
    }
}

/// A builder for [`GetFlowLogsIntegrationTemplateInput`](crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetFlowLogsIntegrationTemplateInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) flow_log_id: ::std::option::Option<::std::string::String>,
    pub(crate) config_delivery_s3_destination_arn: ::std::option::Option<::std::string::String>,
    pub(crate) integrate_services: ::std::option::Option<crate::types::IntegrateServices>,
}
impl GetFlowLogsIntegrationTemplateInputBuilder {
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
    /// <p>The ID of the flow log.</p>
    /// This field is required.
    pub fn flow_log_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.flow_log_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the flow log.</p>
    pub fn set_flow_log_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.flow_log_id = input;
        self
    }
    /// <p>The ID of the flow log.</p>
    pub fn get_flow_log_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.flow_log_id
    }
    /// <p>To store the CloudFormation template in Amazon S3, specify the location in Amazon S3.</p>
    /// This field is required.
    pub fn config_delivery_s3_destination_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.config_delivery_s3_destination_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>To store the CloudFormation template in Amazon S3, specify the location in Amazon S3.</p>
    pub fn set_config_delivery_s3_destination_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.config_delivery_s3_destination_arn = input;
        self
    }
    /// <p>To store the CloudFormation template in Amazon S3, specify the location in Amazon S3.</p>
    pub fn get_config_delivery_s3_destination_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.config_delivery_s3_destination_arn
    }
    /// <p>Information about the service integration.</p>
    /// This field is required.
    pub fn integrate_services(mut self, input: crate::types::IntegrateServices) -> Self {
        self.integrate_services = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the service integration.</p>
    pub fn set_integrate_services(mut self, input: ::std::option::Option<crate::types::IntegrateServices>) -> Self {
        self.integrate_services = input;
        self
    }
    /// <p>Information about the service integration.</p>
    pub fn get_integrate_services(&self) -> &::std::option::Option<crate::types::IntegrateServices> {
        &self.integrate_services
    }
    /// Consumes the builder and constructs a [`GetFlowLogsIntegrationTemplateInput`](crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::get_flow_logs_integration_template::GetFlowLogsIntegrationTemplateInput {
                dry_run: self.dry_run,
                flow_log_id: self.flow_log_id,
                config_delivery_s3_destination_arn: self.config_delivery_s3_destination_arn,
                integrate_services: self.integrate_services,
            },
        )
    }
}