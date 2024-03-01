// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The error codes and error messages that are returned for the parameters or parameter combinations that are not valid when a new launch template or new version of a launch template is created.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ValidationWarning {
    /// <p>The error codes and error messages.</p>
    pub errors: ::std::option::Option<::std::vec::Vec<crate::types::ValidationError>>,
}
impl ValidationWarning {
    /// <p>The error codes and error messages.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.errors.is_none()`.
    pub fn errors(&self) -> &[crate::types::ValidationError] {
        self.errors.as_deref().unwrap_or_default()
    }
}
impl ValidationWarning {
    /// Creates a new builder-style object to manufacture [`ValidationWarning`](crate::types::ValidationWarning).
    pub fn builder() -> crate::types::builders::ValidationWarningBuilder {
        crate::types::builders::ValidationWarningBuilder::default()
    }
}

/// A builder for [`ValidationWarning`](crate::types::ValidationWarning).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ValidationWarningBuilder {
    pub(crate) errors: ::std::option::Option<::std::vec::Vec<crate::types::ValidationError>>,
}
impl ValidationWarningBuilder {
    /// Appends an item to `errors`.
    ///
    /// To override the contents of this collection use [`set_errors`](Self::set_errors).
    ///
    /// <p>The error codes and error messages.</p>
    pub fn errors(mut self, input: crate::types::ValidationError) -> Self {
        let mut v = self.errors.unwrap_or_default();
        v.push(input);
        self.errors = ::std::option::Option::Some(v);
        self
    }
    /// <p>The error codes and error messages.</p>
    pub fn set_errors(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ValidationError>>) -> Self {
        self.errors = input;
        self
    }
    /// <p>The error codes and error messages.</p>
    pub fn get_errors(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ValidationError>> {
        &self.errors
    }
    /// Consumes the builder and constructs a [`ValidationWarning`](crate::types::ValidationWarning).
    pub fn build(self) -> crate::types::ValidationWarning {
        crate::types::ValidationWarning { errors: self.errors }
    }
}
