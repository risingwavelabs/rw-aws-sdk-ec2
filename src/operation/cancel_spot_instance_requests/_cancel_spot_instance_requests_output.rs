// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of CancelSpotInstanceRequests.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelSpotInstanceRequestsOutput {
    /// <p>The Spot Instance requests.</p>
    pub cancelled_spot_instance_requests: ::std::option::Option<::std::vec::Vec<crate::types::CancelledSpotInstanceRequest>>,
    _request_id: Option<String>,
}
impl CancelSpotInstanceRequestsOutput {
    /// <p>The Spot Instance requests.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.cancelled_spot_instance_requests.is_none()`.
    pub fn cancelled_spot_instance_requests(&self) -> &[crate::types::CancelledSpotInstanceRequest] {
        self.cancelled_spot_instance_requests.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for CancelSpotInstanceRequestsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CancelSpotInstanceRequestsOutput {
    /// Creates a new builder-style object to manufacture [`CancelSpotInstanceRequestsOutput`](crate::operation::cancel_spot_instance_requests::CancelSpotInstanceRequestsOutput).
    pub fn builder() -> crate::operation::cancel_spot_instance_requests::builders::CancelSpotInstanceRequestsOutputBuilder {
        crate::operation::cancel_spot_instance_requests::builders::CancelSpotInstanceRequestsOutputBuilder::default()
    }
}

/// A builder for [`CancelSpotInstanceRequestsOutput`](crate::operation::cancel_spot_instance_requests::CancelSpotInstanceRequestsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CancelSpotInstanceRequestsOutputBuilder {
    pub(crate) cancelled_spot_instance_requests: ::std::option::Option<::std::vec::Vec<crate::types::CancelledSpotInstanceRequest>>,
    _request_id: Option<String>,
}
impl CancelSpotInstanceRequestsOutputBuilder {
    /// Appends an item to `cancelled_spot_instance_requests`.
    ///
    /// To override the contents of this collection use [`set_cancelled_spot_instance_requests`](Self::set_cancelled_spot_instance_requests).
    ///
    /// <p>The Spot Instance requests.</p>
    pub fn cancelled_spot_instance_requests(mut self, input: crate::types::CancelledSpotInstanceRequest) -> Self {
        let mut v = self.cancelled_spot_instance_requests.unwrap_or_default();
        v.push(input);
        self.cancelled_spot_instance_requests = ::std::option::Option::Some(v);
        self
    }
    /// <p>The Spot Instance requests.</p>
    pub fn set_cancelled_spot_instance_requests(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CancelledSpotInstanceRequest>>,
    ) -> Self {
        self.cancelled_spot_instance_requests = input;
        self
    }
    /// <p>The Spot Instance requests.</p>
    pub fn get_cancelled_spot_instance_requests(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CancelledSpotInstanceRequest>> {
        &self.cancelled_spot_instance_requests
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CancelSpotInstanceRequestsOutput`](crate::operation::cancel_spot_instance_requests::CancelSpotInstanceRequestsOutput).
    pub fn build(self) -> crate::operation::cancel_spot_instance_requests::CancelSpotInstanceRequestsOutput {
        crate::operation::cancel_spot_instance_requests::CancelSpotInstanceRequestsOutput {
            cancelled_spot_instance_requests: self.cancelled_spot_instance_requests,
            _request_id: self._request_id,
        }
    }
}
