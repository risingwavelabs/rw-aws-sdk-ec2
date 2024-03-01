// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeInstancesOutput {
    /// <p>Information about the reservations.</p>
    pub reservations: ::std::option::Option<::std::vec::Vec<crate::types::Reservation>>,
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeInstancesOutput {
    /// <p>Information about the reservations.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.reservations.is_none()`.
    pub fn reservations(&self) -> &[crate::types::Reservation] {
        self.reservations.as_deref().unwrap_or_default()
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeInstancesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeInstancesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeInstancesOutput`](crate::operation::describe_instances::DescribeInstancesOutput).
    pub fn builder() -> crate::operation::describe_instances::builders::DescribeInstancesOutputBuilder {
        crate::operation::describe_instances::builders::DescribeInstancesOutputBuilder::default()
    }
}

/// A builder for [`DescribeInstancesOutput`](crate::operation::describe_instances::DescribeInstancesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeInstancesOutputBuilder {
    pub(crate) reservations: ::std::option::Option<::std::vec::Vec<crate::types::Reservation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeInstancesOutputBuilder {
    /// Appends an item to `reservations`.
    ///
    /// To override the contents of this collection use [`set_reservations`](Self::set_reservations).
    ///
    /// <p>Information about the reservations.</p>
    pub fn reservations(mut self, input: crate::types::Reservation) -> Self {
        let mut v = self.reservations.unwrap_or_default();
        v.push(input);
        self.reservations = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the reservations.</p>
    pub fn set_reservations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Reservation>>) -> Self {
        self.reservations = input;
        self
    }
    /// <p>Information about the reservations.</p>
    pub fn get_reservations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Reservation>> {
        &self.reservations
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeInstancesOutput`](crate::operation::describe_instances::DescribeInstancesOutput).
    pub fn build(self) -> crate::operation::describe_instances::DescribeInstancesOutput {
        crate::operation::describe_instances::DescribeInstancesOutput {
            reservations: self.reservations,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}