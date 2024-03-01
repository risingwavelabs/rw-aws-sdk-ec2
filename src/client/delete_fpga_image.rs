// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteFpgaImage`](crate::operation::delete_fpga_image::builders::DeleteFpgaImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::delete_fpga_image::builders::DeleteFpgaImageFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::delete_fpga_image::builders::DeleteFpgaImageFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`fpga_image_id(impl Into<String>)`](crate::operation::delete_fpga_image::builders::DeleteFpgaImageFluentBuilder::fpga_image_id) / [`set_fpga_image_id(Option<String>)`](crate::operation::delete_fpga_image::builders::DeleteFpgaImageFluentBuilder::set_fpga_image_id):<br>required: **true**<br><p>The ID of the AFI.</p><br>
    /// - On success, responds with [`DeleteFpgaImageOutput`](crate::operation::delete_fpga_image::DeleteFpgaImageOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::delete_fpga_image::DeleteFpgaImageOutput::return): <p>Is <code>true</code> if the request succeeds, and an error otherwise.</p>
    /// - On failure, responds with [`SdkError<DeleteFpgaImageError>`](crate::operation::delete_fpga_image::DeleteFpgaImageError)
    pub fn delete_fpga_image(&self) -> crate::operation::delete_fpga_image::builders::DeleteFpgaImageFluentBuilder {
        crate::operation::delete_fpga_image::builders::DeleteFpgaImageFluentBuilder::new(self.handle.clone())
    }
}
