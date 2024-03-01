// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssociateIamInstanceProfileInput {
    /// <p>The IAM instance profile.</p>
    pub iam_instance_profile: ::std::option::Option<crate::types::IamInstanceProfileSpecification>,
    /// <p>The ID of the instance.</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
}
impl AssociateIamInstanceProfileInput {
    /// <p>The IAM instance profile.</p>
    pub fn iam_instance_profile(&self) -> ::std::option::Option<&crate::types::IamInstanceProfileSpecification> {
        self.iam_instance_profile.as_ref()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
}
impl AssociateIamInstanceProfileInput {
    /// Creates a new builder-style object to manufacture [`AssociateIamInstanceProfileInput`](crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileInput).
    pub fn builder() -> crate::operation::associate_iam_instance_profile::builders::AssociateIamInstanceProfileInputBuilder {
        crate::operation::associate_iam_instance_profile::builders::AssociateIamInstanceProfileInputBuilder::default()
    }
}

/// A builder for [`AssociateIamInstanceProfileInput`](crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AssociateIamInstanceProfileInputBuilder {
    pub(crate) iam_instance_profile: ::std::option::Option<crate::types::IamInstanceProfileSpecification>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
}
impl AssociateIamInstanceProfileInputBuilder {
    /// <p>The IAM instance profile.</p>
    /// This field is required.
    pub fn iam_instance_profile(mut self, input: crate::types::IamInstanceProfileSpecification) -> Self {
        self.iam_instance_profile = ::std::option::Option::Some(input);
        self
    }
    /// <p>The IAM instance profile.</p>
    pub fn set_iam_instance_profile(mut self, input: ::std::option::Option<crate::types::IamInstanceProfileSpecification>) -> Self {
        self.iam_instance_profile = input;
        self
    }
    /// <p>The IAM instance profile.</p>
    pub fn get_iam_instance_profile(&self) -> &::std::option::Option<crate::types::IamInstanceProfileSpecification> {
        &self.iam_instance_profile
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
    /// Consumes the builder and constructs a [`AssociateIamInstanceProfileInput`](crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::associate_iam_instance_profile::AssociateIamInstanceProfileInput {
            iam_instance_profile: self.iam_instance_profile,
            instance_id: self.instance_id,
        })
    }
}
