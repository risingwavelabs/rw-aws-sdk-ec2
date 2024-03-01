// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the status of an instance.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceStatus {
    /// <p>The Availability Zone of the instance.</p>
    pub availability_zone: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the Outpost.</p>
    pub outpost_arn: ::std::option::Option<::std::string::String>,
    /// <p>Any scheduled events associated with the instance.</p>
    pub events: ::std::option::Option<::std::vec::Vec<crate::types::InstanceStatusEvent>>,
    /// <p>The ID of the instance.</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
    /// <p>The intended state of the instance. <code>DescribeInstanceStatus</code> requires that an instance be in the <code>running</code> state.</p>
    pub instance_state: ::std::option::Option<crate::types::InstanceState>,
    /// <p>Reports impaired functionality that stems from issues internal to the instance, such as impaired reachability.</p>
    pub instance_status: ::std::option::Option<crate::types::InstanceStatusSummary>,
    /// <p>Reports impaired functionality that stems from issues related to the systems that support an instance, such as hardware failures and network connectivity problems.</p>
    pub system_status: ::std::option::Option<crate::types::InstanceStatusSummary>,
}
impl InstanceStatus {
    /// <p>The Availability Zone of the instance.</p>
    pub fn availability_zone(&self) -> ::std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost.</p>
    pub fn outpost_arn(&self) -> ::std::option::Option<&str> {
        self.outpost_arn.as_deref()
    }
    /// <p>Any scheduled events associated with the instance.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.events.is_none()`.
    pub fn events(&self) -> &[crate::types::InstanceStatusEvent] {
        self.events.as_deref().unwrap_or_default()
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
    /// <p>The intended state of the instance. <code>DescribeInstanceStatus</code> requires that an instance be in the <code>running</code> state.</p>
    pub fn instance_state(&self) -> ::std::option::Option<&crate::types::InstanceState> {
        self.instance_state.as_ref()
    }
    /// <p>Reports impaired functionality that stems from issues internal to the instance, such as impaired reachability.</p>
    pub fn instance_status(&self) -> ::std::option::Option<&crate::types::InstanceStatusSummary> {
        self.instance_status.as_ref()
    }
    /// <p>Reports impaired functionality that stems from issues related to the systems that support an instance, such as hardware failures and network connectivity problems.</p>
    pub fn system_status(&self) -> ::std::option::Option<&crate::types::InstanceStatusSummary> {
        self.system_status.as_ref()
    }
}
impl InstanceStatus {
    /// Creates a new builder-style object to manufacture [`InstanceStatus`](crate::types::InstanceStatus).
    pub fn builder() -> crate::types::builders::InstanceStatusBuilder {
        crate::types::builders::InstanceStatusBuilder::default()
    }
}

/// A builder for [`InstanceStatus`](crate::types::InstanceStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InstanceStatusBuilder {
    pub(crate) availability_zone: ::std::option::Option<::std::string::String>,
    pub(crate) outpost_arn: ::std::option::Option<::std::string::String>,
    pub(crate) events: ::std::option::Option<::std::vec::Vec<crate::types::InstanceStatusEvent>>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
    pub(crate) instance_state: ::std::option::Option<crate::types::InstanceState>,
    pub(crate) instance_status: ::std::option::Option<crate::types::InstanceStatusSummary>,
    pub(crate) system_status: ::std::option::Option<crate::types::InstanceStatusSummary>,
}
impl InstanceStatusBuilder {
    /// <p>The Availability Zone of the instance.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.availability_zone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Availability Zone of the instance.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>The Availability Zone of the instance.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        &self.availability_zone
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost.</p>
    pub fn outpost_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.outpost_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost.</p>
    pub fn set_outpost_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.outpost_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Outpost.</p>
    pub fn get_outpost_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.outpost_arn
    }
    /// Appends an item to `events`.
    ///
    /// To override the contents of this collection use [`set_events`](Self::set_events).
    ///
    /// <p>Any scheduled events associated with the instance.</p>
    pub fn events(mut self, input: crate::types::InstanceStatusEvent) -> Self {
        let mut v = self.events.unwrap_or_default();
        v.push(input);
        self.events = ::std::option::Option::Some(v);
        self
    }
    /// <p>Any scheduled events associated with the instance.</p>
    pub fn set_events(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceStatusEvent>>) -> Self {
        self.events = input;
        self
    }
    /// <p>Any scheduled events associated with the instance.</p>
    pub fn get_events(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceStatusEvent>> {
        &self.events
    }
    /// <p>The ID of the instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The ID of the instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_id
    }
    /// <p>The intended state of the instance. <code>DescribeInstanceStatus</code> requires that an instance be in the <code>running</code> state.</p>
    pub fn instance_state(mut self, input: crate::types::InstanceState) -> Self {
        self.instance_state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The intended state of the instance. <code>DescribeInstanceStatus</code> requires that an instance be in the <code>running</code> state.</p>
    pub fn set_instance_state(mut self, input: ::std::option::Option<crate::types::InstanceState>) -> Self {
        self.instance_state = input;
        self
    }
    /// <p>The intended state of the instance. <code>DescribeInstanceStatus</code> requires that an instance be in the <code>running</code> state.</p>
    pub fn get_instance_state(&self) -> &::std::option::Option<crate::types::InstanceState> {
        &self.instance_state
    }
    /// <p>Reports impaired functionality that stems from issues internal to the instance, such as impaired reachability.</p>
    pub fn instance_status(mut self, input: crate::types::InstanceStatusSummary) -> Self {
        self.instance_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Reports impaired functionality that stems from issues internal to the instance, such as impaired reachability.</p>
    pub fn set_instance_status(mut self, input: ::std::option::Option<crate::types::InstanceStatusSummary>) -> Self {
        self.instance_status = input;
        self
    }
    /// <p>Reports impaired functionality that stems from issues internal to the instance, such as impaired reachability.</p>
    pub fn get_instance_status(&self) -> &::std::option::Option<crate::types::InstanceStatusSummary> {
        &self.instance_status
    }
    /// <p>Reports impaired functionality that stems from issues related to the systems that support an instance, such as hardware failures and network connectivity problems.</p>
    pub fn system_status(mut self, input: crate::types::InstanceStatusSummary) -> Self {
        self.system_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>Reports impaired functionality that stems from issues related to the systems that support an instance, such as hardware failures and network connectivity problems.</p>
    pub fn set_system_status(mut self, input: ::std::option::Option<crate::types::InstanceStatusSummary>) -> Self {
        self.system_status = input;
        self
    }
    /// <p>Reports impaired functionality that stems from issues related to the systems that support an instance, such as hardware failures and network connectivity problems.</p>
    pub fn get_system_status(&self) -> &::std::option::Option<crate::types::InstanceStatusSummary> {
        &self.system_status
    }
    /// Consumes the builder and constructs a [`InstanceStatus`](crate::types::InstanceStatus).
    pub fn build(self) -> crate::types::InstanceStatus {
        crate::types::InstanceStatus {
            availability_zone: self.availability_zone,
            outpost_arn: self.outpost_arn,
            events: self.events,
            instance_id: self.instance_id,
            instance_state: self.instance_state,
            instance_status: self.instance_status,
            system_status: self.system_status,
        }
    }
}