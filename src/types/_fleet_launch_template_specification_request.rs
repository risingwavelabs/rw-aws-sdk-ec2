// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The Amazon EC2 launch template that can be used by an EC2 Fleet to configure Amazon EC2 instances. You must specify either the ID or name of the launch template in the request, but not both.</p>
/// <p>For information about launch templates, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-launch-templates.html">Launch an instance from a launch template</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FleetLaunchTemplateSpecificationRequest {
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub launch_template_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub launch_template_name: ::std::option::Option<::std::string::String>,
    /// <p>The launch template version number, <code>$Latest</code>, or <code>$Default</code>. You must specify a value, otherwise the request fails.</p>
    /// <p>If the value is <code>$Latest</code>, Amazon EC2 uses the latest version of the launch template.</p>
    /// <p>If the value is <code>$Default</code>, Amazon EC2 uses the default version of the launch template.</p>
    pub version: ::std::option::Option<::std::string::String>,
}
impl FleetLaunchTemplateSpecificationRequest {
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn launch_template_id(&self) -> ::std::option::Option<&str> {
        self.launch_template_id.as_deref()
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn launch_template_name(&self) -> ::std::option::Option<&str> {
        self.launch_template_name.as_deref()
    }
    /// <p>The launch template version number, <code>$Latest</code>, or <code>$Default</code>. You must specify a value, otherwise the request fails.</p>
    /// <p>If the value is <code>$Latest</code>, Amazon EC2 uses the latest version of the launch template.</p>
    /// <p>If the value is <code>$Default</code>, Amazon EC2 uses the default version of the launch template.</p>
    pub fn version(&self) -> ::std::option::Option<&str> {
        self.version.as_deref()
    }
}
impl FleetLaunchTemplateSpecificationRequest {
    /// Creates a new builder-style object to manufacture [`FleetLaunchTemplateSpecificationRequest`](crate::types::FleetLaunchTemplateSpecificationRequest).
    pub fn builder() -> crate::types::builders::FleetLaunchTemplateSpecificationRequestBuilder {
        crate::types::builders::FleetLaunchTemplateSpecificationRequestBuilder::default()
    }
}

/// A builder for [`FleetLaunchTemplateSpecificationRequest`](crate::types::FleetLaunchTemplateSpecificationRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct FleetLaunchTemplateSpecificationRequestBuilder {
    pub(crate) launch_template_id: ::std::option::Option<::std::string::String>,
    pub(crate) launch_template_name: ::std::option::Option<::std::string::String>,
    pub(crate) version: ::std::option::Option<::std::string::String>,
}
impl FleetLaunchTemplateSpecificationRequestBuilder {
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn launch_template_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.launch_template_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn set_launch_template_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.launch_template_id = input;
        self
    }
    /// <p>The ID of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p>
    pub fn get_launch_template_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.launch_template_id
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn launch_template_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.launch_template_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn set_launch_template_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.launch_template_name = input;
        self
    }
    /// <p>The name of the launch template.</p>
    /// <p>You must specify the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p>
    pub fn get_launch_template_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.launch_template_name
    }
    /// <p>The launch template version number, <code>$Latest</code>, or <code>$Default</code>. You must specify a value, otherwise the request fails.</p>
    /// <p>If the value is <code>$Latest</code>, Amazon EC2 uses the latest version of the launch template.</p>
    /// <p>If the value is <code>$Default</code>, Amazon EC2 uses the default version of the launch template.</p>
    pub fn version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The launch template version number, <code>$Latest</code>, or <code>$Default</code>. You must specify a value, otherwise the request fails.</p>
    /// <p>If the value is <code>$Latest</code>, Amazon EC2 uses the latest version of the launch template.</p>
    /// <p>If the value is <code>$Default</code>, Amazon EC2 uses the default version of the launch template.</p>
    pub fn set_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.version = input;
        self
    }
    /// <p>The launch template version number, <code>$Latest</code>, or <code>$Default</code>. You must specify a value, otherwise the request fails.</p>
    /// <p>If the value is <code>$Latest</code>, Amazon EC2 uses the latest version of the launch template.</p>
    /// <p>If the value is <code>$Default</code>, Amazon EC2 uses the default version of the launch template.</p>
    pub fn get_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.version
    }
    /// Consumes the builder and constructs a [`FleetLaunchTemplateSpecificationRequest`](crate::types::FleetLaunchTemplateSpecificationRequest).
    pub fn build(self) -> crate::types::FleetLaunchTemplateSpecificationRequest {
        crate::types::FleetLaunchTemplateSpecificationRequest {
            launch_template_id: self.launch_template_id,
            launch_template_name: self.launch_template_name,
            version: self.version,
        }
    }
}
