// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetTransitGatewayPolicyTableAssociationsOutput {
    /// <p>Returns details about the transit gateway policy table association.</p>
    pub associations: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayPolicyTableAssociation>>,
    /// <p>The token for the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetTransitGatewayPolicyTableAssociationsOutput {
    /// <p>Returns details about the transit gateway policy table association.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.associations.is_none()`.
    pub fn associations(&self) -> &[crate::types::TransitGatewayPolicyTableAssociation] {
        self.associations.as_deref().unwrap_or_default()
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for GetTransitGatewayPolicyTableAssociationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetTransitGatewayPolicyTableAssociationsOutput {
    /// Creates a new builder-style object to manufacture [`GetTransitGatewayPolicyTableAssociationsOutput`](crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsOutput).
    pub fn builder(
    ) -> crate::operation::get_transit_gateway_policy_table_associations::builders::GetTransitGatewayPolicyTableAssociationsOutputBuilder {
        crate::operation::get_transit_gateway_policy_table_associations::builders::GetTransitGatewayPolicyTableAssociationsOutputBuilder::default()
    }
}

/// A builder for [`GetTransitGatewayPolicyTableAssociationsOutput`](crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct GetTransitGatewayPolicyTableAssociationsOutputBuilder {
    pub(crate) associations: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayPolicyTableAssociation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl GetTransitGatewayPolicyTableAssociationsOutputBuilder {
    /// Appends an item to `associations`.
    ///
    /// To override the contents of this collection use [`set_associations`](Self::set_associations).
    ///
    /// <p>Returns details about the transit gateway policy table association.</p>
    pub fn associations(mut self, input: crate::types::TransitGatewayPolicyTableAssociation) -> Self {
        let mut v = self.associations.unwrap_or_default();
        v.push(input);
        self.associations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Returns details about the transit gateway policy table association.</p>
    pub fn set_associations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayPolicyTableAssociation>>) -> Self {
        self.associations = input;
        self
    }
    /// <p>Returns details about the transit gateway policy table association.</p>
    pub fn get_associations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TransitGatewayPolicyTableAssociation>> {
        &self.associations
    }
    /// <p>The token for the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token for the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token for the next page of results.</p>
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
    /// Consumes the builder and constructs a [`GetTransitGatewayPolicyTableAssociationsOutput`](crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsOutput).
    pub fn build(self) -> crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsOutput {
        crate::operation::get_transit_gateway_policy_table_associations::GetTransitGatewayPolicyTableAssociationsOutput {
            associations: self.associations,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}