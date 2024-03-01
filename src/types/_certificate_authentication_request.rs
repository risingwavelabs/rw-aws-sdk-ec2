// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the client certificate to be used for authentication.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CertificateAuthenticationRequest {
    /// <p>The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in Certificate Manager (ACM).</p>
    pub client_root_certificate_chain_arn: ::std::option::Option<::std::string::String>,
}
impl CertificateAuthenticationRequest {
    /// <p>The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in Certificate Manager (ACM).</p>
    pub fn client_root_certificate_chain_arn(&self) -> ::std::option::Option<&str> {
        self.client_root_certificate_chain_arn.as_deref()
    }
}
impl CertificateAuthenticationRequest {
    /// Creates a new builder-style object to manufacture [`CertificateAuthenticationRequest`](crate::types::CertificateAuthenticationRequest).
    pub fn builder() -> crate::types::builders::CertificateAuthenticationRequestBuilder {
        crate::types::builders::CertificateAuthenticationRequestBuilder::default()
    }
}

/// A builder for [`CertificateAuthenticationRequest`](crate::types::CertificateAuthenticationRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CertificateAuthenticationRequestBuilder {
    pub(crate) client_root_certificate_chain_arn: ::std::option::Option<::std::string::String>,
}
impl CertificateAuthenticationRequestBuilder {
    /// <p>The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in Certificate Manager (ACM).</p>
    pub fn client_root_certificate_chain_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_root_certificate_chain_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in Certificate Manager (ACM).</p>
    pub fn set_client_root_certificate_chain_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_root_certificate_chain_arn = input;
        self
    }
    /// <p>The ARN of the client certificate. The certificate must be signed by a certificate authority (CA) and it must be provisioned in Certificate Manager (ACM).</p>
    pub fn get_client_root_certificate_chain_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_root_certificate_chain_arn
    }
    /// Consumes the builder and constructs a [`CertificateAuthenticationRequest`](crate::types::CertificateAuthenticationRequest).
    pub fn build(self) -> crate::types::CertificateAuthenticationRequest {
        crate::types::CertificateAuthenticationRequest {
            client_root_certificate_chain_arn: self.client_root_certificate_chain_arn,
        }
    }
}