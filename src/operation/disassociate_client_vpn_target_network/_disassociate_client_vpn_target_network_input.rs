// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateClientVpnTargetNetworkInput {
    /// <p>The ID of the Client VPN endpoint from which to disassociate the target network.</p>
    pub client_vpn_endpoint_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the target network association.</p>
    pub association_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DisassociateClientVpnTargetNetworkInput {
    /// <p>The ID of the Client VPN endpoint from which to disassociate the target network.</p>
    pub fn client_vpn_endpoint_id(&self) -> ::std::option::Option<&str> {
        self.client_vpn_endpoint_id.as_deref()
    }
    /// <p>The ID of the target network association.</p>
    pub fn association_id(&self) -> ::std::option::Option<&str> {
        self.association_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DisassociateClientVpnTargetNetworkInput {
    /// Creates a new builder-style object to manufacture [`DisassociateClientVpnTargetNetworkInput`](crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkInput).
    pub fn builder() -> crate::operation::disassociate_client_vpn_target_network::builders::DisassociateClientVpnTargetNetworkInputBuilder {
        crate::operation::disassociate_client_vpn_target_network::builders::DisassociateClientVpnTargetNetworkInputBuilder::default()
    }
}

/// A builder for [`DisassociateClientVpnTargetNetworkInput`](crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisassociateClientVpnTargetNetworkInputBuilder {
    pub(crate) client_vpn_endpoint_id: ::std::option::Option<::std::string::String>,
    pub(crate) association_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DisassociateClientVpnTargetNetworkInputBuilder {
    /// <p>The ID of the Client VPN endpoint from which to disassociate the target network.</p>
    /// This field is required.
    pub fn client_vpn_endpoint_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_vpn_endpoint_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Client VPN endpoint from which to disassociate the target network.</p>
    pub fn set_client_vpn_endpoint_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_vpn_endpoint_id = input;
        self
    }
    /// <p>The ID of the Client VPN endpoint from which to disassociate the target network.</p>
    pub fn get_client_vpn_endpoint_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_vpn_endpoint_id
    }
    /// <p>The ID of the target network association.</p>
    /// This field is required.
    pub fn association_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.association_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the target network association.</p>
    pub fn set_association_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.association_id = input;
        self
    }
    /// <p>The ID of the target network association.</p>
    pub fn get_association_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.association_id
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
    /// Consumes the builder and constructs a [`DisassociateClientVpnTargetNetworkInput`](crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::operation::disassociate_client_vpn_target_network::DisassociateClientVpnTargetNetworkInput {
                client_vpn_endpoint_id: self.client_vpn_endpoint_id,
                association_id: self.association_id,
                dry_run: self.dry_run,
            },
        )
    }
}