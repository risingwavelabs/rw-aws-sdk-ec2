// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteFlowLogsOutput {
    /// <p>Information about the flow logs that could not be deleted successfully.</p>
    pub unsuccessful: ::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulItem>>,
    _request_id: Option<String>,
}
impl DeleteFlowLogsOutput {
    /// <p>Information about the flow logs that could not be deleted successfully.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.unsuccessful.is_none()`.
    pub fn unsuccessful(&self) -> &[crate::types::UnsuccessfulItem] {
        self.unsuccessful.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for DeleteFlowLogsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteFlowLogsOutput {
    /// Creates a new builder-style object to manufacture [`DeleteFlowLogsOutput`](crate::operation::delete_flow_logs::DeleteFlowLogsOutput).
    pub fn builder() -> crate::operation::delete_flow_logs::builders::DeleteFlowLogsOutputBuilder {
        crate::operation::delete_flow_logs::builders::DeleteFlowLogsOutputBuilder::default()
    }
}

/// A builder for [`DeleteFlowLogsOutput`](crate::operation::delete_flow_logs::DeleteFlowLogsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteFlowLogsOutputBuilder {
    pub(crate) unsuccessful: ::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulItem>>,
    _request_id: Option<String>,
}
impl DeleteFlowLogsOutputBuilder {
    /// Appends an item to `unsuccessful`.
    ///
    /// To override the contents of this collection use [`set_unsuccessful`](Self::set_unsuccessful).
    ///
    /// <p>Information about the flow logs that could not be deleted successfully.</p>
    pub fn unsuccessful(mut self, input: crate::types::UnsuccessfulItem) -> Self {
        let mut v = self.unsuccessful.unwrap_or_default();
        v.push(input);
        self.unsuccessful = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the flow logs that could not be deleted successfully.</p>
    pub fn set_unsuccessful(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulItem>>) -> Self {
        self.unsuccessful = input;
        self
    }
    /// <p>Information about the flow logs that could not be deleted successfully.</p>
    pub fn get_unsuccessful(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulItem>> {
        &self.unsuccessful
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteFlowLogsOutput`](crate::operation::delete_flow_logs::DeleteFlowLogsOutput).
    pub fn build(self) -> crate::operation::delete_flow_logs::DeleteFlowLogsOutput {
        crate::operation::delete_flow_logs::DeleteFlowLogsOutput {
            unsuccessful: self.unsuccessful,
            _request_id: self._request_id,
        }
    }
}