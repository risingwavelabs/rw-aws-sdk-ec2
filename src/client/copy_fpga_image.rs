// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CopyFpgaImage`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`source_fpga_image_id(impl Into<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::source_fpga_image_id) / [`set_source_fpga_image_id(Option<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::set_source_fpga_image_id):<br>required: **true**<br><p>The ID of the source AFI.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::set_description):<br>required: **false**<br><p>The description for the new AFI.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::set_name):<br>required: **false**<br><p>The name for the new AFI. The default is the name of the source AFI.</p><br>
    ///   - [`source_region(impl Into<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::source_region) / [`set_source_region(Option<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::set_source_region):<br>required: **true**<br><p>The Region that contains the source AFI.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::set_client_token):<br>required: **false**<br><p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p><br>
    /// - On success, responds with [`CopyFpgaImageOutput`](crate::operation::copy_fpga_image::CopyFpgaImageOutput) with field(s):
    ///   - [`fpga_image_id(Option<String>)`](crate::operation::copy_fpga_image::CopyFpgaImageOutput::fpga_image_id): <p>The ID of the new AFI.</p>
    /// - On failure, responds with [`SdkError<CopyFpgaImageError>`](crate::operation::copy_fpga_image::CopyFpgaImageError)
    pub fn copy_fpga_image(&self) -> crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder {
        crate::operation::copy_fpga_image::builders::CopyFpgaImageFluentBuilder::new(self.handle.clone())
    }
}
