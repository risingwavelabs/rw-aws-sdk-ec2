// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateInstanceConnectEndpointOutput {
    /// <p>Information about the EC2 Instance Connect Endpoint.</p>
    pub instance_connect_endpoint: ::std::option::Option<crate::types::Ec2InstanceConnectEndpoint>,
    /// <p>Unique, case-sensitive idempotency token provided by the client in the the request.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateInstanceConnectEndpointOutput {
    /// <p>Information about the EC2 Instance Connect Endpoint.</p>
    pub fn instance_connect_endpoint(&self) -> ::std::option::Option<&crate::types::Ec2InstanceConnectEndpoint> {
        self.instance_connect_endpoint.as_ref()
    }
    /// <p>Unique, case-sensitive idempotency token provided by the client in the the request.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for CreateInstanceConnectEndpointOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CreateInstanceConnectEndpointOutput {
    /// Creates a new builder-style object to manufacture [`CreateInstanceConnectEndpointOutput`](crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput).
    pub fn builder() -> crate::operation::create_instance_connect_endpoint::builders::CreateInstanceConnectEndpointOutputBuilder {
        crate::operation::create_instance_connect_endpoint::builders::CreateInstanceConnectEndpointOutputBuilder::default()
    }
}

/// A builder for [`CreateInstanceConnectEndpointOutput`](crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateInstanceConnectEndpointOutputBuilder {
    pub(crate) instance_connect_endpoint: ::std::option::Option<crate::types::Ec2InstanceConnectEndpoint>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl CreateInstanceConnectEndpointOutputBuilder {
    /// <p>Information about the EC2 Instance Connect Endpoint.</p>
    pub fn instance_connect_endpoint(mut self, input: crate::types::Ec2InstanceConnectEndpoint) -> Self {
        self.instance_connect_endpoint = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the EC2 Instance Connect Endpoint.</p>
    pub fn set_instance_connect_endpoint(mut self, input: ::std::option::Option<crate::types::Ec2InstanceConnectEndpoint>) -> Self {
        self.instance_connect_endpoint = input;
        self
    }
    /// <p>Information about the EC2 Instance Connect Endpoint.</p>
    pub fn get_instance_connect_endpoint(&self) -> &::std::option::Option<crate::types::Ec2InstanceConnectEndpoint> {
        &self.instance_connect_endpoint
    }
    /// <p>Unique, case-sensitive idempotency token provided by the client in the the request.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive idempotency token provided by the client in the the request.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Unique, case-sensitive idempotency token provided by the client in the the request.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CreateInstanceConnectEndpointOutput`](crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput).
    pub fn build(self) -> crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput {
        crate::operation::create_instance_connect_endpoint::CreateInstanceConnectEndpointOutput {
            instance_connect_endpoint: self.instance_connect_endpoint,
            client_token: self.client_token,
            _request_id: self._request_id,
        }
    }
}
