// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateKeyPair`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`key_name(impl Into<String>)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::key_name) / [`set_key_name(Option<String>)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::set_key_name):<br>required: **true**<br><p>A unique name for the key pair.</p>  <p>Constraints: Up to 255 ASCII characters</p><br>
    ///   - [`dry_run(bool)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`key_type(KeyType)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::key_type) / [`set_key_type(Option<KeyType>)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::set_key_type):<br>required: **false**<br><p>The type of key pair. Note that ED25519 keys are not supported for Windows instances.</p>  <p>Default: <code>rsa</code> </p><br>
    ///   - [`tag_specifications(TagSpecification)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::tag_specifications) / [`set_tag_specifications(Option<Vec::<TagSpecification>>)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::set_tag_specifications):<br>required: **false**<br><p>The tags to apply to the new key pair.</p><br>
    ///   - [`key_format(KeyFormat)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::key_format) / [`set_key_format(Option<KeyFormat>)`](crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::set_key_format):<br>required: **false**<br><p>The format of the key pair.</p>  <p>Default: <code>pem</code> </p><br>
    /// - On success, responds with [`CreateKeyPairOutput`](crate::operation::create_key_pair::CreateKeyPairOutput) with field(s):
    ///   - [`key_fingerprint(Option<String>)`](crate::operation::create_key_pair::CreateKeyPairOutput::key_fingerprint): <ul>   <li> <p>For RSA key pairs, the key fingerprint is the SHA-1 digest of the DER encoded private key.</p> </li>   <li> <p>For ED25519 key pairs, the key fingerprint is the base64-encoded SHA-256 digest, which is the default for OpenSSH, starting with OpenSSH 6.8.</p> </li>  </ul>
    ///   - [`key_material(Option<String>)`](crate::operation::create_key_pair::CreateKeyPairOutput::key_material): <p>An unencrypted PEM encoded RSA or ED25519 private key.</p>
    ///   - [`key_name(Option<String>)`](crate::operation::create_key_pair::CreateKeyPairOutput::key_name): <p>The name of the key pair.</p>
    ///   - [`key_pair_id(Option<String>)`](crate::operation::create_key_pair::CreateKeyPairOutput::key_pair_id): <p>The ID of the key pair.</p>
    ///   - [`tags(Option<Vec::<Tag>>)`](crate::operation::create_key_pair::CreateKeyPairOutput::tags): <p>Any tags applied to the key pair.</p>
    /// - On failure, responds with [`SdkError<CreateKeyPairError>`](crate::operation::create_key_pair::CreateKeyPairError)
    pub fn create_key_pair(&self) -> crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder {
        crate::operation::create_key_pair::builders::CreateKeyPairFluentBuilder::new(self.handle.clone())
    }
}
