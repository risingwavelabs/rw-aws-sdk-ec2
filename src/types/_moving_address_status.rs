// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <note>
/// <p>This action is deprecated.</p>
/// </note>
/// <p>Describes the status of a moving Elastic IP address.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct MovingAddressStatus {
    /// <p>The status of the Elastic IP address that's being moved or restored.</p>
    pub move_status: ::std::option::Option<crate::types::MoveStatus>,
    /// <p>The Elastic IP address.</p>
    pub public_ip: ::std::option::Option<::std::string::String>,
}
impl MovingAddressStatus {
    /// <p>The status of the Elastic IP address that's being moved or restored.</p>
    pub fn move_status(&self) -> ::std::option::Option<&crate::types::MoveStatus> {
        self.move_status.as_ref()
    }
    /// <p>The Elastic IP address.</p>
    pub fn public_ip(&self) -> ::std::option::Option<&str> {
        self.public_ip.as_deref()
    }
}
impl MovingAddressStatus {
    /// Creates a new builder-style object to manufacture [`MovingAddressStatus`](crate::types::MovingAddressStatus).
    pub fn builder() -> crate::types::builders::MovingAddressStatusBuilder {
        crate::types::builders::MovingAddressStatusBuilder::default()
    }
}

/// A builder for [`MovingAddressStatus`](crate::types::MovingAddressStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct MovingAddressStatusBuilder {
    pub(crate) move_status: ::std::option::Option<crate::types::MoveStatus>,
    pub(crate) public_ip: ::std::option::Option<::std::string::String>,
}
impl MovingAddressStatusBuilder {
    /// <p>The status of the Elastic IP address that's being moved or restored.</p>
    pub fn move_status(mut self, input: crate::types::MoveStatus) -> Self {
        self.move_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the Elastic IP address that's being moved or restored.</p>
    pub fn set_move_status(mut self, input: ::std::option::Option<crate::types::MoveStatus>) -> Self {
        self.move_status = input;
        self
    }
    /// <p>The status of the Elastic IP address that's being moved or restored.</p>
    pub fn get_move_status(&self) -> &::std::option::Option<crate::types::MoveStatus> {
        &self.move_status
    }
    /// <p>The Elastic IP address.</p>
    pub fn public_ip(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.public_ip = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Elastic IP address.</p>
    pub fn set_public_ip(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.public_ip = input;
        self
    }
    /// <p>The Elastic IP address.</p>
    pub fn get_public_ip(&self) -> &::std::option::Option<::std::string::String> {
        &self.public_ip
    }
    /// Consumes the builder and constructs a [`MovingAddressStatus`](crate::types::MovingAddressStatus).
    pub fn build(self) -> crate::types::MovingAddressStatus {
        crate::types::MovingAddressStatus {
            move_status: self.move_status,
            public_ip: self.public_ip,
        }
    }
}
