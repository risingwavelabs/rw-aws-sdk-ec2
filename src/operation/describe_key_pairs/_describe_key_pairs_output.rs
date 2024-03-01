// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeKeyPairsOutput {
    /// <p>Information about the key pairs.</p>
    pub key_pairs: ::std::option::Option<::std::vec::Vec<crate::types::KeyPairInfo>>,
    _request_id: Option<String>,
}
impl DescribeKeyPairsOutput {
    /// <p>Information about the key pairs.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.key_pairs.is_none()`.
    pub fn key_pairs(&self) -> &[crate::types::KeyPairInfo] {
        self.key_pairs.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DescribeKeyPairsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeKeyPairsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeKeyPairsOutput`](crate::operation::describe_key_pairs::DescribeKeyPairsOutput).
    pub fn builder() -> crate::operation::describe_key_pairs::builders::DescribeKeyPairsOutputBuilder {
        crate::operation::describe_key_pairs::builders::DescribeKeyPairsOutputBuilder::default()
    }
}

/// A builder for [`DescribeKeyPairsOutput`](crate::operation::describe_key_pairs::DescribeKeyPairsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeKeyPairsOutputBuilder {
    pub(crate) key_pairs: ::std::option::Option<::std::vec::Vec<crate::types::KeyPairInfo>>,
    _request_id: Option<String>,
}
impl DescribeKeyPairsOutputBuilder {
    /// Appends an item to `key_pairs`.
    ///
    /// To override the contents of this collection use [`set_key_pairs`](Self::set_key_pairs).
    ///
    /// <p>Information about the key pairs.</p>
    pub fn key_pairs(mut self, input: crate::types::KeyPairInfo) -> Self {
        let mut v = self.key_pairs.unwrap_or_default();
        v.push(input);
        self.key_pairs = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the key pairs.</p>
    pub fn set_key_pairs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::KeyPairInfo>>) -> Self {
        self.key_pairs = input;
        self
    }
    /// <p>Information about the key pairs.</p>
    pub fn get_key_pairs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::KeyPairInfo>> {
        &self.key_pairs
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeKeyPairsOutput`](crate::operation::describe_key_pairs::DescribeKeyPairsOutput).
    pub fn build(self) -> crate::operation::describe_key_pairs::DescribeKeyPairsOutput {
        crate::operation::describe_key_pairs::DescribeKeyPairsOutput {
            key_pairs: self.key_pairs,
            _request_id: self._request_id,
        }
    }
}
