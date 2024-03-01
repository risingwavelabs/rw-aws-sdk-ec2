// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Launch instances with ENA Express settings configured from your launch template.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnaSrdSpecificationRequest {
    /// <p>Specifies whether ENA Express is enabled for the network interface when you launch an instance from your launch template.</p>
    pub ena_srd_enabled: ::std::option::Option<bool>,
    /// <p>Contains ENA Express settings for UDP network traffic in your launch template.</p>
    pub ena_srd_udp_specification: ::std::option::Option<crate::types::EnaSrdUdpSpecificationRequest>,
}
impl EnaSrdSpecificationRequest {
    /// <p>Specifies whether ENA Express is enabled for the network interface when you launch an instance from your launch template.</p>
    pub fn ena_srd_enabled(&self) -> ::std::option::Option<bool> {
        self.ena_srd_enabled
    }
    /// <p>Contains ENA Express settings for UDP network traffic in your launch template.</p>
    pub fn ena_srd_udp_specification(&self) -> ::std::option::Option<&crate::types::EnaSrdUdpSpecificationRequest> {
        self.ena_srd_udp_specification.as_ref()
    }
}
impl EnaSrdSpecificationRequest {
    /// Creates a new builder-style object to manufacture [`EnaSrdSpecificationRequest`](crate::types::EnaSrdSpecificationRequest).
    pub fn builder() -> crate::types::builders::EnaSrdSpecificationRequestBuilder {
        crate::types::builders::EnaSrdSpecificationRequestBuilder::default()
    }
}

/// A builder for [`EnaSrdSpecificationRequest`](crate::types::EnaSrdSpecificationRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EnaSrdSpecificationRequestBuilder {
    pub(crate) ena_srd_enabled: ::std::option::Option<bool>,
    pub(crate) ena_srd_udp_specification: ::std::option::Option<crate::types::EnaSrdUdpSpecificationRequest>,
}
impl EnaSrdSpecificationRequestBuilder {
    /// <p>Specifies whether ENA Express is enabled for the network interface when you launch an instance from your launch template.</p>
    pub fn ena_srd_enabled(mut self, input: bool) -> Self {
        self.ena_srd_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether ENA Express is enabled for the network interface when you launch an instance from your launch template.</p>
    pub fn set_ena_srd_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.ena_srd_enabled = input;
        self
    }
    /// <p>Specifies whether ENA Express is enabled for the network interface when you launch an instance from your launch template.</p>
    pub fn get_ena_srd_enabled(&self) -> &::std::option::Option<bool> {
        &self.ena_srd_enabled
    }
    /// <p>Contains ENA Express settings for UDP network traffic in your launch template.</p>
    pub fn ena_srd_udp_specification(mut self, input: crate::types::EnaSrdUdpSpecificationRequest) -> Self {
        self.ena_srd_udp_specification = ::std::option::Option::Some(input);
        self
    }
    /// <p>Contains ENA Express settings for UDP network traffic in your launch template.</p>
    pub fn set_ena_srd_udp_specification(mut self, input: ::std::option::Option<crate::types::EnaSrdUdpSpecificationRequest>) -> Self {
        self.ena_srd_udp_specification = input;
        self
    }
    /// <p>Contains ENA Express settings for UDP network traffic in your launch template.</p>
    pub fn get_ena_srd_udp_specification(&self) -> &::std::option::Option<crate::types::EnaSrdUdpSpecificationRequest> {
        &self.ena_srd_udp_specification
    }
    /// Consumes the builder and constructs a [`EnaSrdSpecificationRequest`](crate::types::EnaSrdSpecificationRequest).
    pub fn build(self) -> crate::types::EnaSrdSpecificationRequest {
        crate::types::EnaSrdSpecificationRequest {
            ena_srd_enabled: self.ena_srd_enabled,
            ena_srd_udp_specification: self.ena_srd_udp_specification,
        }
    }
}
