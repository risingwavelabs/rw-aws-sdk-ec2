// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeCustomerGateways`](crate::operation::describe_customer_gateways::builders::DescribeCustomerGatewaysFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`customer_gateway_ids(impl Into<String>)`](crate::operation::describe_customer_gateways::builders::DescribeCustomerGatewaysFluentBuilder::customer_gateway_ids) / [`set_customer_gateway_ids(Option<Vec::<String>>)`](crate::operation::describe_customer_gateways::builders::DescribeCustomerGatewaysFluentBuilder::set_customer_gateway_ids):<br>required: **false**<br><p>One or more customer gateway IDs.</p>  <p>Default: Describes all your customer gateways.</p><br>
    ///   - [`filters(Filter)`](crate::operation::describe_customer_gateways::builders::DescribeCustomerGatewaysFluentBuilder::filters) / [`set_filters(Option<Vec::<Filter>>)`](crate::operation::describe_customer_gateways::builders::DescribeCustomerGatewaysFluentBuilder::set_filters):<br>required: **false**<br><p>One or more filters.</p>  <ul>   <li> <p> <code>bgp-asn</code> - The customer gateway's Border Gateway Protocol (BGP) Autonomous System Number (ASN).</p> </li>   <li> <p> <code>customer-gateway-id</code> - The ID of the customer gateway.</p> </li>   <li> <p> <code>ip-address</code> - The IP address of the customer gateway device's external interface.</p> </li>   <li> <p> <code>state</code> - The state of the customer gateway (<code>pending</code> | <code>available</code> | <code>deleting</code> | <code>deleted</code>).</p> </li>   <li> <p> <code>type</code> - The type of customer gateway. Currently, the only supported type is <code>ipsec.1</code>.</p> </li>   <li> <p> <code>tag</code>:<key>      - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key      <code>Owner</code> and the value      <code>TeamA</code>, specify      <code>tag:Owner</code> for the filter name and      <code>TeamA</code> for the filter value.    </key></p> </li>   <li> <p> <code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p> </li>  </ul><br>
    ///   - [`dry_run(bool)`](crate::operation::describe_customer_gateways::builders::DescribeCustomerGatewaysFluentBuilder::dry_run) / [`set_dry_run(Option<bool>)`](crate::operation::describe_customer_gateways::builders::DescribeCustomerGatewaysFluentBuilder::set_dry_run):<br>required: **false**<br><p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p><br>
    /// - On success, responds with [`DescribeCustomerGatewaysOutput`](crate::operation::describe_customer_gateways::DescribeCustomerGatewaysOutput) with field(s):
    ///   - [`customer_gateways(Option<Vec::<CustomerGateway>>)`](crate::operation::describe_customer_gateways::DescribeCustomerGatewaysOutput::customer_gateways): <p>Information about one or more customer gateways.</p>
    /// - On failure, responds with [`SdkError<DescribeCustomerGatewaysError>`](crate::operation::describe_customer_gateways::DescribeCustomerGatewaysError)
    pub fn describe_customer_gateways(&self) -> crate::operation::describe_customer_gateways::builders::DescribeCustomerGatewaysFluentBuilder {
        crate::operation::describe_customer_gateways::builders::DescribeCustomerGatewaysFluentBuilder::new(self.handle.clone())
    }
}
