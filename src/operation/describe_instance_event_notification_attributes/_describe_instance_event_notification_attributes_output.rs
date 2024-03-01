// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeInstanceEventNotificationAttributesOutput {
    /// <p>Information about the registered tag keys.</p>
    pub instance_tag_attribute: ::std::option::Option<crate::types::InstanceTagNotificationAttribute>,
    _request_id: Option<String>,
}
impl DescribeInstanceEventNotificationAttributesOutput {
    /// <p>Information about the registered tag keys.</p>
    pub fn instance_tag_attribute(&self) -> ::std::option::Option<&crate::types::InstanceTagNotificationAttribute> {
        self.instance_tag_attribute.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for DescribeInstanceEventNotificationAttributesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DescribeInstanceEventNotificationAttributesOutput {
    /// Creates a new builder-style object to manufacture [`DescribeInstanceEventNotificationAttributesOutput`](crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesOutput).
    pub fn builder(
    ) -> crate::operation::describe_instance_event_notification_attributes::builders::DescribeInstanceEventNotificationAttributesOutputBuilder {
        crate::operation::describe_instance_event_notification_attributes::builders::DescribeInstanceEventNotificationAttributesOutputBuilder::default(
        )
    }
}

/// A builder for [`DescribeInstanceEventNotificationAttributesOutput`](crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeInstanceEventNotificationAttributesOutputBuilder {
    pub(crate) instance_tag_attribute: ::std::option::Option<crate::types::InstanceTagNotificationAttribute>,
    _request_id: Option<String>,
}
impl DescribeInstanceEventNotificationAttributesOutputBuilder {
    /// <p>Information about the registered tag keys.</p>
    pub fn instance_tag_attribute(mut self, input: crate::types::InstanceTagNotificationAttribute) -> Self {
        self.instance_tag_attribute = ::std::option::Option::Some(input);
        self
    }
    /// <p>Information about the registered tag keys.</p>
    pub fn set_instance_tag_attribute(mut self, input: ::std::option::Option<crate::types::InstanceTagNotificationAttribute>) -> Self {
        self.instance_tag_attribute = input;
        self
    }
    /// <p>Information about the registered tag keys.</p>
    pub fn get_instance_tag_attribute(&self) -> &::std::option::Option<crate::types::InstanceTagNotificationAttribute> {
        &self.instance_tag_attribute
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DescribeInstanceEventNotificationAttributesOutput`](crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesOutput).
    pub fn build(self) -> crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesOutput {
        crate::operation::describe_instance_event_notification_attributes::DescribeInstanceEventNotificationAttributesOutput {
            instance_tag_attribute: self.instance_tag_attribute,
            _request_id: self._request_id,
        }
    }
}
