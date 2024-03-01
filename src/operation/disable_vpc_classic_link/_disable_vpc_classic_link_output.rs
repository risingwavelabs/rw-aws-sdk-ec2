// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisableVpcClassicLinkOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub r#return: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl DisableVpcClassicLinkOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn r#return(&self) -> ::std::option::Option<bool> {
        self.r#return
    }
}
impl ::aws_types::request_id::RequestId for DisableVpcClassicLinkOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisableVpcClassicLinkOutput {
    /// Creates a new builder-style object to manufacture [`DisableVpcClassicLinkOutput`](crate::operation::disable_vpc_classic_link::DisableVpcClassicLinkOutput).
    pub fn builder() -> crate::operation::disable_vpc_classic_link::builders::DisableVpcClassicLinkOutputBuilder {
        crate::operation::disable_vpc_classic_link::builders::DisableVpcClassicLinkOutputBuilder::default()
    }
}

/// A builder for [`DisableVpcClassicLinkOutput`](crate::operation::disable_vpc_classic_link::DisableVpcClassicLinkOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisableVpcClassicLinkOutputBuilder {
    pub(crate) r#return: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl DisableVpcClassicLinkOutputBuilder {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn r#return(mut self, input: bool) -> Self {
        self.r#return = ::std::option::Option::Some(input);
        self
    }
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn set_return(mut self, input: ::std::option::Option<bool>) -> Self {
        self.r#return = input;
        self
    }
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn get_return(&self) -> &::std::option::Option<bool> {
        &self.r#return
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisableVpcClassicLinkOutput`](crate::operation::disable_vpc_classic_link::DisableVpcClassicLinkOutput).
    pub fn build(self) -> crate::operation::disable_vpc_classic_link::DisableVpcClassicLinkOutput {
        crate::operation::disable_vpc_classic_link::DisableVpcClassicLinkOutput {
            r#return: self.r#return,
            _request_id: self._request_id,
        }
    }
}
