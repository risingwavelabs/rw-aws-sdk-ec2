// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for BundleInstance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BundleInstanceInput {
    /// <p>The ID of the instance to bundle.</p>
    /// <p>Type: String</p>
    /// <p>Default: None</p>
    /// <p>Required: Yes</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    pub storage: ::std::option::Option<crate::types::Storage>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl BundleInstanceInput {
    /// <p>The ID of the instance to bundle.</p>
    /// <p>Type: String</p>
    /// <p>Default: None</p>
    /// <p>Required: Yes</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    pub fn storage(&self) -> ::std::option::Option<&crate::types::Storage> {
        self.storage.as_ref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl BundleInstanceInput {
    /// Creates a new builder-style object to manufacture [`BundleInstanceInput`](crate::operation::bundle_instance::BundleInstanceInput).
    pub fn builder() -> crate::operation::bundle_instance::builders::BundleInstanceInputBuilder {
        crate::operation::bundle_instance::builders::BundleInstanceInputBuilder::default()
    }
}

/// A builder for [`BundleInstanceInput`](crate::operation::bundle_instance::BundleInstanceInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct BundleInstanceInputBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) storage: ::std::option::Option<crate::types::Storage>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl BundleInstanceInputBuilder {
    /// <p>The ID of the instance to bundle.</p>
    /// <p>Type: String</p>
    /// <p>Default: None</p>
    /// <p>Required: Yes</p>
    /// This field is required.
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the instance to bundle.</p>
    /// <p>Type: String</p>
    /// <p>Default: None</p>
    /// <p>Required: Yes</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The ID of the instance to bundle.</p>
    /// <p>Type: String</p>
    /// <p>Default: None</p>
    /// <p>Required: Yes</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_id
    }
    /// <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    /// This field is required.
    pub fn storage(mut self, input: crate::types::Storage) -> Self {
        self.storage = ::std::option::Option::Some(input);
        self
    }
    /// <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    pub fn set_storage(mut self, input: ::std::option::Option<crate::types::Storage>) -> Self {
        self.storage = input;
        self
    }
    /// <p>The bucket in which to store the AMI. You can specify a bucket that you already own or a new bucket that Amazon EC2 creates on your behalf. If you specify a bucket that belongs to someone else, Amazon EC2 returns an error.</p>
    pub fn get_storage(&self) -> &::std::option::Option<crate::types::Storage> {
        &self.storage
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
    /// Consumes the builder and constructs a [`BundleInstanceInput`](crate::operation::bundle_instance::BundleInstanceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::bundle_instance::BundleInstanceInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::bundle_instance::BundleInstanceInput {
            instance_id: self.instance_id,
            storage: self.storage,
            dry_run: self.dry_run,
        })
    }
}