// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateInstanceConnectEndpointInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p>
    pub subnet_id: ::std::option::Option<::std::string::String>,
    /// <p>One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for your VPC will be associated with the endpoint.</p>
    pub security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Indicates whether your client's IP address is preserved as the source. The value is <code>true</code> or <code>false</code>.</p>
    /// <ul>
    /// <li> <p>If <code>true</code>, your client's IP address is used when you connect to a resource.</p> </li>
    /// <li> <p>If <code>false</code>, the elastic network interface IP address is used when you connect to a resource.</p> </li>
    /// </ul>
    /// <p>Default: <code>true</code> </p>
    pub preserve_client_ip: ::std::option::Option<bool>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The tags to apply to the EC2 Instance Connect Endpoint during creation.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
}
impl CreateInstanceConnectEndpointInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p>
    pub fn subnet_id(&self) -> ::std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for your VPC will be associated with the endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.security_group_ids.is_none()`.
    pub fn security_group_ids(&self) -> &[::std::string::String] {
        self.security_group_ids.as_deref().unwrap_or_default()
    }
    /// <p>Indicates whether your client's IP address is preserved as the source. The value is <code>true</code> or <code>false</code>.</p>
    /// <ul>
    /// <li> <p>If <code>true</code>, your client's IP address is used when you connect to a resource.</p> </li>
    /// <li> <p>If <code>false</code>, the elastic network interface IP address is used when you connect to a resource.</p> </li>
    /// </ul>
    /// <p>Default: <code>true</code> </p>
    pub fn preserve_client_ip(&self) -> ::std::option::Option<bool> {
        self.preserve_client_ip
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The tags to apply to the EC2 Instance Connect Endpoint during creation.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
}
impl CreateInstanceConnectEndpointInput {
    /// Creates a new builder-style object to manufacture [`CreateInstanceConnectEndpointInput`](crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointInput).
    pub fn builder() -> crate::operation::create_instance_connect_endpoint::builders::CreateInstanceConnectEndpointInputBuilder {
        crate::operation::create_instance_connect_endpoint::builders::CreateInstanceConnectEndpointInputBuilder::default()
    }
}

/// A builder for [`CreateInstanceConnectEndpointInput`](crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateInstanceConnectEndpointInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) subnet_id: ::std::option::Option<::std::string::String>,
    pub(crate) security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) preserve_client_ip: ::std::option::Option<bool>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
}
impl CreateInstanceConnectEndpointInputBuilder {
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
    /// <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p>
    /// This field is required.
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subnet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// <p>The ID of the subnet in which to create the EC2 Instance Connect Endpoint.</p>
    pub fn get_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.subnet_id
    }
    /// Appends an item to `security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for your VPC will be associated with the endpoint.</p>
    pub fn security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for your VPC will be associated with the endpoint.</p>
    pub fn set_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.security_group_ids = input;
        self
    }
    /// <p>One or more security groups to associate with the endpoint. If you don't specify a security group, the default security group for your VPC will be associated with the endpoint.</p>
    pub fn get_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.security_group_ids
    }
    /// <p>Indicates whether your client's IP address is preserved as the source. The value is <code>true</code> or <code>false</code>.</p>
    /// <ul>
    /// <li> <p>If <code>true</code>, your client's IP address is used when you connect to a resource.</p> </li>
    /// <li> <p>If <code>false</code>, the elastic network interface IP address is used when you connect to a resource.</p> </li>
    /// </ul>
    /// <p>Default: <code>true</code> </p>
    pub fn preserve_client_ip(mut self, input: bool) -> Self {
        self.preserve_client_ip = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether your client's IP address is preserved as the source. The value is <code>true</code> or <code>false</code>.</p>
    /// <ul>
    /// <li> <p>If <code>true</code>, your client's IP address is used when you connect to a resource.</p> </li>
    /// <li> <p>If <code>false</code>, the elastic network interface IP address is used when you connect to a resource.</p> </li>
    /// </ul>
    /// <p>Default: <code>true</code> </p>
    pub fn set_preserve_client_ip(mut self, input: ::std::option::Option<bool>) -> Self {
        self.preserve_client_ip = input;
        self
    }
    /// <p>Indicates whether your client's IP address is preserved as the source. The value is <code>true</code> or <code>false</code>.</p>
    /// <ul>
    /// <li> <p>If <code>true</code>, your client's IP address is used when you connect to a resource.</p> </li>
    /// <li> <p>If <code>false</code>, the elastic network interface IP address is used when you connect to a resource.</p> </li>
    /// </ul>
    /// <p>Default: <code>true</code> </p>
    pub fn get_preserve_client_ip(&self) -> &::std::option::Option<bool> {
        &self.preserve_client_ip
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the EC2 Instance Connect Endpoint during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the EC2 Instance Connect Endpoint during creation.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to apply to the EC2 Instance Connect Endpoint during creation.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
    /// Consumes the builder and constructs a [`CreateInstanceConnectEndpointInput`](crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointInput {
            dry_run: self.dry_run,
            subnet_id: self.subnet_id,
            security_group_ids: self.security_group_ids,
            preserve_client_ip: self.preserve_client_ip,
            client_token: self.client_token,
            tag_specifications: self.tag_specifications,
        })
    }
}