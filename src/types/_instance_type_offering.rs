// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The instance types offered.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InstanceTypeOffering {
    /// <p>The instance type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub instance_type: ::std::option::Option<crate::types::InstanceType>,
    /// <p>The location type.</p>
    pub location_type: ::std::option::Option<crate::types::LocationType>,
    /// <p>The identifier for the location. This depends on the location type. For example, if the location type is <code>region</code>, the location is the Region code (for example, <code>us-east-2</code>.)</p>
    pub location: ::std::option::Option<::std::string::String>,
}
impl InstanceTypeOffering {
    /// <p>The instance type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn instance_type(&self) -> ::std::option::Option<&crate::types::InstanceType> {
        self.instance_type.as_ref()
    }
    /// <p>The location type.</p>
    pub fn location_type(&self) -> ::std::option::Option<&crate::types::LocationType> {
        self.location_type.as_ref()
    }
    /// <p>The identifier for the location. This depends on the location type. For example, if the location type is <code>region</code>, the location is the Region code (for example, <code>us-east-2</code>.)</p>
    pub fn location(&self) -> ::std::option::Option<&str> {
        self.location.as_deref()
    }
}
impl InstanceTypeOffering {
    /// Creates a new builder-style object to manufacture [`InstanceTypeOffering`](crate::types::InstanceTypeOffering).
    pub fn builder() -> crate::types::builders::InstanceTypeOfferingBuilder {
        crate::types::builders::InstanceTypeOfferingBuilder::default()
    }
}

/// A builder for [`InstanceTypeOffering`](crate::types::InstanceTypeOffering).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InstanceTypeOfferingBuilder {
    pub(crate) instance_type: ::std::option::Option<crate::types::InstanceType>,
    pub(crate) location_type: ::std::option::Option<crate::types::LocationType>,
    pub(crate) location: ::std::option::Option<::std::string::String>,
}
impl InstanceTypeOfferingBuilder {
    /// <p>The instance type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn instance_type(mut self, input: crate::types::InstanceType) -> Self {
        self.instance_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The instance type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn set_instance_type(mut self, input: ::std::option::Option<crate::types::InstanceType>) -> Self {
        self.instance_type = input;
        self
    }
    /// <p>The instance type. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/instance-types.html">Instance types</a> in the <i>Amazon EC2 User Guide</i>.</p>
    pub fn get_instance_type(&self) -> &::std::option::Option<crate::types::InstanceType> {
        &self.instance_type
    }
    /// <p>The location type.</p>
    pub fn location_type(mut self, input: crate::types::LocationType) -> Self {
        self.location_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The location type.</p>
    pub fn set_location_type(mut self, input: ::std::option::Option<crate::types::LocationType>) -> Self {
        self.location_type = input;
        self
    }
    /// <p>The location type.</p>
    pub fn get_location_type(&self) -> &::std::option::Option<crate::types::LocationType> {
        &self.location_type
    }
    /// <p>The identifier for the location. This depends on the location type. For example, if the location type is <code>region</code>, the location is the Region code (for example, <code>us-east-2</code>.)</p>
    pub fn location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.location = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the location. This depends on the location type. For example, if the location type is <code>region</code>, the location is the Region code (for example, <code>us-east-2</code>.)</p>
    pub fn set_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.location = input;
        self
    }
    /// <p>The identifier for the location. This depends on the location type. For example, if the location type is <code>region</code>, the location is the Region code (for example, <code>us-east-2</code>.)</p>
    pub fn get_location(&self) -> &::std::option::Option<::std::string::String> {
        &self.location
    }
    /// Consumes the builder and constructs a [`InstanceTypeOffering`](crate::types::InstanceTypeOffering).
    pub fn build(self) -> crate::types::InstanceTypeOffering {
        crate::types::InstanceTypeOffering {
            instance_type: self.instance_type,
            location_type: self.location_type,
            location: self.location,
        }
    }
}
