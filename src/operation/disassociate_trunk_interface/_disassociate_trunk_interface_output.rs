// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DisassociateTrunkInterfaceOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub r#return: ::std::option::Option<bool>,
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub client_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DisassociateTrunkInterfaceOutput {
    /// <p>Returns <code>true</code> if the request succeeds; otherwise, it returns an error.</p>
    pub fn r#return(&self) -> ::std::option::Option<bool> {
        self.r#return
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn client_token(&self) -> ::std::option::Option<&str> {
        self.client_token.as_deref()
    }
}
impl ::aws_types::request_id::RequestId for DisassociateTrunkInterfaceOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DisassociateTrunkInterfaceOutput {
    /// Creates a new builder-style object to manufacture [`DisassociateTrunkInterfaceOutput`](crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceOutput).
    pub fn builder() -> crate::operation::disassociate_trunk_interface::builders::DisassociateTrunkInterfaceOutputBuilder {
        crate::operation::disassociate_trunk_interface::builders::DisassociateTrunkInterfaceOutputBuilder::default()
    }
}

/// A builder for [`DisassociateTrunkInterfaceOutput`](crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DisassociateTrunkInterfaceOutputBuilder {
    pub(crate) r#return: ::std::option::Option<bool>,
    pub(crate) client_token: ::std::option::Option<::std::string::String>,
    _request_id: Option<String>,
}
impl DisassociateTrunkInterfaceOutputBuilder {
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
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_token = input;
        self
    }
    /// <p>Unique, case-sensitive identifier that you provide to ensure the idempotency of the request. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Run_Instance_Idempotency.html">How to Ensure Idempotency</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_token
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DisassociateTrunkInterfaceOutput`](crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceOutput).
    pub fn build(self) -> crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceOutput {
        crate::operation::disassociate_trunk_interface::DisassociateTrunkInterfaceOutput {
            r#return: self.r#return,
            client_token: self.client_token,
            _request_id: self._request_id,
        }
    }
}