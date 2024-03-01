// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about an address range that is provisioned for use with your Amazon Web Services resources through bring your own IP addresses (BYOIP).</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ByoipCidr {
    /// <p>The address range, in CIDR notation.</p>
    pub cidr: ::std::option::Option<::std::string::String>,
    /// <p>The description of the address range.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The BYOIP CIDR associations with ASNs.</p>
    pub asn_associations: ::std::option::Option<::std::vec::Vec<crate::types::AsnAssociation>>,
    /// <p>Upon success, contains the ID of the address pool. Otherwise, contains an error message.</p>
    pub status_message: ::std::option::Option<::std::string::String>,
    /// <p>The state of the address pool.</p>
    pub state: ::std::option::Option<crate::types::ByoipCidrState>,
}
impl ByoipCidr {
    /// <p>The address range, in CIDR notation.</p>
    pub fn cidr(&self) -> ::std::option::Option<&str> {
        self.cidr.as_deref()
    }
    /// <p>The description of the address range.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The BYOIP CIDR associations with ASNs.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.asn_associations.is_none()`.
    pub fn asn_associations(&self) -> &[crate::types::AsnAssociation] {
        self.asn_associations.as_deref().unwrap_or_default()
    }
    /// <p>Upon success, contains the ID of the address pool. Otherwise, contains an error message.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>The state of the address pool.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::ByoipCidrState> {
        self.state.as_ref()
    }
}
impl ByoipCidr {
    /// Creates a new builder-style object to manufacture [`ByoipCidr`](crate::types::ByoipCidr).
    pub fn builder() -> crate::types::builders::ByoipCidrBuilder {
        crate::types::builders::ByoipCidrBuilder::default()
    }
}

/// A builder for [`ByoipCidr`](crate::types::ByoipCidr).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ByoipCidrBuilder {
    pub(crate) cidr: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) asn_associations: ::std::option::Option<::std::vec::Vec<crate::types::AsnAssociation>>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::ByoipCidrState>,
}
impl ByoipCidrBuilder {
    /// <p>The address range, in CIDR notation.</p>
    pub fn cidr(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.cidr = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The address range, in CIDR notation.</p>
    pub fn set_cidr(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.cidr = input;
        self
    }
    /// <p>The address range, in CIDR notation.</p>
    pub fn get_cidr(&self) -> &::std::option::Option<::std::string::String> {
        &self.cidr
    }
    /// <p>The description of the address range.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the address range.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the address range.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Appends an item to `asn_associations`.
    ///
    /// To override the contents of this collection use [`set_asn_associations`](Self::set_asn_associations).
    ///
    /// <p>The BYOIP CIDR associations with ASNs.</p>
    pub fn asn_associations(mut self, input: crate::types::AsnAssociation) -> Self {
        let mut v = self.asn_associations.unwrap_or_default();
        v.push(input);
        self.asn_associations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The BYOIP CIDR associations with ASNs.</p>
    pub fn set_asn_associations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AsnAssociation>>) -> Self {
        self.asn_associations = input;
        self
    }
    /// <p>The BYOIP CIDR associations with ASNs.</p>
    pub fn get_asn_associations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AsnAssociation>> {
        &self.asn_associations
    }
    /// <p>Upon success, contains the ID of the address pool. Otherwise, contains an error message.</p>
    pub fn status_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Upon success, contains the ID of the address pool. Otherwise, contains an error message.</p>
    pub fn set_status_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// <p>Upon success, contains the ID of the address pool. Otherwise, contains an error message.</p>
    pub fn get_status_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_message
    }
    /// <p>The state of the address pool.</p>
    pub fn state(mut self, input: crate::types::ByoipCidrState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the address pool.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::ByoipCidrState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the address pool.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::ByoipCidrState> {
        &self.state
    }
    /// Consumes the builder and constructs a [`ByoipCidr`](crate::types::ByoipCidr).
    pub fn build(self) -> crate::types::ByoipCidr {
        crate::types::ByoipCidr {
            cidr: self.cidr,
            description: self.description,
            asn_associations: self.asn_associations,
            status_message: self.status_message,
            state: self.state,
        }
    }
}