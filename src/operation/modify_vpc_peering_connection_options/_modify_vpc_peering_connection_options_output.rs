// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyVpcPeeringConnectionOptionsOutput {
    /// <p>Information about the VPC peering connection options for the accepter VPC.</p>
    pub accepter_peering_connection_options: ::std::option::Option<crate::types::PeeringConnectionOptions>,
    /// <p>Information about the VPC peering connection options for the requester VPC.</p>
    pub requester_peering_connection_options: ::std::option::Option<crate::types::PeeringConnectionOptions>,
    _request_id: Option<String>,
}
impl ModifyVpcPeeringConnectionOptionsOutput {
    /// <p>Information about the VPC peering connection options for the accepter VPC.</p>
    pub fn accepter_peering_connection_options(&self) -> ::std::option::Option<&crate::types::PeeringConnectionOptions> {
        self.accepter_peering_connection_options.as_ref()
    }
    /// <p>Information about the VPC peering connection options for the requester VPC.</p>
    pub fn requester_peering_connection_options(&self) -> ::std::option::Option<&crate::types::PeeringConnectionOptions> {
        self.requester_peering_connection_options.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ModifyVpcPeeringConnectionOptionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyVpcPeeringConnectionOptionsOutput {
    /// Creates a new builder-style object to manufacture [`ModifyVpcPeeringConnectionOptionsOutput`](crate::operation::modify_vpc_peering_connection_options::ModifyVpcPeeringConnectionOptionsOutput).
    pub fn builder() -> crate::operation::modify_vpc_peering_connection_options::builders::ModifyVpcPeeringConnectionOptionsOutputBuilder {
        crate::operation::modify_vpc_peering_connection_options::builders::ModifyVpcPeeringConnectionOptionsOutputBuilder::default()
    }
}

/// A builder for [`ModifyVpcPeeringConnectionOptionsOutput`](crate::operation::modify_vpc_peering_connection_options::ModifyVpcPeeringConnectionOptionsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyVpcPeeringConnectionOptionsOutputBuilder {
    pub(crate) accepter_peering_connection_options: ::std::option::Option<crate::types::PeeringConnectionOptions>,
    pub(crate) requester_peering_connection_options: ::std::option::Option<crate::types::PeeringConnectionOptions>,
    _request_id: Option<String>,
}
impl ModifyVpcPeeringConnectionOptionsOutputBuilder {
    /// <p>Information about the VPC peering connection options for the accepter VPC.</p>
    pub fn accepter_peering_connection_options(mut self, input: crate::types::PeeringConnectionOptions) -> Self {
        self.accepter_peering_connection_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the VPC peering connection options for the accepter VPC.</p>
    pub fn set_accepter_peering_connection_options(mut self, input: ::std::option::Option<crate::types::PeeringConnectionOptions>) -> Self {
        self.accepter_peering_connection_options = input;
        self
    }
    /// <p>Information about the VPC peering connection options for the accepter VPC.</p>
    pub fn get_accepter_peering_connection_options(&self) -> &::std::option::Option<crate::types::PeeringConnectionOptions> {
        &self.accepter_peering_connection_options
    }
    /// <p>Information about the VPC peering connection options for the requester VPC.</p>
    pub fn requester_peering_connection_options(mut self, input: crate::types::PeeringConnectionOptions) -> Self {
        self.requester_peering_connection_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the VPC peering connection options for the requester VPC.</p>
    pub fn set_requester_peering_connection_options(mut self, input: ::std::option::Option<crate::types::PeeringConnectionOptions>) -> Self {
        self.requester_peering_connection_options = input;
        self
    }
    /// <p>Information about the VPC peering connection options for the requester VPC.</p>
    pub fn get_requester_peering_connection_options(&self) -> &::std::option::Option<crate::types::PeeringConnectionOptions> {
        &self.requester_peering_connection_options
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifyVpcPeeringConnectionOptionsOutput`](crate::operation::modify_vpc_peering_connection_options::ModifyVpcPeeringConnectionOptionsOutput).
    pub fn build(self) -> crate::operation::modify_vpc_peering_connection_options::ModifyVpcPeeringConnectionOptionsOutput {
        crate::operation::modify_vpc_peering_connection_options::ModifyVpcPeeringConnectionOptionsOutput {
            accepter_peering_connection_options: self.accepter_peering_connection_options,
            requester_peering_connection_options: self.requester_peering_connection_options,
            _request_id: self._request_id,
        }
    }
}