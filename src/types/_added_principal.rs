// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a principal.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AddedPrincipal {
    /// <p>The type of principal.</p>
    pub principal_type: ::std::option::Option<crate::types::PrincipalType>,
    /// <p>The Amazon Resource Name (ARN) of the principal.</p>
    pub principal: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the service permission.</p>
    pub service_permission_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the service.</p>
    pub service_id: ::std::option::Option<::std::string::String>,
}
impl AddedPrincipal {
    /// <p>The type of principal.</p>
    pub fn principal_type(&self) -> ::std::option::Option<&crate::types::PrincipalType> {
        self.principal_type.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the principal.</p>
    pub fn principal(&self) -> ::std::option::Option<&str> {
        self.principal.as_deref()
    }
    /// <p>The ID of the service permission.</p>
    pub fn service_permission_id(&self) -> ::std::option::Option<&str> {
        self.service_permission_id.as_deref()
    }
    /// <p>The ID of the service.</p>
    pub fn service_id(&self) -> ::std::option::Option<&str> {
        self.service_id.as_deref()
    }
}
impl AddedPrincipal {
    /// Creates a new builder-style object to manufacture [`AddedPrincipal`](crate::types::AddedPrincipal).
    pub fn builder() -> crate::types::builders::AddedPrincipalBuilder {
        crate::types::builders::AddedPrincipalBuilder::default()
    }
}

/// A builder for [`AddedPrincipal`](crate::types::AddedPrincipal).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AddedPrincipalBuilder {
    pub(crate) principal_type: ::std::option::Option<crate::types::PrincipalType>,
    pub(crate) principal: ::std::option::Option<::std::string::String>,
    pub(crate) service_permission_id: ::std::option::Option<::std::string::String>,
    pub(crate) service_id: ::std::option::Option<::std::string::String>,
}
impl AddedPrincipalBuilder {
    /// <p>The type of principal.</p>
    pub fn principal_type(mut self, input: crate::types::PrincipalType) -> Self {
        self.principal_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of principal.</p>
    pub fn set_principal_type(mut self, input: ::std::option::Option<crate::types::PrincipalType>) -> Self {
        self.principal_type = input;
        self
    }
    /// <p>The type of principal.</p>
    pub fn get_principal_type(&self) -> &::std::option::Option<crate::types::PrincipalType> {
        &self.principal_type
    }
    /// <p>The Amazon Resource Name (ARN) of the principal.</p>
    pub fn principal(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.principal = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the principal.</p>
    pub fn set_principal(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.principal = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the principal.</p>
    pub fn get_principal(&self) -> &::std::option::Option<::std::string::String> {
        &self.principal
    }
    /// <p>The ID of the service permission.</p>
    pub fn service_permission_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_permission_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the service permission.</p>
    pub fn set_service_permission_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_permission_id = input;
        self
    }
    /// <p>The ID of the service permission.</p>
    pub fn get_service_permission_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.service_permission_id
    }
    /// <p>The ID of the service.</p>
    pub fn service_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.service_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the service.</p>
    pub fn set_service_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.service_id = input;
        self
    }
    /// <p>The ID of the service.</p>
    pub fn get_service_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.service_id
    }
    /// Consumes the builder and constructs a [`AddedPrincipal`](crate::types::AddedPrincipal).
    pub fn build(self) -> crate::types::AddedPrincipal {
        crate::types::AddedPrincipal {
            principal_type: self.principal_type,
            principal: self.principal,
            service_permission_id: self.service_permission_id,
            service_id: self.service_id,
        }
    }
}
