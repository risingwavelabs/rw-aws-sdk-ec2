// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateVerifiedAccessInstanceInput {
    /// <p>A description for the Verified Access instance.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The tags to assign to the Verified Access instance.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Enable or disable support for Federal Information Processing Standards (FIPS) on the instance.</p>
    pub fips_enabled: ::std::option::Option<bool>,
}
impl CreateVerifiedAccessInstanceInput {
    /// <p>A description for the Verified Access instance.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The tags to assign to the Verified Access instance.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Enable or disable support for Federal Information Processing Standards (FIPS) on the instance.</p>
    pub fn fips_enabled(&self) -> ::std::option::Option<bool> {
        self.fips_enabled
    }
}
impl CreateVerifiedAccessInstanceInput {
    /// Creates a new builder-style object to manufacture [`CreateVerifiedAccessInstanceInput`](crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceInput).
    pub fn builder() -> crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceInputBuilder {
        crate::operation::create_verified_access_instance::builders::CreateVerifiedAccessInstanceInputBuilder::default()
    }
}

/// A builder for [`CreateVerifiedAccessInstanceInput`](crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateVerifiedAccessInstanceInputBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) fips_enabled: ::std::option::Option<bool>,
}
impl CreateVerifiedAccessInstanceInputBuilder {
    /// <p>A description for the Verified Access instance.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the Verified Access instance.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description for the Verified Access instance.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to assign to the Verified Access instance.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to assign to the Verified Access instance.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to assign to the Verified Access instance.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>A unique, case-sensitive token that you provide to ensure idempotency of your modification request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
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
    /// <p>Enable or disable support for Federal Information Processing Standards (FIPS) on the instance.</p>
    pub fn fips_enabled(mut self, input: bool) -> Self {
        self.fips_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable support for Federal Information Processing Standards (FIPS) on the instance.</p>
    pub fn set_fips_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.fips_enabled = input;
        self
    }
    /// <p>Enable or disable support for Federal Information Processing Standards (FIPS) on the instance.</p>
    pub fn get_fips_enabled(&self) -> &::std::option::Option<bool> {
        &self.fips_enabled
    }
    /// Consumes the builder and constructs a [`CreateVerifiedAccessInstanceInput`](crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::create_verified_access_instance::CreateVerifiedAccessInstanceInput {
            description: self.description,
            tag_specifications: self.tag_specifications,
            client_token: self.client_token,
            dry_run: self.dry_run,
            fips_enabled: self.fips_enabled,
        })
    }
}
