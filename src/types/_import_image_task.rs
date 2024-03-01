// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an import image task.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ImportImageTask {
    /// <p>The architecture of the virtual machine.</p>
    /// <p>Valid values: <code>i386</code> | <code>x86_64</code> | <code>arm64</code> </p>
    pub architecture: ::std::option::Option<::std::string::String>,
    /// <p>A description of the import task.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>Indicates whether the image is encrypted.</p>
    pub encrypted: ::std::option::Option<bool>,
    /// <p>The target hypervisor for the import task.</p>
    /// <p>Valid values: <code>xen</code> </p>
    pub hypervisor: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Amazon Machine Image (AMI) of the imported virtual machine.</p>
    pub image_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the import image task.</p>
    pub import_task_id: ::std::option::Option<::std::string::String>,
    /// <p>The identifier for the KMS key that was used to create the encrypted image.</p>
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>The license type of the virtual machine.</p>
    pub license_type: ::std::option::Option<::std::string::String>,
    /// <p>The description string for the import image task.</p>
    pub platform: ::std::option::Option<::std::string::String>,
    /// <p>The percentage of progress of the import image task.</p>
    pub progress: ::std::option::Option<::std::string::String>,
    /// <p>Information about the snapshots.</p>
    pub snapshot_details: ::std::option::Option<::std::vec::Vec<crate::types::SnapshotDetail>>,
    /// <p>A brief status for the import image task.</p>
    pub status: ::std::option::Option<::std::string::String>,
    /// <p>A descriptive status message for the import image task.</p>
    pub status_message: ::std::option::Option<::std::string::String>,
    /// <p>The tags for the import image task.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The ARNs of the license configurations that are associated with the import image task.</p>
    pub license_specifications: ::std::option::Option<::std::vec::Vec<crate::types::ImportImageLicenseConfigurationResponse>>,
    /// <p>The usage operation value.</p>
    pub usage_operation: ::std::option::Option<::std::string::String>,
    /// <p>The boot mode of the virtual machine.</p>
    pub boot_mode: ::std::option::Option<crate::types::BootModeValues>,
}
impl ImportImageTask {
    /// <p>The architecture of the virtual machine.</p>
    /// <p>Valid values: <code>i386</code> | <code>x86_64</code> | <code>arm64</code> </p>
    pub fn architecture(&self) -> ::std::option::Option<&str> {
        self.architecture.as_deref()
    }
    /// <p>A description of the import task.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>Indicates whether the image is encrypted.</p>
    pub fn encrypted(&self) -> ::std::option::Option<bool> {
        self.encrypted
    }
    /// <p>The target hypervisor for the import task.</p>
    /// <p>Valid values: <code>xen</code> </p>
    pub fn hypervisor(&self) -> ::std::option::Option<&str> {
        self.hypervisor.as_deref()
    }
    /// <p>The ID of the Amazon Machine Image (AMI) of the imported virtual machine.</p>
    pub fn image_id(&self) -> ::std::option::Option<&str> {
        self.image_id.as_deref()
    }
    /// <p>The ID of the import image task.</p>
    pub fn import_task_id(&self) -> ::std::option::Option<&str> {
        self.import_task_id.as_deref()
    }
    /// <p>The identifier for the KMS key that was used to create the encrypted image.</p>
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>The license type of the virtual machine.</p>
    pub fn license_type(&self) -> ::std::option::Option<&str> {
        self.license_type.as_deref()
    }
    /// <p>The description string for the import image task.</p>
    pub fn platform(&self) -> ::std::option::Option<&str> {
        self.platform.as_deref()
    }
    /// <p>The percentage of progress of the import image task.</p>
    pub fn progress(&self) -> ::std::option::Option<&str> {
        self.progress.as_deref()
    }
    /// <p>Information about the snapshots.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.snapshot_details.is_none()`.
    pub fn snapshot_details(&self) -> &[crate::types::SnapshotDetail] {
        self.snapshot_details.as_deref().unwrap_or_default()
    }
    /// <p>A brief status for the import image task.</p>
    pub fn status(&self) -> ::std::option::Option<&str> {
        self.status.as_deref()
    }
    /// <p>A descriptive status message for the import image task.</p>
    pub fn status_message(&self) -> ::std::option::Option<&str> {
        self.status_message.as_deref()
    }
    /// <p>The tags for the import image task.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
    /// <p>The ARNs of the license configurations that are associated with the import image task.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.license_specifications.is_none()`.
    pub fn license_specifications(&self) -> &[crate::types::ImportImageLicenseConfigurationResponse] {
        self.license_specifications.as_deref().unwrap_or_default()
    }
    /// <p>The usage operation value.</p>
    pub fn usage_operation(&self) -> ::std::option::Option<&str> {
        self.usage_operation.as_deref()
    }
    /// <p>The boot mode of the virtual machine.</p>
    pub fn boot_mode(&self) -> ::std::option::Option<&crate::types::BootModeValues> {
        self.boot_mode.as_ref()
    }
}
impl ImportImageTask {
    /// Creates a new builder-style object to manufacture [`ImportImageTask`](crate::types::ImportImageTask).
    pub fn builder() -> crate::types::builders::ImportImageTaskBuilder {
        crate::types::builders::ImportImageTaskBuilder::default()
    }
}

/// A builder for [`ImportImageTask`](crate::types::ImportImageTask).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ImportImageTaskBuilder {
    pub(crate) architecture: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) encrypted: ::std::option::Option<bool>,
    pub(crate) hypervisor: ::std::option::Option<::std::string::String>,
    pub(crate) image_id: ::std::option::Option<::std::string::String>,
    pub(crate) import_task_id: ::std::option::Option<::std::string::String>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) license_type: ::std::option::Option<::std::string::String>,
    pub(crate) platform: ::std::option::Option<::std::string::String>,
    pub(crate) progress: ::std::option::Option<::std::string::String>,
    pub(crate) snapshot_details: ::std::option::Option<::std::vec::Vec<crate::types::SnapshotDetail>>,
    pub(crate) status: ::std::option::Option<::std::string::String>,
    pub(crate) status_message: ::std::option::Option<::std::string::String>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) license_specifications: ::std::option::Option<::std::vec::Vec<crate::types::ImportImageLicenseConfigurationResponse>>,
    pub(crate) usage_operation: ::std::option::Option<::std::string::String>,
    pub(crate) boot_mode: ::std::option::Option<crate::types::BootModeValues>,
}
impl ImportImageTaskBuilder {
    /// <p>The architecture of the virtual machine.</p>
    /// <p>Valid values: <code>i386</code> | <code>x86_64</code> | <code>arm64</code> </p>
    pub fn architecture(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.architecture = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The architecture of the virtual machine.</p>
    /// <p>Valid values: <code>i386</code> | <code>x86_64</code> | <code>arm64</code> </p>
    pub fn set_architecture(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.architecture = input;
        self
    }
    /// <p>The architecture of the virtual machine.</p>
    /// <p>Valid values: <code>i386</code> | <code>x86_64</code> | <code>arm64</code> </p>
    pub fn get_architecture(&self) -> &::std::option::Option<::std::string::String> {
        &self.architecture
    }
    /// <p>A description of the import task.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the import task.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description of the import task.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>Indicates whether the image is encrypted.</p>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the image is encrypted.</p>
    pub fn set_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.encrypted = input;
        self
    }
    /// <p>Indicates whether the image is encrypted.</p>
    pub fn get_encrypted(&self) -> &::std::option::Option<bool> {
        &self.encrypted
    }
    /// <p>The target hypervisor for the import task.</p>
    /// <p>Valid values: <code>xen</code> </p>
    pub fn hypervisor(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.hypervisor = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The target hypervisor for the import task.</p>
    /// <p>Valid values: <code>xen</code> </p>
    pub fn set_hypervisor(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.hypervisor = input;
        self
    }
    /// <p>The target hypervisor for the import task.</p>
    /// <p>Valid values: <code>xen</code> </p>
    pub fn get_hypervisor(&self) -> &::std::option::Option<::std::string::String> {
        &self.hypervisor
    }
    /// <p>The ID of the Amazon Machine Image (AMI) of the imported virtual machine.</p>
    pub fn image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Machine Image (AMI) of the imported virtual machine.</p>
    pub fn set_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.image_id = input;
        self
    }
    /// <p>The ID of the Amazon Machine Image (AMI) of the imported virtual machine.</p>
    pub fn get_image_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.image_id
    }
    /// <p>The ID of the import image task.</p>
    pub fn import_task_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.import_task_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the import image task.</p>
    pub fn set_import_task_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.import_task_id = input;
        self
    }
    /// <p>The ID of the import image task.</p>
    pub fn get_import_task_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.import_task_id
    }
    /// <p>The identifier for the KMS key that was used to create the encrypted image.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.kms_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the KMS key that was used to create the encrypted image.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.kms_key_id = input;
        self
    }
    /// <p>The identifier for the KMS key that was used to create the encrypted image.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.kms_key_id
    }
    /// <p>The license type of the virtual machine.</p>
    pub fn license_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.license_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The license type of the virtual machine.</p>
    pub fn set_license_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.license_type = input;
        self
    }
    /// <p>The license type of the virtual machine.</p>
    pub fn get_license_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.license_type
    }
    /// <p>The description string for the import image task.</p>
    pub fn platform(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.platform = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description string for the import image task.</p>
    pub fn set_platform(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.platform = input;
        self
    }
    /// <p>The description string for the import image task.</p>
    pub fn get_platform(&self) -> &::std::option::Option<::std::string::String> {
        &self.platform
    }
    /// <p>The percentage of progress of the import image task.</p>
    pub fn progress(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.progress = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The percentage of progress of the import image task.</p>
    pub fn set_progress(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.progress = input;
        self
    }
    /// <p>The percentage of progress of the import image task.</p>
    pub fn get_progress(&self) -> &::std::option::Option<::std::string::String> {
        &self.progress
    }
    /// Appends an item to `snapshot_details`.
    ///
    /// To override the contents of this collection use [`set_snapshot_details`](Self::set_snapshot_details).
    ///
    /// <p>Information about the snapshots.</p>
    pub fn snapshot_details(mut self, input: crate::types::SnapshotDetail) -> Self {
        let mut v = self.snapshot_details.unwrap_or_default();
        v.push(input);
        self.snapshot_details = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the snapshots.</p>
    pub fn set_snapshot_details(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SnapshotDetail>>) -> Self {
        self.snapshot_details = input;
        self
    }
    /// <p>Information about the snapshots.</p>
    pub fn get_snapshot_details(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SnapshotDetail>> {
        &self.snapshot_details
    }
    /// <p>A brief status for the import image task.</p>
    pub fn status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A brief status for the import image task.</p>
    pub fn set_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status = input;
        self
    }
    /// <p>A brief status for the import image task.</p>
    pub fn get_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.status
    }
    /// <p>A descriptive status message for the import image task.</p>
    pub fn status_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.status_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A descriptive status message for the import image task.</p>
    pub fn set_status_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.status_message = input;
        self
    }
    /// <p>A descriptive status message for the import image task.</p>
    pub fn get_status_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.status_message
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the import image task.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags for the import image task.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags for the import image task.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Appends an item to `license_specifications`.
    ///
    /// To override the contents of this collection use [`set_license_specifications`](Self::set_license_specifications).
    ///
    /// <p>The ARNs of the license configurations that are associated with the import image task.</p>
    pub fn license_specifications(mut self, input: crate::types::ImportImageLicenseConfigurationResponse) -> Self {
        let mut v = self.license_specifications.unwrap_or_default();
        v.push(input);
        self.license_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The ARNs of the license configurations that are associated with the import image task.</p>
    pub fn set_license_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ImportImageLicenseConfigurationResponse>>,
    ) -> Self {
        self.license_specifications = input;
        self
    }
    /// <p>The ARNs of the license configurations that are associated with the import image task.</p>
    pub fn get_license_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ImportImageLicenseConfigurationResponse>> {
        &self.license_specifications
    }
    /// <p>The usage operation value.</p>
    pub fn usage_operation(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.usage_operation = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The usage operation value.</p>
    pub fn set_usage_operation(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.usage_operation = input;
        self
    }
    /// <p>The usage operation value.</p>
    pub fn get_usage_operation(&self) -> &::std::option::Option<::std::string::String> {
        &self.usage_operation
    }
    /// <p>The boot mode of the virtual machine.</p>
    pub fn boot_mode(mut self, input: crate::types::BootModeValues) -> Self {
        self.boot_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The boot mode of the virtual machine.</p>
    pub fn set_boot_mode(mut self, input: ::std::option::Option<crate::types::BootModeValues>) -> Self {
        self.boot_mode = input;
        self
    }
    /// <p>The boot mode of the virtual machine.</p>
    pub fn get_boot_mode(&self) -> &::std::option::Option<crate::types::BootModeValues> {
        &self.boot_mode
    }
    /// Consumes the builder and constructs a [`ImportImageTask`](crate::types::ImportImageTask).
    pub fn build(self) -> crate::types::ImportImageTask {
        crate::types::ImportImageTask {
            architecture: self.architecture,
            description: self.description,
            encrypted: self.encrypted,
            hypervisor: self.hypervisor,
            image_id: self.image_id,
            import_task_id: self.import_task_id,
            kms_key_id: self.kms_key_id,
            license_type: self.license_type,
            platform: self.platform,
            progress: self.progress,
            snapshot_details: self.snapshot_details,
            status: self.status,
            status_message: self.status_message,
            tags: self.tags,
            license_specifications: self.license_specifications,
            usage_operation: self.usage_operation,
            boot_mode: self.boot_mode,
        }
    }
}