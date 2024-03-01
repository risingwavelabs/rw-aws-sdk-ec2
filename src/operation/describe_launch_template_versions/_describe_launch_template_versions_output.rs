// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeLaunchTemplateVersionsOutput {
    /// <p>Information about the launch template versions.</p>
    pub launch_template_versions: ::std::option::Option<::std::vec::Vec<crate::types::LaunchTemplateVersion>>,
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLaunchTemplateVersionsOutput {
    /// <p>Information about the launch template versions.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.launch_template_versions.is_none()`.
    pub fn launch_template_versions(&self) -> &[crate::types::LaunchTemplateVersion] {
        self.launch_template_versions.as_deref().unwrap_or_default()
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeLaunchTemplateVersionsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeLaunchTemplateVersionsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeLaunchTemplateVersionsOutput`](crate::operation::describe_launch_template_versions::DescribeLaunchTemplateVersionsOutput).
    pub fn builder() -> crate::operation::describe_launch_template_versions::builders::DescribeLaunchTemplateVersionsOutputBuilder {
        crate::operation::describe_launch_template_versions::builders::DescribeLaunchTemplateVersionsOutputBuilder::default()
    }
}

/// A builder for [`DescribeLaunchTemplateVersionsOutput`](crate::operation::describe_launch_template_versions::DescribeLaunchTemplateVersionsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeLaunchTemplateVersionsOutputBuilder {
    pub(crate) launch_template_versions: ::std::option::Option<::std::vec::Vec<crate::types::LaunchTemplateVersion>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeLaunchTemplateVersionsOutputBuilder {
    /// Appends an item to `launch_template_versions`.
    ///
    /// To override the contents of this collection use [`set_launch_template_versions`](Self::set_launch_template_versions).
    ///
    /// <p>Information about the launch template versions.</p>
    pub fn launch_template_versions(mut self, input: crate::types::LaunchTemplateVersion) -> Self {
        let mut v = self.launch_template_versions.unwrap_or_default();
        v.push(input);
        self.launch_template_versions = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the launch template versions.</p>
    pub fn set_launch_template_versions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::LaunchTemplateVersion>>) -> Self {
        self.launch_template_versions = input;
        self
    }
    /// <p>Information about the launch template versions.</p>
    pub fn get_launch_template_versions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::LaunchTemplateVersion>> {
        &self.launch_template_versions
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
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
    /// Consumes the builder and constructs a [`DescribeLaunchTemplateVersionsOutput`](crate::operation::describe_launch_template_versions::DescribeLaunchTemplateVersionsOutput).
    pub fn build(self) -> crate::operation::describe_launch_template_versions::DescribeLaunchTemplateVersionsOutput {
        crate::operation::describe_launch_template_versions::DescribeLaunchTemplateVersionsOutput {
            launch_template_versions: self.launch_template_versions,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}
