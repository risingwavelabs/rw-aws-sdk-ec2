// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes an Active Directory.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DirectoryServiceAuthentication {
    /// <p>The ID of the Active Directory used for authentication.</p>
    pub directory_id: ::std::option::Option<::std::string::String>,
}
impl DirectoryServiceAuthentication {
    /// <p>The ID of the Active Directory used for authentication.</p>
    pub fn directory_id(&self) -> ::std::option::Option<&str> {
        self.directory_id.as_deref()
    }
}
impl DirectoryServiceAuthentication {
    /// Creates a new builder-style object to manufacture [`DirectoryServiceAuthentication`](crate::types::DirectoryServiceAuthentication).
    pub fn builder() -> crate::types::builders::DirectoryServiceAuthenticationBuilder {
        crate::types::builders::DirectoryServiceAuthenticationBuilder::default()
    }
}

/// A builder for [`DirectoryServiceAuthentication`](crate::types::DirectoryServiceAuthentication).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DirectoryServiceAuthenticationBuilder {
    pub(crate) directory_id: ::std::option::Option<::std::string::String>,
}
impl DirectoryServiceAuthenticationBuilder {
    /// <p>The ID of the Active Directory used for authentication.</p>
    pub fn directory_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.directory_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Active Directory used for authentication.</p>
    pub fn set_directory_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.directory_id = input;
        self
    }
    /// <p>The ID of the Active Directory used for authentication.</p>
    pub fn get_directory_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.directory_id
    }
    /// Consumes the builder and constructs a [`DirectoryServiceAuthentication`](crate::types::DirectoryServiceAuthentication).
    pub fn build(self) -> crate::types::DirectoryServiceAuthentication {
        crate::types::DirectoryServiceAuthentication {
            directory_id: self.directory_id,
        }
    }
}
