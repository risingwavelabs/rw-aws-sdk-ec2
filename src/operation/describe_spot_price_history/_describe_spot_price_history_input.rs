// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains the parameters for DescribeSpotPriceHistory.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DescribeSpotPriceHistoryInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for which prices should be returned.</p> </li>
    /// <li> <p> <code>instance-type</code> - The type of instance (for example, <code>m3.medium</code>).</p> </li>
    /// <li> <p> <code>product-description</code> - The product description for the Spot price (<code>Linux/UNIX</code> | <code>Red Hat Enterprise Linux</code> | <code>SUSE Linux</code> | <code>Windows</code> | <code>Linux/UNIX (Amazon VPC)</code> | <code>Red Hat Enterprise Linux (Amazon VPC)</code> | <code>SUSE Linux (Amazon VPC)</code> | <code>Windows (Amazon VPC)</code>).</p> </li>
    /// <li> <p> <code>spot-price</code> - The Spot price. The value must match exactly (or use wildcards; greater than or less than comparison is not supported).</p> </li>
    /// <li> <p> <code>timestamp</code> - The time stamp of the Spot price history, in UTC format (for example, <i>ddd MMM dd HH</i>:<i>mm</i>:<i>ss</i> UTC <i>YYYY</i>). You can use wildcards (<code>*</code> and <code>?</code>). Greater than or less than comparison is not supported.</p> </li>
    /// </ul>
    pub filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    /// <p>Filters the results by the specified Availability Zone.</p>
    pub availability_zone: ::std::option::Option<::std::string::String>,
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub dry_run: ::std::option::Option<bool>,
    /// <p>The date and time, up to the current date, from which to stop retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>Filters the results by the specified instance types.</p>
    pub instance_types: ::std::option::Option<::std::vec::Vec<crate::types::InstanceType>>,
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub max_results: ::std::option::Option<i32>,
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>Filters the results by the specified basic product descriptions.</p>
    pub product_descriptions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// <p>The date and time, up to the past 90 days, from which to start retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl DescribeSpotPriceHistoryInput {
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for which prices should be returned.</p> </li>
    /// <li> <p> <code>instance-type</code> - The type of instance (for example, <code>m3.medium</code>).</p> </li>
    /// <li> <p> <code>product-description</code> - The product description for the Spot price (<code>Linux/UNIX</code> | <code>Red Hat Enterprise Linux</code> | <code>SUSE Linux</code> | <code>Windows</code> | <code>Linux/UNIX (Amazon VPC)</code> | <code>Red Hat Enterprise Linux (Amazon VPC)</code> | <code>SUSE Linux (Amazon VPC)</code> | <code>Windows (Amazon VPC)</code>).</p> </li>
    /// <li> <p> <code>spot-price</code> - The Spot price. The value must match exactly (or use wildcards; greater than or less than comparison is not supported).</p> </li>
    /// <li> <p> <code>timestamp</code> - The time stamp of the Spot price history, in UTC format (for example, <i>ddd MMM dd HH</i>:<i>mm</i>:<i>ss</i> UTC <i>YYYY</i>). You can use wildcards (<code>*</code> and <code>?</code>). Greater than or less than comparison is not supported.</p> </li>
    /// </ul>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.filters.is_none()`.
    pub fn filters(&self) -> &[crate::types::Filter] {
        self.filters.as_deref().unwrap_or_default()
    }
    /// <p>Filters the results by the specified Availability Zone.</p>
    pub fn availability_zone(&self) -> ::std::option::Option<&str> {
        self.availability_zone.as_deref()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(&self) -> ::std::option::Option<bool> {
        self.dry_run
    }
    /// <p>The date and time, up to the current date, from which to stop retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn end_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.end_time.as_ref()
    }
    /// <p>Filters the results by the specified instance types.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.instance_types.is_none()`.
    pub fn instance_types(&self) -> &[crate::types::InstanceType] {
        self.instance_types.as_deref().unwrap_or_default()
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(&self) -> ::std::option::Option<i32> {
        self.max_results
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>Filters the results by the specified basic product descriptions.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.product_descriptions.is_none()`.
    pub fn product_descriptions(&self) -> &[::std::string::String] {
        self.product_descriptions.as_deref().unwrap_or_default()
    }
    /// <p>The date and time, up to the past 90 days, from which to start retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn start_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.start_time.as_ref()
    }
}
impl DescribeSpotPriceHistoryInput {
    /// Creates a new builder-style object to manufacture [`DescribeSpotPriceHistoryInput`](crate::operation::describe_spot_price_history::DescribeSpotPriceHistoryInput).
    pub fn builder() -> crate::operation::describe_spot_price_history::builders::DescribeSpotPriceHistoryInputBuilder {
        crate::operation::describe_spot_price_history::builders::DescribeSpotPriceHistoryInputBuilder::default()
    }
}

/// A builder for [`DescribeSpotPriceHistoryInput`](crate::operation::describe_spot_price_history::DescribeSpotPriceHistoryInput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DescribeSpotPriceHistoryInputBuilder {
    pub(crate) filters: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>,
    pub(crate) availability_zone: ::std::option::Option<::std::string::String>,
    pub(crate) dry_run: ::std::option::Option<bool>,
    pub(crate) end_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) instance_types: ::std::option::Option<::std::vec::Vec<crate::types::InstanceType>>,
    pub(crate) max_results: ::std::option::Option<i32>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) product_descriptions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl DescribeSpotPriceHistoryInputBuilder {
    /// Appends an item to `filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for which prices should be returned.</p> </li>
    /// <li> <p> <code>instance-type</code> - The type of instance (for example, <code>m3.medium</code>).</p> </li>
    /// <li> <p> <code>product-description</code> - The product description for the Spot price (<code>Linux/UNIX</code> | <code>Red Hat Enterprise Linux</code> | <code>SUSE Linux</code> | <code>Windows</code> | <code>Linux/UNIX (Amazon VPC)</code> | <code>Red Hat Enterprise Linux (Amazon VPC)</code> | <code>SUSE Linux (Amazon VPC)</code> | <code>Windows (Amazon VPC)</code>).</p> </li>
    /// <li> <p> <code>spot-price</code> - The Spot price. The value must match exactly (or use wildcards; greater than or less than comparison is not supported).</p> </li>
    /// <li> <p> <code>timestamp</code> - The time stamp of the Spot price history, in UTC format (for example, <i>ddd MMM dd HH</i>:<i>mm</i>:<i>ss</i> UTC <i>YYYY</i>). You can use wildcards (<code>*</code> and <code>?</code>). Greater than or less than comparison is not supported.</p> </li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        let mut v = self.filters.unwrap_or_default();
        v.push(input);
        self.filters = ::std::option::Option::Some(v);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for which prices should be returned.</p> </li>
    /// <li> <p> <code>instance-type</code> - The type of instance (for example, <code>m3.medium</code>).</p> </li>
    /// <li> <p> <code>product-description</code> - The product description for the Spot price (<code>Linux/UNIX</code> | <code>Red Hat Enterprise Linux</code> | <code>SUSE Linux</code> | <code>Windows</code> | <code>Linux/UNIX (Amazon VPC)</code> | <code>Red Hat Enterprise Linux (Amazon VPC)</code> | <code>SUSE Linux (Amazon VPC)</code> | <code>Windows (Amazon VPC)</code>).</p> </li>
    /// <li> <p> <code>spot-price</code> - The Spot price. The value must match exactly (or use wildcards; greater than or less than comparison is not supported).</p> </li>
    /// <li> <p> <code>timestamp</code> - The time stamp of the Spot price history, in UTC format (for example, <i>ddd MMM dd HH</i>:<i>mm</i>:<i>ss</i> UTC <i>YYYY</i>). You can use wildcards (<code>*</code> and <code>?</code>). Greater than or less than comparison is not supported.</p> </li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.filters = input;
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li> <p> <code>availability-zone</code> - The Availability Zone for which prices should be returned.</p> </li>
    /// <li> <p> <code>instance-type</code> - The type of instance (for example, <code>m3.medium</code>).</p> </li>
    /// <li> <p> <code>product-description</code> - The product description for the Spot price (<code>Linux/UNIX</code> | <code>Red Hat Enterprise Linux</code> | <code>SUSE Linux</code> | <code>Windows</code> | <code>Linux/UNIX (Amazon VPC)</code> | <code>Red Hat Enterprise Linux (Amazon VPC)</code> | <code>SUSE Linux (Amazon VPC)</code> | <code>Windows (Amazon VPC)</code>).</p> </li>
    /// <li> <p> <code>spot-price</code> - The Spot price. The value must match exactly (or use wildcards; greater than or less than comparison is not supported).</p> </li>
    /// <li> <p> <code>timestamp</code> - The time stamp of the Spot price history, in UTC format (for example, <i>ddd MMM dd HH</i>:<i>mm</i>:<i>ss</i> UTC <i>YYYY</i>). You can use wildcards (<code>*</code> and <code>?</code>). Greater than or less than comparison is not supported.</p> </li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        &self.filters
    }
    /// <p>Filters the results by the specified Availability Zone.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.availability_zone = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Filters the results by the specified Availability Zone.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.availability_zone = input;
        self
    }
    /// <p>Filters the results by the specified Availability Zone.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        &self.availability_zone
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.dry_run = ::std::option::Option::Some(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.dry_run = input;
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        &self.dry_run
    }
    /// <p>The date and time, up to the current date, from which to stop retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.end_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time, up to the current date, from which to stop retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.end_time = input;
        self
    }
    /// <p>The date and time, up to the current date, from which to stop retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.end_time
    }
    /// Appends an item to `instance_types`.
    ///
    /// To override the contents of this collection use [`set_instance_types`](Self::set_instance_types).
    ///
    /// <p>Filters the results by the specified instance types.</p>
    pub fn instance_types(mut self, input: crate::types::InstanceType) -> Self {
        let mut v = self.instance_types.unwrap_or_default();
        v.push(input);
        self.instance_types = ::std::option::Option::Some(v);
        self
    }
    /// <p>Filters the results by the specified instance types.</p>
    pub fn set_instance_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::InstanceType>>) -> Self {
        self.instance_types = input;
        self
    }
    /// <p>Filters the results by the specified instance types.</p>
    pub fn get_instance_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::InstanceType>> {
        &self.instance_types
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.max_results = ::std::option::Option::Some(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.max_results = input;
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        &self.max_results
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// Appends an item to `product_descriptions`.
    ///
    /// To override the contents of this collection use [`set_product_descriptions`](Self::set_product_descriptions).
    ///
    /// <p>Filters the results by the specified basic product descriptions.</p>
    pub fn product_descriptions(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.product_descriptions.unwrap_or_default();
        v.push(input.into());
        self.product_descriptions = ::std::option::Option::Some(v);
        self
    }
    /// <p>Filters the results by the specified basic product descriptions.</p>
    pub fn set_product_descriptions(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.product_descriptions = input;
        self
    }
    /// <p>Filters the results by the specified basic product descriptions.</p>
    pub fn get_product_descriptions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.product_descriptions
    }
    /// <p>The date and time, up to the past 90 days, from which to start retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time, up to the past 90 days, from which to start retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The date and time, up to the past 90 days, from which to start retrieving the price history data, in UTC format (for example, <i>YYYY</i>-<i>MM</i>-<i>DD</i>T<i>HH</i>:<i>MM</i>:<i>SS</i>Z).</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.start_time
    }
    /// Consumes the builder and constructs a [`DescribeSpotPriceHistoryInput`](crate::operation::describe_spot_price_history::DescribeSpotPriceHistoryInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::describe_spot_price_history::DescribeSpotPriceHistoryInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::describe_spot_price_history::DescribeSpotPriceHistoryInput {
            filters: self.filters,
            availability_zone: self.availability_zone,
            dry_run: self.dry_run,
            end_time: self.end_time,
            instance_types: self.instance_types,
            max_results: self.max_results,
            next_token: self.next_token,
            product_descriptions: self.product_descriptions,
            start_time: self.start_time,
        })
    }
}
