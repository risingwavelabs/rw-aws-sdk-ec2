// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteInstanceConnectEndpointOutput {
    /// <p>Information about the EC2 Instance Connect Endpoint.</p>
    pub instance_connect_endpoint: ::std::option::Option<crate::types::Ec2InstanceConnectEndpoint>,
    _request_id: Option<String>,
}
impl DeleteInstanceConnectEndpointOutput {
    /// <p>Information about the EC2 Instance Connect Endpoint.</p>
    pub fn instance_connect_endpoint(&self) -> ::std::option::Option<&crate::types::Ec2InstanceConnectEndpoint> {
        self.instance_connect_endpoint.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for DeleteInstanceConnectEndpointOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteInstanceConnectEndpointOutput {
    /// Creates a new builder-style object to manufacture [`DeleteInstanceConnectEndpointOutput`](crate::operation::delete_instance_connect_endpoint::DeleteInstanceConnectEndpointOutput).
    pub fn builder() -> crate::operation::delete_instance_connect_endpoint::builders::DeleteInstanceConnectEndpointOutputBuilder {
        crate::operation::delete_instance_connect_endpoint::builders::DeleteInstanceConnectEndpointOutputBuilder::default()
    }
}

/// A builder for [`DeleteInstanceConnectEndpointOutput`](crate::operation::delete_instance_connect_endpoint::DeleteInstanceConnectEndpointOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteInstanceConnectEndpointOutputBuilder {
    pub(crate) instance_connect_endpoint: ::std::option::Option<crate::types::Ec2InstanceConnectEndpoint>,
    _request_id: Option<String>,
}
impl DeleteInstanceConnectEndpointOutputBuilder {
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
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteInstanceConnectEndpointOutput`](crate::operation::delete_instance_connect_endpoint::DeleteInstanceConnectEndpointOutput).
    pub fn build(self) -> crate::operation::delete_instance_connect_endpoint::DeleteInstanceConnectEndpointOutput {
        crate::operation::delete_instance_connect_endpoint::DeleteInstanceConnectEndpointOutput {
            instance_connect_endpoint: self.instance_connect_endpoint,
            _request_id: self._request_id,
        }
    }
}
