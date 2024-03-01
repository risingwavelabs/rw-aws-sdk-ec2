// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Request to create a launch template for a Windows fast launch enabled AMI.</p> <note>
/// <p>Note - You can specify either the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
/// </note>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FastLaunchLaunchTemplateSpecificationRequest {
    /// <p>Specify the ID of the launch template that the AMI should use for Windows fast launch.</p>
    pub launch_template_id: ::std::option::Option<::std::string::String>,
    /// <p>Specify the name of the launch template that the AMI should use for Windows fast launch.</p>
    pub launch_template_name: ::std::option::Option<::std::string::String>,
    /// <p>Specify the version of the launch template that the AMI should use for Windows fast launch.</p>
    pub version: ::std::option::Option<::std::string::String>,
}
impl FastLaunchLaunchTemplateSpecificationRequest {
    /// <p>Specify the ID of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn launch_template_id(&self) -> ::std::option::Option<&str> {
        self.launch_template_id.as_deref()
    }
    /// <p>Specify the name of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn launch_template_name(&self) -> ::std::option::Option<&str> {
        self.launch_template_name.as_deref()
    }
    /// <p>Specify the version of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl FastLaunchLaunchTemplateSpecificationRequest {
    /// Creates a new builder-style object to manufacture [`FastLaunchLaunchTemplateSpecificationRequest`](crate::types::FastLaunchLaunchTemplateSpecificationRequest).
    pub fn builder() -> crate::types::builders::FastLaunchLaunchTemplateSpecificationRequestBuilder {
        crate::types::builders::FastLaunchLaunchTemplateSpecificationRequestBuilder::default()
    }
}

/// A builder for [`FastLaunchLaunchTemplateSpecificationRequest`](crate::types::FastLaunchLaunchTemplateSpecificationRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct FastLaunchLaunchTemplateSpecificationRequestBuilder {
    pub(crate) launch_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) launch_template_name: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
}
impl FastLaunchLaunchTemplateSpecificationRequestBuilder {
    /// <p>Specify the ID of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn launch_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.launch_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify the ID of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn set_launch_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.launch_template_id = input;
        self
    }
    /// <p>Specify the ID of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn get_launch_template_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.launch_template_id
    }
    /// <p>Specify the name of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn launch_template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.launch_template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify the name of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn set_launch_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.launch_template_name = input;
        self
    }
    /// <p>Specify the name of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn get_launch_template_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.launch_template_name
    }
    /// <p>Specify the version of the launch template that the AMI should use for Windows fast launch.</p>
    /// This field is required.
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify the version of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// <p>Specify the version of the launch template that the AMI should use for Windows fast launch.</p>
    pub fn get_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.version
    }
    /// Consumes the builder and constructs a [`FastLaunchLaunchTemplateSpecificationRequest`](crate::types::FastLaunchLaunchTemplateSpecificationRequest).
    pub fn build(self) -> crate::types::FastLaunchLaunchTemplateSpecificationRequest {
        crate::types::FastLaunchLaunchTemplateSpecificationRequest {
            launch_template_id: self.launch_template_id,
            launch_template_name: self.launch_template_name,
            version: self.version,
        }
    }
}