// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeAddressesAttribute`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`allocation_ids(impl Into<String>)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::allocation_ids) / [`set_allocation_ids(Option<Vec::<String>>)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::set_allocation_ids):<br>required: **false**<br><p>[EC2-VPC] The allocation IDs.</p><br>
    ///   - [`attribute(AddressAttributeName)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::attribute) / [`set_attribute(Option<AddressAttributeName>)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::set_attribute):<br>required: **false**<br><p>The attribute of the IP address.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DescribeAddressesAttributeOutput`](crate::operation::describe_addresses_attribute::DescribeAddressesAttributeOutput) with field(s):
    ///   - [`addresses(Option<Vec::<AddressAttribute>>)`](crate::operation::describe_addresses_attribute::DescribeAddressesAttributeOutput::addresses): <p>Information about the IP addresses.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_addresses_attribute::DescribeAddressesAttributeOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeAddressesAttributeError>`](crate::operation::describe_addresses_attribute::DescribeAddressesAttributeError)
    pub fn describe_addresses_attribute(&self) -> crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder {
        crate::operation::describe_addresses_attribute::builders::DescribeAddressesAttributeFluentBuilder::new(self.handle.clone())
    }
}
