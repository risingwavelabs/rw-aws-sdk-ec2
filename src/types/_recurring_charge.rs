// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a recurring charge.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RecurringCharge {
    /// <p>The amount of the recurring charge.</p>
    pub amount: ::std::option::Option<f64>,
    /// <p>The frequency of the recurring charge.</p>
    pub frequency: ::std::option::Option<crate::types::RecurringChargeFrequency>,
}
impl RecurringCharge {
    /// <p>The amount of the recurring charge.</p>
    pub fn amount(&self) -> ::std::option::Option<f64> {
        self.amount
    }
    /// <p>The frequency of the recurring charge.</p>
    pub fn frequency(&self) -> ::std::option::Option<&crate::types::RecurringChargeFrequency> {
        self.frequency.as_ref()
    }
}
impl RecurringCharge {
    /// Creates a new builder-style object to manufacture [`RecurringCharge`](crate::types::RecurringCharge).
    pub fn builder() -> crate::types::builders::RecurringChargeBuilder {
        crate::types::builders::RecurringChargeBuilder::default()
    }
}

/// A builder for [`RecurringCharge`](crate::types::RecurringCharge).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct RecurringChargeBuilder {
    pub(crate) amount: ::std::option::Option<f64>,
    pub(crate) frequency: ::std::option::Option<crate::types::RecurringChargeFrequency>,
}
impl RecurringChargeBuilder {
    /// <p>The amount of the recurring charge.</p>
    pub fn amount(mut self, input: f64) -> Self {
        self.amount = ::std::option::Option::Some(input);
        self
    }
    /// <p>The amount of the recurring charge.</p>
    pub fn set_amount(mut self, input: ::std::option::Option<f64>) -> Self {
        self.amount = input;
        self
    }
    /// <p>The amount of the recurring charge.</p>
    pub fn get_amount(&self) -> &::std::option::Option<f64> {
        &self.amount
    }
    /// <p>The frequency of the recurring charge.</p>
    pub fn frequency(mut self, input: crate::types::RecurringChargeFrequency) -> Self {
        self.frequency = ::std::option::Option::Some(input);
        self
    }
    /// <p>The frequency of the recurring charge.</p>
    pub fn set_frequency(mut self, input: ::std::option::Option<crate::types::RecurringChargeFrequency>) -> Self {
        self.frequency = input;
        self
    }
    /// <p>The frequency of the recurring charge.</p>
    pub fn get_frequency(&self) -> &::std::option::Option<crate::types::RecurringChargeFrequency> {
        &self.frequency
    }
    /// Consumes the builder and constructs a [`RecurringCharge`](crate::types::RecurringCharge).
    pub fn build(self) -> crate::types::RecurringCharge {
        crate::types::RecurringCharge {
            amount: self.amount,
            frequency: self.frequency,
        }
    }
}
