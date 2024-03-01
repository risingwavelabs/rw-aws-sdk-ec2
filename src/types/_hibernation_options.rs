// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Indicates whether your instance is configured for hibernation. This parameter is valid only if the instance meets the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/hibernating-prerequisites.html">hibernation prerequisites</a>. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Hibernate.html">Hibernate your instance</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct HibernationOptions {
    /// <p>If <code>true</code>, your instance is enabled for hibernation; otherwise, it is not enabled for hibernation.</p>
    pub configured: ::std::option::Option<bool>,
}
impl HibernationOptions {
    /// <p>If <code>true</code>, your instance is enabled for hibernation; otherwise, it is not enabled for hibernation.</p>
    pub fn configured(&self) -> ::std::option::Option<bool> {
        self.configured
    }
}
impl HibernationOptions {
    /// Creates a new builder-style object to manufacture [`HibernationOptions`](crate::types::HibernationOptions).
    pub fn builder() -> crate::types::builders::HibernationOptionsBuilder {
        crate::types::builders::HibernationOptionsBuilder::default()
    }
}

/// A builder for [`HibernationOptions`](crate::types::HibernationOptions).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct HibernationOptionsBuilder {
    pub(crate) configured: ::std::option::Option<bool>,
}
impl HibernationOptionsBuilder {
    /// <p>If <code>true</code>, your instance is enabled for hibernation; otherwise, it is not enabled for hibernation.</p>
    pub fn configured(mut self, input: bool) -> Self {
        self.configured = ::std::option::Option::Some(input);
        self
    }
    /// <p>If <code>true</code>, your instance is enabled for hibernation; otherwise, it is not enabled for hibernation.</p>
    pub fn set_configured(mut self, input: ::std::option::Option<bool>) -> Self {
        self.configured = input;
        self
    }
    /// <p>If <code>true</code>, your instance is enabled for hibernation; otherwise, it is not enabled for hibernation.</p>
    pub fn get_configured(&self) -> &::std::option::Option<bool> {
        &self.configured
    }
    /// Consumes the builder and constructs a [`HibernationOptions`](crate::types::HibernationOptions).
    pub fn build(self) -> crate::types::HibernationOptions {
        crate::types::HibernationOptions { configured: self.configured }
    }
}