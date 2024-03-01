// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StopInstancesOutput {
    /// <p>Information about the stopped instances.</p>
    pub stopping_instances: ::std::option::Option<::std::vec::Vec<crate::types::InstanceStateChange>>,
    _request_id: Option<String>,
}
impl StopInstancesOutput {
    /// <p>Information about the stopped instances.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.stopping_instances.is_none()`.
    pub fn stopping_instances(&self) -> &[crate::types::InstanceStateChange] {
        self.stopping_instances.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for StopInstancesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StopInstancesOutput {
    /// Creates a new builder-style object to manufacture [`StopInstancesOutput`](crate::operation::stop_instances::StopInstancesOutput).
    pub fn builder() -> crate::operation::stop_instances::builders::StopInstancesOutputBuilder {
        crate::operation::stop_instances::builders::StopInstancesOutputBuilder::default()
    }
}

/// A builder for [`StopInstancesOutput`](crate::operation::stop_instances::StopInstancesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct StopInstancesOutputBuilder {
    pub(crate) stopping_instances: ::std::option::Option<::std::vec::Vec<crate::types::InstanceStateChange>>,
    _request_id: Option<String>,
}
impl StopInstancesOutputBuilder {
    /// Appends an item to `stopping_instances`.
    ///
    /// To override the contents of this collection use [`set_stopping_instances`](Self::set_stopping_instances).
    ///
    /// <p>Information about the stopped instances.</p>
    pub fn stopping_instances(mut self, input: crate::types::InstanceStateChange) -> Self {
        let mut v = self.stopping_instances.unwrap_or_default();
        v.push(input);
        self.stopping_instances = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the stopped instances.</p>
    pub fn set_stopping_instances(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceStateChange>>) -> Self {
        self.stopping_instances = input;
        self
    }
    /// <p>Information about the stopped instances.</p>
    pub fn get_stopping_instances(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceStateChange>> {
        &self.stopping_instances
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StopInstancesOutput`](crate::operation::stop_instances::StopInstancesOutput).
    pub fn build(self) -> crate::operation::stop_instances::StopInstancesOutput {
        crate::operation::stop_instances::StopInstancesOutput {
            stopping_instances: self.stopping_instances,
            _request_id: self._request_id,
        }
    }
}