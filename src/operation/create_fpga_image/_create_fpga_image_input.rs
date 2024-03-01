// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CreateFpgaImageInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The location of the encrypted design checkpoint in Amazon S3. The input must be a tarball.</p>
    pub input_storage_location: ::std::option::Option<crate::types::StorageLocation>,
    /// <p>The location in Amazon S3 for the output logs.</p>
    pub logs_storage_location: ::std::option::Option<crate::types::StorageLocation>,
    /// <p>A description for the AFI.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>A name for the AFI.</p>
    pub name: ::std::option::Option<::std::string::String>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    /// <p>The tags to apply to the FPGA image during creation.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
}
impl CreateFpgaImageInput {
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The location of the encrypted design checkpoint in Amazon S3. The input must be a tarball.</p>
    pub fn input_storage_location(&self) -> ::std::option::Option<&crate::types::StorageLocation> {
        self.input_storage_location.as_ref()
    }
    /// <p>The location in Amazon S3 for the output logs.</p>
    pub fn logs_storage_location(&self) -> ::std::option::Option<&crate::types::StorageLocation> {
        self.logs_storage_location.as_ref()
    }
    /// <p>A description for the AFI.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>A name for the AFI.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
    /// <p>The tags to apply to the FPGA image during creation.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
}
impl CreateFpgaImageInput {
    /// Creates a new builder-style object to manufacture [`CreateFpgaImageInput`](crate::operation::create_fpga_image::CreateFpgaImageInput).
    pub fn builder() -> crate::operation::create_fpga_image::builders::CreateFpgaImageInputBuilder {
        crate::operation::create_fpga_image::builders::CreateFpgaImageInputBuilder::default()
    }
}

/// A builder for [`CreateFpgaImageInput`](crate::operation::create_fpga_image::CreateFpgaImageInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct CreateFpgaImageInputBuilder {
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) input_storage_location: ::std::option::Option<crate::types::StorageLocation>,
    pub(crate) logs_storage_location: ::std::option::Option<crate::types::StorageLocation>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
}
impl CreateFpgaImageInputBuilder {
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
    /// <p>The location of the encrypted design checkpoint in Amazon S3. The input must be a tarball.</p>
    /// This field is required.
    pub fn input_storage_location(mut self, input: crate::types::StorageLocation) -> Self {
        self.input_storage_location = ::std::option::Option::Some(input);
        self
    }
    /// <p>The location of the encrypted design checkpoint in Amazon S3. The input must be a tarball.</p>
    pub fn set_input_storage_location(mut self, input: ::std::option::Option<crate::types::StorageLocation>) -> Self {
        self.input_storage_location = input;
        self
    }
    /// <p>The location of the encrypted design checkpoint in Amazon S3. The input must be a tarball.</p>
    pub fn get_input_storage_location(&self) -> &::std::option::Option<crate::types::StorageLocation> {
        &self.input_storage_location
    }
    /// <p>The location in Amazon S3 for the output logs.</p>
    pub fn logs_storage_location(mut self, input: crate::types::StorageLocation) -> Self {
        self.logs_storage_location = ::std::option::Option::Some(input);
        self
    }
    /// <p>The location in Amazon S3 for the output logs.</p>
    pub fn set_logs_storage_location(mut self, input: ::std::option::Option<crate::types::StorageLocation>) -> Self {
        self.logs_storage_location = input;
        self
    }
    /// <p>The location in Amazon S3 for the output logs.</p>
    pub fn get_logs_storage_location(&self) -> &::std::option::Option<crate::types::StorageLocation> {
        &self.logs_storage_location
    }
    /// <p>A description for the AFI.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the AFI.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description for the AFI.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>A name for the AFI.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A name for the AFI.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A name for the AFI.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">Ensuring Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the FPGA image during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the FPGA image during creation.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to apply to the FPGA image during creation.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
    /// Consumes the builder and constructs a [`CreateFpgaImageInput`](crate::operation::create_fpga_image::CreateFpgaImageInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::create_fpga_image::CreateFpgaImageInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::create_fpga_image::CreateFpgaImageInput {
            dry_run: self.dry_run,
            input_storage_location: self.input_storage_location,
            logs_storage_location: self.logs_storage_location,
            description: self.description,
            name: self.name,
            client_token: self.client_token,
            tag_specifications: self.tag_specifications,
        })
    }
}
