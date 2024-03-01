// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The discovery failure reason.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct IpamDiscoveryFailureReason {
    /// <p>The discovery failure code.</p>
    /// <ul>
    /// <li> <p> <code>assume-role-failure</code> - IPAM could not assume the Amazon Web Services IAM service-linked role. This could be because of any of the following:</p>
    /// <ul>
    /// <li> <p>SLR has not been created yet and IPAM is still creating it.</p> </li>
    /// <li> <p>You have opted-out of the IPAM home Region.</p> </li>
    /// <li> <p>Account you are using as your IPAM account has been suspended.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>throttling-failure</code> - IPAM account is already using the allotted transactions per second and IPAM is receiving a throttling error when assuming the Amazon Web Services IAM SLR.</p> </li>
    /// <li> <p> <code>unauthorized-failure</code> - Amazon Web Services account making the request is not authorized. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">AuthFailure</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </li>
    /// </ul>
    pub code: ::std::option::Option<crate::types::IpamDiscoveryFailureCode>,
    /// <p>The discovery failure message.</p>
    pub message: ::std::option::Option<::std::string::String>,
}
impl IpamDiscoveryFailureReason {
    /// <p>The discovery failure code.</p>
    /// <ul>
    /// <li> <p> <code>assume-role-failure</code> - IPAM could not assume the Amazon Web Services IAM service-linked role. This could be because of any of the following:</p>
    /// <ul>
    /// <li> <p>SLR has not been created yet and IPAM is still creating it.</p> </li>
    /// <li> <p>You have opted-out of the IPAM home Region.</p> </li>
    /// <li> <p>Account you are using as your IPAM account has been suspended.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>throttling-failure</code> - IPAM account is already using the allotted transactions per second and IPAM is receiving a throttling error when assuming the Amazon Web Services IAM SLR.</p> </li>
    /// <li> <p> <code>unauthorized-failure</code> - Amazon Web Services account making the request is not authorized. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">AuthFailure</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </li>
    /// </ul>
    pub fn code(&self) -> ::std::option::Option<&crate::types::IpamDiscoveryFailureCode> {
        self.code.as_ref()
    }
    /// <p>The discovery failure message.</p>
    pub fn message(&self) -> ::std::option::Option<&str> {
        self.message.as_deref()
    }
}
impl IpamDiscoveryFailureReason {
    /// Creates a new builder-style object to manufacture [`IpamDiscoveryFailureReason`](crate::types::IpamDiscoveryFailureReason).
    pub fn builder() -> crate::types::builders::IpamDiscoveryFailureReasonBuilder {
        crate::types::builders::IpamDiscoveryFailureReasonBuilder::default()
    }
}

/// A builder for [`IpamDiscoveryFailureReason`](crate::types::IpamDiscoveryFailureReason).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct IpamDiscoveryFailureReasonBuilder {
    pub(crate) code: ::std::option::Option<crate::types::IpamDiscoveryFailureCode>,
    pub(crate) message: ::std::option::Option<::std::string::String>,
}
impl IpamDiscoveryFailureReasonBuilder {
    /// <p>The discovery failure code.</p>
    /// <ul>
    /// <li> <p> <code>assume-role-failure</code> - IPAM could not assume the Amazon Web Services IAM service-linked role. This could be because of any of the following:</p>
    /// <ul>
    /// <li> <p>SLR has not been created yet and IPAM is still creating it.</p> </li>
    /// <li> <p>You have opted-out of the IPAM home Region.</p> </li>
    /// <li> <p>Account you are using as your IPAM account has been suspended.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>throttling-failure</code> - IPAM account is already using the allotted transactions per second and IPAM is receiving a throttling error when assuming the Amazon Web Services IAM SLR.</p> </li>
    /// <li> <p> <code>unauthorized-failure</code> - Amazon Web Services account making the request is not authorized. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">AuthFailure</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </li>
    /// </ul>
    pub fn code(mut self, input: crate::types::IpamDiscoveryFailureCode) -> Self {
        self.code = ::std::option::Option::Some(input);
        self
    }
    /// <p>The discovery failure code.</p>
    /// <ul>
    /// <li> <p> <code>assume-role-failure</code> - IPAM could not assume the Amazon Web Services IAM service-linked role. This could be because of any of the following:</p>
    /// <ul>
    /// <li> <p>SLR has not been created yet and IPAM is still creating it.</p> </li>
    /// <li> <p>You have opted-out of the IPAM home Region.</p> </li>
    /// <li> <p>Account you are using as your IPAM account has been suspended.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>throttling-failure</code> - IPAM account is already using the allotted transactions per second and IPAM is receiving a throttling error when assuming the Amazon Web Services IAM SLR.</p> </li>
    /// <li> <p> <code>unauthorized-failure</code> - Amazon Web Services account making the request is not authorized. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">AuthFailure</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </li>
    /// </ul>
    pub fn set_code(mut self, input: ::std::option::Option<crate::types::IpamDiscoveryFailureCode>) -> Self {
        self.code = input;
        self
    }
    /// <p>The discovery failure code.</p>
    /// <ul>
    /// <li> <p> <code>assume-role-failure</code> - IPAM could not assume the Amazon Web Services IAM service-linked role. This could be because of any of the following:</p>
    /// <ul>
    /// <li> <p>SLR has not been created yet and IPAM is still creating it.</p> </li>
    /// <li> <p>You have opted-out of the IPAM home Region.</p> </li>
    /// <li> <p>Account you are using as your IPAM account has been suspended.</p> </li>
    /// </ul> </li>
    /// <li> <p> <code>throttling-failure</code> - IPAM account is already using the allotted transactions per second and IPAM is receiving a throttling error when assuming the Amazon Web Services IAM SLR.</p> </li>
    /// <li> <p> <code>unauthorized-failure</code> - Amazon Web Services account making the request is not authorized. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/errors-overview.html">AuthFailure</a> in the <i>Amazon Elastic Compute Cloud API Reference</i>.</p> </li>
    /// </ul>
    pub fn get_code(&self) -> &::std::option::Option<crate::types::IpamDiscoveryFailureCode> {
        &self.code
    }
    /// <p>The discovery failure message.</p>
    pub fn message(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.message = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The discovery failure message.</p>
    pub fn set_message(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.message = input;
        self
    }
    /// <p>The discovery failure message.</p>
    pub fn get_message(&self) -> &::std::option::Option<::std::string::String> {
        &self.message
    }
    /// Consumes the builder and constructs a [`IpamDiscoveryFailureReason`](crate::types::IpamDiscoveryFailureReason).
    pub fn build(self) -> crate::types::IpamDiscoveryFailureReason {
        crate::types::IpamDiscoveryFailureReason {
            code: self.code,
            message: self.message,
        }
    }
}