// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeFlowLogsOutput {
    /// <p>Information about the flow logs.</p>
    pub flow_logs: ::std::option::Option<::std::vec::Vec<crate::types::FlowLog>>,
    /// <p>The token to request the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeFlowLogsOutput {
    /// <p>Information about the flow logs.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.flow_logs.is_none()`.
    pub fn flow_logs(&self) -> &[crate::types::FlowLog] {
        self.flow_logs.as_deref().unwrap_or_default()
    }
    /// <p>The token to request the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeFlowLogsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeFlowLogsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeFlowLogsOutput`](crate::operation::describe_flow_logs::DescribeFlowLogsOutput).
    pub fn builder() -> crate::operation::describe_flow_logs::builders::DescribeFlowLogsOutputBuilder {
        crate::operation::describe_flow_logs::builders::DescribeFlowLogsOutputBuilder::default()
    }
}

/// A builder for [`DescribeFlowLogsOutput`](crate::operation::describe_flow_logs::DescribeFlowLogsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeFlowLogsOutputBuilder {
    pub(crate) flow_logs: ::std::option::Option<::std::vec::Vec<crate::types::FlowLog>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeFlowLogsOutputBuilder {
    /// Appends an item to `flow_logs`.
    ///
    /// To override the contents of this collection use [`set_flow_logs`](Self::set_flow_logs).
    ///
    /// <p>Information about the flow logs.</p>
    pub fn flow_logs(mut self, input: crate::types::FlowLog) -> Self {
        let mut v = self.flow_logs.unwrap_or_default();
        v.push(input);
        self.flow_logs = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the flow logs.</p>
    pub fn set_flow_logs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FlowLog>>) -> Self {
        self.flow_logs = input;
        self
    }
    /// <p>Information about the flow logs.</p>
    pub fn get_flow_logs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FlowLog>> {
        &self.flow_logs
    }
    /// <p>The token to request the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to request the next page of items. This value is <code>null</code> when there are no more items to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to request the next page of items. This value is <code>null</code> when there are no more items to return.</p>
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
    /// Consumes the builder and constructs a [`DescribeFlowLogsOutput`](crate::operation::describe_flow_logs::DescribeFlowLogsOutput).
    pub fn build(self) -> crate::operation::describe_flow_logs::DescribeFlowLogsOutput {
        crate::operation::describe_flow_logs::DescribeFlowLogsOutput {
            flow_logs: self.flow_logs,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
