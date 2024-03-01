// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ResetFpgaImageAttribute`](crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`fpga_image_id(impl Into<String>)`](crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeFluentBuilder::fpga_image_id) / [`set_fpga_image_id(Option<String>)`](crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeFluentBuilder::set_fpga_image_id):<br>required: **true**<br><p>The ID of the AFI.</p><br>
    ///   - [`attribute(ResetFpgaImageAttributeName)`](crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeFluentBuilder::attribute) / [`set_attribute(Option<ResetFpgaImageAttributeName>)`](crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeFluentBuilder::set_attribute):<br>required: **false**<br><p>The attribute.</p><br>
    /// - On success, responds with [`ResetFpgaImageAttributeOutput`](crate::operation::reset_fpga_image_attribute::ResetFpgaImageAttributeOutput) with field(s):
    ///   - [`r#return(Option<bool>)`](crate::operation::reset_fpga_image_attribute::ResetFpgaImageAttributeOutput::return): <p>Is <code>true</code> if the request succeeds, and an error otherwise.</p>
    /// - On failure, responds with [`SdkError<ResetFpgaImageAttributeError>`](crate::operation::reset_fpga_image_attribute::ResetFpgaImageAttributeError)
    pub fn reset_fpga_image_attribute(&self) -> crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeFluentBuilder {
        crate::operation::reset_fpga_image_attribute::builders::ResetFpgaImageAttributeFluentBuilder::new(self.handle.clone())
    }
}