// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteLaunchTemplateOutput {
    /// <p>Information about the launch template.</p>
    pub launch_template: ::std::option::Option<crate::types::LaunchTemplate>,
    _request_id: Option<String>,
}
impl DeleteLaunchTemplateOutput {
    /// <p>Information about the launch template.</p>
    pub fn launch_template(&self) -> ::std::option::Option<&crate::types::LaunchTemplate> {
        self.launch_template.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for DeleteLaunchTemplateOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteLaunchTemplateOutput {
    /// Creates a new builder-style object to manufacture [`DeleteLaunchTemplateOutput`](crate::operation::delete_launch_template::DeleteLaunchTemplateOutput).
    pub fn builder() -> crate::operation::delete_launch_template::builders::DeleteLaunchTemplateOutputBuilder {
        crate::operation::delete_launch_template::builders::DeleteLaunchTemplateOutputBuilder::default()
    }
}

/// A builder for [`DeleteLaunchTemplateOutput`](crate::operation::delete_launch_template::DeleteLaunchTemplateOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteLaunchTemplateOutputBuilder {
    pub(crate) launch_template: ::std::option::Option<crate::types::LaunchTemplate>,
    _request_id: Option<String>,
}
impl DeleteLaunchTemplateOutputBuilder {
    /// <p>Information about the launch template.</p>
    pub fn launch_template(mut self, input: crate::types::LaunchTemplate) -> Self {
        self.launch_template = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the launch template.</p>
    pub fn set_launch_template(mut self, input: ::std::option::Option<crate::types::LaunchTemplate>) -> Self {
        self.launch_template = input;
        self
    }
    /// <p>Information about the launch template.</p>
    pub fn get_launch_template(&self) -> &::std::option::Option<crate::types::LaunchTemplate> {
        &self.launch_template
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteLaunchTemplateOutput`](crate::operation::delete_launch_template::DeleteLaunchTemplateOutput).
    pub fn build(self) -> crate::operation::delete_launch_template::DeleteLaunchTemplateOutput {
        crate::operation::delete_launch_template::DeleteLaunchTemplateOutput {
            launch_template: self.launch_template,
            _request_id: self._request_id,
        }
    }
}
