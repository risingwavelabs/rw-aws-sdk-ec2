// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeInstanceTypeOfferingsOutput {
    /// <p>The instance types offered.</p>
    pub instance_type_offerings: ::std::option::Option<::std::vec::Vec<crate::types::InstanceTypeOffering>>,
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeInstanceTypeOfferingsOutput {
    /// <p>The instance types offered.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.instance_type_offerings.is_none()`.
    pub fn instance_type_offerings(&self) -> &[crate::types::InstanceTypeOffering] {
        self.instance_type_offerings.as_deref().unwrap_or_default()
    }
    /// <p>The token to include in another request to get the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeInstanceTypeOfferingsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeInstanceTypeOfferingsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeInstanceTypeOfferingsOutput`](crate::operation::describe_instance_type_offerings::DescribeInstanceTypeOfferingsOutput).
    pub fn builder() -> crate::operation::describe_instance_type_offerings::builders::DescribeInstanceTypeOfferingsOutputBuilder {
        crate::operation::describe_instance_type_offerings::builders::DescribeInstanceTypeOfferingsOutputBuilder::default()
    }
}

/// A builder for [`DescribeInstanceTypeOfferingsOutput`](crate::operation::describe_instance_type_offerings::DescribeInstanceTypeOfferingsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeInstanceTypeOfferingsOutputBuilder {
    pub(crate) instance_type_offerings: ::std::option::Option<::std::vec::Vec<crate::types::InstanceTypeOffering>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeInstanceTypeOfferingsOutputBuilder {
    /// Appends an item to `instance_type_offerings`.
    ///
    /// To override the contents of this collection use [`set_instance_type_offerings`](Self::set_instance_type_offerings).
    ///
    /// <p>The instance types offered.</p>
    pub fn instance_type_offerings(mut self, input: crate::types::InstanceTypeOffering) -> Self {
        let mut v = self.instance_type_offerings.unwrap_or_default();
        v.push(input);
        self.instance_type_offerings = ::std::option::Option::Some(v);
        self
    }
    /// <p>The instance types offered.</p>
    pub fn set_instance_type_offerings(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceTypeOffering>>) -> Self {
        self.instance_type_offerings = input;
        self
    }
    /// <p>The instance types offered.</p>
    pub fn get_instance_type_offerings(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceTypeOffering>> {
        &self.instance_type_offerings
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
    /// Consumes the builder and constructs a [`DescribeInstanceTypeOfferingsOutput`](crate::operation::describe_instance_type_offerings::DescribeInstanceTypeOfferingsOutput).
    pub fn build(self) -> crate::operation::describe_instance_type_offerings::DescribeInstanceTypeOfferingsOutput {
        crate::operation::describe_instance_type_offerings::DescribeInstanceTypeOfferingsOutput {
            instance_type_offerings: self.instance_type_offerings,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
