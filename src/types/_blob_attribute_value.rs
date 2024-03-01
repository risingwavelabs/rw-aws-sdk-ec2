// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BlobAttributeValue {
    #[allow(missing_docs)] // documentation missing in model
    pub value: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl BlobAttributeValue {
    #[allow(missing_docs)] // documentation missing in model
    pub fn value(&self) -> ::std::option::Option<&::aws_smithy_types::Blob> {
        self.value.as_ref()
    }
}
impl BlobAttributeValue {
    /// Creates a new builder-style object to manufacture [`BlobAttributeValue`](crate::types::BlobAttributeValue).
    pub fn builder() -> crate::types::builders::BlobAttributeValueBuilder {
        crate::types::builders::BlobAttributeValueBuilder::default()
    }
}

/// A builder for [`BlobAttributeValue`](crate::types::BlobAttributeValue).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct BlobAttributeValueBuilder {
    pub(crate) value: ::std::option::Option<::aws_smithy_types::Blob>,
}
impl BlobAttributeValueBuilder {
    #[allow(missing_docs)] // documentation missing in model
    pub fn value(mut self, input: ::aws_smithy_types::Blob) -> Self {
        self.value = ::std::option::Option::Some(input);
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn set_value(mut self, input: ::std::option::Option<::aws_smithy_types::Blob>) -> Self {
        self.value = input;
        self
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn get_value(&self) -> &::std::option::Option<::aws_smithy_types::Blob> {
        &self.value
    }
    /// Consumes the builder and constructs a [`BlobAttributeValue`](crate::types::BlobAttributeValue).
    pub fn build(self) -> crate::types::BlobAttributeValue {
        crate::types::BlobAttributeValue { value: self.value }
    }
}
