// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeIpamResourceDiscoveryAssociationsOutput {
    /// <p>The resource discovery associations.</p>
    pub ipam_resource_discovery_associations: ::std::option::Option<::std::vec::Vec<crate::types::IpamResourceDiscoveryAssociation>>,
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeIpamResourceDiscoveryAssociationsOutput {
    /// <p>The resource discovery associations.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.ipam_resource_discovery_associations.is_none()`.
    pub fn ipam_resource_discovery_associations(&self) -> &[crate::types::IpamResourceDiscoveryAssociation] {
        self.ipam_resource_discovery_associations.as_deref().unwrap_or_default()
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeIpamResourceDiscoveryAssociationsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeIpamResourceDiscoveryAssociationsOutput {
    /// Creates a new builder-style object to manufacture [`DescribeIpamResourceDiscoveryAssociationsOutput`](crate::operation::describe_ipam_resource_discovery_associations::DescribeIpamResourceDiscoveryAssociationsOutput).
    pub fn builder(
    ) -> crate::operation::describe_ipam_resource_discovery_associations::builders::DescribeIpamResourceDiscoveryAssociationsOutputBuilder {
        crate::operation::describe_ipam_resource_discovery_associations::builders::DescribeIpamResourceDiscoveryAssociationsOutputBuilder::default()
    }
}

/// A builder for [`DescribeIpamResourceDiscoveryAssociationsOutput`](crate::operation::describe_ipam_resource_discovery_associations::DescribeIpamResourceDiscoveryAssociationsOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeIpamResourceDiscoveryAssociationsOutputBuilder {
    pub(crate) ipam_resource_discovery_associations: ::std::option::Option<::std::vec::Vec<crate::types::IpamResourceDiscoveryAssociation>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DescribeIpamResourceDiscoveryAssociationsOutputBuilder {
    /// Appends an item to `ipam_resource_discovery_associations`.
    ///
    /// To override the contents of this collection use [`set_ipam_resource_discovery_associations`](Self::set_ipam_resource_discovery_associations).
    ///
    /// <p>The resource discovery associations.</p>
    pub fn ipam_resource_discovery_associations(mut self, input: crate::types::IpamResourceDiscoveryAssociation) -> Self {
        let mut v = self.ipam_resource_discovery_associations.unwrap_or_default();
        v.push(input);
        self.ipam_resource_discovery_associations = ::std::option::Option::Some(v);
        self
    }
    /// <p>The resource discovery associations.</p>
    pub fn set_ipam_resource_discovery_associations(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::IpamResourceDiscoveryAssociation>>,
    ) -> Self {
        self.ipam_resource_discovery_associations = input;
        self
    }
    /// <p>The resource discovery associations.</p>
    pub fn get_ipam_resource_discovery_associations(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::IpamResourceDiscoveryAssociation>> {
        &self.ipam_resource_discovery_associations
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>Specify the pagination token from a previous request to retrieve the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeIpamResourceDiscoveryAssociationsOutput`](crate::operation::describe_ipam_resource_discovery_associations::DescribeIpamResourceDiscoveryAssociationsOutput).
    pub fn build(self) -> crate::operation::describe_ipam_resource_discovery_associations::DescribeIpamResourceDiscoveryAssociationsOutput {
        crate::operation::describe_ipam_resource_discovery_associations::DescribeIpamResourceDiscoveryAssociationsOutput {
            ipam_resource_discovery_associations: self.ipam_resource_discovery_associations,
            next_token: self.next_token,
            _request_id: self._request_id,
        }
    }
}