// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::modify_instance_attribute::_modify_instance_attribute_output::ModifyInstanceAttributeOutputBuilder;

pub use crate::operation::modify_instance_attribute::_modify_instance_attribute_input::ModifyInstanceAttributeInputBuilder;

impl ModifyInstanceAttributeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::modify_instance_attribute::ModifyInstanceAttributeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_instance_attribute::ModifyInstanceAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.modify_instance_attribute();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ModifyInstanceAttribute`.
///
/// <p>Modifies the specified attribute of the specified instance. You can specify only one attribute at a time.</p>
/// <p> <b>Note: </b>Using this action to change the security groups associated with an elastic network interface (ENI) attached to an instance can result in an error if the instance has more than one ENI. To change the security groups associated with an ENI attached to an instance that has multiple ENIs, we recommend that you use the <code>ModifyNetworkInterfaceAttribute</code> action.</p>
/// <p>To modify some attributes, the instance must be stopped. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_ChangingAttributesWhileInstanceStopped.html">Modify a stopped instance</a> in the <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ModifyInstanceAttributeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::modify_instance_attribute::builders::ModifyInstanceAttributeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::modify_instance_attribute::ModifyInstanceAttributeOutput,
        crate::operation::modify_instance_attribute::ModifyInstanceAttributeError,
    > for ModifyInstanceAttributeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::modify_instance_attribute::ModifyInstanceAttributeOutput,
            crate::operation::modify_instance_attribute::ModifyInstanceAttributeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ModifyInstanceAttributeFluentBuilder {
    /// Creates a new `ModifyInstanceAttribute`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ModifyInstanceAttribute as a reference.
    pub fn as_input(&self) -> &crate::operation::modify_instance_attribute::builders::ModifyInstanceAttributeInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::modify_instance_attribute::ModifyInstanceAttributeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::modify_instance_attribute::ModifyInstanceAttributeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::modify_instance_attribute::ModifyInstanceAttribute::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::modify_instance_attribute::ModifyInstanceAttribute::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::modify_instance_attribute::ModifyInstanceAttributeOutput,
        crate::operation::modify_instance_attribute::ModifyInstanceAttributeError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn source_dest_check(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.inner = self.inner.source_dest_check(input);
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn set_source_dest_check(mut self, input: ::std::option::Option<crate::types::AttributeBooleanValue>) -> Self {
        self.inner = self.inner.set_source_dest_check(input);
        self
    }
    /// <p>Enable or disable source/destination checks, which ensure that the instance is either the source or the destination of any traffic that it receives. If the value is <code>true</code>, source/destination checks are enabled; otherwise, they are disabled. The default value is <code>true</code>. You must disable source/destination checks if the instance runs services such as network address translation, routing, or firewalls.</p>
    pub fn get_source_dest_check(&self) -> &::std::option::Option<crate::types::AttributeBooleanValue> {
        self.inner.get_source_dest_check()
    }
    /// <p>The name of the attribute to modify.</p> <important>
    /// <p>You can modify the following attributes only: <code>disableApiTermination</code> | <code>instanceType</code> | <code>kernel</code> | <code>ramdisk</code> | <code>instanceInitiatedShutdownBehavior</code> | <code>blockDeviceMapping</code> | <code>userData</code> | <code>sourceDestCheck</code> | <code>groupSet</code> | <code>ebsOptimized</code> | <code>sriovNetSupport</code> | <code>enaSupport</code> | <code>nvmeSupport</code> | <code>disableApiStop</code> | <code>enclaveOptions</code> </p>
    /// </important>
    pub fn attribute(mut self, input: crate::types::InstanceAttributeName) -> Self {
        self.inner = self.inner.attribute(input);
        self
    }
    /// <p>The name of the attribute to modify.</p> <important>
    /// <p>You can modify the following attributes only: <code>disableApiTermination</code> | <code>instanceType</code> | <code>kernel</code> | <code>ramdisk</code> | <code>instanceInitiatedShutdownBehavior</code> | <code>blockDeviceMapping</code> | <code>userData</code> | <code>sourceDestCheck</code> | <code>groupSet</code> | <code>ebsOptimized</code> | <code>sriovNetSupport</code> | <code>enaSupport</code> | <code>nvmeSupport</code> | <code>disableApiStop</code> | <code>enclaveOptions</code> </p>
    /// </important>
    pub fn set_attribute(mut self, input: ::std::option::Option<crate::types::InstanceAttributeName>) -> Self {
        self.inner = self.inner.set_attribute(input);
        self
    }
    /// <p>The name of the attribute to modify.</p> <important>
    /// <p>You can modify the following attributes only: <code>disableApiTermination</code> | <code>instanceType</code> | <code>kernel</code> | <code>ramdisk</code> | <code>instanceInitiatedShutdownBehavior</code> | <code>blockDeviceMapping</code> | <code>userData</code> | <code>sourceDestCheck</code> | <code>groupSet</code> | <code>ebsOptimized</code> | <code>sriovNetSupport</code> | <code>enaSupport</code> | <code>nvmeSupport</code> | <code>disableApiStop</code> | <code>enclaveOptions</code> </p>
    /// </important>
    pub fn get_attribute(&self) -> &::std::option::Option<crate::types::InstanceAttributeName> {
        self.inner.get_attribute()
    }
    /// Appends an item to `BlockDeviceMappings`.
    ///
    /// To override the contents of this collection use [`set_block_device_mappings`](Self::set_block_device_mappings).
    ///
    /// <p>Modifies the <code>DeleteOnTermination</code> attribute for volumes that are currently attached. The volume must be owned by the caller. If no value is specified for <code>DeleteOnTermination</code>, the default is <code>true</code> and the volume is deleted when the instance is terminated.</p>
    /// <p>To add instance store volumes to an Amazon EBS-backed instance, you must add them when you launch the instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html#Using_OverridingAMIBDM">Update the block device mapping when launching an instance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn block_device_mappings(mut self, input: crate::types::InstanceBlockDeviceMappingSpecification) -> Self {
        self.inner = self.inner.block_device_mappings(input);
        self
    }
    /// <p>Modifies the <code>DeleteOnTermination</code> attribute for volumes that are currently attached. The volume must be owned by the caller. If no value is specified for <code>DeleteOnTermination</code>, the default is <code>true</code> and the volume is deleted when the instance is terminated.</p>
    /// <p>To add instance store volumes to an Amazon EBS-backed instance, you must add them when you launch the instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html#Using_OverridingAMIBDM">Update the block device mapping when launching an instance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_block_device_mappings(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceBlockDeviceMappingSpecification>>,
    ) -> Self {
        self.inner = self.inner.set_block_device_mappings(input);
        self
    }
    /// <p>Modifies the <code>DeleteOnTermination</code> attribute for volumes that are currently attached. The volume must be owned by the caller. If no value is specified for <code>DeleteOnTermination</code>, the default is <code>true</code> and the volume is deleted when the instance is terminated.</p>
    /// <p>To add instance store volumes to an Amazon EBS-backed instance, you must add them when you launch the instance. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/block-device-mapping-concepts.html#Using_OverridingAMIBDM">Update the block device mapping when launching an instance</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn get_block_device_mappings(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceBlockDeviceMappingSpecification>> {
        self.inner.get_block_device_mappings()
    }
    /// <p>If the value is <code>true</code>, you can't terminate the instance using the Amazon EC2 console, CLI, or API; otherwise, you can. You cannot use this parameter for Spot Instances.</p>
    pub fn disable_api_termination(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.inner = self.inner.disable_api_termination(input);
        self
    }
    /// <p>If the value is <code>true</code>, you can't terminate the instance using the Amazon EC2 console, CLI, or API; otherwise, you can. You cannot use this parameter for Spot Instances.</p>
    pub fn set_disable_api_termination(mut self, input: ::std::option::Option<crate::types::AttributeBooleanValue>) -> Self {
        self.inner = self.inner.set_disable_api_termination(input);
        self
    }
    /// <p>If the value is <code>true</code>, you can't terminate the instance using the Amazon EC2 console, CLI, or API; otherwise, you can. You cannot use this parameter for Spot Instances.</p>
    pub fn get_disable_api_termination(&self) -> &::std::option::Option<crate::types::AttributeBooleanValue> {
        self.inner.get_disable_api_termination()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>Specifies whether the instance is optimized for Amazon EBS I/O. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal EBS I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS Optimized instance.</p>
    pub fn ebs_optimized(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.inner = self.inner.ebs_optimized(input);
        self
    }
    /// <p>Specifies whether the instance is optimized for Amazon EBS I/O. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal EBS I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS Optimized instance.</p>
    pub fn set_ebs_optimized(mut self, input: ::std::option::Option<crate::types::AttributeBooleanValue>) -> Self {
        self.inner = self.inner.set_ebs_optimized(input);
        self
    }
    /// <p>Specifies whether the instance is optimized for Amazon EBS I/O. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal EBS I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS Optimized instance.</p>
    pub fn get_ebs_optimized(&self) -> &::std::option::Option<crate::types::AttributeBooleanValue> {
        self.inner.get_ebs_optimized()
    }
    /// <p>Set to <code>true</code> to enable enhanced networking with ENA for the instance.</p>
    /// <p>This option is supported only for HVM instances. Specifying this option with a PV instance can make it unreachable.</p>
    pub fn ena_support(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.inner = self.inner.ena_support(input);
        self
    }
    /// <p>Set to <code>true</code> to enable enhanced networking with ENA for the instance.</p>
    /// <p>This option is supported only for HVM instances. Specifying this option with a PV instance can make it unreachable.</p>
    pub fn set_ena_support(mut self, input: ::std::option::Option<crate::types::AttributeBooleanValue>) -> Self {
        self.inner = self.inner.set_ena_support(input);
        self
    }
    /// <p>Set to <code>true</code> to enable enhanced networking with ENA for the instance.</p>
    /// <p>This option is supported only for HVM instances. Specifying this option with a PV instance can make it unreachable.</p>
    pub fn get_ena_support(&self) -> &::std::option::Option<crate::types::AttributeBooleanValue> {
        self.inner.get_ena_support()
    }
    /// Appends an item to `Groups`.
    ///
    /// To override the contents of this collection use [`set_groups`](Self::set_groups).
    ///
    /// <p>Replaces the security groups of the instance with the specified security groups. You must specify the ID of at least one security group, even if it's just the default security group for the VPC.</p>
    pub fn groups(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.groups(input.into());
        self
    }
    /// <p>Replaces the security groups of the instance with the specified security groups. You must specify the ID of at least one security group, even if it's just the default security group for the VPC.</p>
    pub fn set_groups(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_groups(input);
        self
    }
    /// <p>Replaces the security groups of the instance with the specified security groups. You must specify the ID of at least one security group, even if it's just the default security group for the VPC.</p>
    pub fn get_groups(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_groups()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>Specifies whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn instance_initiated_shutdown_behavior(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.instance_initiated_shutdown_behavior(input);
        self
    }
    /// <p>Specifies whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn set_instance_initiated_shutdown_behavior(mut self, input: ::std::option::Option<crate::types::AttributeValue>) -> Self {
        self.inner = self.inner.set_instance_initiated_shutdown_behavior(input);
        self
    }
    /// <p>Specifies whether an instance stops or terminates when you initiate shutdown from the instance (using the operating system command for system shutdown).</p>
    pub fn get_instance_initiated_shutdown_behavior(&self) -> &::std::option::Option<crate::types::AttributeValue> {
        self.inner.get_instance_initiated_shutdown_behavior()
    }
    /// <p>Changes the instance type to the specified value. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>. If the instance type is not valid, the error returned is <code>InvalidInstanceAttributeValue</code>.</p>
    pub fn instance_type(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.instance_type(input);
        self
    }
    /// <p>Changes the instance type to the specified value. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>. If the instance type is not valid, the error returned is <code>InvalidInstanceAttributeValue</code>.</p>
    pub fn set_instance_type(mut self, input: ::std::option::Option<crate::types::AttributeValue>) -> Self {
        self.inner = self.inner.set_instance_type(input);
        self
    }
    /// <p>Changes the instance type to the specified value. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>. If the instance type is not valid, the error returned is <code>InvalidInstanceAttributeValue</code>.</p>
    pub fn get_instance_type(&self) -> &::std::option::Option<crate::types::AttributeValue> {
        self.inner.get_instance_type()
    }
    /// <p>Changes the instance's kernel to the specified value. We recommend that you use PV-GRUB instead of kernels and RAM disks. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedKernels.html">PV-GRUB</a>.</p>
    pub fn kernel(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.kernel(input);
        self
    }
    /// <p>Changes the instance's kernel to the specified value. We recommend that you use PV-GRUB instead of kernels and RAM disks. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedKernels.html">PV-GRUB</a>.</p>
    pub fn set_kernel(mut self, input: ::std::option::Option<crate::types::AttributeValue>) -> Self {
        self.inner = self.inner.set_kernel(input);
        self
    }
    /// <p>Changes the instance's kernel to the specified value. We recommend that you use PV-GRUB instead of kernels and RAM disks. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedKernels.html">PV-GRUB</a>.</p>
    pub fn get_kernel(&self) -> &::std::option::Option<crate::types::AttributeValue> {
        self.inner.get_kernel()
    }
    /// <p>Changes the instance's RAM disk to the specified value. We recommend that you use PV-GRUB instead of kernels and RAM disks. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedKernels.html">PV-GRUB</a>.</p>
    pub fn ramdisk(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.ramdisk(input);
        self
    }
    /// <p>Changes the instance's RAM disk to the specified value. We recommend that you use PV-GRUB instead of kernels and RAM disks. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedKernels.html">PV-GRUB</a>.</p>
    pub fn set_ramdisk(mut self, input: ::std::option::Option<crate::types::AttributeValue>) -> Self {
        self.inner = self.inner.set_ramdisk(input);
        self
    }
    /// <p>Changes the instance's RAM disk to the specified value. We recommend that you use PV-GRUB instead of kernels and RAM disks. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/UserProvidedKernels.html">PV-GRUB</a>.</p>
    pub fn get_ramdisk(&self) -> &::std::option::Option<crate::types::AttributeValue> {
        self.inner.get_ramdisk()
    }
    /// <p>Set to <code>simple</code> to enable enhanced networking with the Intel 82599 Virtual Function interface for the instance.</p>
    /// <p>There is no way to disable enhanced networking with the Intel 82599 Virtual Function interface at this time.</p>
    /// <p>This option is supported only for HVM instances. Specifying this option with a PV instance can make it unreachable.</p>
    pub fn sriov_net_support(mut self, input: crate::types::AttributeValue) -> Self {
        self.inner = self.inner.sriov_net_support(input);
        self
    }
    /// <p>Set to <code>simple</code> to enable enhanced networking with the Intel 82599 Virtual Function interface for the instance.</p>
    /// <p>There is no way to disable enhanced networking with the Intel 82599 Virtual Function interface at this time.</p>
    /// <p>This option is supported only for HVM instances. Specifying this option with a PV instance can make it unreachable.</p>
    pub fn set_sriov_net_support(mut self, input: ::std::option::Option<crate::types::AttributeValue>) -> Self {
        self.inner = self.inner.set_sriov_net_support(input);
        self
    }
    /// <p>Set to <code>simple</code> to enable enhanced networking with the Intel 82599 Virtual Function interface for the instance.</p>
    /// <p>There is no way to disable enhanced networking with the Intel 82599 Virtual Function interface at this time.</p>
    /// <p>This option is supported only for HVM instances. Specifying this option with a PV instance can make it unreachable.</p>
    pub fn get_sriov_net_support(&self) -> &::std::option::Option<crate::types::AttributeValue> {
        self.inner.get_sriov_net_support()
    }
    /// <p>Changes the instance's user data to the specified value. If you are using an Amazon Web Services SDK or command line tool, base64-encoding is performed for you, and you can load the text from a file. Otherwise, you must provide base64-encoded text.</p>
    pub fn user_data(mut self, input: crate::types::BlobAttributeValue) -> Self {
        self.inner = self.inner.user_data(input);
        self
    }
    /// <p>Changes the instance's user data to the specified value. If you are using an Amazon Web Services SDK or command line tool, base64-encoding is performed for you, and you can load the text from a file. Otherwise, you must provide base64-encoded text.</p>
    pub fn set_user_data(mut self, input: ::std::option::Option<crate::types::BlobAttributeValue>) -> Self {
        self.inner = self.inner.set_user_data(input);
        self
    }
    /// <p>Changes the instance's user data to the specified value. If you are using an Amazon Web Services SDK or command line tool, base64-encoding is performed for you, and you can load the text from a file. Otherwise, you must provide base64-encoded text.</p>
    pub fn get_user_data(&self) -> &::std::option::Option<crate::types::BlobAttributeValue> {
        self.inner.get_user_data()
    }
    /// <p>A new value for the attribute. Use only with the <code>kernel</code>, <code>ramdisk</code>, <code>userData</code>, <code>disableApiTermination</code>, or <code>instanceInitiatedShutdownBehavior</code> attribute.</p>
    pub fn value(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.value(input.into());
        self
    }
    /// <p>A new value for the attribute. Use only with the <code>kernel</code>, <code>ramdisk</code>, <code>userData</code>, <code>disableApiTermination</code>, or <code>instanceInitiatedShutdownBehavior</code> attribute.</p>
    pub fn set_value(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_value(input);
        self
    }
    /// <p>A new value for the attribute. Use only with the <code>kernel</code>, <code>ramdisk</code>, <code>userData</code>, <code>disableApiTermination</code>, or <code>instanceInitiatedShutdownBehavior</code> attribute.</p>
    pub fn get_value(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_value()
    }
    /// <p>Indicates whether an instance is enabled for stop protection. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection">Stop Protection</a>.</p>
    /// <p></p>
    pub fn disable_api_stop(mut self, input: crate::types::AttributeBooleanValue) -> Self {
        self.inner = self.inner.disable_api_stop(input);
        self
    }
    /// <p>Indicates whether an instance is enabled for stop protection. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection">Stop Protection</a>.</p>
    /// <p></p>
    pub fn set_disable_api_stop(mut self, input: ::std::option::Option<crate::types::AttributeBooleanValue>) -> Self {
        self.inner = self.inner.set_disable_api_stop(input);
        self
    }
    /// <p>Indicates whether an instance is enabled for stop protection. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Stop_Start.html#Using_StopProtection">Stop Protection</a>.</p>
    /// <p></p>
    pub fn get_disable_api_stop(&self) -> &::std::option::Option<crate::types::AttributeBooleanValue> {
        self.inner.get_disable_api_stop()
    }
}
