// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an Amazon FPGA image (AFI).</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct FpgaImage {
    /// <p>The FPGA image identifier (AFI ID).</p>
    pub fpga_image_id: ::std::option::Option<::std::string::String>,
    /// <p>The global FPGA image identifier (AGFI ID).</p>
    pub fpga_image_global_id: ::std::option::Option<::std::string::String>,
    /// <p>The name of the AFI.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>The description of the AFI.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The version of the Amazon Web Services Shell that was used to create the bitstream.</p>
    pub shell_version: ::std::option::Option<::std::string::String>,
    /// <p>Information about the PCI bus.</p>
    pub pci_id: ::std::option::Option<crate::types::PciId>,
    /// <p>Information about the state of the AFI.</p>
    pub state: ::std::option::Option<crate::types::FpgaImageState>,
    /// <p>The date and time the AFI was created.</p>
    pub create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The time of the most recent update to the AFI.</p>
    pub update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The ID of the Amazon Web Services account that owns the AFI.</p>
    pub owner_id: ::std::option::Option<::std::string::String>,
    /// <p>The alias of the AFI owner. Possible values include <code>self</code>, <code>amazon</code>, and <code>aws-marketplace</code>.</p>
    pub owner_alias: ::std::option::Option<::std::string::String>,
    /// <p>The product codes for the AFI.</p>
    pub product_codes: ::std::option::Option<::std::vec::Vec<crate::types::ProductCode>>,
    /// <p>Any tags assigned to the AFI.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>Indicates whether the AFI is public.</p>
    pub public: ::std::option::Option<bool>,
    /// <p>Indicates whether data retention support is enabled for the AFI.</p>
    pub data_retention_support: ::std::option::Option<bool>,
    /// <p>The instance types supported by the AFI.</p>
    pub instance_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FpgaImage {
    /// <p>The FPGA image identifier (AFI ID).</p>
    pub fn fpga_image_id(&self) -> ::std::option::Option<&str> {
        self.fpga_image_id.as_deref()
    }
    /// <p>The global FPGA image identifier (AGFI ID).</p>
    pub fn fpga_image_global_id(&self) -> ::std::option::Option<&str> {
        self.fpga_image_global_id.as_deref()
    }
    /// <p>The name of the AFI.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>The description of the AFI.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The version of the Amazon Web Services Shell that was used to create the bitstream.</p>
    pub fn shell_version(&self) -> ::std::option::Option<&str> {
        self.shell_version.as_deref()
    }
    /// <p>Information about the PCI bus.</p>
    pub fn pci_id(&self) -> ::std::option::Option<&crate::types::PciId> {
        self.pci_id.as_ref()
    }
    /// <p>Information about the state of the AFI.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::FpgaImageState> {
        self.state.as_ref()
    }
    /// <p>The date and time the AFI was created.</p>
    pub fn create_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.create_time.as_ref()
    }
    /// <p>The time of the most recent update to the AFI.</p>
    pub fn update_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.update_time.as_ref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the AFI.</p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The alias of the AFI owner. Possible values include <code>self</code>, <code>amazon</code>, and <code>aws-marketplace</code>.</p>
    pub fn owner_alias(&self) -> ::std::option::Option<&str> {
        self.owner_alias.as_deref()
    }
    /// <p>The product codes for the AFI.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.product_codes.is_none()`.
    pub fn product_codes(&self) -> &[crate::types::ProductCode] {
        self.product_codes.as_deref().unwrap_or_default()
    }
    /// <p>Any tags assigned to the AFI.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
    /// <p>Indicates whether the AFI is public.</p>
    pub fn public(&self) -> ::std::option::Option<bool> {
        self.public
    }
    /// <p>Indicates whether data retention support is enabled for the AFI.</p>
    pub fn data_retention_support(&self) -> ::std::option::Option<bool> {
        self.data_retention_support
    }
    /// <p>The instance types supported by the AFI.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.instance_types.is_none()`.
    pub fn instance_types(&self) -> &[::std::string::String] {
        self.instance_types.as_deref().unwrap_or_default()
    }
}
impl FpgaImage {
    /// Creates a new builder-style object to manufacture [`FpgaImage`](crate::types::FpgaImage).
    pub fn builder() -> crate::types::builders::FpgaImageBuilder {
        crate::types::builders::FpgaImageBuilder::default()
    }
}

/// A builder for [`FpgaImage`](crate::types::FpgaImage).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct FpgaImageBuilder {
    pub(crate) fpga_image_id: ::std::option::Option<::std::string::String>,
    pub(crate) fpga_image_global_id: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) shell_version: ::std::option::Option<::std::string::String>,
    pub(crate) pci_id: ::std::option::Option<crate::types::PciId>,
    pub(crate) state: ::std::option::Option<crate::types::FpgaImageState>,
    pub(crate) create_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) update_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) owner_alias: ::std::option::Option<::std::string::String>,
    pub(crate) product_codes: ::std::option::Option<::std::vec::Vec<crate::types::ProductCode>>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) public: ::std::option::Option<bool>,
    pub(crate) data_retention_support: ::std::option::Option<bool>,
    pub(crate) instance_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl FpgaImageBuilder {
    /// <p>The FPGA image identifier (AFI ID).</p>
    pub fn fpga_image_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fpga_image_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The FPGA image identifier (AFI ID).</p>
    pub fn set_fpga_image_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fpga_image_id = input;
        self
    }
    /// <p>The FPGA image identifier (AFI ID).</p>
    pub fn get_fpga_image_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.fpga_image_id
    }
    /// <p>The global FPGA image identifier (AGFI ID).</p>
    pub fn fpga_image_global_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.fpga_image_global_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The global FPGA image identifier (AGFI ID).</p>
    pub fn set_fpga_image_global_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.fpga_image_global_id = input;
        self
    }
    /// <p>The global FPGA image identifier (AGFI ID).</p>
    pub fn get_fpga_image_global_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.fpga_image_global_id
    }
    /// <p>The name of the AFI.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the AFI.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the AFI.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>The description of the AFI.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the AFI.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the AFI.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The version of the Amazon Web Services Shell that was used to create the bitstream.</p>
    pub fn shell_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.shell_version = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The version of the Amazon Web Services Shell that was used to create the bitstream.</p>
    pub fn set_shell_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.shell_version = input;
        self
    }
    /// <p>The version of the Amazon Web Services Shell that was used to create the bitstream.</p>
    pub fn get_shell_version(&self) -> &::std::option::Option<::std::string::String> {
        &self.shell_version
    }
    /// <p>Information about the PCI bus.</p>
    pub fn pci_id(mut self, input: crate::types::PciId) -> Self {
        self.pci_id = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the PCI bus.</p>
    pub fn set_pci_id(mut self, input: ::std::option::Option<crate::types::PciId>) -> Self {
        self.pci_id = input;
        self
    }
    /// <p>Information about the PCI bus.</p>
    pub fn get_pci_id(&self) -> &::std::option::Option<crate::types::PciId> {
        &self.pci_id
    }
    /// <p>Information about the state of the AFI.</p>
    pub fn state(mut self, input: crate::types::FpgaImageState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the state of the AFI.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::FpgaImageState>) -> Self {
        self.state = input;
        self
    }
    /// <p>Information about the state of the AFI.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::FpgaImageState> {
        &self.state
    }
    /// <p>The date and time the AFI was created.</p>
    pub fn create_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.create_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time the AFI was created.</p>
    pub fn set_create_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.create_time = input;
        self
    }
    /// <p>The date and time the AFI was created.</p>
    pub fn get_create_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.create_time
    }
    /// <p>The time of the most recent update to the AFI.</p>
    pub fn update_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.update_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time of the most recent update to the AFI.</p>
    pub fn set_update_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.update_time = input;
        self
    }
    /// <p>The time of the most recent update to the AFI.</p>
    pub fn get_update_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.update_time
    }
    /// <p>The ID of the Amazon Web Services account that owns the AFI.</p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the AFI.</p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the AFI.</p>
    pub fn get_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner_id
    }
    /// <p>The alias of the AFI owner. Possible values include <code>self</code>, <code>amazon</code>, and <code>aws-marketplace</code>.</p>
    pub fn owner_alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_alias = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The alias of the AFI owner. Possible values include <code>self</code>, <code>amazon</code>, and <code>aws-marketplace</code>.</p>
    pub fn set_owner_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_alias = input;
        self
    }
    /// <p>The alias of the AFI owner. Possible values include <code>self</code>, <code>amazon</code>, and <code>aws-marketplace</code>.</p>
    pub fn get_owner_alias(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner_alias
    }
    /// Appends an item to `product_codes`.
    ///
    /// To override the contents of this collection use [`set_product_codes`](Self::set_product_codes).
    ///
    /// <p>The product codes for the AFI.</p>
    pub fn product_codes(mut self, input: crate::types::ProductCode) -> Self {
        let mut v = self.product_codes.unwrap_or_default();
        v.push(input);
        self.product_codes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The product codes for the AFI.</p>
    pub fn set_product_codes(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProductCode>>) -> Self {
        self.product_codes = input;
        self
    }
    /// <p>The product codes for the AFI.</p>
    pub fn get_product_codes(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ProductCode>> {
        &self.product_codes
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Any tags assigned to the AFI.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>Any tags assigned to the AFI.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>Any tags assigned to the AFI.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// <p>Indicates whether the AFI is public.</p>
    pub fn public(mut self, input: bool) -> Self {
        self.public = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether the AFI is public.</p>
    pub fn set_public(mut self, input: ::std::option::Option<bool>) -> Self {
        self.public = input;
        self
    }
    /// <p>Indicates whether the AFI is public.</p>
    pub fn get_public(&self) -> &::std::option::Option<bool> {
        &self.public
    }
    /// <p>Indicates whether data retention support is enabled for the AFI.</p>
    pub fn data_retention_support(mut self, input: bool) -> Self {
        self.data_retention_support = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether data retention support is enabled for the AFI.</p>
    pub fn set_data_retention_support(mut self, input: ::std::option::Option<bool>) -> Self {
        self.data_retention_support = input;
        self
    }
    /// <p>Indicates whether data retention support is enabled for the AFI.</p>
    pub fn get_data_retention_support(&self) -> &::std::option::Option<bool> {
        &self.data_retention_support
    }
    /// Appends an item to `instance_types`.
    ///
    /// To override the contents of this collection use [`set_instance_types`](Self::set_instance_types).
    ///
    /// <p>The instance types supported by the AFI.</p>
    pub fn instance_types(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.instance_types.unwrap_or_default();
        v.push(input.into());
        self.instance_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>The instance types supported by the AFI.</p>
    pub fn set_instance_types(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.instance_types = input;
        self
    }
    /// <p>The instance types supported by the AFI.</p>
    pub fn get_instance_types(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.instance_types
    }
    /// Consumes the builder and constructs a [`FpgaImage`](crate::types::FpgaImage).
    pub fn build(self) -> crate::types::FpgaImage {
        crate::types::FpgaImage {
            fpga_image_id: self.fpga_image_id,
            fpga_image_global_id: self.fpga_image_global_id,
            name: self.name,
            description: self.description,
            shell_version: self.shell_version,
            pci_id: self.pci_id,
            state: self.state,
            create_time: self.create_time,
            update_time: self.update_time,
            owner_id: self.owner_id,
            owner_alias: self.owner_alias,
            product_codes: self.product_codes,
            tags: self.tags,
            public: self.public,
            data_retention_support: self.data_retention_support,
            instance_types: self.instance_types,
        }
    }
}