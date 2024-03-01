// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_volume::_create_volume_output::CreateVolumeOutputBuilder;

pub use crate::operation::create_volume::_create_volume_input::CreateVolumeInputBuilder;

impl CreateVolumeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_volume::CreateVolumeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_volume::CreateVolumeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_volume();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateVolume`.
///
/// <p>Creates an EBS volume that can be attached to an instance in the same Availability Zone.</p>
/// <p>You can create a new empty volume or restore a volume from an EBS snapshot. Any Amazon Web Services Marketplace product codes from the snapshot are propagated to the volume.</p>
/// <p>You can create encrypted volumes. Encrypted volumes must be attached to instances that support Amazon EBS encryption. Volumes that are created from encrypted snapshots are also automatically encrypted. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// <p>You can tag your volumes during creation. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html">Tag your Amazon EC2 resources</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-creating-volume.html">Create an Amazon EBS volume</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateVolumeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_volume::builders::CreateVolumeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_volume::CreateVolumeOutput,
        crate::operation::create_volume::CreateVolumeError,
    > for CreateVolumeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_volume::CreateVolumeOutput,
            crate::operation::create_volume::CreateVolumeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateVolumeFluentBuilder {
    /// Creates a new `CreateVolume`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateVolume as a reference.
    pub fn as_input(&self) -> &crate::operation::create_volume::builders::CreateVolumeInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_volume::CreateVolumeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_volume::CreateVolumeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_volume::CreateVolume::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_volume::CreateVolume::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_volume::CreateVolumeOutput,
        crate::operation::create_volume::CreateVolumeError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl Into<crate::config::Builder>) -> Self {
        self.set_config_override(Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The ID of the Availability Zone in which to create the volume. For example, <code>us-east-1a</code>.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The ID of the Availability Zone in which to create the volume. For example, <code>us-east-1a</code>.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The ID of the Availability Zone in which to create the volume. For example, <code>us-east-1a</code>.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone()
    }
    /// <p>Indicates whether the volume should be encrypted. The effect of setting the encryption state to <code>true</code> depends on the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#encryption-by-default">Encryption by default</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>Encrypted Amazon EBS volumes must be attached to instances that support Amazon EBS encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported instance types</a>.</p>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.inner = self.inner.encrypted(input);
        self
    }
    /// <p>Indicates whether the volume should be encrypted. The effect of setting the encryption state to <code>true</code> depends on the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#encryption-by-default">Encryption by default</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>Encrypted Amazon EBS volumes must be attached to instances that support Amazon EBS encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported instance types</a>.</p>
    pub fn set_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_encrypted(input);
        self
    }
    /// <p>Indicates whether the volume should be encrypted. The effect of setting the encryption state to <code>true</code> depends on the volume origin (new or from a snapshot), starting encryption state, ownership, and whether encryption by default is enabled. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#encryption-by-default">Encryption by default</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>Encrypted Amazon EBS volumes must be attached to instances that support Amazon EBS encryption. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html#EBSEncryption_supported_instances">Supported instance types</a>.</p>
    pub fn get_encrypted(&self) -> &::std::option::Option<bool> {
        self.inner.get_encrypted()
    }
    /// <p>The number of I/O operations per second (IOPS). For <code>gp3</code>, <code>io1</code>, and <code>io2</code> volumes, this represents the number of IOPS that are provisioned for the volume. For <code>gp2</code> volumes, this represents the baseline performance of the volume and the rate at which the volume accumulates I/O credits for bursting.</p>
    /// <p>The following are the supported values for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp3</code>: 3,000 - 16,000 IOPS</p> </li>
    /// <li> <p> <code>io1</code>: 100 - 64,000 IOPS</p> </li>
    /// <li> <p> <code>io2</code>: 100 - 256,000 IOPS</p> </li>
    /// </ul>
    /// <p>For <code>io2</code> volumes, you can achieve up to 256,000 IOPS on <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">instances built on the Nitro System</a>. On other instances, you can achieve performance up to 32,000 IOPS.</p>
    /// <p>This parameter is required for <code>io1</code> and <code>io2</code> volumes. The default for <code>gp3</code> volumes is 3,000 IOPS. This parameter is not supported for <code>gp2</code>, <code>st1</code>, <code>sc1</code>, or <code>standard</code> volumes.</p>
    pub fn iops(mut self, input: i32) -> Self {
        self.inner = self.inner.iops(input);
        self
    }
    /// <p>The number of I/O operations per second (IOPS). For <code>gp3</code>, <code>io1</code>, and <code>io2</code> volumes, this represents the number of IOPS that are provisioned for the volume. For <code>gp2</code> volumes, this represents the baseline performance of the volume and the rate at which the volume accumulates I/O credits for bursting.</p>
    /// <p>The following are the supported values for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp3</code>: 3,000 - 16,000 IOPS</p> </li>
    /// <li> <p> <code>io1</code>: 100 - 64,000 IOPS</p> </li>
    /// <li> <p> <code>io2</code>: 100 - 256,000 IOPS</p> </li>
    /// </ul>
    /// <p>For <code>io2</code> volumes, you can achieve up to 256,000 IOPS on <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">instances built on the Nitro System</a>. On other instances, you can achieve performance up to 32,000 IOPS.</p>
    /// <p>This parameter is required for <code>io1</code> and <code>io2</code> volumes. The default for <code>gp3</code> volumes is 3,000 IOPS. This parameter is not supported for <code>gp2</code>, <code>st1</code>, <code>sc1</code>, or <code>standard</code> volumes.</p>
    pub fn set_iops(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_iops(input);
        self
    }
    /// <p>The number of I/O operations per second (IOPS). For <code>gp3</code>, <code>io1</code>, and <code>io2</code> volumes, this represents the number of IOPS that are provisioned for the volume. For <code>gp2</code> volumes, this represents the baseline performance of the volume and the rate at which the volume accumulates I/O credits for bursting.</p>
    /// <p>The following are the supported values for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp3</code>: 3,000 - 16,000 IOPS</p> </li>
    /// <li> <p> <code>io1</code>: 100 - 64,000 IOPS</p> </li>
    /// <li> <p> <code>io2</code>: 100 - 256,000 IOPS</p> </li>
    /// </ul>
    /// <p>For <code>io2</code> volumes, you can achieve up to 256,000 IOPS on <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">instances built on the Nitro System</a>. On other instances, you can achieve performance up to 32,000 IOPS.</p>
    /// <p>This parameter is required for <code>io1</code> and <code>io2</code> volumes. The default for <code>gp3</code> volumes is 3,000 IOPS. This parameter is not supported for <code>gp2</code>, <code>st1</code>, <code>sc1</code>, or <code>standard</code> volumes.</p>
    pub fn get_iops(&self) -> &::std::option::Option<i32> {
        self.inner.get_iops()
    }
    /// <p>The identifier of the Key Management Service (KMS) KMS key to use for Amazon EBS encryption. If this parameter is not specified, your KMS key for Amazon EBS is used. If <code>KmsKeyId</code> is specified, the encrypted state must be <code>true</code>.</p>
    /// <p>You can specify the KMS key using any of the following:</p>
    /// <ul>
    /// <li> <p>Key ID. For example, 1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>
    /// <li> <p>Key alias. For example, alias/ExampleAlias.</p> </li>
    /// <li> <p>Key ARN. For example, arn:aws:kms:us-east-1:012345678910:key/1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>
    /// <li> <p>Alias ARN. For example, arn:aws:kms:us-east-1:012345678910:alias/ExampleAlias.</p> </li>
    /// </ul>
    /// <p>Amazon Web Services authenticates the KMS key asynchronously. Therefore, if you specify an ID, alias, or ARN that is not valid, the action can appear to complete, but eventually fails.</p>
    pub fn kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_id(input.into());
        self
    }
    /// <p>The identifier of the Key Management Service (KMS) KMS key to use for Amazon EBS encryption. If this parameter is not specified, your KMS key for Amazon EBS is used. If <code>KmsKeyId</code> is specified, the encrypted state must be <code>true</code>.</p>
    /// <p>You can specify the KMS key using any of the following:</p>
    /// <ul>
    /// <li> <p>Key ID. For example, 1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>
    /// <li> <p>Key alias. For example, alias/ExampleAlias.</p> </li>
    /// <li> <p>Key ARN. For example, arn:aws:kms:us-east-1:012345678910:key/1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>
    /// <li> <p>Alias ARN. For example, arn:aws:kms:us-east-1:012345678910:alias/ExampleAlias.</p> </li>
    /// </ul>
    /// <p>Amazon Web Services authenticates the KMS key asynchronously. Therefore, if you specify an ID, alias, or ARN that is not valid, the action can appear to complete, but eventually fails.</p>
    pub fn set_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_id(input);
        self
    }
    /// <p>The identifier of the Key Management Service (KMS) KMS key to use for Amazon EBS encryption. If this parameter is not specified, your KMS key for Amazon EBS is used. If <code>KmsKeyId</code> is specified, the encrypted state must be <code>true</code>.</p>
    /// <p>You can specify the KMS key using any of the following:</p>
    /// <ul>
    /// <li> <p>Key ID. For example, 1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>
    /// <li> <p>Key alias. For example, alias/ExampleAlias.</p> </li>
    /// <li> <p>Key ARN. For example, arn:aws:kms:us-east-1:012345678910:key/1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>
    /// <li> <p>Alias ARN. For example, arn:aws:kms:us-east-1:012345678910:alias/ExampleAlias.</p> </li>
    /// </ul>
    /// <p>Amazon Web Services authenticates the KMS key asynchronously. Therefore, if you specify an ID, alias, or ARN that is not valid, the action can appear to complete, but eventually fails.</p>
    pub fn get_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_id()
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost.</p>
    pub fn outpost_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.outpost_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost.</p>
    pub fn set_outpost_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_outpost_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost.</p>
    pub fn get_outpost_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_outpost_arn()
    }
    /// <p>The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size. If you specify a snapshot, the default is the snapshot size. You can specify a volume size that is equal to or larger than the snapshot size.</p>
    /// <p>The following are the supported volumes sizes for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> and <code>gp3</code>: 1 - 16,384 GiB</p> </li>
    /// <li> <p> <code>io1</code>: 4 - 16,384 GiB</p> </li>
    /// <li> <p> <code>io2</code>: 4 - 65,536 GiB</p> </li>
    /// <li> <p> <code>st1</code> and <code>sc1</code>: 125 - 16,384 GiB</p> </li>
    /// <li> <p> <code>standard</code>: 1 - 1024 GiB</p> </li>
    /// </ul>
    pub fn size(mut self, input: i32) -> Self {
        self.inner = self.inner.size(input);
        self
    }
    /// <p>The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size. If you specify a snapshot, the default is the snapshot size. You can specify a volume size that is equal to or larger than the snapshot size.</p>
    /// <p>The following are the supported volumes sizes for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> and <code>gp3</code>: 1 - 16,384 GiB</p> </li>
    /// <li> <p> <code>io1</code>: 4 - 16,384 GiB</p> </li>
    /// <li> <p> <code>io2</code>: 4 - 65,536 GiB</p> </li>
    /// <li> <p> <code>st1</code> and <code>sc1</code>: 125 - 16,384 GiB</p> </li>
    /// <li> <p> <code>standard</code>: 1 - 1024 GiB</p> </li>
    /// </ul>
    pub fn set_size(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_size(input);
        self
    }
    /// <p>The size of the volume, in GiBs. You must specify either a snapshot ID or a volume size. If you specify a snapshot, the default is the snapshot size. You can specify a volume size that is equal to or larger than the snapshot size.</p>
    /// <p>The following are the supported volumes sizes for each volume type:</p>
    /// <ul>
    /// <li> <p> <code>gp2</code> and <code>gp3</code>: 1 - 16,384 GiB</p> </li>
    /// <li> <p> <code>io1</code>: 4 - 16,384 GiB</p> </li>
    /// <li> <p> <code>io2</code>: 4 - 65,536 GiB</p> </li>
    /// <li> <p> <code>st1</code> and <code>sc1</code>: 125 - 16,384 GiB</p> </li>
    /// <li> <p> <code>standard</code>: 1 - 1024 GiB</p> </li>
    /// </ul>
    pub fn get_size(&self) -> &::std::option::Option<i32> {
        self.inner.get_size()
    }
    /// <p>The snapshot from which to create the volume. You must specify either a snapshot ID or a volume size.</p>
    pub fn snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.snapshot_id(input.into());
        self
    }
    /// <p>The snapshot from which to create the volume. You must specify either a snapshot ID or a volume size.</p>
    pub fn set_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_snapshot_id(input);
        self
    }
    /// <p>The snapshot from which to create the volume. You must specify either a snapshot ID or a volume size.</p>
    pub fn get_snapshot_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_snapshot_id()
    }
    /// <p>The volume type. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p>General Purpose SSD: <code>gp2</code> | <code>gp3</code> </p> </li>
    /// <li> <p>Provisioned IOPS SSD: <code>io1</code> | <code>io2</code> </p> </li>
    /// <li> <p>Throughput Optimized HDD: <code>st1</code> </p> </li>
    /// <li> <p>Cold HDD: <code>sc1</code> </p> </li>
    /// <li> <p>Magnetic: <code>standard</code> </p> </li>
    /// </ul> <important>
    /// <p>Throughput Optimized HDD (<code>st1</code>) and Cold HDD (<code>sc1</code>) volumes can't be used as boot volumes.</p>
    /// </important>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS volume types</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>Default: <code>gp2</code> </p>
    pub fn volume_type(mut self, input: crate::types::VolumeType) -> Self {
        self.inner = self.inner.volume_type(input);
        self
    }
    /// <p>The volume type. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p>General Purpose SSD: <code>gp2</code> | <code>gp3</code> </p> </li>
    /// <li> <p>Provisioned IOPS SSD: <code>io1</code> | <code>io2</code> </p> </li>
    /// <li> <p>Throughput Optimized HDD: <code>st1</code> </p> </li>
    /// <li> <p>Cold HDD: <code>sc1</code> </p> </li>
    /// <li> <p>Magnetic: <code>standard</code> </p> </li>
    /// </ul> <important>
    /// <p>Throughput Optimized HDD (<code>st1</code>) and Cold HDD (<code>sc1</code>) volumes can't be used as boot volumes.</p>
    /// </important>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS volume types</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>Default: <code>gp2</code> </p>
    pub fn set_volume_type(mut self, input: ::std::option::Option<crate::types::VolumeType>) -> Self {
        self.inner = self.inner.set_volume_type(input);
        self
    }
    /// <p>The volume type. This parameter can be one of the following values:</p>
    /// <ul>
    /// <li> <p>General Purpose SSD: <code>gp2</code> | <code>gp3</code> </p> </li>
    /// <li> <p>Provisioned IOPS SSD: <code>io1</code> | <code>io2</code> </p> </li>
    /// <li> <p>Throughput Optimized HDD: <code>st1</code> </p> </li>
    /// <li> <p>Cold HDD: <code>sc1</code> </p> </li>
    /// <li> <p>Magnetic: <code>standard</code> </p> </li>
    /// </ul> <important>
    /// <p>Throughput Optimized HDD (<code>st1</code>) and Cold HDD (<code>sc1</code>) volumes can't be used as boot volumes.</p>
    /// </important>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSVolumeTypes.html">Amazon EBS volume types</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    /// <p>Default: <code>gp2</code> </p>
    pub fn get_volume_type(&self) -> &::std::option::Option<crate::types::VolumeType> {
        self.inner.get_volume_type()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// Appends an item to `TagSpecifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the volume during creation.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        self.inner = self.inner.tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the volume during creation.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.inner = self.inner.set_tag_specifications(input);
        self
    }
    /// <p>The tags to apply to the volume during creation.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        self.inner.get_tag_specifications()
    }
    /// <p>Indicates whether to enable Amazon EBS Multi-Attach. If you enable Multi-Attach, you can attach the volume to up to 16 <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Instances built on the Nitro System</a> in the same Availability Zone. This parameter is supported with <code>io1</code> and <code>io2</code> volumes only. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-volumes-multi.html"> Amazon EBS Multi-Attach</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn multi_attach_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.multi_attach_enabled(input);
        self
    }
    /// <p>Indicates whether to enable Amazon EBS Multi-Attach. If you enable Multi-Attach, you can attach the volume to up to 16 <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Instances built on the Nitro System</a> in the same Availability Zone. This parameter is supported with <code>io1</code> and <code>io2</code> volumes only. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-volumes-multi.html"> Amazon EBS Multi-Attach</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn set_multi_attach_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_multi_attach_enabled(input);
        self
    }
    /// <p>Indicates whether to enable Amazon EBS Multi-Attach. If you enable Multi-Attach, you can attach the volume to up to 16 <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html#ec2-nitro-instances">Instances built on the Nitro System</a> in the same Availability Zone. This parameter is supported with <code>io1</code> and <code>io2</code> volumes only. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-volumes-multi.html"> Amazon EBS Multi-Attach</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn get_multi_attach_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_multi_attach_enabled()
    }
    /// <p>The throughput to provision for a volume, with a maximum of 1,000 MiB/s.</p>
    /// <p>This parameter is valid only for <code>gp3</code> volumes.</p>
    /// <p>Valid Range: Minimum value of 125. Maximum value of 1000.</p>
    pub fn throughput(mut self, input: i32) -> Self {
        self.inner = self.inner.throughput(input);
        self
    }
    /// <p>The throughput to provision for a volume, with a maximum of 1,000 MiB/s.</p>
    /// <p>This parameter is valid only for <code>gp3</code> volumes.</p>
    /// <p>Valid Range: Minimum value of 125. Maximum value of 1000.</p>
    pub fn set_throughput(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_throughput(input);
        self
    }
    /// <p>The throughput to provision for a volume, with a maximum of 1,000 MiB/s.</p>
    /// <p>This parameter is valid only for <code>gp3</code> volumes.</p>
    /// <p>Valid Range: Minimum value of 125. Maximum value of 1000.</p>
    pub fn get_throughput(&self) -> &::std::option::Option<i32> {
        self.inner.get_throughput()
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Run_Instance_Idempotency.html">Ensure Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
