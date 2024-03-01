// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a Region.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Region {
    /// <p>The Region service endpoint.</p>
    pub endpoint: ::std::option::Option<::std::string::String>,
    /// <p>The name of the Region.</p>
    pub region_name: ::std::option::Option<::std::string::String>,
    /// <p>The Region opt-in status. The possible values are <code>opt-in-not-required</code>, <code>opted-in</code>, and <code>not-opted-in</code>.</p>
    pub opt_in_status: ::std::option::Option<::std::string::String>,
}
impl Region {
    /// <p>The Region service endpoint.</p>
    pub fn endpoint(&self) -> ::std::option::Option<&str> {
        self.endpoint.as_deref()
    }
    /// <p>The name of the Region.</p>
    pub fn region_name(&self) -> ::std::option::Option<&str> {
        self.region_name.as_deref()
    }
    /// <p>The Region opt-in status. The possible values are <code>opt-in-not-required</code>, <code>opted-in</code>, and <code>not-opted-in</code>.</p>
    pub fn opt_in_status(&self) -> ::std::option::Option<&str> {
        self.opt_in_status.as_deref()
    }
}
impl Region {
    /// Creates a new builder-style object to manufacture [`Region`](crate::types::Region).
    pub fn builder() -> crate::types::builders::RegionBuilder {
        crate::types::builders::RegionBuilder::default()
    }
}

/// A builder for [`Region`](crate::types::Region).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RegionBuilder {
    pub(crate) endpoint: ::std::option::Option<::std::string::String>,
    pub(crate) region_name: ::std::option::Option<::std::string::String>,
    pub(crate) opt_in_status: ::std::option::Option<::std::string::String>,
}
impl RegionBuilder {
    /// <p>The Region service endpoint.</p>
    pub fn endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.endpoint = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region service endpoint.</p>
    pub fn set_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.endpoint = input;
        self
    }
    /// <p>The Region service endpoint.</p>
    pub fn get_endpoint(&self) -> &::std::option::Option<::std::string::String> {
        &self.endpoint
    }
    /// <p>The name of the Region.</p>
    pub fn region_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.region_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the Region.</p>
    pub fn set_region_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.region_name = input;
        self
    }
    /// <p>The name of the Region.</p>
    pub fn get_region_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.region_name
    }
    /// <p>The Region opt-in status. The possible values are <code>opt-in-not-required</code>, <code>opted-in</code>, and <code>not-opted-in</code>.</p>
    pub fn opt_in_status(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.opt_in_status = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Region opt-in status. The possible values are <code>opt-in-not-required</code>, <code>opted-in</code>, and <code>not-opted-in</code>.</p>
    pub fn set_opt_in_status(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.opt_in_status = input;
        self
    }
    /// <p>The Region opt-in status. The possible values are <code>opt-in-not-required</code>, <code>opted-in</code>, and <code>not-opted-in</code>.</p>
    pub fn get_opt_in_status(&self) -> &::std::option::Option<::std::string::String> {
        &self.opt_in_status
    }
    /// Consumes the builder and constructs a [`Region`](crate::types::Region).
    pub fn build(self) -> crate::types::Region {
        crate::types::Region {
            endpoint: self.endpoint,
            region_name: self.region_name,
            opt_in_status: self.opt_in_status,
        }
    }
}