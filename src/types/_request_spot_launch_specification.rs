// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the launch specification for an instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct RequestSpotLaunchSpecification {
    /// <p>The IDs of the security groups.</p>
    pub security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Not supported.</p>
    pub security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>Deprecated.</p>
    pub addressing_type: ::std::option::Option<::std::string::String>,
    /// <p>The block device mapping entries. You can't specify both a snapshot ID and an encryption value. This is because only blank volumes can be encrypted on creation. If a snapshot is the basis for a volume, it is not blank and its encryption status is used for the volume encryption status.</p>
    pub block_device_mappings: ::std::option::Option<::std::vec::Vec<crate::types::BlockDeviceMapping>>,
    /// <p>Indicates whether the instance is optimized for EBS I/O. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal EBS I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS Optimized instance.</p>
    /// <p>Default: <code>false</code> </p>
    pub ebs_optimized: ::std::option::Option<bool>,
    /// <p>The IAM instance profile.</p>
    pub iam_instance_profile: ::std::option::Option<crate::types::IamInstanceProfileSpecification>,
    /// <p>The ID of the AMI.</p>
    pub image_id: ::std::option::Option<::std::string::String>,
    /// <p>The instance type. Only one instance type can be specified.</p>
    pub instance_type: ::std::option::Option<crate::types::InstanceType>,
    /// <p>The ID of the kernel.</p>
    pub kernel_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the key pair.</p>
    pub key_name: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether basic or detailed monitoring is enabled for the instance.</p>
    /// <p>Default: Disabled</p>
    pub monitoring: ::std::option::Option<crate::types::RunInstancesMonitoringEnabled>,
    /// <p>The network interfaces. If you specify a network interface, you must specify subnet IDs and security group IDs using the network interface.</p>
    pub network_interfaces: ::std::option::Option<::std::vec::Vec<crate::types::InstanceNetworkInterfaceSpecification>>,
    /// <p>The placement information for the instance.</p>
    pub placement: ::std::option::Option<crate::types::SpotPlacement>,
    /// <p>The ID of the RAM disk.</p>
    pub ramdisk_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the subnet in which to launch the instance.</p>
    pub subnet_id: ::std::option::Option<::std::string::String>,
    /// <p>The base64-encoded user data that instances use when starting up. User data is limited to 16 KB.</p>
    pub user_data: ::std::option::Option<::std::string::String>,
}
impl RequestSpotLaunchSpecification {
    /// <p>The IDs of the security groups.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.security_group_ids.is_none()`.
    pub fn security_group_ids(&self) -> &[::std::string::String] {
        self.security_group_ids.as_deref().unwrap_or_default()
    }
    /// <p>Not supported.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.security_groups.is_none()`.
    pub fn security_groups(&self) -> &[::std::string::String] {
        self.security_groups.as_deref().unwrap_or_default()
    }
    /// <p>Deprecated.</p>
    pub fn addressing_type(&self) -> ::std::option::Option<&str> {
        self.addressing_type.as_deref()
    }
    /// <p>The block device mapping entries. You can't specify both a snapshot ID and an encryption value. This is because only blank volumes can be encrypted on creation. If a snapshot is the basis for a volume, it is not blank and its encryption status is used for the volume encryption status.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.block_device_mappings.is_none()`.
    pub fn block_device_mappings(&self) -> &[crate::types::BlockDeviceMapping] {
        self.block_device_mappings.as_deref().unwrap_or_default()
    }
    /// <p>Indicates whether the instance is optimized for EBS I/O. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal EBS I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS Optimized instance.</p>
    /// <p>Default: <code>false</code> </p>
    pub fn ebs_optimized(&self) -> ::std::option::Option<bool> {
        self.ebs_optimized
    }
    /// <p>The IAM instance profile.</p>
    pub fn iam_instance_profile(&self) -> ::std::option::Option<&crate::types::IamInstanceProfileSpecification> {
        self.iam_instance_profile.as_ref()
    }
    /// <p>The ID of the AMI.</p>
    pub fn image_id(&self) -> ::std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>The instance type. Only one instance type can be specified.</p>
    pub fn instance_type(&self) -> ::std::option::Option<&crate::types::InstanceType> {
        self.instance_type.as_ref()
    }
    /// <p>The ID of the kernel.</p>
    pub fn kernel_id(&self) -> ::std::option::Option<&str> {
        self.kernel_id.as_deref()
    }
    /// <p>The name of the key pair.</p>
    pub fn key_name(&self) -> ::std::option::Option<&str> {
        self.key_name.as_deref()
    }
    /// <p>Indicates whether basic or detailed monitoring is enabled for the instance.</p>
    /// <p>Default: Disabled</p>
    pub fn monitoring(&self) -> ::std::option::Option<&crate::types::RunInstancesMonitoringEnabled> {
        self.monitoring.as_ref()
    }
    /// <p>The network interfaces. If you specify a network interface, you must specify subnet IDs and security group IDs using the network interface.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.network_interfaces.is_none()`.
    pub fn network_interfaces(&self) -> &[crate::types::InstanceNetworkInterfaceSpecification] {
        self.network_interfaces.as_deref().unwrap_or_default()
    }
    /// <p>The placement information for the instance.</p>
    pub fn placement(&self) -> ::std::option::Option<&crate::types::SpotPlacement> {
        self.placement.as_ref()
    }
    /// <p>The ID of the RAM disk.</p>
    pub fn ramdisk_id(&self) -> ::std::option::Option<&str> {
        self.ramdisk_id.as_deref()
    }
    /// <p>The ID of the subnet in which to launch the instance.</p>
    pub fn subnet_id(&self) -> ::std::option::Option<&str> {
        self.subnet_id.as_deref()
    }
    /// <p>The base64-encoded user data that instances use when starting up. User data is limited to 16 KB.</p>
    pub fn user_data(&self) -> ::std::option::Option<&str> {
        self.user_data.as_deref()
    }
}
impl ::std::fmt::Debug for RequestSpotLaunchSpecification {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("RequestSpotLaunchSpecification");
        formatter.field("security_group_ids", &self.security_group_ids);
        formatter.field("security_groups", &self.security_groups);
        formatter.field("addressing_type", &self.addressing_type);
        formatter.field("block_device_mappings", &self.block_device_mappings);
        formatter.field("ebs_optimized", &self.ebs_optimized);
        formatter.field("iam_instance_profile", &self.iam_instance_profile);
        formatter.field("image_id", &self.image_id);
        formatter.field("instance_type", &self.instance_type);
        formatter.field("kernel_id", &self.kernel_id);
        formatter.field("key_name", &self.key_name);
        formatter.field("monitoring", &self.monitoring);
        formatter.field("network_interfaces", &self.network_interfaces);
        formatter.field("placement", &self.placement);
        formatter.field("ramdisk_id", &self.ramdisk_id);
        formatter.field("subnet_id", &self.subnet_id);
        formatter.field("user_data", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}
impl RequestSpotLaunchSpecification {
    /// Creates a new builder-style object to manufacture [`RequestSpotLaunchSpecification`](crate::types::RequestSpotLaunchSpecification).
    pub fn builder() -> crate::types::builders::RequestSpotLaunchSpecificationBuilder {
        crate::types::builders::RequestSpotLaunchSpecificationBuilder::default()
    }
}

/// A builder for [`RequestSpotLaunchSpecification`](crate::types::RequestSpotLaunchSpecification).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct RequestSpotLaunchSpecificationBuilder {
    pub(crate) security_group_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) security_groups: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) addressing_type: ::std::option::Option<::std::string::String>,
    pub(crate) block_device_mappings: ::std::option::Option<::std::vec::Vec<crate::types::BlockDeviceMapping>>,
    pub(crate) ebs_optimized: ::std::option::Option<bool>,
    pub(crate) iam_instance_profile: ::std::option::Option<crate::types::IamInstanceProfileSpecification>,
    pub(crate) image_id: ::std::option::Option<::std::string::String>,
    pub(crate) instance_type: ::std::option::Option<crate::types::InstanceType>,
    pub(crate) kernel_id: ::std::option::Option<::std::string::String>,
    pub(crate) key_name: ::std::option::Option<::std::string::String>,
    pub(crate) monitoring: ::std::option::Option<crate::types::RunInstancesMonitoringEnabled>,
    pub(crate) network_interfaces: ::std::option::Option<::std::vec::Vec<crate::types::InstanceNetworkInterfaceSpecification>>,
    pub(crate) placement: ::std::option::Option<crate::types::SpotPlacement>,
    pub(crate) ramdisk_id: ::std::option::Option<::std::string::String>,
    pub(crate) subnet_id: ::std::option::Option<::std::string::String>,
    pub(crate) user_data: ::std::option::Option<::std::string::String>,
}
impl RequestSpotLaunchSpecificationBuilder {
    /// Appends an item to `security_group_ids`.
    ///
    /// To override the contents of this collection use [`set_security_group_ids`](Self::set_security_group_ids).
    ///
    /// <p>The IDs of the security groups.</p>
    pub fn security_group_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.security_group_ids.unwrap_or_default();
        v.push(input.into());
        self.security_group_ids = ::std::option::Option::Some(v);
        self
    }
    /// <p>The IDs of the security groups.</p>
    pub fn set_security_group_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.security_group_ids = input;
        self
    }
    /// <p>The IDs of the security groups.</p>
    pub fn get_security_group_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.security_group_ids
    }
    /// Appends an item to `security_groups`.
    ///
    /// To override the contents of this collection use [`set_security_groups`](Self::set_security_groups).
    ///
    /// <p>Not supported.</p>
    pub fn security_groups(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.security_groups.unwrap_or_default();
        v.push(input.into());
        self.security_groups = ::std::option::Option::Some(v);
        self
    }
    /// <p>Not supported.</p>
    pub fn set_security_groups(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.security_groups = input;
        self
    }
    /// <p>Not supported.</p>
    pub fn get_security_groups(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.security_groups
    }
    /// <p>Deprecated.</p>
    pub fn addressing_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.addressing_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Deprecated.</p>
    pub fn set_addressing_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.addressing_type = input;
        self
    }
    /// <p>Deprecated.</p>
    pub fn get_addressing_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.addressing_type
    }
    /// Appends an item to `block_device_mappings`.
    ///
    /// To override the contents of this collection use [`set_block_device_mappings`](Self::set_block_device_mappings).
    ///
    /// <p>The block device mapping entries. You can't specify both a snapshot ID and an encryption value. This is because only blank volumes can be encrypted on creation. If a snapshot is the basis for a volume, it is not blank and its encryption status is used for the volume encryption status.</p>
    pub fn block_device_mappings(mut self, input: crate::types::BlockDeviceMapping) -> Self {
        let mut v = self.block_device_mappings.unwrap_or_default();
        v.push(input);
        self.block_device_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>The block device mapping entries. You can't specify both a snapshot ID and an encryption value. This is because only blank volumes can be encrypted on creation. If a snapshot is the basis for a volume, it is not blank and its encryption status is used for the volume encryption status.</p>
    pub fn set_block_device_mappings(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::BlockDeviceMapping>>) -> Self {
        self.block_device_mappings = input;
        self
    }
    /// <p>The block device mapping entries. You can't specify both a snapshot ID and an encryption value. This is because only blank volumes can be encrypted on creation. If a snapshot is the basis for a volume, it is not blank and its encryption status is used for the volume encryption status.</p>
    pub fn get_block_device_mappings(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::BlockDeviceMapping>> {
        &self.block_device_mappings
    }
    /// <p>Indicates whether the instance is optimized for EBS I/O. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal EBS I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS Optimized instance.</p>
    /// <p>Default: <code>false</code> </p>
    pub fn ebs_optimized(mut self, input: bool) -> Self {
        self.ebs_optimized = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the instance is optimized for EBS I/O. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal EBS I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS Optimized instance.</p>
    /// <p>Default: <code>false</code> </p>
    pub fn set_ebs_optimized(mut self, input: ::std::option::Option<bool>) -> Self {
        self.ebs_optimized = input;
        self
    }
    /// <p>Indicates whether the instance is optimized for EBS I/O. This optimization provides dedicated throughput to Amazon EBS and an optimized configuration stack to provide optimal EBS I/O performance. This optimization isn't available with all instance types. Additional usage charges apply when using an EBS Optimized instance.</p>
    /// <p>Default: <code>false</code> </p>
    pub fn get_ebs_optimized(&self) -> &::std::option::Option<bool> {
        &self.ebs_optimized
    }
    /// <p>The IAM instance profile.</p>
    pub fn iam_instance_profile(mut self, input: crate::types::IamInstanceProfileSpecification) -> Self {
        self.iam_instance_profile = ::std::option::Option::Some(input);
        self
    }
    /// <p>The IAM instance profile.</p>
    pub fn set_iam_instance_profile(mut self, input: ::std::option::Option<crate::types::IamInstanceProfileSpecification>) -> Self {
        self.iam_instance_profile = input;
        self
    }
    /// <p>The IAM instance profile.</p>
    pub fn get_iam_instance_profile(&self) -> &::std::option::Option<crate::types::IamInstanceProfileSpecification> {
        &self.iam_instance_profile
    }
    /// <p>The ID of the AMI.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>The ID of the AMI.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.image_id
    }
    /// <p>The instance type. Only one instance type can be specified.</p>
    pub fn instance_type(mut self, input: crate::types::InstanceType) -> Self {
        self.instance_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The instance type. Only one instance type can be specified.</p>
    pub fn set_instance_type(mut self, input: ::std::option::Option<crate::types::InstanceType>) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The instance type. Only one instance type can be specified.</p>
    pub fn get_instance_type(&self) -> &::std::option::Option<crate::types::InstanceType> {
        &self.instance_type
    }
    /// <p>The ID of the kernel.</p>
    pub fn kernel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kernel_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the kernel.</p>
    pub fn set_kernel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kernel_id = input;
        self
    }
    /// <p>The ID of the kernel.</p>
    pub fn get_kernel_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kernel_id
    }
    /// <p>The name of the key pair.</p>
    pub fn key_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.key_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the key pair.</p>
    pub fn set_key_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.key_name = input;
        self
    }
    /// <p>The name of the key pair.</p>
    pub fn get_key_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.key_name
    }
    /// <p>Indicates whether basic or detailed monitoring is enabled for the instance.</p>
    /// <p>Default: Disabled</p>
    pub fn monitoring(mut self, input: crate::types::RunInstancesMonitoringEnabled) -> Self {
        self.monitoring = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether basic or detailed monitoring is enabled for the instance.</p>
    /// <p>Default: Disabled</p>
    pub fn set_monitoring(mut self, input: ::std::option::Option<crate::types::RunInstancesMonitoringEnabled>) -> Self {
        self.monitoring = input;
        self
    }
    /// <p>Indicates whether basic or detailed monitoring is enabled for the instance.</p>
    /// <p>Default: Disabled</p>
    pub fn get_monitoring(&self) -> &::std::option::Option<crate::types::RunInstancesMonitoringEnabled> {
        &self.monitoring
    }
    /// Appends an item to `network_interfaces`.
    ///
    /// To override the contents of this collection use [`set_network_interfaces`](Self::set_network_interfaces).
    ///
    /// <p>The network interfaces. If you specify a network interface, you must specify subnet IDs and security group IDs using the network interface.</p>
    pub fn network_interfaces(mut self, input: crate::types::InstanceNetworkInterfaceSpecification) -> Self {
        let mut v = self.network_interfaces.unwrap_or_default();
        v.push(input);
        self.network_interfaces = ::std::option::Option::Some(v);
        self
    }
    /// <p>The network interfaces. If you specify a network interface, you must specify subnet IDs and security group IDs using the network interface.</p>
    pub fn set_network_interfaces(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceNetworkInterfaceSpecification>>,
    ) -> Self {
        self.network_interfaces = input;
        self
    }
    /// <p>The network interfaces. If you specify a network interface, you must specify subnet IDs and security group IDs using the network interface.</p>
    pub fn get_network_interfaces(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceNetworkInterfaceSpecification>> {
        &self.network_interfaces
    }
    /// <p>The placement information for the instance.</p>
    pub fn placement(mut self, input: crate::types::SpotPlacement) -> Self {
        self.placement = ::std::option::Option::Some(input);
        self
    }
    /// <p>The placement information for the instance.</p>
    pub fn set_placement(mut self, input: ::std::option::Option<crate::types::SpotPlacement>) -> Self {
        self.placement = input;
        self
    }
    /// <p>The placement information for the instance.</p>
    pub fn get_placement(&self) -> &::std::option::Option<crate::types::SpotPlacement> {
        &self.placement
    }
    /// <p>The ID of the RAM disk.</p>
    pub fn ramdisk_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.ramdisk_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the RAM disk.</p>
    pub fn set_ramdisk_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.ramdisk_id = input;
        self
    }
    /// <p>The ID of the RAM disk.</p>
    pub fn get_ramdisk_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.ramdisk_id
    }
    /// <p>The ID of the subnet in which to launch the instance.</p>
    pub fn subnet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.subnet_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the subnet in which to launch the instance.</p>
    pub fn set_subnet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.subnet_id = input;
        self
    }
    /// <p>The ID of the subnet in which to launch the instance.</p>
    pub fn get_subnet_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.subnet_id
    }
    /// <p>The base64-encoded user data that instances use when starting up. User data is limited to 16 KB.</p>
    pub fn user_data(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.user_data = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The base64-encoded user data that instances use when starting up. User data is limited to 16 KB.</p>
    pub fn set_user_data(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.user_data = input;
        self
    }
    /// <p>The base64-encoded user data that instances use when starting up. User data is limited to 16 KB.</p>
    pub fn get_user_data(&self) -> &::std::option::Option<::std::string::String> {
        &self.user_data
    }
    /// Consumes the builder and constructs a [`RequestSpotLaunchSpecification`](crate::types::RequestSpotLaunchSpecification).
    pub fn build(self) -> crate::types::RequestSpotLaunchSpecification {
        crate::types::RequestSpotLaunchSpecification {
            security_group_ids: self.security_group_ids,
            security_groups: self.security_groups,
            addressing_type: self.addressing_type,
            block_device_mappings: self.block_device_mappings,
            ebs_optimized: self.ebs_optimized,
            iam_instance_profile: self.iam_instance_profile,
            image_id: self.image_id,
            instance_type: self.instance_type,
            kernel_id: self.kernel_id,
            key_name: self.key_name,
            monitoring: self.monitoring,
            network_interfaces: self.network_interfaces,
            placement: self.placement,
            ramdisk_id: self.ramdisk_id,
            subnet_id: self.subnet_id,
            user_data: self.user_data,
        }
    }
}
impl ::std::fmt::Debug for RequestSpotLaunchSpecificationBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("RequestSpotLaunchSpecificationBuilder");
        formatter.field("security_group_ids", &self.security_group_ids);
        formatter.field("security_groups", &self.security_groups);
        formatter.field("addressing_type", &self.addressing_type);
        formatter.field("block_device_mappings", &self.block_device_mappings);
        formatter.field("ebs_optimized", &self.ebs_optimized);
        formatter.field("iam_instance_profile", &self.iam_instance_profile);
        formatter.field("image_id", &self.image_id);
        formatter.field("instance_type", &self.instance_type);
        formatter.field("kernel_id", &self.kernel_id);
        formatter.field("key_name", &self.key_name);
        formatter.field("monitoring", &self.monitoring);
        formatter.field("network_interfaces", &self.network_interfaces);
        formatter.field("placement", &self.placement);
        formatter.field("ramdisk_id", &self.ramdisk_id);
        formatter.field("subnet_id", &self.subnet_id);
        formatter.field("user_data", &"*** Sensitive Data Redacted ***");
        formatter.finish()
    }
}