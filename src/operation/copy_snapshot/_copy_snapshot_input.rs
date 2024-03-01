// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq)]
pub struct CopySnapshotInput {
    /// <p>A description for the EBS snapshot.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Outpost to which to copy the snapshot. Only specify this parameter when copying a snapshot from an Amazon Web Services Region to an Outpost. The snapshot must be in the Region for the destination Outpost. You cannot copy a snapshot from an Outpost to a Region, from one Outpost to another, or within the same Outpost.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#copy-snapshots"> Copy snapshots from an Amazon Web Services Region to an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub destination_outpost_arn: ::std::option::Option<::std::string::String>,
    /// <p>The destination Region to use in the <code>PresignedUrl</code> parameter of a snapshot copy operation. This parameter is only valid for specifying the destination Region in a <code>PresignedUrl</code> parameter, where it is required.</p>
    /// <p>The snapshot copy is sent to the regional endpoint that you sent the HTTP request to (for example, <code>ec2.us-east-1.amazonaws.com</code>). With the CLI, this is specified using the <code>--region</code> parameter or the default Region in your Amazon Web Services configuration file.</p>
    pub destination_region: ::std::option::Option<::std::string::String>,
    /// <p>To encrypt a copy of an unencrypted snapshot if encryption by default is not enabled, enable encryption using this parameter. Otherwise, omit this parameter. Encrypted snapshots are encrypted, even if you omit this parameter and encryption by default is not enabled. You cannot set this parameter to false. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub encrypted: ::std::option::Option<bool>,
    /// <p>The identifier of the Key Management Service (KMS) KMS key to use for Amazon EBS encryption. If this parameter is not specified, your KMS key for Amazon EBS is used. If <code>KmsKeyId</code> is specified, the encrypted state must be <code>true</code>.</p>
    /// <p>You can specify the KMS key using any of the following:</p>
    /// <ul>
    /// <li> <p>Key ID. For example, 1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>
    /// <li> <p>Key alias. For example, alias/ExampleAlias.</p> </li>
    /// <li> <p>Key ARN. For example, arn:aws:kms:us-east-1:012345678910:key/1234abcd-12ab-34cd-56ef-1234567890ab.</p> </li>
    /// <li> <p>Alias ARN. For example, arn:aws:kms:us-east-1:012345678910:alias/ExampleAlias.</p> </li>
    /// </ul>
    /// <p>Amazon Web Services authenticates the KMS key asynchronously. Therefore, if you specify an ID, alias, or ARN that is not valid, the action can appear to complete, but eventually fails.</p>
    pub kms_key_id: ::std::option::Option<::std::string::String>,
    /// <p>When you copy an encrypted source snapshot using the Amazon EC2 Query API, you must supply a pre-signed URL. This parameter is optional for unencrypted snapshots. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html">Query requests</a>.</p>
    /// <p>The <code>PresignedUrl</code> should use the snapshot source endpoint, the <code>CopySnapshot</code> action, and include the <code>SourceRegion</code>, <code>SourceSnapshotId</code>, and <code>DestinationRegion</code> parameters. The <code>PresignedUrl</code> must be signed using Amazon Web Services Signature Version 4. Because EBS snapshots are stored in Amazon S3, the signing algorithm for this parameter uses the same logic that is described in <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html">Authenticating Requests: Using Query Parameters (Amazon Web Services Signature Version 4)</a> in the <i>Amazon Simple Storage Service API Reference</i>. An invalid or improperly signed <code>PresignedUrl</code> will cause the copy operation to fail asynchronously, and the snapshot will move to an <code>error</code> state.</p>
    pub presigned_url: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Region that contains the snapshot to be copied.</p>
    pub source_region: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the EBS snapshot to copy.</p>
    pub source_snapshot_id: ::std::option::Option<::std::string::String>,
    /// <p>The tags to apply to the new snapshot.</p>
    pub tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
}
impl CopySnapshotInput {
    /// <p>A description for the EBS snapshot.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost to which to copy the snapshot. Only specify this parameter when copying a snapshot from an Amazon Web Services Region to an Outpost. The snapshot must be in the Region for the destination Outpost. You cannot copy a snapshot from an Outpost to a Region, from one Outpost to another, or within the same Outpost.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#copy-snapshots"> Copy snapshots from an Amazon Web Services Region to an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn destination_outpost_arn(&self) -> ::std::option::Option<&str> {
        self.destination_outpost_arn.as_deref()
    }
    /// <p>The destination Region to use in the <code>PresignedUrl</code> parameter of a snapshot copy operation. This parameter is only valid for specifying the destination Region in a <code>PresignedUrl</code> parameter, where it is required.</p>
    /// <p>The snapshot copy is sent to the regional endpoint that you sent the HTTP request to (for example, <code>ec2.us-east-1.amazonaws.com</code>). With the CLI, this is specified using the <code>--region</code> parameter or the default Region in your Amazon Web Services configuration file.</p>
    pub fn destination_region(&self) -> ::std::option::Option<&str> {
        self.destination_region.as_deref()
    }
    /// <p>To encrypt a copy of an unencrypted snapshot if encryption by default is not enabled, enable encryption using this parameter. Otherwise, omit this parameter. Encrypted snapshots are encrypted, even if you omit this parameter and encryption by default is not enabled. You cannot set this parameter to false. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn encrypted(&self) -> ::std::option::Option<bool> {
        self.encrypted
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
    pub fn kms_key_id(&self) -> ::std::option::Option<&str> {
        self.kms_key_id.as_deref()
    }
    /// <p>When you copy an encrypted source snapshot using the Amazon EC2 Query API, you must supply a pre-signed URL. This parameter is optional for unencrypted snapshots. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html">Query requests</a>.</p>
    /// <p>The <code>PresignedUrl</code> should use the snapshot source endpoint, the <code>CopySnapshot</code> action, and include the <code>SourceRegion</code>, <code>SourceSnapshotId</code>, and <code>DestinationRegion</code> parameters. The <code>PresignedUrl</code> must be signed using Amazon Web Services Signature Version 4. Because EBS snapshots are stored in Amazon S3, the signing algorithm for this parameter uses the same logic that is described in <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html">Authenticating Requests: Using Query Parameters (Amazon Web Services Signature Version 4)</a> in the <i>Amazon Simple Storage Service API Reference</i>. An invalid or improperly signed <code>PresignedUrl</code> will cause the copy operation to fail asynchronously, and the snapshot will move to an <code>error</code> state.</p>
    pub fn presigned_url(&self) -> ::std::option::Option<&str> {
        self.presigned_url.as_deref()
    }
    /// <p>The ID of the Region that contains the snapshot to be copied.</p>
    pub fn source_region(&self) -> ::std::option::Option<&str> {
        self.source_region.as_deref()
    }
    /// <p>The ID of the EBS snapshot to copy.</p>
    pub fn source_snapshot_id(&self) -> ::std::option::Option<&str> {
        self.source_snapshot_id.as_deref()
    }
    /// <p>The tags to apply to the new snapshot.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tag_specifications.is_none()`.
    pub fn tag_specifications(&self) -> &[crate::types::TagSpecification] {
        self.tag_specifications.as_deref().unwrap_or_default()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
}
impl ::std::fmt::Debug for CopySnapshotInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CopySnapshotInput");
        formatter.field("description", &self.description);
        formatter.field("destination_outpost_arn", &self.destination_outpost_arn);
        formatter.field("destination_region", &self.destination_region);
        formatter.field("encrypted", &self.encrypted);
        formatter.field("kms_key_id", &self.kms_key_id);
        formatter.field("presigned_url", &"*** Sensitive Data Redacted ***");
        formatter.field("source_region", &self.source_region);
        formatter.field("source_snapshot_id", &self.source_snapshot_id);
        formatter.field("tag_specifications", &self.tag_specifications);
        formatter.field("dry_run", &self.dry_run);
        formatter.finish()
    }
}
impl CopySnapshotInput {
    /// Creates a new builder-style object to manufacture [`CopySnapshotInput`](crate::operation::copy_snapshot::CopySnapshotInput).
    pub fn builder() -> crate::operation::copy_snapshot::builders::CopySnapshotInputBuilder {
        crate::operation::copy_snapshot::builders::CopySnapshotInputBuilder::default()
    }
}

/// A builder for [`CopySnapshotInput`](crate::operation::copy_snapshot::CopySnapshotInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default)]
pub struct CopySnapshotInputBuilder {
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) destination_outpost_arn: ::std::option::Option<::std::string::String>,
    pub(crate) destination_region: ::std::option::Option<::std::string::String>,
    pub(crate) encrypted: ::std::option::Option<bool>,
    pub(crate) kms_key_id: ::std::option::Option<::std::string::String>,
    pub(crate) presigned_url: ::std::option::Option<::std::string::String>,
    pub(crate) source_region: ::std::option::Option<::std::string::String>,
    pub(crate) source_snapshot_id: ::std::option::Option<::std::string::String>,
    pub(crate) tag_specifications: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>,
    pub(crate) dry_run: ::std::option::Option<bool>,
}
impl CopySnapshotInputBuilder {
    /// <p>A description for the EBS snapshot.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description for the EBS snapshot.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description for the EBS snapshot.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost to which to copy the snapshot. Only specify this parameter when copying a snapshot from an Amazon Web Services Region to an Outpost. The snapshot must be in the Region for the destination Outpost. You cannot copy a snapshot from an Outpost to a Region, from one Outpost to another, or within the same Outpost.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#copy-snapshots"> Copy snapshots from an Amazon Web Services Region to an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn destination_outpost_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination_outpost_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost to which to copy the snapshot. Only specify this parameter when copying a snapshot from an Amazon Web Services Region to an Outpost. The snapshot must be in the Region for the destination Outpost. You cannot copy a snapshot from an Outpost to a Region, from one Outpost to another, or within the same Outpost.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#copy-snapshots"> Copy snapshots from an Amazon Web Services Region to an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn set_destination_outpost_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination_outpost_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost to which to copy the snapshot. Only specify this parameter when copying a snapshot from an Amazon Web Services Region to an Outpost. The snapshot must be in the Region for the destination Outpost. You cannot copy a snapshot from an Outpost to a Region, from one Outpost to another, or within the same Outpost.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/snapshots-outposts.html#copy-snapshots"> Copy snapshots from an Amazon Web Services Region to an Outpost</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn get_destination_outpost_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination_outpost_arn
    }
    /// <p>The destination Region to use in the <code>PresignedUrl</code> parameter of a snapshot copy operation. This parameter is only valid for specifying the destination Region in a <code>PresignedUrl</code> parameter, where it is required.</p>
    /// <p>The snapshot copy is sent to the regional endpoint that you sent the HTTP request to (for example, <code>ec2.us-east-1.amazonaws.com</code>). With the CLI, this is specified using the <code>--region</code> parameter or the default Region in your Amazon Web Services configuration file.</p>
    pub fn destination_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.destination_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The destination Region to use in the <code>PresignedUrl</code> parameter of a snapshot copy operation. This parameter is only valid for specifying the destination Region in a <code>PresignedUrl</code> parameter, where it is required.</p>
    /// <p>The snapshot copy is sent to the regional endpoint that you sent the HTTP request to (for example, <code>ec2.us-east-1.amazonaws.com</code>). With the CLI, this is specified using the <code>--region</code> parameter or the default Region in your Amazon Web Services configuration file.</p>
    pub fn set_destination_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.destination_region = input;
        self
    }
    /// <p>The destination Region to use in the <code>PresignedUrl</code> parameter of a snapshot copy operation. This parameter is only valid for specifying the destination Region in a <code>PresignedUrl</code> parameter, where it is required.</p>
    /// <p>The snapshot copy is sent to the regional endpoint that you sent the HTTP request to (for example, <code>ec2.us-east-1.amazonaws.com</code>). With the CLI, this is specified using the <code>--region</code> parameter or the default Region in your Amazon Web Services configuration file.</p>
    pub fn get_destination_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.destination_region
    }
    /// <p>To encrypt a copy of an unencrypted snapshot if encryption by default is not enabled, enable encryption using this parameter. Otherwise, omit this parameter. Encrypted snapshots are encrypted, even if you omit this parameter and encryption by default is not enabled. You cannot set this parameter to false. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn encrypted(mut self, input: bool) -> Self {
        self.encrypted = ::std::option::Option::Some(input);
        self
    }
    /// <p>To encrypt a copy of an unencrypted snapshot if encryption by default is not enabled, enable encryption using this parameter. Otherwise, omit this parameter. Encrypted snapshots are encrypted, even if you omit this parameter and encryption by default is not enabled. You cannot set this parameter to false. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn set_encrypted(mut self, input: ::std::option::Option<bool>) -> Self {
        self.encrypted = input;
        self
    }
    /// <p>To encrypt a copy of an unencrypted snapshot if encryption by default is not enabled, enable encryption using this parameter. Otherwise, omit this parameter. Encrypted snapshots are encrypted, even if you omit this parameter and encryption by default is not enabled. You cannot set this parameter to false. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/EBSEncryption.html">Amazon EBS encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    pub fn get_encrypted(&self) -> &::std::option::Option<bool> {
        &self.encrypted
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
        self.kms_key_id = ::std::option::Option::Some(input.into());
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
        self.kms_key_id = input;
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
        &self.kms_key_id
    }
    /// <p>When you copy an encrypted source snapshot using the Amazon EC2 Query API, you must supply a pre-signed URL. This parameter is optional for unencrypted snapshots. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html">Query requests</a>.</p>
    /// <p>The <code>PresignedUrl</code> should use the snapshot source endpoint, the <code>CopySnapshot</code> action, and include the <code>SourceRegion</code>, <code>SourceSnapshotId</code>, and <code>DestinationRegion</code> parameters. The <code>PresignedUrl</code> must be signed using Amazon Web Services Signature Version 4. Because EBS snapshots are stored in Amazon S3, the signing algorithm for this parameter uses the same logic that is described in <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html">Authenticating Requests: Using Query Parameters (Amazon Web Services Signature Version 4)</a> in the <i>Amazon Simple Storage Service API Reference</i>. An invalid or improperly signed <code>PresignedUrl</code> will cause the copy operation to fail asynchronously, and the snapshot will move to an <code>error</code> state.</p>
    pub fn presigned_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.presigned_url = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>When you copy an encrypted source snapshot using the Amazon EC2 Query API, you must supply a pre-signed URL. This parameter is optional for unencrypted snapshots. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html">Query requests</a>.</p>
    /// <p>The <code>PresignedUrl</code> should use the snapshot source endpoint, the <code>CopySnapshot</code> action, and include the <code>SourceRegion</code>, <code>SourceSnapshotId</code>, and <code>DestinationRegion</code> parameters. The <code>PresignedUrl</code> must be signed using Amazon Web Services Signature Version 4. Because EBS snapshots are stored in Amazon S3, the signing algorithm for this parameter uses the same logic that is described in <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html">Authenticating Requests: Using Query Parameters (Amazon Web Services Signature Version 4)</a> in the <i>Amazon Simple Storage Service API Reference</i>. An invalid or improperly signed <code>PresignedUrl</code> will cause the copy operation to fail asynchronously, and the snapshot will move to an <code>error</code> state.</p>
    pub fn set_presigned_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.presigned_url = input;
        self
    }
    /// <p>When you copy an encrypted source snapshot using the Amazon EC2 Query API, you must supply a pre-signed URL. This parameter is optional for unencrypted snapshots. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html">Query requests</a>.</p>
    /// <p>The <code>PresignedUrl</code> should use the snapshot source endpoint, the <code>CopySnapshot</code> action, and include the <code>SourceRegion</code>, <code>SourceSnapshotId</code>, and <code>DestinationRegion</code> parameters. The <code>PresignedUrl</code> must be signed using Amazon Web Services Signature Version 4. Because EBS snapshots are stored in Amazon S3, the signing algorithm for this parameter uses the same logic that is described in <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/sigv4-query-string-auth.html">Authenticating Requests: Using Query Parameters (Amazon Web Services Signature Version 4)</a> in the <i>Amazon Simple Storage Service API Reference</i>. An invalid or improperly signed <code>PresignedUrl</code> will cause the copy operation to fail asynchronously, and the snapshot will move to an <code>error</code> state.</p>
    pub fn get_presigned_url(&self) -> &::std::option::Option<::std::string::String> {
        &self.presigned_url
    }
    /// <p>The ID of the Region that contains the snapshot to be copied.</p>
    /// This field is required.
    pub fn source_region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_region = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Region that contains the snapshot to be copied.</p>
    pub fn set_source_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_region = input;
        self
    }
    /// <p>The ID of the Region that contains the snapshot to be copied.</p>
    pub fn get_source_region(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_region
    }
    /// <p>The ID of the EBS snapshot to copy.</p>
    /// This field is required.
    pub fn source_snapshot_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.source_snapshot_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the EBS snapshot to copy.</p>
    pub fn set_source_snapshot_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.source_snapshot_id = input;
        self
    }
    /// <p>The ID of the EBS snapshot to copy.</p>
    pub fn get_source_snapshot_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.source_snapshot_id
    }
    /// Appends an item to `tag_specifications`.
    ///
    /// To override the contents of this collection use [`set_tag_specifications`](Self::set_tag_specifications).
    ///
    /// <p>The tags to apply to the new snapshot.</p>
    pub fn tag_specifications(mut self, input: crate::types::TagSpecification) -> Self {
        let mut v = self.tag_specifications.unwrap_or_default();
        v.push(input);
        self.tag_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags to apply to the new snapshot.</p>
    pub fn set_tag_specifications(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>>) -> Self {
        self.tag_specifications = input;
        self
    }
    /// <p>The tags to apply to the new snapshot.</p>
    pub fn get_tag_specifications(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::TagSpecification>> {
        &self.tag_specifications
    }
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
    /// Consumes the builder and constructs a [`CopySnapshotInput`](crate::operation::copy_snapshot::CopySnapshotInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::copy_snapshot::CopySnapshotInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::copy_snapshot::CopySnapshotInput {
            description: self.description,
            destination_outpost_arn: self.destination_outpost_arn,
            destination_region: self.destination_region,
            encrypted: self.encrypted,
            kms_key_id: self.kms_key_id,
            presigned_url: self.presigned_url,
            source_region: self.source_region,
            source_snapshot_id: self.source_snapshot_id,
            tag_specifications: self.tag_specifications,
            dry_run: self.dry_run,
        })
    }
}
impl ::std::fmt::Debug for CopySnapshotInputBuilder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let mut formatter = f.debug_struct("CopySnapshotInputBuilder");
        formatter.field("description", &self.description);
        formatter.field("destination_outpost_arn", &self.destination_outpost_arn);
        formatter.field("destination_region", &self.destination_region);
        formatter.field("encrypted", &self.encrypted);
        formatter.field("kms_key_id", &self.kms_key_id);
        formatter.field("presigned_url", &"*** Sensitive Data Redacted ***");
        formatter.field("source_region", &self.source_region);
        formatter.field("source_snapshot_id", &self.source_snapshot_id);
        formatter.field("tag_specifications", &self.tag_specifications);
        formatter.field("dry_run", &self.dry_run);
        formatter.finish()
    }
}
