// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the memory available to the inference accelerator.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct InferenceDeviceMemoryInfo {
    /// <p>The size of the memory available to the inference accelerator, in MiB.</p>
    pub size_in_mib: ::std::option::Option<i32>,
}
impl InferenceDeviceMemoryInfo {
    /// <p>The size of the memory available to the inference accelerator, in MiB.</p>
    pub fn size_in_mib(&self) -> ::std::option::Option<i32> {
        self.size_in_mib
    }
}
impl InferenceDeviceMemoryInfo {
    /// Creates a new builder-style object to manufacture [`InferenceDeviceMemoryInfo`](crate::types::InferenceDeviceMemoryInfo).
    pub fn builder() -> crate::types::builders::InferenceDeviceMemoryInfoBuilder {
        crate::types::builders::InferenceDeviceMemoryInfoBuilder::default()
    }
}

/// A builder for [`InferenceDeviceMemoryInfo`](crate::types::InferenceDeviceMemoryInfo).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct InferenceDeviceMemoryInfoBuilder {
    pub(crate) size_in_mib: ::std::option::Option<i32>,
}
impl InferenceDeviceMemoryInfoBuilder {
    /// <p>The size of the memory available to the inference accelerator, in MiB.</p>
    pub fn size_in_mib(mut self, input: i32) -> Self {
        self.size_in_mib = ::std::option::Option::Some(input);
        self
    }
    /// <p>The size of the memory available to the inference accelerator, in MiB.</p>
    pub fn set_size_in_mib(mut self, input: ::std::option::Option<i32>) -> Self {
        self.size_in_mib = input;
        self
    }
    /// <p>The size of the memory available to the inference accelerator, in MiB.</p>
    pub fn get_size_in_mib(&self) -> &::std::option::Option<i32> {
        &self.size_in_mib
    }
    /// Consumes the builder and constructs a [`InferenceDeviceMemoryInfo`](crate::types::InferenceDeviceMemoryInfo).
    pub fn build(self) -> crate::types::InferenceDeviceMemoryInfo {
        crate::types::InferenceDeviceMemoryInfo {
            size_in_mib: self.size_in_mib,
        }
    }
}
