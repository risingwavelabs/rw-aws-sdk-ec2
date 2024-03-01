// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Information about the instances to which the volume is attached.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct VolumeStatusAttachmentStatus {
    /// <p>The maximum IOPS supported by the attached instance.</p>
    pub io_performance: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the attached instance.</p>
    pub instance_id: ::std::option::Option<::std::string::String>,
}
impl VolumeStatusAttachmentStatus {
    /// <p>The maximum IOPS supported by the attached instance.</p>
    pub fn io_performance(&self) -> ::std::option::Option<&str> {
        self.io_performance.as_deref()
    }
    /// <p>The ID of the attached instance.</p>
    pub fn instance_id(&self) -> ::std::option::Option<&str> {
        self.instance_id.as_deref()
    }
}
impl VolumeStatusAttachmentStatus {
    /// Creates a new builder-style object to manufacture [`VolumeStatusAttachmentStatus`](crate::types::VolumeStatusAttachmentStatus).
    pub fn builder() -> crate::types::builders::VolumeStatusAttachmentStatusBuilder {
        crate::types::builders::VolumeStatusAttachmentStatusBuilder::default()
    }
}

/// A builder for [`VolumeStatusAttachmentStatus`](crate::types::VolumeStatusAttachmentStatus).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct VolumeStatusAttachmentStatusBuilder {
    pub(crate) io_performance: ::std::option::Option<::std::string::String>,
    pub(crate) instance_id: ::std::option::Option<::std::string::String>,
}
impl VolumeStatusAttachmentStatusBuilder {
    /// <p>The maximum IOPS supported by the attached instance.</p>
    pub fn io_performance(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.io_performance = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The maximum IOPS supported by the attached instance.</p>
    pub fn set_io_performance(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.io_performance = input;
        self
    }
    /// <p>The maximum IOPS supported by the attached instance.</p>
    pub fn get_io_performance(&self) -> &::std::option::Option<::std::string::String> {
        &self.io_performance
    }
    /// <p>The ID of the attached instance.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.instance_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the attached instance.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.instance_id = input;
        self
    }
    /// <p>The ID of the attached instance.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.instance_id
    }
    /// Consumes the builder and constructs a [`VolumeStatusAttachmentStatus`](crate::types::VolumeStatusAttachmentStatus).
    pub fn build(self) -> crate::types::VolumeStatusAttachmentStatus {
        crate::types::VolumeStatusAttachmentStatus {
            io_performance: self.io_performance,
            instance_id: self.instance_id,
        }
    }
}
