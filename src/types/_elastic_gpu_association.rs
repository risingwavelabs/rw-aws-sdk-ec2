// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the association between an instance and an Elastic Graphics accelerator.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ElasticGpuAssociation {
    /// <p>The ID of the Elastic Graphics accelerator.</p>
    pub elastic_gpu_id: ::std::option::Option<::std::string::String>,
    /// <p>The ID of the association.</p>
    pub elastic_gpu_association_id: ::std::option::Option<::std::string::String>,
    /// <p>The state of the association between the instance and the Elastic Graphics accelerator.</p>
    pub elastic_gpu_association_state: ::std::option::Option<::std::string::String>,
    /// <p>The time the Elastic Graphics accelerator was associated with the instance.</p>
    pub elastic_gpu_association_time: ::std::option::Option<::std::string::String>,
}
impl ElasticGpuAssociation {
    /// <p>The ID of the Elastic Graphics accelerator.</p>
    pub fn elastic_gpu_id(&self) -> ::std::option::Option<&str> {
        self.elastic_gpu_id.as_deref()
    }
    /// <p>The ID of the association.</p>
    pub fn elastic_gpu_association_id(&self) -> ::std::option::Option<&str> {
        self.elastic_gpu_association_id.as_deref()
    }
    /// <p>The state of the association between the instance and the Elastic Graphics accelerator.</p>
    pub fn elastic_gpu_association_state(&self) -> ::std::option::Option<&str> {
        self.elastic_gpu_association_state.as_deref()
    }
    /// <p>The time the Elastic Graphics accelerator was associated with the instance.</p>
    pub fn elastic_gpu_association_time(&self) -> ::std::option::Option<&str> {
        self.elastic_gpu_association_time.as_deref()
    }
}
impl ElasticGpuAssociation {
    /// Creates a new builder-style object to manufacture [`ElasticGpuAssociation`](crate::types::ElasticGpuAssociation).
    pub fn builder() -> crate::types::builders::ElasticGpuAssociationBuilder {
        crate::types::builders::ElasticGpuAssociationBuilder::default()
    }
}

/// A builder for [`ElasticGpuAssociation`](crate::types::ElasticGpuAssociation).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct ElasticGpuAssociationBuilder {
    pub(crate) elastic_gpu_id: ::std::option::Option<::std::string::String>,
    pub(crate) elastic_gpu_association_id: ::std::option::Option<::std::string::String>,
    pub(crate) elastic_gpu_association_state: ::std::option::Option<::std::string::String>,
    pub(crate) elastic_gpu_association_time: ::std::option::Option<::std::string::String>,
}
impl ElasticGpuAssociationBuilder {
    /// <p>The ID of the Elastic Graphics accelerator.</p>
    pub fn elastic_gpu_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.elastic_gpu_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Elastic Graphics accelerator.</p>
    pub fn set_elastic_gpu_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.elastic_gpu_id = input;
        self
    }
    /// <p>The ID of the Elastic Graphics accelerator.</p>
    pub fn get_elastic_gpu_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.elastic_gpu_id
    }
    /// <p>The ID of the association.</p>
    pub fn elastic_gpu_association_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.elastic_gpu_association_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the association.</p>
    pub fn set_elastic_gpu_association_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.elastic_gpu_association_id = input;
        self
    }
    /// <p>The ID of the association.</p>
    pub fn get_elastic_gpu_association_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.elastic_gpu_association_id
    }
    /// <p>The state of the association between the instance and the Elastic Graphics accelerator.</p>
    pub fn elastic_gpu_association_state(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.elastic_gpu_association_state = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The state of the association between the instance and the Elastic Graphics accelerator.</p>
    pub fn set_elastic_gpu_association_state(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.elastic_gpu_association_state = input;
        self
    }
    /// <p>The state of the association between the instance and the Elastic Graphics accelerator.</p>
    pub fn get_elastic_gpu_association_state(&self) -> &::std::option::Option<::std::string::String> {
        &self.elastic_gpu_association_state
    }
    /// <p>The time the Elastic Graphics accelerator was associated with the instance.</p>
    pub fn elastic_gpu_association_time(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.elastic_gpu_association_time = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The time the Elastic Graphics accelerator was associated with the instance.</p>
    pub fn set_elastic_gpu_association_time(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.elastic_gpu_association_time = input;
        self
    }
    /// <p>The time the Elastic Graphics accelerator was associated with the instance.</p>
    pub fn get_elastic_gpu_association_time(&self) -> &::std::option::Option<::std::string::String> {
        &self.elastic_gpu_association_time
    }
    /// Consumes the builder and constructs a [`ElasticGpuAssociation`](crate::types::ElasticGpuAssociation).
    pub fn build(self) -> crate::types::ElasticGpuAssociation {
        crate::types::ElasticGpuAssociation {
            elastic_gpu_id: self.elastic_gpu_id,
            elastic_gpu_association_id: self.elastic_gpu_association_id,
            elastic_gpu_association_state: self.elastic_gpu_association_state,
            elastic_gpu_association_time: self.elastic_gpu_association_time,
        }
    }
}
