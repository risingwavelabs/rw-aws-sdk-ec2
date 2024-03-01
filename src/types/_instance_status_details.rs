// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the instance status.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceStatusDetails {
    /// <p>The time when a status check failed. For an instance that was launched and impaired, this is the time when the instance was launched.</p>
    pub impaired_since: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The type of instance status.</p>
    pub name: ::std::option::Option<crate::types::StatusName>,
    /// <p>The status.</p>
    pub status: ::std::option::Option<crate::types::StatusType>,
}
impl InstanceStatusDetails {
    /// <p>The time when a status check failed. For an instance that was launched and impaired, this is the time when the instance was launched.</p>
    pub fn impaired_since(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.impaired_since.as_ref()
    }
    /// <p>The type of instance status.</p>
    pub fn name(&self) -> ::std::option::Option<&crate::types::StatusName> {
        self.name.as_ref()
    }
    /// <p>The status.</p>
    pub fn status(&self) -> ::std::option::Option<&crate::types::StatusType> {
        self.status.as_ref()
    }
}
impl InstanceStatusDetails {
    /// Creates a new builder-style object to manufacture [`InstanceStatusDetails`](crate::types::InstanceStatusDetails).
    pub fn builder() -> crate::types::builders::InstanceStatusDetailsBuilder {
        crate::types::builders::InstanceStatusDetailsBuilder::default()
    }
}

/// A builder for [`InstanceStatusDetails`](crate::types::InstanceStatusDetails).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InstanceStatusDetailsBuilder {
    pub(crate) impaired_since: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) name: ::std::option::Option<crate::types::StatusName>,
    pub(crate) status: ::std::option::Option<crate::types::StatusType>,
}
impl InstanceStatusDetailsBuilder {
    /// <p>The time when a status check failed. For an instance that was launched and impaired, this is the time when the instance was launched.</p>
    pub fn impaired_since(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.impaired_since = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time when a status check failed. For an instance that was launched and impaired, this is the time when the instance was launched.</p>
    pub fn set_impaired_since(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.impaired_since = input;
        self
    }
    /// <p>The time when a status check failed. For an instance that was launched and impaired, this is the time when the instance was launched.</p>
    pub fn get_impaired_since(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.impaired_since
    }
    /// <p>The type of instance status.</p>
    pub fn name(mut self, input: crate::types::StatusName) -> Self {
        self.name = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of instance status.</p>
    pub fn set_name(mut self, input: ::std::option::Option<crate::types::StatusName>) -> Self {
        self.name = input;
        self
    }
    /// <p>The type of instance status.</p>
    pub fn get_name(&self) -> &::std::option::Option<crate::types::StatusName> {
        &self.name
    }
    /// <p>The status.</p>
    pub fn status(mut self, input: crate::types::StatusType) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::StatusType>) -> Self {
        self.status = input;
        self
    }
    /// <p>The status.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::StatusType> {
        &self.status
    }
    /// Consumes the builder and constructs a [`InstanceStatusDetails`](crate::types::InstanceStatusDetails).
    pub fn build(self) -> crate::types::InstanceStatusDetails {
        crate::types::InstanceStatusDetails {
            impaired_since: self.impaired_since,
            name: self.name,
            status: self.status,
        }
    }
}