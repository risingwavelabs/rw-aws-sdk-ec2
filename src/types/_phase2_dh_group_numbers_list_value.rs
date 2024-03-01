// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Diffie-Hellmann group number for phase 2 IKE negotiations.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Phase2DhGroupNumbersListValue {
    /// <p>The Diffie-Hellmann group number.</p>
    pub value: ::std::option::Option<i32>,
}
impl Phase2DhGroupNumbersListValue {
    /// <p>The Diffie-Hellmann group number.</p>
    pub fn value(&self) -> ::std::option::Option<i32> {
        self.value
    }
}
impl Phase2DhGroupNumbersListValue {
    /// Creates a new builder-style object to manufacture [`Phase2DhGroupNumbersListValue`](crate::types::Phase2DhGroupNumbersListValue).
    pub fn builder() -> crate::types::builders::Phase2DhGroupNumbersListValueBuilder {
        crate::types::builders::Phase2DhGroupNumbersListValueBuilder::default()
    }
}

/// A builder for [`Phase2DhGroupNumbersListValue`](crate::types::Phase2DhGroupNumbersListValue).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct Phase2DhGroupNumbersListValueBuilder {
    pub(crate) value: ::std::option::Option<i32>,
}
impl Phase2DhGroupNumbersListValueBuilder {
    /// <p>The Diffie-Hellmann group number.</p>
    pub fn value(mut self, input: i32) -> Self {
        self.value = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Diffie-Hellmann group number.</p>
    pub fn set_value(mut self, input: ::std::option::Option<i32>) -> Self {
        self.value = input;
        self
    }
    /// <p>The Diffie-Hellmann group number.</p>
    pub fn get_value(&self) -> &::std::option::Option<i32> {
        &self.value
    }
    /// Consumes the builder and constructs a [`Phase2DhGroupNumbersListValue`](crate::types::Phase2DhGroupNumbersListValue).
    pub fn build(self) -> crate::types::Phase2DhGroupNumbersListValue {
        crate::types::Phase2DhGroupNumbersListValue { value: self.value }
    }
}
