// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an IAM instance profile for a Scheduled Instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ScheduledInstancesIamInstanceProfile {
    /// <p>The Amazon Resource Name (ARN).</p>
    pub arn: ::std::option::Option<::std::string::String>,
    /// <p>The name.</p>
    pub name: ::std::option::Option<::std::string::String>,
}
impl ScheduledInstancesIamInstanceProfile {
    /// <p>The Amazon Resource Name (ARN).</p>
    pub fn arn(&self) -> ::std::option::Option<&str> {
        self.arn.as_deref()
    }
    /// <p>The name.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl ScheduledInstancesIamInstanceProfile {
    /// Creates a new builder-style object to manufacture [`ScheduledInstancesIamInstanceProfile`](crate::types::ScheduledInstancesIamInstanceProfile).
    pub fn builder() -> crate::types::builders::ScheduledInstancesIamInstanceProfileBuilder {
        crate::types::builders::ScheduledInstancesIamInstanceProfileBuilder::default()
    }
}

/// A builder for [`ScheduledInstancesIamInstanceProfile`](crate::types::ScheduledInstancesIamInstanceProfile).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ScheduledInstancesIamInstanceProfileBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl ScheduledInstancesIamInstanceProfileBuilder {
    /// <p>The Amazon Resource Name (ARN).</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN).</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN).</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The name.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Consumes the builder and constructs a [`ScheduledInstancesIamInstanceProfile`](crate::types::ScheduledInstancesIamInstanceProfile).
    pub fn build(self) -> crate::types::ScheduledInstancesIamInstanceProfile {
        crate::types::ScheduledInstancesIamInstanceProfile {
            arn: self.arn,
            name: self.name,
        }
    }
}
