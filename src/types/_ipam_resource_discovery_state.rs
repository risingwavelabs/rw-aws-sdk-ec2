// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `IpamResourceDiscoveryState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let ipamresourcediscoverystate = unimplemented!();
/// match ipamresourcediscoverystate {
///     IpamResourceDiscoveryState::CreateComplete => { /* ... */ },
///     IpamResourceDiscoveryState::CreateFailed => { /* ... */ },
///     IpamResourceDiscoveryState::CreateInProgress => { /* ... */ },
///     IpamResourceDiscoveryState::DeleteComplete => { /* ... */ },
///     IpamResourceDiscoveryState::DeleteFailed => { /* ... */ },
///     IpamResourceDiscoveryState::DeleteInProgress => { /* ... */ },
///     IpamResourceDiscoveryState::IsolateComplete => { /* ... */ },
///     IpamResourceDiscoveryState::IsolateInProgress => { /* ... */ },
///     IpamResourceDiscoveryState::ModifyComplete => { /* ... */ },
///     IpamResourceDiscoveryState::ModifyFailed => { /* ... */ },
///     IpamResourceDiscoveryState::ModifyInProgress => { /* ... */ },
///     IpamResourceDiscoveryState::RestoreInProgress => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `ipamresourcediscoverystate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `IpamResourceDiscoveryState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `IpamResourceDiscoveryState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `IpamResourceDiscoveryState::NewFeature` is defined.
/// Specifically, when `ipamresourcediscoverystate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `IpamResourceDiscoveryState::NewFeature` also yielding `"NewFeature"`.
///
/// Explicitly matching on the `Unknown` variant should
/// be avoided for two reasons:
/// - The inner data `UnknownVariantValue` is opaque, and no further information can be extracted.
/// - It might inadvertently shadow other intended match arms.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::Eq, ::std::cmp::Ord, ::std::cmp::PartialEq, ::std::cmp::PartialOrd, ::std::fmt::Debug, ::std::hash::Hash,
)]
pub enum IpamResourceDiscoveryState {
    #[allow(missing_docs)] // documentation missing in model
    CreateComplete,
    #[allow(missing_docs)] // documentation missing in model
    CreateFailed,
    #[allow(missing_docs)] // documentation missing in model
    CreateInProgress,
    #[allow(missing_docs)] // documentation missing in model
    DeleteComplete,
    #[allow(missing_docs)] // documentation missing in model
    DeleteFailed,
    #[allow(missing_docs)] // documentation missing in model
    DeleteInProgress,
    #[allow(missing_docs)] // documentation missing in model
    IsolateComplete,
    #[allow(missing_docs)] // documentation missing in model
    IsolateInProgress,
    #[allow(missing_docs)] // documentation missing in model
    ModifyComplete,
    #[allow(missing_docs)] // documentation missing in model
    ModifyFailed,
    #[allow(missing_docs)] // documentation missing in model
    ModifyInProgress,
    #[allow(missing_docs)] // documentation missing in model
    RestoreInProgress,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for IpamResourceDiscoveryState {
    fn from(s: &str) -> Self {
        match s {
            "create-complete" => IpamResourceDiscoveryState::CreateComplete,
            "create-failed" => IpamResourceDiscoveryState::CreateFailed,
            "create-in-progress" => IpamResourceDiscoveryState::CreateInProgress,
            "delete-complete" => IpamResourceDiscoveryState::DeleteComplete,
            "delete-failed" => IpamResourceDiscoveryState::DeleteFailed,
            "delete-in-progress" => IpamResourceDiscoveryState::DeleteInProgress,
            "isolate-complete" => IpamResourceDiscoveryState::IsolateComplete,
            "isolate-in-progress" => IpamResourceDiscoveryState::IsolateInProgress,
            "modify-complete" => IpamResourceDiscoveryState::ModifyComplete,
            "modify-failed" => IpamResourceDiscoveryState::ModifyFailed,
            "modify-in-progress" => IpamResourceDiscoveryState::ModifyInProgress,
            "restore-in-progress" => IpamResourceDiscoveryState::RestoreInProgress,
            other => IpamResourceDiscoveryState::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for IpamResourceDiscoveryState {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(IpamResourceDiscoveryState::from(s))
    }
}
impl IpamResourceDiscoveryState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            IpamResourceDiscoveryState::CreateComplete => "create-complete",
            IpamResourceDiscoveryState::CreateFailed => "create-failed",
            IpamResourceDiscoveryState::CreateInProgress => "create-in-progress",
            IpamResourceDiscoveryState::DeleteComplete => "delete-complete",
            IpamResourceDiscoveryState::DeleteFailed => "delete-failed",
            IpamResourceDiscoveryState::DeleteInProgress => "delete-in-progress",
            IpamResourceDiscoveryState::IsolateComplete => "isolate-complete",
            IpamResourceDiscoveryState::IsolateInProgress => "isolate-in-progress",
            IpamResourceDiscoveryState::ModifyComplete => "modify-complete",
            IpamResourceDiscoveryState::ModifyFailed => "modify-failed",
            IpamResourceDiscoveryState::ModifyInProgress => "modify-in-progress",
            IpamResourceDiscoveryState::RestoreInProgress => "restore-in-progress",
            IpamResourceDiscoveryState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "create-complete",
            "create-failed",
            "create-in-progress",
            "delete-complete",
            "delete-failed",
            "delete-in-progress",
            "isolate-complete",
            "isolate-in-progress",
            "modify-complete",
            "modify-failed",
            "modify-in-progress",
            "restore-in-progress",
        ]
    }
}
impl ::std::convert::AsRef<str> for IpamResourceDiscoveryState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl IpamResourceDiscoveryState {
    /// Parses the enum value while disallowing unknown variants.
    ///
    /// Unknown variants will result in an error.
    pub fn try_parse(value: &str) -> ::std::result::Result<Self, crate::error::UnknownVariantError> {
        match Self::from(value) {
            #[allow(deprecated)]
            Self::Unknown(_) => ::std::result::Result::Err(crate::error::UnknownVariantError::new(value)),
            known => Ok(known),
        }
    }
}
