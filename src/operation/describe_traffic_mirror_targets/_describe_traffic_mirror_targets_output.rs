// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeTrafficMirrorTargetsOutput {
    /// <p>Information about one or more Traffic Mirror targets.</p>
    pub traffic_mirror_targets: ::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorTarget>>,
    /// <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTrafficMirrorTargetsOutput {
    /// <p>Information about one or more Traffic Mirror targets.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.traffic_mirror_targets.is_none()`.
    pub fn traffic_mirror_targets(&self) -> &[crate::types::TrafficMirrorTarget] {
        self.traffic_mirror_targets.as_deref().unwrap_or_default()
    }
    /// <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeTrafficMirrorTargetsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeTrafficMirrorTargetsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeTrafficMirrorTargetsOutput`](crate::operation::describe_traffic_mirror_targets::DescribeTrafficMirrorTargetsOutput).
    pub fn builder() -> crate::operation::describe_traffic_mirror_targets::builders::DescribeTrafficMirrorTargetsOutputBuilder {
        crate::operation::describe_traffic_mirror_targets::builders::DescribeTrafficMirrorTargetsOutputBuilder::default()
    }
}

/// A builder for [`DescribeTrafficMirrorTargetsOutput`](crate::operation::describe_traffic_mirror_targets::DescribeTrafficMirrorTargetsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeTrafficMirrorTargetsOutputBuilder {
    pub(crate) traffic_mirror_targets: ::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorTarget>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeTrafficMirrorTargetsOutputBuilder {
    /// Appends an item to `traffic_mirror_targets`.
    ///
    /// To override the contents of this collection use [`set_traffic_mirror_targets`](Self::set_traffic_mirror_targets).
    ///
    /// <p>Information about one or more Traffic Mirror targets.</p>
    pub fn traffic_mirror_targets(mut self, input: crate::types::TrafficMirrorTarget) -> Self {
        let mut v = self.traffic_mirror_targets.unwrap_or_default();
        v.push(input);
        self.traffic_mirror_targets = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about one or more Traffic Mirror targets.</p>
    pub fn set_traffic_mirror_targets(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorTarget>>) -> Self {
        self.traffic_mirror_targets = input;
        self
    }
    /// <p>Information about one or more Traffic Mirror targets.</p>
    pub fn get_traffic_mirror_targets(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TrafficMirrorTarget>> {
        &self.traffic_mirror_targets
    }
    /// <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. The value is <code>null</code> when there are no more results to return.</p>
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
    /// Consumes the builder and constructs a [`DescribeTrafficMirrorTargetsOutput`](crate::operation::describe_traffic_mirror_targets::DescribeTrafficMirrorTargetsOutput).
    pub fn build(self) -> crate::operation::describe_traffic_mirror_targets::DescribeTrafficMirrorTargetsOutput {
        crate::operation::describe_traffic_mirror_targets::DescribeTrafficMirrorTargetsOutput {
            traffic_mirror_targets: self.traffic_mirror_targets,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
