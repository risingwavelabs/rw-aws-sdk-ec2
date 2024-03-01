// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for DetachNetworkInterface.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DetachNetworkInterfaceInput {
    /// <p>The ID of the attachment.</p>
    pub attachment_id: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>Specifies whether to force a detachment.</p> <note>
    /// <ul>
    /// <li> <p>Use the <code>Force</code> parameter only as a last resort to detach a network interface from a failed instance. </p> </li>
    /// <li> <p>If you use the <code>Force</code> parameter to detach a network interface, you might not be able to attach a different network interface to the same index on the instance without first stopping and starting the instance.</p> </li>
    /// <li> <p>If you force the detachment of a network interface, the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">instance metadata</a> might not get updated. This means that the attributes associated with the detached network interface might still be visible. The instance metadata will get updated when you stop and start the instance.</p> </li>
    /// </ul>
    /// </note>
    pub force: ::std::option::Option<bool>,
}
impl DetachNetworkInterfaceInput {
    /// <p>The ID of the attachment.</p>
    pub fn attachment_id(&self) -> ::std::option::Option<&str> {
        self.attachment_id.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>Specifies whether to force a detachment.</p> <note>
    /// <ul>
    /// <li> <p>Use the <code>Force</code> parameter only as a last resort to detach a network interface from a failed instance. </p> </li>
    /// <li> <p>If you use the <code>Force</code> parameter to detach a network interface, you might not be able to attach a different network interface to the same index on the instance without first stopping and starting the instance.</p> </li>
    /// <li> <p>If you force the detachment of a network interface, the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">instance metadata</a> might not get updated. This means that the attributes associated with the detached network interface might still be visible. The instance metadata will get updated when you stop and start the instance.</p> </li>
    /// </ul>
    /// </note>
    pub fn force(&self) -> ::std::option::Option<bool> {
        self.force
    }
}
impl DetachNetworkInterfaceInput {
    /// Creates a new builder-style object to manufacture [`DetachNetworkInterfaceInput`](crate::operation::detach_network_interface::DetachNetworkInterfaceInput).
    pub fn builder() -> crate::operation::detach_network_interface::builders::DetachNetworkInterfaceInputBuilder {
        crate::operation::detach_network_interface::builders::DetachNetworkInterfaceInputBuilder::default()
    }
}

/// A builder for [`DetachNetworkInterfaceInput`](crate::operation::detach_network_interface::DetachNetworkInterfaceInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DetachNetworkInterfaceInputBuilder {
    pub(crate) attachment_id: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) force: ::std::option::Option<bool>,
}
impl DetachNetworkInterfaceInputBuilder {
    /// <p>The ID of the attachment.</p>
    /// This field is required.
    pub fn attachment_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.attachment_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn set_attachment_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.attachment_id = input;
        self
    }
    /// <p>The ID of the attachment.</p>
    pub fn get_attachment_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.attachment_id
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
    /// <p>Specifies whether to force a detachment.</p> <note>
    /// <ul>
    /// <li> <p>Use the <code>Force</code> parameter only as a last resort to detach a network interface from a failed instance. </p> </li>
    /// <li> <p>If you use the <code>Force</code> parameter to detach a network interface, you might not be able to attach a different network interface to the same index on the instance without first stopping and starting the instance.</p> </li>
    /// <li> <p>If you force the detachment of a network interface, the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">instance metadata</a> might not get updated. This means that the attributes associated with the detached network interface might still be visible. The instance metadata will get updated when you stop and start the instance.</p> </li>
    /// </ul>
    /// </note>
    pub fn force(mut self, input: bool) -> Self {
        self.force = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies whether to force a detachment.</p> <note>
    /// <ul>
    /// <li> <p>Use the <code>Force</code> parameter only as a last resort to detach a network interface from a failed instance. </p> </li>
    /// <li> <p>If you use the <code>Force</code> parameter to detach a network interface, you might not be able to attach a different network interface to the same index on the instance without first stopping and starting the instance.</p> </li>
    /// <li> <p>If you force the detachment of a network interface, the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">instance metadata</a> might not get updated. This means that the attributes associated with the detached network interface might still be visible. The instance metadata will get updated when you stop and start the instance.</p> </li>
    /// </ul>
    /// </note>
    pub fn set_force(mut self, input: ::std::option::Option<bool>) -> Self {
        self.force = input;
        self
    }
    /// <p>Specifies whether to force a detachment.</p> <note>
    /// <ul>
    /// <li> <p>Use the <code>Force</code> parameter only as a last resort to detach a network interface from a failed instance. </p> </li>
    /// <li> <p>If you use the <code>Force</code> parameter to detach a network interface, you might not be able to attach a different network interface to the same index on the instance without first stopping and starting the instance.</p> </li>
    /// <li> <p>If you force the detachment of a network interface, the <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-instance-metadata.html">instance metadata</a> might not get updated. This means that the attributes associated with the detached network interface might still be visible. The instance metadata will get updated when you stop and start the instance.</p> </li>
    /// </ul>
    /// </note>
    pub fn get_force(&self) -> &::std::option::Option<bool> {
        &self.force
    }
    /// Consumes the builder and constructs a [`DetachNetworkInterfaceInput`](crate::operation::detach_network_interface::DetachNetworkInterfaceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::detach_network_interface::DetachNetworkInterfaceInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::detach_network_interface::DetachNetworkInterfaceInput {
            attachment_id: self.attachment_id,
            dry_run: self.dry_run,
            force: self.force,
        })
    }
}
