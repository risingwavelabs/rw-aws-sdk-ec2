// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeKeyPairs`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`filters(Filter)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::set_filters):<br>required: **false**<br><p>The filters.</p>  <ul>   <li> <p> <code>key-pair-id</code> - The ID of the key pair.</p> </li>   <li> <p> <code>fingerprint</code> - The fingerprint of the key pair.</p> </li>   <li> <p> <code>key-name</code> - The name of the key pair.</p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>  </ul><br>
    ///   - [`key_names(impl Into<String>)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::key_names) / [`set_key_names(Option<Vec::<String>>)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::set_key_names):<br>required: **false**<br><p>The key pair names.</p>  <p>Default: Describes all of your key pairs.</p><br>
    ///   - [`key_pair_ids(impl Into<String>)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::key_pair_ids) / [`set_key_pair_ids(Option<Vec::<String>>)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::set_key_pair_ids):<br>required: **false**<br><p>The IDs of the key pairs.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`include_public_key(bool)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::include_public_key) / [`set_include_public_key(Option<bool>)`](crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::set_include_public_key):<br>required: **false**<br><p>If <code>true</code>, the public key material is included in the response.</p>  <p>Default: <code>false</code> </p><br>
    /// - On success, responds with [`DescribeKeyPairsOutput`](crate::operation::describe_key_pairs::DescribeKeyPairsOutput) with field(s):
    ///   - [`key_pairs(Option<Vec::<KeyPairInfo>>)`](crate::operation::describe_key_pairs::DescribeKeyPairsOutput::key_pairs): <p>Information about the key pairs.</p>
    /// - On failure, responds with [`SdkError<DescribeKeyPairsError>`](crate::operation::describe_key_pairs::DescribeKeyPairsError)
    pub fn describe_key_pairs(&self) -> crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder {
        crate::operation::describe_key_pairs::builders::DescribeKeyPairsFluentBuilder::new(self.handle.clone())
    }
}
