// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The options in use for server side encryption.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VerifiedAccessSseSpecificationResponse {
    /// <p>Indicates whether customer managed KMS keys are in use for server side encryption.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub customer_managed_key_enabled: ::std::option::Option<bool>,
    /// <p>The ARN of the KMS key.</p>
    pub kms_key_arn: ::std::option::Option<::std::string::String>,
}
impl VerifiedAccessSseSpecificationResponse {
    /// <p>Indicates whether customer managed KMS keys are in use for server side encryption.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn customer_managed_key_enabled(&self) -> ::std::option::Option<bool> {
        self.customer_managed_key_enabled
    }
    /// <p>The ARN of the KMS key.</p>
    pub fn kms_key_arn(&self) -> ::std::option::Option<&str> {
        self.kms_key_arn.as_deref()
    }
}
impl VerifiedAccessSseSpecificationResponse {
    /// Creates a new builder-style object to manufacture [`VerifiedAccessSseSpecificationResponse`](crate::types::VerifiedAccessSseSpecificationResponse).
    pub fn builder() -> crate::types::builders::VerifiedAccessSseSpecificationResponseBuilder {
        crate::types::builders::VerifiedAccessSseSpecificationResponseBuilder::default()
    }
}

/// A builder for [`VerifiedAccessSseSpecificationResponse`](crate::types::VerifiedAccessSseSpecificationResponse).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VerifiedAccessSseSpecificationResponseBuilder {
    pub(crate) customer_managed_key_enabled: ::std::option::Option<bool>,
    pub(crate) kms_key_arn: ::std::option::Option<::std::string::String>,
}
impl VerifiedAccessSseSpecificationResponseBuilder {
    /// <p>Indicates whether customer managed KMS keys are in use for server side encryption.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn customer_managed_key_enabled(mut self, input: bool) -> Self {
        self.customer_managed_key_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether customer managed KMS keys are in use for server side encryption.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn set_customer_managed_key_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.customer_managed_key_enabled = input;
        self
    }
    /// <p>Indicates whether customer managed KMS keys are in use for server side encryption.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn get_customer_managed_key_enabled(&self) -> &::std::option::Option<bool> {
        &self.customer_managed_key_enabled
    }
    /// <p>The ARN of the KMS key.</p>
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ARN of the KMS key.</p>
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_arn = input;
        self
    }
    /// <p>The ARN of the KMS key.</p>
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_arn
    }
    /// Consumes the builder and constructs a [`VerifiedAccessSseSpecificationResponse`](crate::types::VerifiedAccessSseSpecificationResponse).
    pub fn build(self) -> crate::types::VerifiedAccessSseSpecificationResponse {
        crate::types::VerifiedAccessSseSpecificationResponse {
            customer_managed_key_enabled: self.customer_managed_key_enabled,
            kms_key_arn: self.kms_key_arn,
        }
    }
}
