// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ModifyInstanceCreditSpecificationOutput {
    /// <p>Information about the instances whose credit option for CPU usage was successfully modified.</p>
    pub successful_instance_credit_specifications: ::std::option::Option<::std::vec::Vec<crate::types::SuccessfulInstanceCreditSpecificationItem>>,
    /// <p>Information about the instances whose credit option for CPU usage was not modified.</p>
    pub unsuccessful_instance_credit_specifications:
        ::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulInstanceCreditSpecificationItem>>,
    _request_id: Option<String>,
}
impl ModifyInstanceCreditSpecificationOutput {
    /// <p>Information about the instances whose credit option for CPU usage was successfully modified.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.successful_instance_credit_specifications.is_none()`.
    pub fn successful_instance_credit_specifications(&self) -> &[crate::types::SuccessfulInstanceCreditSpecificationItem] {
        self.successful_instance_credit_specifications.as_deref().unwrap_or_default()
    }
    /// <p>Information about the instances whose credit option for CPU usage was not modified.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.unsuccessful_instance_credit_specifications.is_none()`.
    pub fn unsuccessful_instance_credit_specifications(&self) -> &[crate::types::UnsuccessfulInstanceCreditSpecificationItem] {
        self.unsuccessful_instance_credit_specifications.as_deref().unwrap_or_default()
    }
}
impl ::aws_types::request_id::RequestId for ModifyInstanceCreditSpecificationOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ModifyInstanceCreditSpecificationOutput {
    /// Creates a new builder-style object to manufacture [`ModifyInstanceCreditSpecificationOutput`](crate::operation::modify_instance_credit_specification::ModifyInstanceCreditSpecificationOutput).
    pub fn builder() -> crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationOutputBuilder {
        crate::operation::modify_instance_credit_specification::builders::ModifyInstanceCreditSpecificationOutputBuilder::default()
    }
}

/// A builder for [`ModifyInstanceCreditSpecificationOutput`](crate::operation::modify_instance_credit_specification::ModifyInstanceCreditSpecificationOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ModifyInstanceCreditSpecificationOutputBuilder {
    pub(crate) successful_instance_credit_specifications:
        ::std::option::Option<::std::vec::Vec<crate::types::SuccessfulInstanceCreditSpecificationItem>>,
    pub(crate) unsuccessful_instance_credit_specifications:
        ::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulInstanceCreditSpecificationItem>>,
    _request_id: Option<String>,
}
impl ModifyInstanceCreditSpecificationOutputBuilder {
    /// Appends an item to `successful_instance_credit_specifications`.
    ///
    /// To override the contents of this collection use [`set_successful_instance_credit_specifications`](Self::set_successful_instance_credit_specifications).
    ///
    /// <p>Information about the instances whose credit option for CPU usage was successfully modified.</p>
    pub fn successful_instance_credit_specifications(mut self, input: crate::types::SuccessfulInstanceCreditSpecificationItem) -> Self {
        let mut v = self.successful_instance_credit_specifications.unwrap_or_default();
        v.push(input);
        self.successful_instance_credit_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the instances whose credit option for CPU usage was successfully modified.</p>
    pub fn set_successful_instance_credit_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::SuccessfulInstanceCreditSpecificationItem>>,
    ) -> Self {
        self.successful_instance_credit_specifications = input;
        self
    }
    /// <p>Information about the instances whose credit option for CPU usage was successfully modified.</p>
    pub fn get_successful_instance_credit_specifications(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::SuccessfulInstanceCreditSpecificationItem>> {
        &self.successful_instance_credit_specifications
    }
    /// Appends an item to `unsuccessful_instance_credit_specifications`.
    ///
    /// To override the contents of this collection use [`set_unsuccessful_instance_credit_specifications`](Self::set_unsuccessful_instance_credit_specifications).
    ///
    /// <p>Information about the instances whose credit option for CPU usage was not modified.</p>
    pub fn unsuccessful_instance_credit_specifications(mut self, input: crate::types::UnsuccessfulInstanceCreditSpecificationItem) -> Self {
        let mut v = self.unsuccessful_instance_credit_specifications.unwrap_or_default();
        v.push(input);
        self.unsuccessful_instance_credit_specifications = ::std::option::Option::Some(v);
        self
    }
    /// <p>Information about the instances whose credit option for CPU usage was not modified.</p>
    pub fn set_unsuccessful_instance_credit_specifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulInstanceCreditSpecificationItem>>,
    ) -> Self {
        self.unsuccessful_instance_credit_specifications = input;
        self
    }
    /// <p>Information about the instances whose credit option for CPU usage was not modified.</p>
    pub fn get_unsuccessful_instance_credit_specifications(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::UnsuccessfulInstanceCreditSpecificationItem>> {
        &self.unsuccessful_instance_credit_specifications
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ModifyInstanceCreditSpecificationOutput`](crate::operation::modify_instance_credit_specification::ModifyInstanceCreditSpecificationOutput).
    pub fn build(self) -> crate::operation::modify_instance_credit_specification::ModifyInstanceCreditSpecificationOutput {
        crate::operation::modify_instance_credit_specification::ModifyInstanceCreditSpecificationOutput {
            successful_instance_credit_specifications: self.successful_instance_credit_specifications,
            unsuccessful_instance_credit_specifications: self.unsuccessful_instance_credit_specifications,
            _request_id: self._request_id,
        }
    }
}