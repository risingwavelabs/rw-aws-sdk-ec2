// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTrafficMirrorTargetInput {
    /// <p>The network interface ID that is associated with the target.</p>
    pub network_interface_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.</p>
    pub network_load_balancer_arn: ::std::option::Option<::std::string::String>,
    /// <p>The description of the Traffic Mirror target.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The tags to assign to the Traffic Mirror target.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">How to ensure idempotency</a>.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Gateway Load Balancer endpoint.</p>
    pub gateway_load_balancer_endpoint_id: ::std::option::Option<::std::string::String>,
}
impl CreateTrafficMirrorTargetInput {
    /// <p>The network interface ID that is associated with the target.</p>
    pub fn network_interface_id(&self) -> ::std::option::Option<&str> {
        self.network_interface_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.</p>
    pub fn network_load_balancer_arn(&self) -> ::std::option::Option<&str> {
        self.network_load_balancer_arn.as_deref()
    }
    /// <p>The description of the Traffic Mirror target.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The tags to assign to the Traffic Mirror target.</p>
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
    /// <p>The ID of the Gateway Load Balancer endpoint.</p>
    pub fn gateway_load_balancer_endpoint_id(&self) -> ::std::option::Option<&str> {
        self.gateway_load_balancer_endpoint_id.as_deref()
    }
}
impl CreateTrafficMirrorTargetInput {
    /// Creates a new builder-style object to manufacture [`CreateTrafficMirrorTargetInput`](crate::operation::create_traffic_mirror_target::CreateTrafficMirrorTargetInput).
    pub fn builder() -> crate::operation::create_traffic_mirror_target::builders::CreateTrafficMirrorTargetInputBuilder {
        crate::operation::create_traffic_mirror_target::builders::CreateTrafficMirrorTargetInputBuilder::default()
    }
}

/// A builder for [`CreateTrafficMirrorTargetInput`](crate::operation::create_traffic_mirror_target::CreateTrafficMirrorTargetInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateTrafficMirrorTargetInputBuilder {
    pub(crate) network_interface_id: ::std::option::Option<::std::string::String>,
    pub(crate) network_load_balancer_arn: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) gateway_load_balancer_endpoint_id: ::std::option::Option<::std::string::String>,
}
impl CreateTrafficMirrorTargetInputBuilder {
    /// <p>The network interface ID that is associated with the target.</p>
    pub fn network_interface_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_interface_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The network interface ID that is associated with the target.</p>
    pub fn set_network_interface_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_interface_id = input;
        self
    }
    /// <p>The network interface ID that is associated with the target.</p>
    pub fn get_network_interface_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.network_interface_id
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.</p>
    pub fn network_load_balancer_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.network_load_balancer_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.</p>
    pub fn set_network_load_balancer_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.network_load_balancer_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Network Load Balancer that is associated with the target.</p>
    pub fn get_network_load_balancer_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.network_load_balancer_arn
    }
    /// <p>The description of the Traffic Mirror target.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the Traffic Mirror target.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the Traffic Mirror target.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the Traffic Mirror target.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to assign to the Traffic Mirror target.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to assign to the Traffic Mirror target.</p>
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
    /// <p>The ID of the Gateway Load Balancer endpoint.</p>
    pub fn gateway_load_balancer_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.gateway_load_balancer_endpoint_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Gateway Load Balancer endpoint.</p>
    pub fn set_gateway_load_balancer_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.gateway_load_balancer_endpoint_id = input;
        self
    }
    /// <p>The ID of the Gateway Load Balancer endpoint.</p>
    pub fn get_gateway_load_balancer_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.gateway_load_balancer_endpoint_id
    }
    /// Consumes the builder and constructs a [`CreateTrafficMirrorTargetInput`](crate::operation::create_traffic_mirror_target::CreateTrafficMirrorTargetInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_traffic_mirror_target::CreateTrafficMirrorTargetInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_traffic_mirror_target::CreateTrafficMirrorTargetInput {
            network_interface_id: self.network_interface_id,
            network_load_balancer_arn: self.network_load_balancer_arn,
            description: self.description,
            tag_specifications: self.tag_specifications,
            dry_run: self.dry_run,
            client_token: self.client_token,
            gateway_load_balancer_endpoint_id: self.gateway_load_balancer_endpoint_id,
        })
    }
}
