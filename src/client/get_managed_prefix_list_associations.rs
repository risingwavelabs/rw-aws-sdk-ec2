// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetManagedPrefixListAssociations`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`prefix_list_id(impl Into<String>)`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::prefix_list_id) / [`set_prefix_list_id(Option<String>)`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::set_prefix_list_id):<br>required: **true**<br><p>The ID of the prefix list.</p><br>
    ///   - [`max_results(i32)`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    /// - On success, responds with [`GetManagedPrefixListAssociationsOutput`](crate::operation::get_managed_prefix_list_associations::GetManagedPrefixListAssociationsOutput) with field(s):
    ///   - [`prefix_list_associations(Option<Vec::<PrefixListAssociation>>)`](crate::operation::get_managed_prefix_list_associations::GetManagedPrefixListAssociationsOutput::prefix_list_associations): <p>Information about the associations.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::get_managed_prefix_list_associations::GetManagedPrefixListAssociationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<GetManagedPrefixListAssociationsError>`](crate::operation::get_managed_prefix_list_associations::GetManagedPrefixListAssociationsError)
    pub fn get_managed_prefix_list_associations(
        &self,
    ) -> crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder {
        crate::operation::get_managed_prefix_list_associations::builders::GetManagedPrefixListAssociationsFluentBuilder::new(self.handle.clone())
    }
}
