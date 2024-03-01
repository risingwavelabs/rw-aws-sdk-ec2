// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeAddressesOutput {
    /// <p>Information about the Elastic IP addresses.</p>
    pub addresses: ::std::option::Option<::std::vec::Vec<crate::types::Address>>,
    _request_id: Option<String>,
}
impl DescribeAddressesOutput {
    /// <p>Information about the Elastic IP addresses.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.addresses.is_none()`.
    pub fn addresses(&self) -> &[crate::types::Address] {
        self.addresses.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DescribeAddressesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeAddressesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeAddressesOutput`](crate::operation::describe_addresses::DescribeAddressesOutput).
    pub fn builder() -> crate::operation::describe_addresses::builders::DescribeAddressesOutputBuilder {
        crate::operation::describe_addresses::builders::DescribeAddressesOutputBuilder::default()
    }
}

/// A builder for [`DescribeAddressesOutput`](crate::operation::describe_addresses::DescribeAddressesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeAddressesOutputBuilder {
    pub(crate) addresses: ::std::option::Option<::std::vec::Vec<crate::types::Address>>,
    _request_id: Option<String>,
}
impl DescribeAddressesOutputBuilder {
    /// Appends an item to `addresses`.
    ///
    /// To override the contents of this collection use [`set_addresses`](Self::set_addresses).
    ///
    /// <p>Information about the Elastic IP addresses.</p>
    pub fn addresses(mut self, input: crate::types::Address) -> Self {
        let mut v = self.addresses.unwrap_or_default();
        v.push(input);
        self.addresses = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the Elastic IP addresses.</p>
    pub fn set_addresses(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Address>>) -> Self {
        self.addresses = input;
        self
    }
    /// <p>Information about the Elastic IP addresses.</p>
    pub fn get_addresses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Address>> {
        &self.addresses
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeAddressesOutput`](crate::operation::describe_addresses::DescribeAddressesOutput).
    pub fn build(self) -> crate::operation::describe_addresses::DescribeAddressesOutput {
        crate::operation::describe_addresses::DescribeAddressesOutput {
            addresses: self.addresses,
            _request_id: self._request_id,
        }
    }
}
