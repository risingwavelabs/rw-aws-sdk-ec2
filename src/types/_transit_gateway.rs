// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes a transit gateway.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TransitGateway {
    /// <p>The ID of the transit gateway.</p>
    pub transit_gateway_id: ::std::option::Option<::std::string::String>,
    /// <p>The Amazon Resource Name (ARN) of the transit gateway.</p>
    pub transit_gateway_arn: ::std::option::Option<::std::string::String>,
    /// <p>The state of the transit gateway.</p>
    pub state: ::std::option::Option<crate::types::TransitGatewayState>,
    /// <p>The ID of the Amazon Web Services account that owns the transit gateway.</p>
    pub owner_id: ::std::option::Option<::std::string::String>,
    /// <p>The description of the transit gateway.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The creation time.</p>
    pub creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The transit gateway options.</p>
    pub options: ::std::option::Option<crate::types::TransitGatewayOptions>,
    /// <p>The tags for the transit gateway.</p>
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TransitGateway {
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_id.as_deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the transit gateway.</p>
    pub fn transit_gateway_arn(&self) -> ::std::option::Option<&str> {
        self.transit_gateway_arn.as_deref()
    }
    /// <p>The state of the transit gateway.</p>
    pub fn state(&self) -> ::std::option::Option<&crate::types::TransitGatewayState> {
        self.state.as_ref()
    }
    /// <p>The ID of the Amazon Web Services account that owns the transit gateway.</p>
    pub fn owner_id(&self) -> ::std::option::Option<&str> {
        self.owner_id.as_deref()
    }
    /// <p>The description of the transit gateway.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The creation time.</p>
    pub fn creation_time(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.creation_time.as_ref()
    }
    /// <p>The transit gateway options.</p>
    pub fn options(&self) -> ::std::option::Option<&crate::types::TransitGatewayOptions> {
        self.options.as_ref()
    }
    /// <p>The tags for the transit gateway.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl TransitGateway {
    /// Creates a new builder-style object to manufacture [`TransitGateway`](crate::types::TransitGateway).
    pub fn builder() -> crate::types::builders::TransitGatewayBuilder {
        crate::types::builders::TransitGatewayBuilder::default()
    }
}

/// A builder for [`TransitGateway`](crate::types::TransitGateway).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct TransitGatewayBuilder {
    pub(crate) transit_gateway_id: ::std::option::Option<::std::string::String>,
    pub(crate) transit_gateway_arn: ::std::option::Option<::std::string::String>,
    pub(crate) state: ::std::option::Option<crate::types::TransitGatewayState>,
    pub(crate) owner_id: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) options: ::std::option::Option<crate::types::TransitGatewayOptions>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TransitGatewayBuilder {
    /// <p>The ID of the transit gateway.</p>
    pub fn transit_gateway_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn set_transit_gateway_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_id = input;
        self
    }
    /// <p>The ID of the transit gateway.</p>
    pub fn get_transit_gateway_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_id
    }
    /// <p>The Amazon Resource Name (ARN) of the transit gateway.</p>
    pub fn transit_gateway_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.transit_gateway_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the transit gateway.</p>
    pub fn set_transit_gateway_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.transit_gateway_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the transit gateway.</p>
    pub fn get_transit_gateway_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.transit_gateway_arn
    }
    /// <p>The state of the transit gateway.</p>
    pub fn state(mut self, input: crate::types::TransitGatewayState) -> Self {
        self.state = ::std::option::Option::Some(input);
        self
    }
    /// <p>The state of the transit gateway.</p>
    pub fn set_state(mut self, input: ::std::option::Option<crate::types::TransitGatewayState>) -> Self {
        self.state = input;
        self
    }
    /// <p>The state of the transit gateway.</p>
    pub fn get_state(&self) -> &::std::option::Option<crate::types::TransitGatewayState> {
        &self.state
    }
    /// <p>The ID of the Amazon Web Services account that owns the transit gateway.</p>
    pub fn owner_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.owner_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the transit gateway.</p>
    pub fn set_owner_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.owner_id = input;
        self
    }
    /// <p>The ID of the Amazon Web Services account that owns the transit gateway.</p>
    pub fn get_owner_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.owner_id
    }
    /// <p>The description of the transit gateway.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The description of the transit gateway.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>The description of the transit gateway.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The creation time.</p>
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The creation time.</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>The creation time.</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }
    /// <p>The transit gateway options.</p>
    pub fn options(mut self, input: crate::types::TransitGatewayOptions) -> Self {
        self.options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The transit gateway options.</p>
    pub fn set_options(mut self, input: ::std::option::Option<crate::types::TransitGatewayOptions>) -> Self {
        self.options = input;
        self
    }
    /// <p>The transit gateway options.</p>
    pub fn get_options(&self) -> &::std::option::Option<crate::types::TransitGatewayOptions> {
        &self.options
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags for the transit gateway.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// <p>The tags for the transit gateway.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// <p>The tags for the transit gateway.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`TransitGateway`](crate::types::TransitGateway).
    pub fn build(self) -> crate::types::TransitGateway {
        crate::types::TransitGateway {
            transit_gateway_id: self.transit_gateway_id,
            transit_gateway_arn: self.transit_gateway_arn,
            state: self.state,
            owner_id: self.owner_id,
            description: self.description,
            creation_time: self.creation_time,
            options: self.options,
            tags: self.tags,
        }
    }
}