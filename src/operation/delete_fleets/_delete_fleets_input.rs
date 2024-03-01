// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteFleetsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The IDs of the EC2 Fleets.</p>
    pub fleet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Indicates whether to terminate the associated instances when the EC2 Fleet is deleted. The default is to terminate the instances.</p>
    /// <p>To let the instances continue to run after the EC2 Fleet is deleted, specify <code>no-terminate-instances</code>. Supported only for fleets of type <code>maintain</code> and <code>request</code>.</p>
    /// <p>For <code>instant</code> fleets, you cannot specify <code>NoTerminateInstances</code>. A deleted <code>instant</code> fleet with running instances is not supported.</p>
    pub terminate_instances: ::std::option::Option<bool>,
}
impl DeleteFleetsInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The IDs of the EC2 Fleets.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.fleet_ids.is_none()`.
    pub fn fleet_ids(&self) -> &[::std::string::String] {
        self.fleet_ids.as_deref().unwrap_or_default()
    }
    /// <p>Indicates whether to terminate the associated instances when the EC2 Fleet is deleted. The default is to terminate the instances.</p>
    /// <p>To let the instances continue to run after the EC2 Fleet is deleted, specify <code>no-terminate-instances</code>. Supported only for fleets of type <code>maintain</code> and <code>request</code>.</p>
    /// <p>For <code>instant</code> fleets, you cannot specify <code>NoTerminateInstances</code>. A deleted <code>instant</code> fleet with running instances is not supported.</p>
    pub fn terminate_instances(&self) -> ::std::option::Option<bool> {
        self.terminate_instances
    }
}
impl DeleteFleetsInput {
    /// Creates a new builder-style object to manufacture [`DeleteFleetsInput`](crate::operation::delete_fleets::DeleteFleetsInput).
    pub fn builder() -> crate::operation::delete_fleets::builders::DeleteFleetsInputBuilder {
        crate::operation::delete_fleets::builders::DeleteFleetsInputBuilder::default()
    }
}

/// A builder for [`DeleteFleetsInput`](crate::operation::delete_fleets::DeleteFleetsInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteFleetsInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) fleet_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) terminate_instances: ::std::option::Option<bool>,
}
impl DeleteFleetsInputBuilder {
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
    /// Appends an item to `fleet_ids`.
    ///
    /// To override the contents of this collection use [`set_fleet_ids`](Self::set_fleet_ids).
    ///
    /// <p>The IDs of the EC2 Fleets.</p>
    pub fn fleet_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.fleet_ids.unwrap_or_default();
        v.push(input.into());
        self.fleet_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the EC2 Fleets.</p>
    pub fn set_fleet_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.fleet_ids = input;
        self
    }
    /// <p>The IDs of the EC2 Fleets.</p>
    pub fn get_fleet_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.fleet_ids
    }
    /// <p>Indicates whether to terminate the associated instances when the EC2 Fleet is deleted. The default is to terminate the instances.</p>
    /// <p>To let the instances continue to run after the EC2 Fleet is deleted, specify <code>no-terminate-instances</code>. Supported only for fleets of type <code>maintain</code> and <code>request</code>.</p>
    /// <p>For <code>instant</code> fleets, you cannot specify <code>NoTerminateInstances</code>. A deleted <code>instant</code> fleet with running instances is not supported.</p>
    /// This field is required.
    pub fn terminate_instances(mut self, input: bool) -> Self {
        self.terminate_instances = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to terminate the associated instances when the EC2 Fleet is deleted. The default is to terminate the instances.</p>
    /// <p>To let the instances continue to run after the EC2 Fleet is deleted, specify <code>no-terminate-instances</code>. Supported only for fleets of type <code>maintain</code> and <code>request</code>.</p>
    /// <p>For <code>instant</code> fleets, you cannot specify <code>NoTerminateInstances</code>. A deleted <code>instant</code> fleet with running instances is not supported.</p>
    pub fn set_terminate_instances(mut self, input: ::std::option::Option<bool>) -> Self {
        self.terminate_instances = input;
        self
    }
    /// <p>Indicates whether to terminate the associated instances when the EC2 Fleet is deleted. The default is to terminate the instances.</p>
    /// <p>To let the instances continue to run after the EC2 Fleet is deleted, specify <code>no-terminate-instances</code>. Supported only for fleets of type <code>maintain</code> and <code>request</code>.</p>
    /// <p>For <code>instant</code> fleets, you cannot specify <code>NoTerminateInstances</code>. A deleted <code>instant</code> fleet with running instances is not supported.</p>
    pub fn get_terminate_instances(&self) -> &::std::option::Option<bool> {
        &self.terminate_instances
    }
    /// Consumes the builder and constructs a [`DeleteFleetsInput`](crate::operation::delete_fleets::DeleteFleetsInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::delete_fleets::DeleteFleetsInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::delete_fleets::DeleteFleetsInput {
            dry_run: self.dry_run,
            fleet_ids: self.fleet_ids,
            terminate_instances: self.terminate_instances,
        })
    }
}
