pub(crate) fn reflens_describe_subnets_output_output_next_token(
    input: &crate::operation::describe_subnets::DescribeSubnetsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_describe_vpc_endpoints_output_output_vpc_endpoints(
    input: crate::operation::describe_vpc_endpoints::DescribeVpcEndpointsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::VpcEndpoint>> {
    let input = match input.vpc_endpoints {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}

pub(crate) fn lens_describe_subnets_output_output_subnets(
    input: crate::operation::describe_subnets::DescribeSubnetsOutput,
) -> ::std::option::Option<::std::vec::Vec<crate::types::Subnet>> {
    let input = match input.subnets {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}
pub(crate) fn reflens_describe_vpc_endpoints_output_output_next_token(
    input: &crate::operation::describe_vpc_endpoints::DescribeVpcEndpointsOutput,
) -> ::std::option::Option<&::std::string::String> {
    let input = match &input.next_token {
        ::std::option::Option::None => return ::std::option::Option::None,
        ::std::option::Option::Some(t) => t,
    };
    ::std::option::Option::Some(input)
}
