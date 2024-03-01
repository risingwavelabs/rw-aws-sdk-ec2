// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DetachClassicLinkVpcOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub r#return: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl DetachClassicLinkVpcOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn r#return(&self) -> ::std::option::Option<bool> {
        self.r#return
    }
}
impl ::aws_types::request_id::RequestId for DetachClassicLinkVpcOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DetachClassicLinkVpcOutput {
    /// Creates a new builder-style object to manufacture [`DetachClassicLinkVpcOutput`](crate::operation::detach_classic_link_vpc::DetachClassicLinkVpcOutput).
    pub fn builder() -> crate::operation::detach_classic_link_vpc::builders::DetachClassicLinkVpcOutputBuilder {
        crate::operation::detach_classic_link_vpc::builders::DetachClassicLinkVpcOutputBuilder::default()
    }
}

/// A builder for [`DetachClassicLinkVpcOutput`](crate::operation::detach_classic_link_vpc::DetachClassicLinkVpcOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DetachClassicLinkVpcOutputBuilder {
    pub(crate) r#return: ::std::option::Option<bool>,
    _request_id: Option<String>,
}
impl DetachClassicLinkVpcOutputBuilder {
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
    /// Consumes the builder and constructs a [`DetachClassicLinkVpcOutput`](crate::operation::detach_classic_link_vpc::DetachClassicLinkVpcOutput).
    pub fn build(self) -> crate::operation::detach_classic_link_vpc::DetachClassicLinkVpcOutput {
        crate::operation::detach_classic_link_vpc::DetachClassicLinkVpcOutput {
            r#return: self.r#return,
            _request_id: self._request_id,
        }
    }
}
