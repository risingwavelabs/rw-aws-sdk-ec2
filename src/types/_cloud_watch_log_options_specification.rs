// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Options for sending VPN tunnel logs to CloudWatch.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CloudWatchLogOptionsSpecification {
    /// <p>Enable or disable VPN tunnel logging feature. Default value is <code>False</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub log_enabled: ::std::option::Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch log group to send logs to.</p>
    pub log_group_arn: ::std::option::Option<::std::string::String>,
    /// <p>Set log format. Default format is <code>json</code>.</p>
    /// <p>Valid values: <code>json</code> | <code>text</code> </p>
    pub log_output_format: ::std::option::Option<::std::string::String>,
}
impl CloudWatchLogOptionsSpecification {
    /// <p>Enable or disable VPN tunnel logging feature. Default value is <code>False</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn log_enabled(&self) -> ::std::option::Option<bool> {
        self.log_enabled
    }
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch log group to send logs to.</p>
    pub fn log_group_arn(&self) -> ::std::option::Option<&str> {
        self.log_group_arn.as_deref()
    }
    /// <p>Set log format. Default format is <code>json</code>.</p>
    /// <p>Valid values: <code>json</code> | <code>text</code> </p>
    pub fn log_output_format(&self) -> ::std::option::Option<&str> {
        self.log_output_format.as_deref()
    }
}
impl CloudWatchLogOptionsSpecification {
    /// Creates a new builder-style object to manufacture [`CloudWatchLogOptionsSpecification`](crate::types::CloudWatchLogOptionsSpecification).
    pub fn builder() -> crate::types::builders::CloudWatchLogOptionsSpecificationBuilder {
        crate::types::builders::CloudWatchLogOptionsSpecificationBuilder::default()
    }
}

/// A builder for [`CloudWatchLogOptionsSpecification`](crate::types::CloudWatchLogOptionsSpecification).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CloudWatchLogOptionsSpecificationBuilder {
    pub(crate) log_enabled: ::std::option::Option<bool>,
    pub(crate) log_group_arn: ::std::option::Option<::std::string::String>,
    pub(crate) log_output_format: ::std::option::Option<::std::string::String>,
}
impl CloudWatchLogOptionsSpecificationBuilder {
    /// <p>Enable or disable VPN tunnel logging feature. Default value is <code>False</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn log_enabled(mut self, input: bool) -> Self {
        self.log_enabled = ::std::option::Option::Some(input);
        self
    }
    /// <p>Enable or disable VPN tunnel logging feature. Default value is <code>False</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn set_log_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.log_enabled = input;
        self
    }
    /// <p>Enable or disable VPN tunnel logging feature. Default value is <code>False</code>.</p>
    /// <p>Valid values: <code>True</code> | <code>False</code> </p>
    pub fn get_log_enabled(&self) -> &::std::option::Option<bool> {
        &self.log_enabled
    }
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch log group to send logs to.</p>
    pub fn log_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_group_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch log group to send logs to.</p>
    pub fn set_log_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_group_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the CloudWatch log group to send logs to.</p>
    pub fn get_log_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_group_arn
    }
    /// <p>Set log format. Default format is <code>json</code>.</p>
    /// <p>Valid values: <code>json</code> | <code>text</code> </p>
    pub fn log_output_format(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.log_output_format = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Set log format. Default format is <code>json</code>.</p>
    /// <p>Valid values: <code>json</code> | <code>text</code> </p>
    pub fn set_log_output_format(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.log_output_format = input;
        self
    }
    /// <p>Set log format. Default format is <code>json</code>.</p>
    /// <p>Valid values: <code>json</code> | <code>text</code> </p>
    pub fn get_log_output_format(&self) -> &::std::option::Option<::std::string::String> {
        &self.log_output_format
    }
    /// Consumes the builder and constructs a [`CloudWatchLogOptionsSpecification`](crate::types::CloudWatchLogOptionsSpecification).
    pub fn build(self) -> crate::types::CloudWatchLogOptionsSpecification {
        crate::types::CloudWatchLogOptionsSpecification {
            log_enabled: self.log_enabled,
            log_group_arn: self.log_group_arn,
            log_output_format: self.log_output_format,
        }
    }
}
