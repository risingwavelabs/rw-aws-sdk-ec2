// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateNetworkInsightsPathInput {
    /// <p>The IP address of the source.</p>
    pub source_ip: ::std::option::Option<::std::string::String>,
    /// <p>The IP address of the destination.</p>
    pub destination_ip: ::std::option::Option<::std::string::String>,
    /// <p>The ID or ARN of the source. If the resource is in another account, you must specify an ARN.</p>
    pub source: ::std::option::Option<::std::string::String>,
    /// <p>The ID or ARN of the destination. If the resource is in another account, you must specify an ARN.</p>
    pub destination: ::std::option::Option<::std::string::String>,
    /// <p>The protocol.</p>
    pub protocol: ::std::option::Option<crate::types::Protocol>,
    /// <p>The destination port.</p>
    pub destination_port: ::std::option::Option<i32>,
    /// <p>The tags to add to the path.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>Scopes the analysis to network paths that match specific filters at the source. If you specify this parameter, you can't specify the parameters for the source IP address or the destination port.</p>
    pub filter_at_source: ::std::option::Option<crate::types::PathRequestFilter>,
    /// <p>Scopes the analysis to network paths that match specific filters at the destination. If you specify this parameter, you can't specify the parameter for the destination IP address.</p>
    pub filter_at_destination: ::std::option::Option<crate::types::PathRequestFilter>,
}
impl CreateNetworkInsightsPathInput {
    /// <p>The IP address of the source.</p>
    pub fn source_ip(&self) -> ::std::option::Option<&str> {
        self.source_ip.as_deref()
    }
    /// <p>The IP address of the destination.</p>
    pub fn destination_ip(&self) -> ::std::option::Option<&str> {
        self.destination_ip.as_deref()
    }
    /// <p>The ID or ARN of the source. If the resource is in another account, you must specify an ARN.</p>
    pub fn source(&self) -> ::std::option::Option<&str> {
        self.source.as_deref()
    }
    /// <p>The ID or ARN of the destination. If the resource is in another account, you must specify an ARN.</p>
    pub fn destination(&self) -> ::std::option::Option<&str> {
        self.destination.as_deref()
    }
    /// <p>The protocol.</p>
    pub fn protocol(&self) -> ::std::option::Option<&crate::types::Protocol> {
        self.protocol.as_ref()
    }
    /// <p>The destination port.</p>
    pub fn destination_port(&self) -> ::std::option::Option<i32> {
        self.destination_port
    }
    /// <p>The tags to add to the path.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the source. If you specify this parameter, you can't specify the parameters for the source IP address or the destination port.</p>
    pub fn filter_at_source(&self) -> ::std::option::Option<&crate::types::PathRequestFilter> {
        self.filter_at_source.as_ref()
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the destination. If you specify this parameter, you can't specify the parameter for the destination IP address.</p>
    pub fn filter_at_destination(&self) -> ::std::option::Option<&crate::types::PathRequestFilter> {
        self.filter_at_destination.as_ref()
    }
}
impl CreateNetworkInsightsPathInput {
    /// Creates a new builder-style object to manufacture [`CreateNetworkInsightsPathInput`](crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput).
    pub fn builder() -> crate::operation::create_network_insights_path::builders::CreateNetworkInsightsPathInputBuilder {
        crate::operation::create_network_insights_path::builders::CreateNetworkInsightsPathInputBuilder::default()
    }
}

/// A builder for [`CreateNetworkInsightsPathInput`](crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateNetworkInsightsPathInputBuilder {
    pub(crate) source_ip: ::std::option::Option<::std::string::String>,
    pub(crate) destination_ip: ::std::option::Option<::std::string::String>,
    pub(crate) source: ::std::option::Option<::std::string::String>,
    pub(crate) destination: ::std::option::Option<::std::string::String>,
    pub(crate) protocol: ::std::option::Option<crate::types::Protocol>,
    pub(crate) destination_port: ::std::option::Option<i32>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) filter_at_source: ::std::option::Option<crate::types::PathRequestFilter>,
    pub(crate) filter_at_destination: ::std::option::Option<crate::types::PathRequestFilter>,
}
impl CreateNetworkInsightsPathInputBuilder {
    /// <p>The IP address of the source.</p>
    pub fn source_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address of the source.</p>
    pub fn set_source_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_ip = input;
        self
    }
    /// <p>The IP address of the source.</p>
    pub fn get_source_ip(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_ip
    }
    /// <p>The IP address of the destination.</p>
    pub fn destination_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address of the destination.</p>
    pub fn set_destination_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination_ip = input;
        self
    }
    /// <p>The IP address of the destination.</p>
    pub fn get_destination_ip(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination_ip
    }
    /// <p>The ID or ARN of the source. If the resource is in another account, you must specify an ARN.</p>
    /// This field is required.
    pub fn source(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID or ARN of the source. If the resource is in another account, you must specify an ARN.</p>
    pub fn set_source(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source = input;
        self
    }
    /// <p>The ID or ARN of the source. If the resource is in another account, you must specify an ARN.</p>
    pub fn get_source(&self) -> &::std::option::Option<::std::string::String> {
        &self.source
    }
    /// <p>The ID or ARN of the destination. If the resource is in another account, you must specify an ARN.</p>
    pub fn destination(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID or ARN of the destination. If the resource is in another account, you must specify an ARN.</p>
    pub fn set_destination(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination = input;
        self
    }
    /// <p>The ID or ARN of the destination. If the resource is in another account, you must specify an ARN.</p>
    pub fn get_destination(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination
    }
    /// <p>The protocol.</p>
    /// This field is required.
    pub fn protocol(mut self, input: crate::types::Protocol) -> Self {
        self.protocol = ::std::option::Option::Some(input);
        self
    }
    /// <p>The protocol.</p>
    pub fn set_protocol(mut self, input: ::std::option::Option<crate::types::Protocol>) -> Self {
        self.protocol = input;
        self
    }
    /// <p>The protocol.</p>
    pub fn get_protocol(&self) -> &::std::option::Option<crate::types::Protocol> {
        &self.protocol
    }
    /// <p>The destination port.</p>
    pub fn destination_port(mut self, input: i32) -> Self {
        self.destination_port = ::std::option::Option::Some(input);
        self
    }
    /// <p>The destination port.</p>
    pub fn set_destination_port(mut self, input: ::std::option::Option<i32>) -> Self {
        self.destination_port = input;
        self
    }
    /// <p>The destination port.</p>
    pub fn get_destination_port(&self) -> &::std::option::Option<i32> {
        &self.destination_port
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to add to the path.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to add to the path.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to add to the path.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    /// This field is required.
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the source. If you specify this parameter, you can't specify the parameters for the source IP address or the destination port.</p>
    pub fn filter_at_source(mut self, input: crate::types::PathRequestFilter) -> Self {
        self.filter_at_source = ::std::option::Option::Some(input);
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the source. If you specify this parameter, you can't specify the parameters for the source IP address or the destination port.</p>
    pub fn set_filter_at_source(mut self, input: ::std::option::Option<crate::types::PathRequestFilter>) -> Self {
        self.filter_at_source = input;
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the source. If you specify this parameter, you can't specify the parameters for the source IP address or the destination port.</p>
    pub fn get_filter_at_source(&self) -> &::std::option::Option<crate::types::PathRequestFilter> {
        &self.filter_at_source
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the destination. If you specify this parameter, you can't specify the parameter for the destination IP address.</p>
    pub fn filter_at_destination(mut self, input: crate::types::PathRequestFilter) -> Self {
        self.filter_at_destination = ::std::option::Option::Some(input);
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the destination. If you specify this parameter, you can't specify the parameter for the destination IP address.</p>
    pub fn set_filter_at_destination(mut self, input: ::std::option::Option<crate::types::PathRequestFilter>) -> Self {
        self.filter_at_destination = input;
        self
    }
    /// <p>Scopes the analysis to network paths that match specific filters at the destination. If you specify this parameter, you can't specify the parameter for the destination IP address.</p>
    pub fn get_filter_at_destination(&self) -> &::std::option::Option<crate::types::PathRequestFilter> {
        &self.filter_at_destination
    }
    /// Consumes the builder and constructs a [`CreateNetworkInsightsPathInput`](crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_network_insights_path::CreateNetworkInsightsPathInput {
            source_ip: self.source_ip,
            destination_ip: self.destination_ip,
            source: self.source,
            destination: self.destination,
            protocol: self.protocol,
            destination_port: self.destination_port,
            tag_specifications: self.tag_specifications,
            dry_run: self.dry_run,
            client_token: self.client_token,
            filter_at_source: self.filter_at_source,
            filter_at_destination: self.filter_at_destination,
        })
    }
}