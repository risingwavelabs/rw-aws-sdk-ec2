// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a launch template version that could not be deleted.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteLaunchTemplateVersionsResponseErrorItem {
    /// <p>The ID of the launch template.</p>
    pub launch_template_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the launch template.</p>
    pub launch_template_name: ::std::option::Option<::std::string::String>,
    /// <p>The version number of the launch template.</p>
    pub version_number: ::std::option::Option<i64>,
    /// <p>Information about the error.</p>
    pub response_error: ::std::option::Option<crate::types::ResponseError>,
}
impl DeleteLaunchTemplateVersionsResponseErrorItem {
    /// <p>The ID of the launch template.</p>
    pub fn launch_template_id(&self) -> ::std::option::Option<&str> {
        self.launch_template_id.as_deref()
    }
    /// <p>The name of the launch template.</p>
    pub fn launch_template_name(&self) -> ::std::option::Option<&str> {
        self.launch_template_name.as_deref()
    }
    /// <p>The version number of the launch template.</p>
    pub fn version_number(&self) -> ::std::option::Option<i64> {
        self.version_number
    }
    /// <p>Information about the error.</p>
    pub fn response_error(&self) -> ::std::option::Option<&crate::types::ResponseError> {
        self.response_error.as_ref()
    }
}
impl DeleteLaunchTemplateVersionsResponseErrorItem {
    /// Creates a new builder-style object to manufacture [`DeleteLaunchTemplateVersionsResponseErrorItem`](crate::types::DeleteLaunchTemplateVersionsResponseErrorItem).
    pub fn builder() -> crate::types::builders::DeleteLaunchTemplateVersionsResponseErrorItemBuilder {
        crate::types::builders::DeleteLaunchTemplateVersionsResponseErrorItemBuilder::default()
    }
}

/// A builder for [`DeleteLaunchTemplateVersionsResponseErrorItem`](crate::types::DeleteLaunchTemplateVersionsResponseErrorItem).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteLaunchTemplateVersionsResponseErrorItemBuilder {
    pub(crate) launch_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) launch_template_name: ::std::option::Option<::std::string::String>,
    pub(crate) version_number: ::std::option::Option<i64>,
    pub(crate) response_error: ::std::option::Option<crate::types::ResponseError>,
}
impl DeleteLaunchTemplateVersionsResponseErrorItemBuilder {
    /// <p>The ID of the launch template.</p>
    pub fn launch_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.launch_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the launch template.</p>
    pub fn set_launch_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.launch_template_id = input;
        self
    }
    /// <p>The ID of the launch template.</p>
    pub fn get_launch_template_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.launch_template_id
    }
    /// <p>The name of the launch template.</p>
    pub fn launch_template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.launch_template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the launch template.</p>
    pub fn set_launch_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.launch_template_name = input;
        self
    }
    /// <p>The name of the launch template.</p>
    pub fn get_launch_template_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.launch_template_name
    }
    /// <p>The version number of the launch template.</p>
    pub fn version_number(mut self, input: i64) -> Self {
        self.version_number = ::std::option::Option::Some(input);
        self
    }
    /// <p>The version number of the launch template.</p>
    pub fn set_version_number(mut self, input: ::std::option::Option<i64>) -> Self {
        self.version_number = input;
        self
    }
    /// <p>The version number of the launch template.</p>
    pub fn get_version_number(&self) -> &::std::option::Option<i64> {
        &self.version_number
    }
    /// <p>Information about the error.</p>
    pub fn response_error(mut self, input: crate::types::ResponseError) -> Self {
        self.response_error = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the error.</p>
    pub fn set_response_error(mut self, input: ::std::option::Option<crate::types::ResponseError>) -> Self {
        self.response_error = input;
        self
    }
    /// <p>Information about the error.</p>
    pub fn get_response_error(&self) -> &::std::option::Option<crate::types::ResponseError> {
        &self.response_error
    }
    /// Consumes the builder and constructs a [`DeleteLaunchTemplateVersionsResponseErrorItem`](crate::types::DeleteLaunchTemplateVersionsResponseErrorItem).
    pub fn build(self) -> crate::types::DeleteLaunchTemplateVersionsResponseErrorItem {
        crate::types::DeleteLaunchTemplateVersionsResponseErrorItem {
            launch_template_id: self.launch_template_id,
            launch_template_name: self.launch_template_name,
            version_number: self.version_number,
            response_error: self.response_error,
        }
    }
}
