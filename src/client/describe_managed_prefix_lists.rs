// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeManagedPrefixLists`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`dry_run(bool)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::set_filters):<br>required: **false**<br><p>One or more filters.</p>  <ul>   <li> <p> <code>owner-id</code> - The ID of the prefix list owner.</p> </li>   <li> <p> <code>prefix-list-id</code> - The ID of the prefix list.</p> </li>   <li> <p> <code>prefix-list-name</code> - The name of the prefix list.</p> </li>  </ul><br>
    ///   - [`max_results(i32)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    ///   - [`prefix_list_ids(impl Into<String>)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::prefix_list_ids) / [`set_prefix_list_ids(Option<Vec::<String>>)`](crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::set_prefix_list_ids):<br>required: **false**<br><p>One or more prefix list IDs.</p><br>
    /// - On success, responds with [`DescribeManagedPrefixListsOutput`](crate::operation::describe_managed_prefix_lists::DescribeManagedPrefixListsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::describe_managed_prefix_lists::DescribeManagedPrefixListsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    ///   - [`prefix_lists(Option<Vec::<ManagedPrefixList>>)`](crate::operation::describe_managed_prefix_lists::DescribeManagedPrefixListsOutput::prefix_lists): <p>Information about the prefix lists.</p>
    /// - On failure, responds with [`SdkError<DescribeManagedPrefixListsError>`](crate::operation::describe_managed_prefix_lists::DescribeManagedPrefixListsError)
    pub fn describe_managed_prefix_lists(
        &self,
    ) -> crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder {
        crate::operation::describe_managed_prefix_lists::builders::DescribeManagedPrefixListsFluentBuilder::new(self.handle.clone())
    }
}