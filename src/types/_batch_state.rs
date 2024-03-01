// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// When writing a match expression against `BatchState`, it is important to ensure
/// your code is forward-compatible. That is, if a match arm handles a case for a
/// feature that is supported by the service but has not been represented as an enum
/// variant in a current version of SDK, your code should continue to work when you
/// upgrade SDK to a future version in which the enum does include a variant for that
/// feature.
///
/// Here is an example of how you can make a match expression forward-compatible:
///
/// ```text
/// # let batchstate = unimplemented!();
/// match batchstate {
///     BatchState::Active => { /* ... */ },
///     BatchState::Cancelled => { /* ... */ },
///     BatchState::CancelledRunning => { /* ... */ },
///     BatchState::CancelledTerminatingInstances => { /* ... */ },
///     BatchState::Failed => { /* ... */ },
///     BatchState::Modifying => { /* ... */ },
///     BatchState::Submitted => { /* ... */ },
///     other @ _ if other.as_str() == "NewFeature" => { /* handles a case for `NewFeature` */ },
///     _ => { /* ... */ },
/// }
/// ```
/// The above code demonstrates that when `batchstate` represents
/// `NewFeature`, the execution path will lead to the second last match arm,
/// even though the enum does not contain a variant `BatchState::NewFeature`
/// in the current version of SDK. The reason is that the variable `other`,
/// created by the `@` operator, is bound to
/// `BatchState::Unknown(UnknownVariantValue("NewFeature".to_owned()))`
/// and calling `as_str` on it yields `"NewFeature"`.
/// This match expression is forward-compatible when executed with a newer
/// version of SDK where the variant `BatchState::NewFeature` is defined.
/// Specifically, when `batchstate` represents `NewFeature`,
/// the execution path will hit the second last match arm as before by virtue of
/// calling `as_str` on `BatchState::NewFeature` also yielding `"NewFeature"`.
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
pub enum BatchState {
    #[allow(missing_docs)] // documentation missing in model
    Active,
    #[allow(missing_docs)] // documentation missing in model
    Cancelled,
    #[allow(missing_docs)] // documentation missing in model
    CancelledRunning,
    #[allow(missing_docs)] // documentation missing in model
    CancelledTerminatingInstances,
    #[allow(missing_docs)] // documentation missing in model
    Failed,
    #[allow(missing_docs)] // documentation missing in model
    Modifying,
    #[allow(missing_docs)] // documentation missing in model
    Submitted,
    /// `Unknown` contains new variants that have been added since this code was generated.
    #[deprecated(note = "Don't directly match on `Unknown`. See the docs on this enum for the correct way to handle unknown variants.")]
    Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue),
}
impl ::std::convert::From<&str> for BatchState {
    fn from(s: &str) -> Self {
        match s {
            "active" => BatchState::Active,
            "cancelled" => BatchState::Cancelled,
            "cancelled_running" => BatchState::CancelledRunning,
            "cancelled_terminating" => BatchState::CancelledTerminatingInstances,
            "failed" => BatchState::Failed,
            "modifying" => BatchState::Modifying,
            "submitted" => BatchState::Submitted,
            other => BatchState::Unknown(crate::primitives::sealed_enum_unknown::UnknownVariantValue(other.to_owned())),
        }
    }
}
impl ::std::str::FromStr for BatchState {
    type Err = ::std::convert::Infallible;

    fn from_str(s: &str) -> ::std::result::Result<Self, <Self as ::std::str::FromStr>::Err> {
        ::std::result::Result::Ok(BatchState::from(s))
    }
}
impl BatchState {
    /// Returns the `&str` value of the enum member.
    pub fn as_str(&self) -> &str {
        match self {
            BatchState::Active => "active",
            BatchState::Cancelled => "cancelled",
            BatchState::CancelledRunning => "cancelled_running",
            BatchState::CancelledTerminatingInstances => "cancelled_terminating",
            BatchState::Failed => "failed",
            BatchState::Modifying => "modifying",
            BatchState::Submitted => "submitted",
            BatchState::Unknown(value) => value.as_str(),
        }
    }
    /// Returns all the `&str` representations of the enum members.
    pub const fn values() -> &'static [&'static str] {
        &[
            "active",
            "cancelled",
            "cancelled_running",
            "cancelled_terminating",
            "failed",
            "modifying",
            "submitted",
        ]
    }
}
impl ::std::convert::AsRef<str> for BatchState {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}
impl BatchState {
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
