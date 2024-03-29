// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableImageBlockPublicAccess`](crate::operation::disable_image_block_public_access::builders::DisableImageBlockPublicAccessFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::disable_image_block_public_access::builders::DisableImageBlockPublicAccessFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::disable_image_block_public_access::builders::DisableImageBlockPublicAccessFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DisableImageBlockPublicAccessOutput`](crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessOutput) with field(s):
    ///   - [`image_block_public_access_state(Option<ImageBlockPublicAccessDisabledState>)`](crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessOutput::image_block_public_access_state): <p>Returns <code>unblocked</code> if the request succeeds; otherwise, it returns an error.</p>
    /// - On failure, responds with [`SdkError<DisableImageBlockPublicAccessError>`](crate::operation::disable_image_block_public_access::DisableImageBlockPublicAccessError)
    pub fn disable_image_block_public_access(
        &self,
    ) -> crate::operation::disable_image_block_public_access::builders::DisableImageBlockPublicAccessFluentBuilder {
        crate::operation::disable_image_block_public_access::builders::DisableImageBlockPublicAccessFluentBuilder::new(self.handle.clone())
    }
}
