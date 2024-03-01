// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateTransitGatewayPeeringAttachmentInput {
    /// <p>The ID of the transit gateway.</p>
    pub transit_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the peer transit gateway with which to create the peering attachment.</p>
    pub peer_transit_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Web Services account that owns the peer transit gateway.</p>
    pub peer_account_id: ::std::option::Option<::std::string::String>,
    /// <p>The Region where the peer transit gateway is located.</p>
    pub peer_region: ::std::option::Option<::std::string::String>,
    /// <p>Requests a transit gateway peering attachment.</p>
    pub options: ::std::option::Option<crate::types::CreateTransitGatewayPeeringAttachmentRequestOptions>,
    /// <p>The tags to apply to the transit gateway peering attachment.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl CreateTransitGatewayPeeringAttachmentInput {
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_id.as_deref()
    }
    /// <p>The ID of the peer transit gateway with which to create the peering attachment.</p>
    pub fn peer_transit_gateway_id(&self) -> ::std::option::Option<&str> {
        self.peer_transit_gateway_id.as_deref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the peer transit gateway.</p>
    pub fn peer_account_id(&self) -> ::std::option::Option<&str> {
        self.peer_account_id.as_deref()
    }
    /// <p>The Region where the peer transit gateway is located.</p>
    pub fn peer_region(&self) -> ::std::option::Option<&str> {
        self.peer_region.as_deref()
    }
    /// <p>Requests a transit gateway peering attachment.</p>
    pub fn options(&self) -> ::std::option::Option<&crate::types::CreateTransitGatewayPeeringAttachmentRequestOptions> {
        self.options.as_ref()
    }
    /// <p>The tags to apply to the transit gateway peering attachment.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl CreateTransitGatewayPeeringAttachmentInput {
    /// Creates a new builder-style object to manufacture [`CreateTransitGatewayPeeringAttachmentInput`](crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentInput).
    pub fn builder() -> crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentInputBuilder {
        crate::operation::create_transit_gateway_peering_attachment::builders::CreateTransitGatewayPeeringAttachmentInputBuilder::default()
    }
}

/// A builder for [`CreateTransitGatewayPeeringAttachmentInput`](crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateTransitGatewayPeeringAttachmentInputBuilder {
    pub(crate) transit_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) peer_transit_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) peer_account_id: ::std::option::Option<::std::string::String>,
    pub(crate) peer_region: ::std::option::Option<::std::string::String>,
    pub(crate) options: ::std::option::Option<crate::types::CreateTransitGatewayPeeringAttachmentRequestOptions>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl CreateTransitGatewayPeeringAttachmentInputBuilder {
    /// <p>The ID of the transit gateway.</p>
    /// This field is required.
    pub fn transit_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn set_transit_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_id = input;
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn get_transit_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_id
    }
    /// <p>The ID of the peer transit gateway with which to create the peering attachment.</p>
    /// This field is required.
    pub fn peer_transit_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.peer_transit_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the peer transit gateway with which to create the peering attachment.</p>
    pub fn set_peer_transit_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.peer_transit_gateway_id = input;
        self
    }
    /// <p>The ID of the peer transit gateway with which to create the peering attachment.</p>
    pub fn get_peer_transit_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.peer_transit_gateway_id
    }
    /// <p>The ID of the Amazon Web Services account that owns the peer transit gateway.</p>
    /// This field is required.
    pub fn peer_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.peer_account_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the peer transit gateway.</p>
    pub fn set_peer_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.peer_account_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the peer transit gateway.</p>
    pub fn get_peer_account_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.peer_account_id
    }
    /// <p>The Region where the peer transit gateway is located.</p>
    /// This field is required.
    pub fn peer_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.peer_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region where the peer transit gateway is located.</p>
    pub fn set_peer_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.peer_region = input;
        self
    }
    /// <p>The Region where the peer transit gateway is located.</p>
    pub fn get_peer_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.peer_region
    }
    /// <p>Requests a transit gateway peering attachment.</p>
    pub fn options(mut self, input: crate::types::CreateTransitGatewayPeeringAttachmentRequestOptions) -> Self {
        self.options = ::std::option::Option::Some(input);
        self
    }
    /// <p>Requests a transit gateway peering attachment.</p>
    pub fn set_options(mut self, input: ::std::option::Option<crate::types::CreateTransitGatewayPeeringAttachmentRequestOptions>) -> Self {
        self.options = input;
        self
    }
    /// <p>Requests a transit gateway peering attachment.</p>
    pub fn get_options(&self) -> &::std::option::Option<crate::types::CreateTransitGatewayPeeringAttachmentRequestOptions> {
        &self.options
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the transit gateway peering attachment.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the transit gateway peering attachment.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to apply to the transit gateway peering attachment.</p>
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
    /// Consumes the builder and constructs a [`CreateTransitGatewayPeeringAttachmentInput`](crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::create_transit_gateway_peering_attachment::CreateTransitGatewayPeeringAttachmentInput {
                transit_gateway_id: self.transit_gateway_id,
                peer_transit_gateway_id: self.peer_transit_gateway_id,
                peer_account_id: self.peer_account_id,
                peer_region: self.peer_region,
                options: self.options,
                tag_specifications: self.tag_specifications,
                dry_run: self.dry_run,
            },
        )
    }
}
