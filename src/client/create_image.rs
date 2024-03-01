// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateImage`](crate::operation::create_image::builders::CreateImageFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`block_device_mappings(BlockDeviceMapping)`](crate::operation::create_image::builders::CreateImageFluentBuilder::block_device_mappings) / [`set_block_device_mappings(Option<Vec::<BlockDeviceMapping>>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_block_device_mappings):<br>required: **false**<br><p>The block device mappings.</p>  <p>When using the CreateImage action:</p>  <ul>   <li> <p>You can't change the volume size using the VolumeSize parameter. If you want a different volume size, you must first change the volume size of the source instance.</p> </li>   <li> <p>You can't modify the encryption status of existing volumes or snapshots. To create an AMI with volumes or snapshots that have a different encryption status (for example, where the source volume and snapshots are unencrypted, and you want to create an AMI with encrypted volumes or snapshots), use the <code>CopyImage</code> action.</p> </li>   <li> <p>The only option that can be changed for existing mappings or snapshots is <code>DeleteOnTermination</code>.</p> </li>  </ul><br>
    ///   - [`description(impl Into<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_description):<br>required: **false**<br><p>A description for the new image.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_image::builders::CreateImageFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`instance_id(impl Into<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::instance_id) / [`set_instance_id(Option<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_instance_id):<br>required: **true**<br><p>The ID of the instance.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_name):<br>required: **true**<br><p>A name for the new image.</p>  <p>Constraints: 3-128 alphanumeric characters, parentheses (()), square brackets ([]), spaces ( ), periods (.), slashes (/), dashes (-), single quotes ('), at-signs (@), or underscores(_)</p><br>
    ///   - [`no_reboot(bool)`](crate::operation::create_image::builders::CreateImageFluentBuilder::no_reboot) / [`set_no_reboot(Option<bool>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_no_reboot):<br>required: **false**<br><p>Indicates whether or not the instance should be automatically rebooted before creating the image. Specify one of the following values:</p>  <ul>   <li> <p> <code>true</code> - The instance is not rebooted before creating the image. This creates crash-consistent snapshots that include only the data that has been written to the volumes at the time the snapshots are created. Buffered data and data in memory that has not yet been written to the volumes is not included in the snapshots.</p> </li>   <li> <p> <code>false</code> - The instance is rebooted before creating the image. This ensures that all buffered data and data in memory is written to the volumes before the snapshots are created.</p> </li>  </ul>  <p>Default: <code>false</code> </p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_image::builders::CreateImageFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_image::builders::CreateImageFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to apply to the AMI and snapshots on creation. You can tag the AMI, the snapshots, or both.</p>  <ul>   <li> <p>To tag the AMI, the value for <code>ResourceType</code> must be <code>image</code>.</p> </li>   <li> <p>To tag the snapshots that are created of the root volume and of other Amazon EBS volumes that are attached to the instance, the value for <code>ResourceType</code> must be <code>snapshot</code>. The same tag is applied to all of the snapshots that are created.</p> </li>  </ul>  <p>If you specify other values for <code>ResourceType</code>, the request fails.</p>  <p>To tag an AMI or snapshot after it has been created, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CreateTags.html">CreateTags</a>. </p><br>
    /// - On success, responds with [`CreateImageOutput`](crate::operation::create_image::CreateImageOutput) with field(s):
    ///   - [`image_id(Option<String>)`](crate::operation::create_image::CreateImageOutput::image_id): <p>The ID of the new AMI.</p>
    /// - On failure, responds with [`SdkError<CreateImageError>`](crate::operation::create_image::CreateImageError)
    pub fn create_image(&self) -> crate::operation::create_image::builders::CreateImageFluentBuilder {
        crate::operation::create_image::builders::CreateImageFluentBuilder::new(self.handle.clone())
    }
}
