// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ModifyLaunchTemplate`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::set_client_token):<br>required: **false**<br><p>Unique, case-sensitive identifier you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensuring idempotency</a>.</p>  <p>Constraint: Maximum 128 ASCII characters.</p><br>
    ///   - [`launch_template_id(impl Into<String>)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::launch_template_id) / [`set_launch_template_id(Option<String>)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::set_launch_template_id):<br>required: **false**<br><p>The ID of the launch template.</p>  <p>You must specify either the <code>LaunchTemplateId</code> or the <code>LaunchTemplateName</code>, but not both.</p><br>
    ///   - [`launch_template_name(impl Into<String>)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::launch_template_name) / [`set_launch_template_name(Option<String>)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::set_launch_template_name):<br>required: **false**<br><p>The name of the launch template.</p>  <p>You must specify either the <code>LaunchTemplateName</code> or the <code>LaunchTemplateId</code>, but not both.</p><br>
    ///   - [`default_version(impl Into<String>)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::default_version) / [`set_default_version(Option<String>)`](crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::set_default_version):<br>required: **false**<br><p>The version number of the launch template to set as the default version.</p><br>
    /// - On success, responds with [`ModifyLaunchTemplateOutput`](crate::operation::modify_launch_template::ModifyLaunchTemplateOutput) with field(s):
    ///   - [`launch_template(Option<LaunchTemplate>)`](crate::operation::modify_launch_template::ModifyLaunchTemplateOutput::launch_template): <p>Information about the launch template.</p>
    /// - On failure, responds with [`SdkError<ModifyLaunchTemplateError>`](crate::operation::modify_launch_template::ModifyLaunchTemplateError)
    pub fn modify_launch_template(&self) -> crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder {
        crate::operation::modify_launch_template::builders::ModifyLaunchTemplateFluentBuilder::new(self.handle.clone())
    }
}