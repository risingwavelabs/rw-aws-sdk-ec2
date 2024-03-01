// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportInstanceOutput {
    /// <p>Information about the conversion task.</p>
    pub conversion_task: ::std::option::Option<crate::types::ConversionTask>,
    _request_id: Option<String>,
}
impl ImportInstanceOutput {
    /// <p>Information about the conversion task.</p>
    pub fn conversion_task(&self) -> ::std::option::Option<&crate::types::ConversionTask> {
        self.conversion_task.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ImportInstanceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ImportInstanceOutput {
    /// Creates a new builder-style object to manufacture [`ImportInstanceOutput`](crate::operation::import_instance::ImportInstanceOutput).
    pub fn builder() -> crate::operation::import_instance::builders::ImportInstanceOutputBuilder {
        crate::operation::import_instance::builders::ImportInstanceOutputBuilder::default()
    }
}

/// A builder for [`ImportInstanceOutput`](crate::operation::import_instance::ImportInstanceOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ImportInstanceOutputBuilder {
    pub(crate) conversion_task: ::std::option::Option<crate::types::ConversionTask>,
    _request_id: Option<String>,
}
impl ImportInstanceOutputBuilder {
    /// <p>Information about the conversion task.</p>
    pub fn conversion_task(mut self, input: crate::types::ConversionTask) -> Self {
        self.conversion_task = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the conversion task.</p>
    pub fn set_conversion_task(mut self, input: ::std::option::Option<crate::types::ConversionTask>) -> Self {
        self.conversion_task = input;
        self
    }
    /// <p>Information about the conversion task.</p>
    pub fn get_conversion_task(&self) -> &::std::option::Option<crate::types::ConversionTask> {
        &self.conversion_task
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ImportInstanceOutput`](crate::operation::import_instance::ImportInstanceOutput).
    pub fn build(self) -> crate::operation::import_instance::ImportInstanceOutput {
        crate::operation::import_instance::ImportInstanceOutput {
            conversion_task: self.conversion_task,
            _request_id: self._request_id,
        }
    }
}
