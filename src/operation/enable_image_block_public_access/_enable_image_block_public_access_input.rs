// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EnableImageBlockPublicAccessInput {
    /// <p>Specify <code>block-new-sharing</code> to enable block public access for AMIs at the account level in the specified Region. This will block any attempt to publicly share your AMIs in the specified Region.</p>
    pub image_block_public_access_state: ::std::option::Option<crate::types::ImageBlockPublicAccessEnabledState>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl EnableImageBlockPublicAccessInput {
    /// <p>Specify <code>block-new-sharing</code> to enable block public access for AMIs at the account level in the specified Region. This will block any attempt to publicly share your AMIs in the specified Region.</p>
    pub fn image_block_public_access_state(&self) -> ::std::option::Option<&crate::types::ImageBlockPublicAccessEnabledState> {
        self.image_block_public_access_state.as_ref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl EnableImageBlockPublicAccessInput {
    /// Creates a new builder-style object to manufacture [`EnableImageBlockPublicAccessInput`](crate::operation::enable_image_block_public_access::EnableImageBlockPublicAccessInput).
    pub fn builder() -> crate::operation::enable_image_block_public_access::builders::EnableImageBlockPublicAccessInputBuilder {
        crate::operation::enable_image_block_public_access::builders::EnableImageBlockPublicAccessInputBuilder::default()
    }
}

/// A builder for [`EnableImageBlockPublicAccessInput`](crate::operation::enable_image_block_public_access::EnableImageBlockPublicAccessInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct EnableImageBlockPublicAccessInputBuilder {
    pub(crate) image_block_public_access_state: ::std::option::Option<crate::types::ImageBlockPublicAccessEnabledState>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl EnableImageBlockPublicAccessInputBuilder {
    /// <p>Specify <code>block-new-sharing</code> to enable block public access for AMIs at the account level in the specified Region. This will block any attempt to publicly share your AMIs in the specified Region.</p>
    /// This field is required.
    pub fn image_block_public_access_state(mut self, input: crate::types::ImageBlockPublicAccessEnabledState) -> Self {
        self.image_block_public_access_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specify <code>block-new-sharing</code> to enable block public access for AMIs at the account level in the specified Region. This will block any attempt to publicly share your AMIs in the specified Region.</p>
    pub fn set_image_block_public_access_state(mut self, input: ::std::option::Option<crate::types::ImageBlockPublicAccessEnabledState>) -> Self {
        self.image_block_public_access_state = input;
        self
    }
    /// <p>Specify <code>block-new-sharing</code> to enable block public access for AMIs at the account level in the specified Region. This will block any attempt to publicly share your AMIs in the specified Region.</p>
    pub fn get_image_block_public_access_state(&self) -> &::std::option::Option<crate::types::ImageBlockPublicAccessEnabledState> {
        &self.image_block_public_access_state
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
    /// Consumes the builder and constructs a [`EnableImageBlockPublicAccessInput`](crate::operation::enable_image_block_public_access::EnableImageBlockPublicAccessInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::enable_image_block_public_access::EnableImageBlockPublicAccessInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::enable_image_block_public_access::EnableImageBlockPublicAccessInput {
            image_block_public_access_state: self.image_block_public_access_state,
            dry_run: self.dry_run,
        })
    }
}