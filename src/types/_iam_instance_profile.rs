// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an IAM instance profile.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IamInstanceProfile {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the instance profile.</p>
    pub id: ::std::option::Option<::std::string::String>,
}
impl IamInstanceProfile {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The ID of the instance profile.</p>
    pub fn id(&self) -> ::std::option::Option<&str> {
        self.id.as_deref()
    }
}
impl IamInstanceProfile {
    /// Creates a new builder-style object to manufacture [`IamInstanceProfile`](crate::types::IamInstanceProfile).
    pub fn builder() -> crate::types::builders::IamInstanceProfileBuilder {
        crate::types::builders::IamInstanceProfileBuilder::default()
    }
}

/// A builder for [`IamInstanceProfile`](crate::types::IamInstanceProfile).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct IamInstanceProfileBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) id: ::std::option::Option<::std::string::String>,
}
impl IamInstanceProfileBuilder {
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the instance profile.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The ID of the instance profile.</p>
    pub fn id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the instance profile.</p>
    pub fn set_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.id = input;
        self
    }
    /// <p>The ID of the instance profile.</p>
    pub fn get_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.id
    }
    /// Consumes the builder and constructs a [`IamInstanceProfile`](crate::types::IamInstanceProfile).
    pub fn build(self) -> crate::types::IamInstanceProfile {
        crate::types::IamInstanceProfile { arn: self.arn, id: self.id }
    }
}
