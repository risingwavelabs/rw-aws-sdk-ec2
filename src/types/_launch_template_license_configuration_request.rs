// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a license configuration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LaunchTemplateLicenseConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the license configuration.</p>
    pub license_configuration_arn: ::std::option::Option<::std::string::String>,
}
impl LaunchTemplateLicenseConfigurationRequest {
    /// <p>The Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn license_configuration_arn(&self) -> ::std::option::Option<&str> {
        self.license_configuration_arn.as_deref()
    }
}
impl LaunchTemplateLicenseConfigurationRequest {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateLicenseConfigurationRequest`](crate::types::LaunchTemplateLicenseConfigurationRequest).
    pub fn builder() -> crate::types::builders::LaunchTemplateLicenseConfigurationRequestBuilder {
        crate::types::builders::LaunchTemplateLicenseConfigurationRequestBuilder::default()
    }
}

/// A builder for [`LaunchTemplateLicenseConfigurationRequest`](crate::types::LaunchTemplateLicenseConfigurationRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LaunchTemplateLicenseConfigurationRequestBuilder {
    pub(crate) license_configuration_arn: ::std::option::Option<::std::string::String>,
}
impl LaunchTemplateLicenseConfigurationRequestBuilder {
    /// <p>The Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn license_configuration_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.license_configuration_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn set_license_configuration_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.license_configuration_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the license configuration.</p>
    pub fn get_license_configuration_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.license_configuration_arn
    }
    /// Consumes the builder and constructs a [`LaunchTemplateLicenseConfigurationRequest`](crate::types::LaunchTemplateLicenseConfigurationRequest).
    pub fn build(self) -> crate::types::LaunchTemplateLicenseConfigurationRequest {
        crate::types::LaunchTemplateLicenseConfigurationRequest {
            license_configuration_arn: self.license_configuration_arn,
        }
    }
}
