// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeTrunkInterfaceAssociations`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`association_ids(impl Into<String>)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::association_ids) / [`set_association_ids(Option<Vec::<String>>)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::set_association_ids):<br>required: **false**<br><p>The IDs of the associations.</p><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::set_filters):<br>required: **false**<br><p>One or more filters.</p>  <ul>   <li> <p> <code>gre-key</code> - The ID of a trunk interface association.</p> </li>   <li> <p> <code>interface-protocol</code> - The interface protocol. Valid values are <code>VLAN</code> and <code>GRE</code>.</p> </li>  </ul><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return with a single call. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p><br>
    /// - On success, responds with [`DescribeTrunkInterfaceAssociationsOutput`](crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsOutput) with field(s):
    ///   - [`interface_associations(Option<Vec::<TrunkInterfaceAssociation>>)`](crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsOutput::interface_associations): <p>Information about the trunk associations.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsOutput::next_token): <p>The token to use to retrieve the next page of results. This value is <code>null</code> when there are no more results to return.</p>
    /// - On failure, responds with [`SdkError<DescribeTrunkInterfaceAssociationsError>`](crate::operation::describe_trunk_interface_associations::DescribeTrunkInterfaceAssociationsError)
    pub fn describe_trunk_interface_associations(
        &self,
    ) -> crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder {
        crate::operation::describe_trunk_interface_associations::builders::DescribeTrunkInterfaceAssociationsFluentBuilder::new(self.handle.clone())
    }
}
