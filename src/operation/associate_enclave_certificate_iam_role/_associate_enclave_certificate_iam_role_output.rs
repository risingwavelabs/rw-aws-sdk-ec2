// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateEnclaveCertificateIamRoleOutput {
    /// <p>The name of the Amazon S3 bucket to which the certificate was uploaded.</p>
    pub certificate_s3_bucket_name: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon S3 object key where the certificate, certificate chain, and encrypted private key bundle are stored. The object key is formatted as follows: <code>role_arn</code>/<code>certificate_arn</code>.</p>
    pub certificate_s3_object_key: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the KMS key used to encrypt the private key of the certificate.</p>
    pub encryption_kms_key_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl AssociateEnclaveCertificateIamRoleOutput {
    /// <p>The name of the Amazon S3 bucket to which the certificate was uploaded.</p>
    pub fn certificate_s3_bucket_name(&self) -> ::std::option::Option<&str> {
        self.certificate_s3_bucket_name.as_deref()
    }
    /// <p>The Amazon S3 object key where the certificate, certificate chain, and encrypted private key bundle are stored. The object key is formatted as follows: <code>role_arn</code>/<code>certificate_arn</code>.</p>
    pub fn certificate_s3_object_key(&self) -> ::std::option::Option<&str> {
        self.certificate_s3_object_key.as_deref()
    }
    /// <p>The ID of the KMS key used to encrypt the private key of the certificate.</p>
    pub fn encryption_kms_key_id(&self) -> ::std::option::Option<&str> {
        self.encryption_kms_key_id.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for AssociateEnclaveCertificateIamRoleOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl AssociateEnclaveCertificateIamRoleOutput {
    /// Creates a new builder-style object to manufacture [`AssociateEnclaveCertificateIamRoleOutput`](crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleOutput).
    pub fn builder() -> crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleOutputBuilder {
        crate::operation::associate_enclave_certificate_iam_role::builders::AssociateEnclaveCertificateIamRoleOutputBuilder::default()
    }
}

/// A builder for [`AssociateEnclaveCertificateIamRoleOutput`](crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AssociateEnclaveCertificateIamRoleOutputBuilder {
    pub(crate) certificate_s3_bucket_name: ::std::option::Option<::std::string::String>,
    pub(crate) certificate_s3_object_key: ::std::option::Option<::std::string::String>,
    pub(crate) encryption_kms_key_id: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl AssociateEnclaveCertificateIamRoleOutputBuilder {
    /// <p>The name of the Amazon S3 bucket to which the certificate was uploaded.</p>
    pub fn certificate_s3_bucket_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.certificate_s3_bucket_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Amazon S3 bucket to which the certificate was uploaded.</p>
    pub fn set_certificate_s3_bucket_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.certificate_s3_bucket_name = input;
        self
    }
    /// <p>The name of the Amazon S3 bucket to which the certificate was uploaded.</p>
    pub fn get_certificate_s3_bucket_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.certificate_s3_bucket_name
    }
    /// <p>The Amazon S3 object key where the certificate, certificate chain, and encrypted private key bundle are stored. The object key is formatted as follows: <code>role_arn</code>/<code>certificate_arn</code>.</p>
    pub fn certificate_s3_object_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.certificate_s3_object_key = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon S3 object key where the certificate, certificate chain, and encrypted private key bundle are stored. The object key is formatted as follows: <code>role_arn</code>/<code>certificate_arn</code>.</p>
    pub fn set_certificate_s3_object_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.certificate_s3_object_key = input;
        self
    }
    /// <p>The Amazon S3 object key where the certificate, certificate chain, and encrypted private key bundle are stored. The object key is formatted as follows: <code>role_arn</code>/<code>certificate_arn</code>.</p>
    pub fn get_certificate_s3_object_key(&self) -> &::std::option::Option<::std::string::String> {
        &self.certificate_s3_object_key
    }
    /// <p>The ID of the KMS key used to encrypt the private key of the certificate.</p>
    pub fn encryption_kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.encryption_kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the KMS key used to encrypt the private key of the certificate.</p>
    pub fn set_encryption_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.encryption_kms_key_id = input;
        self
    }
    /// <p>The ID of the KMS key used to encrypt the private key of the certificate.</p>
    pub fn get_encryption_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.encryption_kms_key_id
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`AssociateEnclaveCertificateIamRoleOutput`](crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleOutput).
    pub fn build(self) -> crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleOutput {
        crate::operation::associate_enclave_certificate_iam_role::AssociateEnclaveCertificateIamRoleOutput {
            certificate_s3_bucket_name: self.certificate_s3_bucket_name,
            certificate_s3_object_key: self.certificate_s3_object_key,
            encryption_kms_key_id: self.encryption_kms_key_id,
            _request_id: self._request_id,
        }
    }
}
