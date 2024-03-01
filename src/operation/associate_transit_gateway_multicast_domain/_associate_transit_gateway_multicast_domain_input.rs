// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateTransitGatewayMulticastDomainInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub transit_gateway_multicast_domain_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p>
    pub transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl AssociateTransitGatewayMulticastDomainInput {
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn transit_gateway_multicast_domain_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_multicast_domain_id.as_deref()
    }
    /// <p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p>
    pub fn transit_gateway_attachment_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_attachment_id.as_deref()
    }
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.subnet_ids.is_none()`.
    pub fn subnet_ids(&self) -> &[::std::string::String] {
        self.subnet_ids.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl AssociateTransitGatewayMulticastDomainInput {
    /// Creates a new builder-style object to manufacture [`AssociateTransitGatewayMulticastDomainInput`](crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainInput).
    pub fn builder() -> crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainInputBuilder {
        crate::operation::associate_transit_gateway_multicast_domain::builders::AssociateTransitGatewayMulticastDomainInputBuilder::default()
    }
}

/// A builder for [`AssociateTransitGatewayMulticastDomainInput`](crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AssociateTransitGatewayMulticastDomainInputBuilder {
    pub(crate) transit_gateway_multicast_domain_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) subnet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl AssociateTransitGatewayMulticastDomainInputBuilder {
    /// <p>The ID of the transit gateway multicast domain.</p>
    /// This field is required.
    pub fn transit_gateway_multicast_domain_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_multicast_domain_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_multicast_domain_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_multicast_domain_id = input;
        self
    }
    /// <p>The ID of the transit gateway multicast domain.</p>
    pub fn get_transit_gateway_multicast_domain_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_multicast_domain_id
    }
    /// <p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p>
    /// This field is required.
    pub fn transit_gateway_attachment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_attachment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p>
    pub fn set_transit_gateway_attachment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_attachment_id = input;
        self
    }
    /// <p>The ID of the transit gateway attachment to associate with the transit gateway multicast domain.</p>
    pub fn get_transit_gateway_attachment_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_attachment_id
    }
    /// Appends an item to `subnet_ids`.
    ///
    /// To override the contents of this collection use [`set_subnet_ids`](Self::set_subnet_ids).
    ///
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub fn subnet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.subnet_ids.unwrap_or_default();
        v.push(input.into());
        self.subnet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub fn set_subnet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.subnet_ids = input;
        self
    }
    /// <p>The IDs of the subnets to associate with the transit gateway multicast domain.</p>
    pub fn get_subnet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.subnet_ids
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
    /// Consumes the builder and constructs a [`AssociateTransitGatewayMulticastDomainInput`](crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::associate_transit_gateway_multicast_domain::AssociateTransitGatewayMulticastDomainInput {
                transit_gateway_multicast_domain_id: self.transit_gateway_multicast_domain_id,
                transit_gateway_attachment_id: self.transit_gateway_attachment_id,
                subnet_ids: self.subnet_ids,
                dry_run: self.dry_run,
            },
        )
    }
}
