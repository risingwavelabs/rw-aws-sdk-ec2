// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the instance topology.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceTopology {
    /// <p>The instance ID.</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The instance type.</p>
    pub instance_type: ::std::option::Option<::std::string::String>,
    /// <p>The name of the placement group that the instance is in.</p>
    pub group_name: ::std::option::Option<::std::string::String>,
    /// <p>The network nodes. The nodes are hashed based on your account. Instances from different accounts running under the same droplet will return a different hashed list of strings.</p>
    pub network_nodes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The name of the Availability Zone or Local Zone that the instance is in.</p>
    pub availability_zone: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the Availability Zone or Local Zone that the instance is in.</p>
    pub zone_id: ::std::option::Option<::std::string::String>,
}
impl InstanceTopology {
    /// <p>The instance ID.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The instance type.</p>
    pub fn instance_type(&self) -> ::std::option::Option<&str> {
        self.instance_type.as_deref()
    }
    /// <p>The name of the placement group that the instance is in.</p>
    pub fn group_name(&self) -> ::std::option::Option<&str> {
        self.group_name.as_deref()
    }
    /// <p>The network nodes. The nodes are hashed based on your account. Instances from different accounts running under the same droplet will return a different hashed list of strings.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.network_nodes.is_none()`.
    pub fn network_nodes(&self) -> &[::std::string::String] {
        self.network_nodes.as_deref().unwrap_or_default()
    }
    /// <p>The name of the Availability Zone or Local Zone that the instance is in.</p>
    pub fn availability_zone(&self) -> ::std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>The ID of the Availability Zone or Local Zone that the instance is in.</p>
    pub fn zone_id(&self) -> ::std::option::Option<&str> {
        self.zone_id.as_deref()
    }
}
impl InstanceTopology {
    /// Creates a new builder-style object to manufacture [`InstanceTopology`](crate::types::InstanceTopology).
    pub fn builder() -> crate::types::builders::InstanceTopologyBuilder {
        crate::types::builders::InstanceTopologyBuilder::default()
    }
}

/// A builder for [`InstanceTopology`](crate::types::InstanceTopology).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InstanceTopologyBuilder {
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) instance_type: ::std::option::Option<::std::string::String>,
    pub(crate) group_name: ::std::option::Option<::std::string::String>,
    pub(crate) network_nodes: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) availability_zone: ::std::option::Option<::std::string::String>,
    pub(crate) zone_id: ::std::option::Option<::std::string::String>,
}
impl InstanceTopologyBuilder {
    /// <p>The instance ID.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The instance ID.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The instance ID.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_id
    }
    /// <p>The instance type.</p>
    pub fn instance_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_type = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The instance type.</p>
    pub fn set_instance_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The instance type.</p>
    pub fn get_instance_type(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_type
    }
    /// <p>The name of the placement group that the instance is in.</p>
    pub fn group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the placement group that the instance is in.</p>
    pub fn set_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.group_name = input;
        self
    }
    /// <p>The name of the placement group that the instance is in.</p>
    pub fn get_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.group_name
    }
    /// Appends an item to `network_nodes`.
    ///
    /// To override the contents of this collection use [`set_network_nodes`](Self::set_network_nodes).
    ///
    /// <p>The network nodes. The nodes are hashed based on your account. Instances from different accounts running under the same droplet will return a different hashed list of strings.</p>
    pub fn network_nodes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.network_nodes.unwrap_or_default();
        v.push(input.into());
        self.network_nodes = ::std::option::Option::Some(v);
        self
    }
    /// <p>The network nodes. The nodes are hashed based on your account. Instances from different accounts running under the same droplet will return a different hashed list of strings.</p>
    pub fn set_network_nodes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.network_nodes = input;
        self
    }
    /// <p>The network nodes. The nodes are hashed based on your account. Instances from different accounts running under the same droplet will return a different hashed list of strings.</p>
    pub fn get_network_nodes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.network_nodes
    }
    /// <p>The name of the Availability Zone or Local Zone that the instance is in.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.availability_zone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Availability Zone or Local Zone that the instance is in.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>The name of the Availability Zone or Local Zone that the instance is in.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        &self.availability_zone
    }
    /// <p>The ID of the Availability Zone or Local Zone that the instance is in.</p>
    pub fn zone_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.zone_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Availability Zone or Local Zone that the instance is in.</p>
    pub fn set_zone_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.zone_id = input;
        self
    }
    /// <p>The ID of the Availability Zone or Local Zone that the instance is in.</p>
    pub fn get_zone_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.zone_id
    }
    /// Consumes the builder and constructs a [`InstanceTopology`](crate::types::InstanceTopology).
    pub fn build(self) -> crate::types::InstanceTopology {
        crate::types::InstanceTopology {
            instance_id: self.instance_id,
            instance_type: self.instance_type,
            group_name: self.group_name,
            network_nodes: self.network_nodes,
            availability_zone: self.availability_zone,
            zone_id: self.zone_id,
        }
    }
}