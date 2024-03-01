// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeBundleTasksInput {
    /// <p>The bundle task IDs.</p>
    /// <p>Default: Describes all your bundle tasks.</p>
    pub bundle_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>bundle-id</code> - The ID of the bundle task.</p> </li>
    /// <li> <p> <code>error-code</code> - If the task failed, the error code returned.</p> </li>
    /// <li> <p> <code>error-message</code> - If the task failed, the error message returned.</p> </li>
    /// <li> <p> <code>instance-id</code> - The ID of the instance.</p> </li>
    /// <li> <p> <code>progress</code> - The level of task completion, as a percentage (for example, 20%).</p> </li>
    /// <li> <p> <code>s3-bucket</code> - The Amazon S3 bucket to store the AMI.</p> </li>
    /// <li> <p> <code>s3-prefix</code> - The beginning of the AMI name.</p> </li>
    /// <li> <p> <code>start-time</code> - The time the task started (for example, 2013-09-15T17:15:20.000Z).</p> </li>
    /// <li> <p> <code>state</code> - The state of the task (<code>pending</code> | <code>waiting-for-shutdown</code> | <code>bundling</code> | <code>storing</code> | <code>cancelling</code> | <code>complete</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>update-time</code> - The time of the most recent update for the task.</p> </li>
    /// </ul>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl DescribeBundleTasksInput {
    /// <p>The bundle task IDs.</p>
    /// <p>Default: Describes all your bundle tasks.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.bundle_ids.is_none()`.
    pub fn bundle_ids(&self) -> &[::std::string::String] {
        self.bundle_ids.as_deref().unwrap_or_default()
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>bundle-id</code> - The ID of the bundle task.</p> </li>
    /// <li> <p> <code>error-code</code> - If the task failed, the error code returned.</p> </li>
    /// <li> <p> <code>error-message</code> - If the task failed, the error message returned.</p> </li>
    /// <li> <p> <code>instance-id</code> - The ID of the instance.</p> </li>
    /// <li> <p> <code>progress</code> - The level of task completion, as a percentage (for example, 20%).</p> </li>
    /// <li> <p> <code>s3-bucket</code> - The Amazon S3 bucket to store the AMI.</p> </li>
    /// <li> <p> <code>s3-prefix</code> - The beginning of the AMI name.</p> </li>
    /// <li> <p> <code>start-time</code> - The time the task started (for example, 2013-09-15T17:15:20.000Z).</p> </li>
    /// <li> <p> <code>state</code> - The state of the task (<code>pending</code> | <code>waiting-for-shutdown</code> | <code>bundling</code> | <code>storing</code> | <code>cancelling</code> | <code>complete</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>update-time</code> - The time of the most recent update for the task.</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl DescribeBundleTasksInput {
    /// Creates a new builder-style object to manufacture [`DescribeBundleTasksInput`](crate::operation::describe_bundle_tasks::DescribeBundleTasksInput).
    pub fn builder() -> crate::operation::describe_bundle_tasks::builders::DescribeBundleTasksInputBuilder {
        crate::operation::describe_bundle_tasks::builders::DescribeBundleTasksInputBuilder::default()
    }
}

/// A builder for [`DescribeBundleTasksInput`](crate::operation::describe_bundle_tasks::DescribeBundleTasksInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeBundleTasksInputBuilder {
    pub(crate) bundle_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl DescribeBundleTasksInputBuilder {
    /// Appends an item to `bundle_ids`.
    ///
    /// To override the contents of this collection use [`set_bundle_ids`](Self::set_bundle_ids).
    ///
    /// <p>The bundle task IDs.</p>
    /// <p>Default: Describes all your bundle tasks.</p>
    pub fn bundle_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.bundle_ids.unwrap_or_default();
        v.push(input.into());
        self.bundle_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The bundle task IDs.</p>
    /// <p>Default: Describes all your bundle tasks.</p>
    pub fn set_bundle_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.bundle_ids = input;
        self
    }
    /// <p>The bundle task IDs.</p>
    /// <p>Default: Describes all your bundle tasks.</p>
    pub fn get_bundle_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.bundle_ids
    }
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>bundle-id</code> - The ID of the bundle task.</p> </li>
    /// <li> <p> <code>error-code</code> - If the task failed, the error code returned.</p> </li>
    /// <li> <p> <code>error-message</code> - If the task failed, the error message returned.</p> </li>
    /// <li> <p> <code>instance-id</code> - The ID of the instance.</p> </li>
    /// <li> <p> <code>progress</code> - The level of task completion, as a percentage (for example, 20%).</p> </li>
    /// <li> <p> <code>s3-bucket</code> - The Amazon S3 bucket to store the AMI.</p> </li>
    /// <li> <p> <code>s3-prefix</code> - The beginning of the AMI name.</p> </li>
    /// <li> <p> <code>start-time</code> - The time the task started (for example, 2013-09-15T17:15:20.000Z).</p> </li>
    /// <li> <p> <code>state</code> - The state of the task (<code>pending</code> | <code>waiting-for-shutdown</code> | <code>bundling</code> | <code>storing</code> | <code>cancelling</code> | <code>complete</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>update-time</code> - The time of the most recent update for the task.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>bundle-id</code> - The ID of the bundle task.</p> </li>
    /// <li> <p> <code>error-code</code> - If the task failed, the error code returned.</p> </li>
    /// <li> <p> <code>error-message</code> - If the task failed, the error message returned.</p> </li>
    /// <li> <p> <code>instance-id</code> - The ID of the instance.</p> </li>
    /// <li> <p> <code>progress</code> - The level of task completion, as a percentage (for example, 20%).</p> </li>
    /// <li> <p> <code>s3-bucket</code> - The Amazon S3 bucket to store the AMI.</p> </li>
    /// <li> <p> <code>s3-prefix</code> - The beginning of the AMI name.</p> </li>
    /// <li> <p> <code>start-time</code> - The time the task started (for example, 2013-09-15T17:15:20.000Z).</p> </li>
    /// <li> <p> <code>state</code> - The state of the task (<code>pending</code> | <code>waiting-for-shutdown</code> | <code>bundling</code> | <code>storing</code> | <code>cancelling</code> | <code>complete</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>update-time</code> - The time of the most recent update for the task.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>bundle-id</code> - The ID of the bundle task.</p> </li>
    /// <li> <p> <code>error-code</code> - If the task failed, the error code returned.</p> </li>
    /// <li> <p> <code>error-message</code> - If the task failed, the error message returned.</p> </li>
    /// <li> <p> <code>instance-id</code> - The ID of the instance.</p> </li>
    /// <li> <p> <code>progress</code> - The level of task completion, as a percentage (for example, 20%).</p> </li>
    /// <li> <p> <code>s3-bucket</code> - The Amazon S3 bucket to store the AMI.</p> </li>
    /// <li> <p> <code>s3-prefix</code> - The beginning of the AMI name.</p> </li>
    /// <li> <p> <code>start-time</code> - The time the task started (for example, 2013-09-15T17:15:20.000Z).</p> </li>
    /// <li> <p> <code>state</code> - The state of the task (<code>pending</code> | <code>waiting-for-shutdown</code> | <code>bundling</code> | <code>storing</code> | <code>cancelling</code> | <code>complete</code> | <code>failed</code>).</p> </li>
    /// <li> <p> <code>update-time</code> - The time of the most recent update for the task.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// Consumes the builder and constructs a [`DescribeBundleTasksInput`](crate::operation::describe_bundle_tasks::DescribeBundleTasksInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::describe_bundle_tasks::DescribeBundleTasksInput, ::aws_smithy_types::error::operation::BuildError>
    {
        ::std::result::Result::Ok(crate::operation::describe_bundle_tasks::DescribeBundleTasksInput {
            bundle_ids: self.bundle_ids,
            filters: self.filters,
            dry_run: self.dry_run,
        })
    }
}
