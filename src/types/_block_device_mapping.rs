// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a block device mapping, which defines the EBS volumes and instance store volumes to attach to an instance at launch.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct BlockDeviceMapping {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub device_name: ::std::option::Option<::std::string::String>,
    /// <p>The virtual device name (<code>ephemeral</code>N). Instance store volumes are numbered starting from 0. An instance type with 2 available instance store volumes can specify mappings for <code>ephemeral0</code> and <code>ephemeral1</code>. The number of available instance store volumes depends on the instance type. After you connect to the instance, you must mount the volume.</p>
    /// <p>NVMe instance store volumes are automatically enumerated and assigned a device name. Including them in your block device mapping has no effect.</p>
    /// <p>Constraints: For M3 instances, you must specify instance store volumes in the block device mapping for the instance. When you launch an M3 instance, we ignore any instance store volumes specified in the block device mapping for the AMI.</p>
    pub virtual_name: ::std::option::Option<::std::string::String>,
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub ebs: ::std::option::Option<crate::types::EbsBlockDevice>,
    /// <p>To omit the device from the block device mapping, specify an empty string. When this property is specified, the device is removed from the block device mapping regardless of the assigned value.</p>
    pub no_device: ::std::option::Option<::std::string::String>,
}
impl BlockDeviceMapping {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn device_name(&self) -> ::std::option::Option<&str> {
        self.device_name.as_deref()
    }
    /// <p>The virtual device name (<code>ephemeral</code>N). Instance store volumes are numbered starting from 0. An instance type with 2 available instance store volumes can specify mappings for <code>ephemeral0</code> and <code>ephemeral1</code>. The number of available instance store volumes depends on the instance type. After you connect to the instance, you must mount the volume.</p>
    /// <p>NVMe instance store volumes are automatically enumerated and assigned a device name. Including them in your block device mapping has no effect.</p>
    /// <p>Constraints: For M3 instances, you must specify instance store volumes in the block device mapping for the instance. When you launch an M3 instance, we ignore any instance store volumes specified in the block device mapping for the AMI.</p>
    pub fn virtual_name(&self) -> ::std::option::Option<&str> {
        self.virtual_name.as_deref()
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn ebs(&self) -> ::std::option::Option<&crate::types::EbsBlockDevice> {
        self.ebs.as_ref()
    }
    /// <p>To omit the device from the block device mapping, specify an empty string. When this property is specified, the device is removed from the block device mapping regardless of the assigned value.</p>
    pub fn no_device(&self) -> ::std::option::Option<&str> {
        self.no_device.as_deref()
    }
}
impl BlockDeviceMapping {
    /// Creates a new builder-style object to manufacture [`BlockDeviceMapping`](crate::types::BlockDeviceMapping).
    pub fn builder() -> crate::types::builders::BlockDeviceMappingBuilder {
        crate::types::builders::BlockDeviceMappingBuilder::default()
    }
}

/// A builder for [`BlockDeviceMapping`](crate::types::BlockDeviceMapping).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct BlockDeviceMappingBuilder {
    pub(crate) device_name: ::std::option::Option<::std::string::String>,
    pub(crate) virtual_name: ::std::option::Option<::std::string::String>,
    pub(crate) ebs: ::std::option::Option<crate::types::EbsBlockDevice>,
    pub(crate) no_device: ::std::option::Option<::std::string::String>,
}
impl BlockDeviceMappingBuilder {
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn device_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.device_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn set_device_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.device_name = input;
        self
    }
    /// <p>The device name (for example, <code>/dev/sdh</code> or <code>xvdh</code>).</p>
    pub fn get_device_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.device_name
    }
    /// <p>The virtual device name (<code>ephemeral</code>N). Instance store volumes are numbered starting from 0. An instance type with 2 available instance store volumes can specify mappings for <code>ephemeral0</code> and <code>ephemeral1</code>. The number of available instance store volumes depends on the instance type. After you connect to the instance, you must mount the volume.</p>
    /// <p>NVMe instance store volumes are automatically enumerated and assigned a device name. Including them in your block device mapping has no effect.</p>
    /// <p>Constraints: For M3 instances, you must specify instance store volumes in the block device mapping for the instance. When you launch an M3 instance, we ignore any instance store volumes specified in the block device mapping for the AMI.</p>
    pub fn virtual_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.virtual_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The virtual device name (<code>ephemeral</code>N). Instance store volumes are numbered starting from 0. An instance type with 2 available instance store volumes can specify mappings for <code>ephemeral0</code> and <code>ephemeral1</code>. The number of available instance store volumes depends on the instance type. After you connect to the instance, you must mount the volume.</p>
    /// <p>NVMe instance store volumes are automatically enumerated and assigned a device name. Including them in your block device mapping has no effect.</p>
    /// <p>Constraints: For M3 instances, you must specify instance store volumes in the block device mapping for the instance. When you launch an M3 instance, we ignore any instance store volumes specified in the block device mapping for the AMI.</p>
    pub fn set_virtual_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.virtual_name = input;
        self
    }
    /// <p>The virtual device name (<code>ephemeral</code>N). Instance store volumes are numbered starting from 0. An instance type with 2 available instance store volumes can specify mappings for <code>ephemeral0</code> and <code>ephemeral1</code>. The number of available instance store volumes depends on the instance type. After you connect to the instance, you must mount the volume.</p>
    /// <p>NVMe instance store volumes are automatically enumerated and assigned a device name. Including them in your block device mapping has no effect.</p>
    /// <p>Constraints: For M3 instances, you must specify instance store volumes in the block device mapping for the instance. When you launch an M3 instance, we ignore any instance store volumes specified in the block device mapping for the AMI.</p>
    pub fn get_virtual_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.virtual_name
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn ebs(mut self, input: crate::types::EbsBlockDevice) -> Self {
        self.ebs = ::std::option::Option::Some(input);
        self
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn set_ebs(mut self, input: ::std::option::Option<crate::types::EbsBlockDevice>) -> Self {
        self.ebs = input;
        self
    }
    /// <p>Parameters used to automatically set up EBS volumes when the instance is launched.</p>
    pub fn get_ebs(&self) -> &::std::option::Option<crate::types::EbsBlockDevice> {
        &self.ebs
    }
    /// <p>To omit the device from the block device mapping, specify an empty string. When this property is specified, the device is removed from the block device mapping regardless of the assigned value.</p>
    pub fn no_device(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.no_device = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>To omit the device from the block device mapping, specify an empty string. When this property is specified, the device is removed from the block device mapping regardless of the assigned value.</p>
    pub fn set_no_device(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.no_device = input;
        self
    }
    /// <p>To omit the device from the block device mapping, specify an empty string. When this property is specified, the device is removed from the block device mapping regardless of the assigned value.</p>
    pub fn get_no_device(&self) -> &::std::option::Option<::std::string::String> {
        &self.no_device
    }
    /// Consumes the builder and constructs a [`BlockDeviceMapping`](crate::types::BlockDeviceMapping).
    pub fn build(self) -> crate::types::BlockDeviceMapping {
        crate::types::BlockDeviceMapping {
            device_name: self.device_name,
            virtual_name: self.virtual_name,
            ebs: self.ebs,
            no_device: self.no_device,
        }
    }
}