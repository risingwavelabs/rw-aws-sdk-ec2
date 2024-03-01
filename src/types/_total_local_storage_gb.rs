// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The minimum and maximum amount of total local storage, in GB.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TotalLocalStorageGb {
    /// <p>The minimum amount of total local storage, in GB. If this parameter is not specified, there is no minimum limit.</p>
    pub min: ::std::option::Option<f64>,
    /// <p>The maximum amount of total local storage, in GB. If this parameter is not specified, there is no maximum limit.</p>
    pub max: ::std::option::Option<f64>,
}
impl TotalLocalStorageGb {
    /// <p>The minimum amount of total local storage, in GB. If this parameter is not specified, there is no minimum limit.</p>
    pub fn min(&self) -> ::std::option::Option<f64> {
        self.min
    }
    /// <p>The maximum amount of total local storage, in GB. If this parameter is not specified, there is no maximum limit.</p>
    pub fn max(&self) -> ::std::option::Option<f64> {
        self.max
    }
}
impl TotalLocalStorageGb {
    /// Creates a new builder-style object to manufacture [`TotalLocalStorageGb`](crate::types::TotalLocalStorageGb).
    pub fn builder() -> crate::types::builders::TotalLocalStorageGbBuilder {
        crate::types::builders::TotalLocalStorageGbBuilder::default()
    }
}

/// A builder for [`TotalLocalStorageGb`](crate::types::TotalLocalStorageGb).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TotalLocalStorageGbBuilder {
    pub(crate) min: ::std::option::Option<f64>,
    pub(crate) max: ::std::option::Option<f64>,
}
impl TotalLocalStorageGbBuilder {
    /// <p>The minimum amount of total local storage, in GB. If this parameter is not specified, there is no minimum limit.</p>
    pub fn min(mut self, input: f64) -> Self {
        self.min = ::std::option::Option::Some(input);
        self
    }
    /// <p>The minimum amount of total local storage, in GB. If this parameter is not specified, there is no minimum limit.</p>
    pub fn set_min(mut self, input: ::std::option::Option<f64>) -> Self {
        self.min = input;
        self
    }
    /// <p>The minimum amount of total local storage, in GB. If this parameter is not specified, there is no minimum limit.</p>
    pub fn get_min(&self) -> &::std::option::Option<f64> {
        &self.min
    }
    /// <p>The maximum amount of total local storage, in GB. If this parameter is not specified, there is no maximum limit.</p>
    pub fn max(mut self, input: f64) -> Self {
        self.max = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum amount of total local storage, in GB. If this parameter is not specified, there is no maximum limit.</p>
    pub fn set_max(mut self, input: ::std::option::Option<f64>) -> Self {
        self.max = input;
        self
    }
    /// <p>The maximum amount of total local storage, in GB. If this parameter is not specified, there is no maximum limit.</p>
    pub fn get_max(&self) -> &::std::option::Option<f64> {
        &self.max
    }
    /// Consumes the builder and constructs a [`TotalLocalStorageGb`](crate::types::TotalLocalStorageGb).
    pub fn build(self) -> crate::types::TotalLocalStorageGb {
        crate::types::TotalLocalStorageGb {
            min: self.min,
            max: self.max,
        }
    }
}
