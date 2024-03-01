// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The minimum and maximum amount of total accelerator memory, in MiB.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AcceleratorTotalMemoryMiBRequest {
    /// <p>The minimum amount of accelerator memory, in MiB. To specify no minimum limit, omit this parameter.</p>
    pub min: ::std::option::Option<i32>,
    /// <p>The maximum amount of accelerator memory, in MiB. To specify no maximum limit, omit this parameter.</p>
    pub max: ::std::option::Option<i32>,
}
impl AcceleratorTotalMemoryMiBRequest {
    /// <p>The minimum amount of accelerator memory, in MiB. To specify no minimum limit, omit this parameter.</p>
    pub fn min(&self) -> ::std::option::Option<i32> {
        self.min
    }
    /// <p>The maximum amount of accelerator memory, in MiB. To specify no maximum limit, omit this parameter.</p>
    pub fn max(&self) -> ::std::option::Option<i32> {
        self.max
    }
}
impl AcceleratorTotalMemoryMiBRequest {
    /// Creates a new builder-style object to manufacture [`AcceleratorTotalMemoryMiBRequest`](crate::types::AcceleratorTotalMemoryMiBRequest).
    pub fn builder() -> crate::types::builders::AcceleratorTotalMemoryMiBRequestBuilder {
        crate::types::builders::AcceleratorTotalMemoryMiBRequestBuilder::default()
    }
}

/// A builder for [`AcceleratorTotalMemoryMiBRequest`](crate::types::AcceleratorTotalMemoryMiBRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct AcceleratorTotalMemoryMiBRequestBuilder {
    pub(crate) min: ::std::option::Option<i32>,
    pub(crate) max: ::std::option::Option<i32>,
}
impl AcceleratorTotalMemoryMiBRequestBuilder {
    /// <p>The minimum amount of accelerator memory, in MiB. To specify no minimum limit, omit this parameter.</p>
    pub fn min(mut self, input: i32) -> Self {
        self.min = ::std::option::Option::Some(input);
        self
    }
    /// <p>The minimum amount of accelerator memory, in MiB. To specify no minimum limit, omit this parameter.</p>
    pub fn set_min(mut self, input: ::std::option::Option<i32>) -> Self {
        self.min = input;
        self
    }
    /// <p>The minimum amount of accelerator memory, in MiB. To specify no minimum limit, omit this parameter.</p>
    pub fn get_min(&self) -> &::std::option::Option<i32> {
        &self.min
    }
    /// <p>The maximum amount of accelerator memory, in MiB. To specify no maximum limit, omit this parameter.</p>
    pub fn max(mut self, input: i32) -> Self {
        self.max = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum amount of accelerator memory, in MiB. To specify no maximum limit, omit this parameter.</p>
    pub fn set_max(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max = input;
        self
    }
    /// <p>The maximum amount of accelerator memory, in MiB. To specify no maximum limit, omit this parameter.</p>
    pub fn get_max(&self) -> &::std::option::Option<i32> {
        &self.max
    }
    /// Consumes the builder and constructs a [`AcceleratorTotalMemoryMiBRequest`](crate::types::AcceleratorTotalMemoryMiBRequest).
    pub fn build(self) -> crate::types::AcceleratorTotalMemoryMiBRequest {
        crate::types::AcceleratorTotalMemoryMiBRequest {
            min: self.min,
            max: self.max,
        }
    }
}
