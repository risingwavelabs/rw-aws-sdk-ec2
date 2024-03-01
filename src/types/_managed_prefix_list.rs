// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a managed prefix list.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ManagedPrefixList {
    /// <p>The ID of the prefix list.</p>
    pub prefix_list_id: ::std::option::Option<::std::string::String>,
    /// <p>The IP address version.</p>
    pub address_family: ::std::option::Option<::std::string::String>,
    /// <p>The current state of the prefix list.</p>
    pub state: ::std::option::Option<crate::types::PrefixListState>,
    /// <p>The state message.</p>
    pub state_message: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) for the prefix list.</p>
    pub prefix_list_arn: ::std::option::Option<::std::string::String>,
    /// <p>The name of the prefix list.</p>
    pub prefix_list_name: ::std::option::Option<::std::string::String>,
    /// <p>The maximum number of entries for the prefix list.</p>
    pub max_entries: ::std::option::Option<i32>,
    /// <p>The version of the prefix list.</p>
    pub version: ::std::option::Option<i64>,
    /// <p>The tags for the prefix list.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    /// <p>The ID of the owner of the prefix list.</p>
    pub owner_id: ::std::option::Option<::std::string::String>,
}
impl ManagedPrefixList {
    /// <p>The ID of the prefix list.</p>
    pub fn prefix_list_id(&self) -> ::std::option::Option<&str> {
        self.prefix_list_id.as_deref()
    }
    /// <p>The IP address version.</p>
    pub fn address_family(&self) -> ::std::option::Option<&str> {
        self.address_family.as_deref()
    }
    /// <p>The current state of the prefix list.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::PrefixListState> {
        self.state.as_ref()
    }
    /// <p>The state message.</p>
    pub fn state_message(&self) -> ::std::option::Option<&str> {
        self.state_message.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) for the prefix list.</p>
    pub fn prefix_list_arn(&self) -> ::std::option::Option<&str> {
        self.prefix_list_arn.as_deref()
    }
    /// <p>The name of the prefix list.</p>
    pub fn prefix_list_name(&self) -> ::std::option::Option<&str> {
        self.prefix_list_name.as_deref()
    }
    /// <p>The maximum number of entries for the prefix list.</p>
    pub fn max_entries(&self) -> ::std::option::Option<i32> {
        self.max_entries
    }
    /// <p>The version of the prefix list.</p>
    pub fn version(&self) -> ::std::option::Option<i64> {
        self.version
    }
    /// <p>The tags for the prefix list.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
    /// <p>The ID of the owner of the prefix list.</p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
}
impl ManagedPrefixList {
    /// Creates a new builder-style object to manufacture [`ManagedPrefixList`](crate::types::ManagedPrefixList).
    pub fn builder() -> crate::types::builders::ManagedPrefixListBuilder {
        crate::types::builders::ManagedPrefixListBuilder::default()
    }
}

/// A builder for [`ManagedPrefixList`](crate::types::ManagedPrefixList).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ManagedPrefixListBuilder {
    pub(crate) prefix_list_id: ::std::option::Option<::std::string::String>,
    pub(crate) address_family: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::PrefixListState>,
    pub(crate) state_message: ::std::option::Option<::std::string::String>,
    pub(crate) prefix_list_arn: ::std::option::Option<::std::string::String>,
    pub(crate) prefix_list_name: ::std::option::Option<::std::string::String>,
    pub(crate) max_entries: ::std::option::Option<i32>,
    pub(crate) version: ::std::option::Option<i64>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
}
impl ManagedPrefixListBuilder {
    /// <p>The ID of the prefix list.</p>
    pub fn prefix_list_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix_list_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn set_prefix_list_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix_list_id = input;
        self
    }
    /// <p>The ID of the prefix list.</p>
    pub fn get_prefix_list_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix_list_id
    }
    /// <p>The IP address version.</p>
    pub fn address_family(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.address_family = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The IP address version.</p>
    pub fn set_address_family(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.address_family = input;
        self
    }
    /// <p>The IP address version.</p>
    pub fn get_address_family(&self) -> &::std::option::Option<::std::string::String> {
        &self.address_family
    }
    /// <p>The current state of the prefix list.</p>
    pub fn state(mut self, input: crate::types::PrefixListState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The current state of the prefix list.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::PrefixListState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The current state of the prefix list.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::PrefixListState> {
        &self.state
    }
    /// <p>The state message.</p>
    pub fn state_message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.state_message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The state message.</p>
    pub fn set_state_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.state_message = input;
        self
    }
    /// <p>The state message.</p>
    pub fn get_state_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.state_message
    }
    /// <p>The Amazon Resource Name (ARN) for the prefix list.</p>
    pub fn prefix_list_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix_list_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the prefix list.</p>
    pub fn set_prefix_list_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix_list_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) for the prefix list.</p>
    pub fn get_prefix_list_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix_list_arn
    }
    /// <p>The name of the prefix list.</p>
    pub fn prefix_list_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.prefix_list_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the prefix list.</p>
    pub fn set_prefix_list_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.prefix_list_name = input;
        self
    }
    /// <p>The name of the prefix list.</p>
    pub fn get_prefix_list_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.prefix_list_name
    }
    /// <p>The maximum number of entries for the prefix list.</p>
    pub fn max_entries(mut self, input: i32) -> Self {
        self.max_entries = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of entries for the prefix list.</p>
    pub fn set_max_entries(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_entries = input;
        self
    }
    /// <p>The maximum number of entries for the prefix list.</p>
    pub fn get_max_entries(&self) -> &::std::option::Option<i32> {
        &self.max_entries
    }
    /// <p>The version of the prefix list.</p>
    pub fn version(mut self, input: i64) -> Self {
        self.version = ::std::option::Option::Some(input);
        self
    }
    /// <p>The version of the prefix list.</p>
    pub fn set_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.version = input;
        self
    }
    /// <p>The version of the prefix list.</p>
    pub fn get_version(&self) -> &::std::option::Option<i64> {
        &self.version
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the prefix list.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags for the prefix list.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags for the prefix list.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// <p>The ID of the owner of the prefix list.</p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the owner of the prefix list.</p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The ID of the owner of the prefix list.</p>
    pub fn get_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner_id
    }
    /// Consumes the builder and constructs a [`ManagedPrefixList`](crate::types::ManagedPrefixList).
    pub fn build(self) -> crate::types::ManagedPrefixList {
        crate::types::ManagedPrefixList {
            prefix_list_id: self.prefix_list_id,
            address_family: self.address_family,
            state: self.state,
            state_message: self.state_message,
            prefix_list_arn: self.prefix_list_arn,
            prefix_list_name: self.prefix_list_name,
            max_entries: self.max_entries,
            version: self.version,
            tags: self.tags,
            owner_id: self.owner_id,
        }
    }
}