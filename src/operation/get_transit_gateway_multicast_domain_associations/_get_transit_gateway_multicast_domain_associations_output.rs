// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTransitGatewayMulticastDomainAssociationsOutput {
    /// <p>Information about the multicast domain associations.</p>
    pub multicast_domain_associations: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayMulticastDomainAssociation>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetTransitGatewayMulticastDomainAssociationsOutput {
    /// <p>Information about the multicast domain associations.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.multicast_domain_associations.is_none()`.
    pub fn multicast_domain_associations(&self) -> &[crate::types::TransitGatewayMulticastDomainAssociation] {
        self.multicast_domain_associations.as_deref().unwrap_or_default()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for GetTransitGatewayMulticastDomainAssociationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetTransitGatewayMulticastDomainAssociationsOutput {
    /// Creates a new builder-style object to manufacture [`GetTransitGatewayMulticastDomainAssociationsOutput`](crate::operation::get_transit_gateway_multicast_domain_associations::GetTransitGatewayMulticastDomainAssociationsOutput).
    pub fn builder(
    ) -> crate::operation::get_transit_gateway_multicast_domain_associations::builders::GetTransitGatewayMulticastDomainAssociationsOutputBuilder
    {
        crate::operation::get_transit_gateway_multicast_domain_associations::builders::GetTransitGatewayMulticastDomainAssociationsOutputBuilder::default()
    }
}

/// A builder for [`GetTransitGatewayMulticastDomainAssociationsOutput`](crate::operation::get_transit_gateway_multicast_domain_associations::GetTransitGatewayMulticastDomainAssociationsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetTransitGatewayMulticastDomainAssociationsOutputBuilder {
    pub(crate) multicast_domain_associations: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayMulticastDomainAssociation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetTransitGatewayMulticastDomainAssociationsOutputBuilder {
    /// Appends an item to `multicast_domain_associations`.
    ///
    /// To override the contents of this collection use [`set_multicast_domain_associations`](Self::set_multicast_domain_associations).
    ///
    /// <p>Information about the multicast domain associations.</p>
    pub fn multicast_domain_associations(mut self, input: crate::types::TransitGatewayMulticastDomainAssociation) -> Self {
        let mut v = self.multicast_domain_associations.unwrap_or_default();
        v.push(input);
        self.multicast_domain_associations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the multicast domain associations.</p>
    pub fn set_multicast_domain_associations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayMulticastDomainAssociation>>,
    ) -> Self {
        self.multicast_domain_associations = input;
        self
    }
    /// <p>Information about the multicast domain associations.</p>
    pub fn get_multicast_domain_associations(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayMulticastDomainAssociation>> {
        &self.multicast_domain_associations
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetTransitGatewayMulticastDomainAssociationsOutput`](crate::operation::get_transit_gateway_multicast_domain_associations::GetTransitGatewayMulticastDomainAssociationsOutput).
    pub fn build(self) -> crate::operation::get_transit_gateway_multicast_domain_associations::GetTransitGatewayMulticastDomainAssociationsOutput {
        crate::operation::get_transit_gateway_multicast_domain_associations::GetTransitGatewayMulticastDomainAssociationsOutput {
            multicast_domain_associations: self.multicast_domain_associations,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
