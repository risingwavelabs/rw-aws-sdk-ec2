// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ResetInstanceAttributeInput {
    /// <p>The attribute to reset.</p> <important>
    /// <p>You can only reset the following attributes: <code>kernel</code> | <code>ramdisk</code> | <code>sourceDestCheck</code>.</p>
    /// </important>
    pub attribute: ::std::option::Option<crate::types::InstanceAttributeName>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The ID of the instance.</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
}
impl ResetInstanceAttributeInput {
    /// <p>The attribute to reset.</p> <important>
    /// <p>You can only reset the following attributes: <code>kernel</code> | <code>ramdisk</code> | <code>sourceDestCheck</code>.</p>
    /// </important>
    pub fn attribute(&self) -> ::std::option::Option<&crate::types::InstanceAttributeName> {
        self.attribute.as_ref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
}
impl ResetInstanceAttributeInput {
    /// Creates a new builder-style object to manufacture [`ResetInstanceAttributeInput`](crate::operation::reset_instance_attribute::ResetInstanceAttributeInput).
    pub fn builder() -> crate::operation::reset_instance_attribute::builders::ResetInstanceAttributeInputBuilder {
        crate::operation::reset_instance_attribute::builders::ResetInstanceAttributeInputBuilder::default()
    }
}

/// A builder for [`ResetInstanceAttributeInput`](crate::operation::reset_instance_attribute::ResetInstanceAttributeInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ResetInstanceAttributeInputBuilder {
    pub(crate) attribute: ::std::option::Option<crate::types::InstanceAttributeName>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
}
impl ResetInstanceAttributeInputBuilder {
    /// <p>The attribute to reset.</p> <important>
    /// <p>You can only reset the following attributes: <code>kernel</code> | <code>ramdisk</code> | <code>sourceDestCheck</code>.</p>
    /// </important>
    /// This field is required.
    pub fn attribute(mut self, input: crate::types::InstanceAttributeName) -> Self {
        self.attribute = ::std::option::Option::Some(input);
        self
    }
    /// <p>The attribute to reset.</p> <important>
    /// <p>You can only reset the following attributes: <code>kernel</code> | <code>ramdisk</code> | <code>sourceDestCheck</code>.</p>
    /// </important>
    pub fn set_attribute(mut self, input: ::std::option::Option<crate::types::InstanceAttributeName>) -> Self {
        self.attribute = input;
        self
    }
    /// <p>The attribute to reset.</p> <important>
    /// <p>You can only reset the following attributes: <code>kernel</code> | <code>ramdisk</code> | <code>sourceDestCheck</code>.</p>
    /// </important>
    pub fn get_attribute(&self) -> &::std::option::Option<crate::types::InstanceAttributeName> {
        &self.attribute
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
    /// <p>The ID of the instance.</p>
    /// This field is required.
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_id
    }
    /// Consumes the builder and constructs a [`ResetInstanceAttributeInput`](crate::operation::reset_instance_attribute::ResetInstanceAttributeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::reset_instance_attribute::ResetInstanceAttributeInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::reset_instance_attribute::ResetInstanceAttributeInput {
            attribute: self.attribute,
            dry_run: self.dry_run,
            instance_id: self.instance_id,
        })
    }
}
