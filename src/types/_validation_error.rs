// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The error code and error message that is returned for a parameter or parameter combination that is not valid when a new launch template or new version of a launch template is created.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ValidationError {
    /// <p>The error code that indicates why the parameter or parameter combination is not valid. For more information about error codes, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub code: ::std::option::Option<::std::string::String>,
    /// <p>The error message that describes why the parameter or parameter combination is not valid. For more information about error messages, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl ValidationError {
    /// <p>The error code that indicates why the parameter or parameter combination is not valid. For more information about error codes, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub fn code(&self) -> ::std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>The error message that describes why the parameter or parameter combination is not valid. For more information about error messages, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl ValidationError {
    /// Creates a new builder-style object to manufacture [`ValidationError`](crate::types::ValidationError).
    pub fn builder() -> crate::types::builders::ValidationErrorBuilder {
        crate::types::builders::ValidationErrorBuilder::default()
    }
}

/// A builder for [`ValidationError`](crate::types::ValidationError).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ValidationErrorBuilder {
    pub(crate) code: ::std::option::Option<::std::string::String>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl ValidationErrorBuilder {
    /// <p>The error code that indicates why the parameter or parameter combination is not valid. For more information about error codes, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub fn code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The error code that indicates why the parameter or parameter combination is not valid. For more information about error codes, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub fn set_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>The error code that indicates why the parameter or parameter combination is not valid. For more information about error codes, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub fn get_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.code
    }
    /// <p>The error message that describes why the parameter or parameter combination is not valid. For more information about error messages, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The error message that describes why the parameter or parameter combination is not valid. For more information about error messages, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The error message that describes why the parameter or parameter combination is not valid. For more information about error messages, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">Error codes</a>.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Consumes the builder and constructs a [`ValidationError`](crate::types::ValidationError).
    pub fn build(self) -> crate::types::ValidationError {
        crate::types::ValidationError {
            code: self.code,
            message: self.message,
        }
    }
}