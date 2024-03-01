// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the output of CancelSpotFleetRequests.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CancelSpotFleetRequestsOutput {
    /// <p>Information about the Spot Fleet requests that are successfully canceled.</p>
    pub successful_fleet_requests: ::std::option::Option<::std::vec::Vec<crate::types::CancelSpotFleetRequestsSuccessItem>>,
    /// <p>Information about the Spot Fleet requests that are not successfully canceled.</p>
    pub unsuccessful_fleet_requests: ::std::option::Option<::std::vec::Vec<crate::types::CancelSpotFleetRequestsErrorItem>>,
    _request_id: Option<String>,
}
impl CancelSpotFleetRequestsOutput {
    /// <p>Information about the Spot Fleet requests that are successfully canceled.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.successful_fleet_requests.is_none()`.
    pub fn successful_fleet_requests(&self) -> &[crate::types::CancelSpotFleetRequestsSuccessItem] {
        self.successful_fleet_requests.as_deref().unwrap_or_default()
    }
    /// <p>Information about the Spot Fleet requests that are not successfully canceled.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.unsuccessful_fleet_requests.is_none()`.
    pub fn unsuccessful_fleet_requests(&self) -> &[crate::types::CancelSpotFleetRequestsErrorItem] {
        self.unsuccessful_fleet_requests.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for CancelSpotFleetRequestsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl CancelSpotFleetRequestsOutput {
    /// Creates a new builder-style object to manufacture [`CancelSpotFleetRequestsOutput`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput).
    pub fn builder() -> crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsOutputBuilder {
        crate::operation::cancel_spot_fleet_requests::builders::CancelSpotFleetRequestsOutputBuilder::default()
    }
}

/// A builder for [`CancelSpotFleetRequestsOutput`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CancelSpotFleetRequestsOutputBuilder {
    pub(crate) successful_fleet_requests: ::std::option::Option<::std::vec::Vec<crate::types::CancelSpotFleetRequestsSuccessItem>>,
    pub(crate) unsuccessful_fleet_requests: ::std::option::Option<::std::vec::Vec<crate::types::CancelSpotFleetRequestsErrorItem>>,
    _request_id: Option<String>,
}
impl CancelSpotFleetRequestsOutputBuilder {
    /// Appends an item to `successful_fleet_requests`.
    ///
    /// To override the contents of this collection use [`set_successful_fleet_requests`](Self::set_successful_fleet_requests).
    ///
    /// <p>Information about the Spot Fleet requests that are successfully canceled.</p>
    pub fn successful_fleet_requests(mut self, input: crate::types::CancelSpotFleetRequestsSuccessItem) -> Self {
        let mut v = self.successful_fleet_requests.unwrap_or_default();
        v.push(input);
        self.successful_fleet_requests = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the Spot Fleet requests that are successfully canceled.</p>
    pub fn set_successful_fleet_requests(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CancelSpotFleetRequestsSuccessItem>>,
    ) -> Self {
        self.successful_fleet_requests = input;
        self
    }
    /// <p>Information about the Spot Fleet requests that are successfully canceled.</p>
    pub fn get_successful_fleet_requests(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CancelSpotFleetRequestsSuccessItem>> {
        &self.successful_fleet_requests
    }
    /// Appends an item to `unsuccessful_fleet_requests`.
    ///
    /// To override the contents of this collection use [`set_unsuccessful_fleet_requests`](Self::set_unsuccessful_fleet_requests).
    ///
    /// <p>Information about the Spot Fleet requests that are not successfully canceled.</p>
    pub fn unsuccessful_fleet_requests(mut self, input: crate::types::CancelSpotFleetRequestsErrorItem) -> Self {
        let mut v = self.unsuccessful_fleet_requests.unwrap_or_default();
        v.push(input);
        self.unsuccessful_fleet_requests = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the Spot Fleet requests that are not successfully canceled.</p>
    pub fn set_unsuccessful_fleet_requests(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CancelSpotFleetRequestsErrorItem>>,
    ) -> Self {
        self.unsuccessful_fleet_requests = input;
        self
    }
    /// <p>Information about the Spot Fleet requests that are not successfully canceled.</p>
    pub fn get_unsuccessful_fleet_requests(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CancelSpotFleetRequestsErrorItem>> {
        &self.unsuccessful_fleet_requests
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`CancelSpotFleetRequestsOutput`](crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput).
    pub fn build(self) -> crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput {
        crate::operation::cancel_spot_fleet_requests::CancelSpotFleetRequestsOutput {
            successful_fleet_requests: self.successful_fleet_requests,
            unsuccessful_fleet_requests: self.unsuccessful_fleet_requests,
            _request_id: self._request_id,
        }
    }
}