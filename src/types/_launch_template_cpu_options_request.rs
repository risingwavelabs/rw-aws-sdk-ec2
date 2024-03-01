// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The CPU options for the instance. Both the core count and threads per core must be specified in the request.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct LaunchTemplateCpuOptionsRequest {
    /// <p>The number of CPU cores for the instance.</p>
    pub core_count: ::std::option::Option<i32>,
    /// <p>The number of threads per CPU core. To disable multithreading for the instance, specify a value of <code>1</code>. Otherwise, specify the default value of <code>2</code>.</p>
    pub threads_per_core: ::std::option::Option<i32>,
    /// <p>Indicates whether to enable the instance for AMD SEV-SNP. AMD SEV-SNP is supported with M6a, R6a, and C6a instance types only. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/sev-snp.html">AMD SEV-SNP</a>.</p>
    pub amd_sev_snp: ::std::option::Option<crate::types::AmdSevSnpSpecification>,
}
impl LaunchTemplateCpuOptionsRequest {
    /// <p>The number of CPU cores for the instance.</p>
    pub fn core_count(&self) -> ::std::option::Option<i32> {
        self.core_count
    }
    /// <p>The number of threads per CPU core. To disable multithreading for the instance, specify a value of <code>1</code>. Otherwise, specify the default value of <code>2</code>.</p>
    pub fn threads_per_core(&self) -> ::std::option::Option<i32> {
        self.threads_per_core
    }
    /// <p>Indicates whether to enable the instance for AMD SEV-SNP. AMD SEV-SNP is supported with M6a, R6a, and C6a instance types only. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/sev-snp.html">AMD SEV-SNP</a>.</p>
    pub fn amd_sev_snp(&self) -> ::std::option::Option<&crate::types::AmdSevSnpSpecification> {
        self.amd_sev_snp.as_ref()
    }
}
impl LaunchTemplateCpuOptionsRequest {
    /// Creates a new builder-style object to manufacture [`LaunchTemplateCpuOptionsRequest`](crate::types::LaunchTemplateCpuOptionsRequest).
    pub fn builder() -> crate::types::builders::LaunchTemplateCpuOptionsRequestBuilder {
        crate::types::builders::LaunchTemplateCpuOptionsRequestBuilder::default()
    }
}

/// A builder for [`LaunchTemplateCpuOptionsRequest`](crate::types::LaunchTemplateCpuOptionsRequest).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct LaunchTemplateCpuOptionsRequestBuilder {
    pub(crate) core_count: ::std::option::Option<i32>,
    pub(crate) threads_per_core: ::std::option::Option<i32>,
    pub(crate) amd_sev_snp: ::std::option::Option<crate::types::AmdSevSnpSpecification>,
}
impl LaunchTemplateCpuOptionsRequestBuilder {
    /// <p>The number of CPU cores for the instance.</p>
    pub fn core_count(mut self, input: i32) -> Self {
        self.core_count = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of CPU cores for the instance.</p>
    pub fn set_core_count(mut self, input: ::std::option::Option<i32>) -> Self {
        self.core_count = input;
        self
    }
    /// <p>The number of CPU cores for the instance.</p>
    pub fn get_core_count(&self) -> &::std::option::Option<i32> {
        &self.core_count
    }
    /// <p>The number of threads per CPU core. To disable multithreading for the instance, specify a value of <code>1</code>. Otherwise, specify the default value of <code>2</code>.</p>
    pub fn threads_per_core(mut self, input: i32) -> Self {
        self.threads_per_core = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of threads per CPU core. To disable multithreading for the instance, specify a value of <code>1</code>. Otherwise, specify the default value of <code>2</code>.</p>
    pub fn set_threads_per_core(mut self, input: ::std::option::Option<i32>) -> Self {
        self.threads_per_core = input;
        self
    }
    /// <p>The number of threads per CPU core. To disable multithreading for the instance, specify a value of <code>1</code>. Otherwise, specify the default value of <code>2</code>.</p>
    pub fn get_threads_per_core(&self) -> &::std::option::Option<i32> {
        &self.threads_per_core
    }
    /// <p>Indicates whether to enable the instance for AMD SEV-SNP. AMD SEV-SNP is supported with M6a, R6a, and C6a instance types only. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/sev-snp.html">AMD SEV-SNP</a>.</p>
    pub fn amd_sev_snp(mut self, input: crate::types::AmdSevSnpSpecification) -> Self {
        self.amd_sev_snp = ::std::option::Option::Some(input);
        self
    }
    /// <p>Indicates whether to enable the instance for AMD SEV-SNP. AMD SEV-SNP is supported with M6a, R6a, and C6a instance types only. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/sev-snp.html">AMD SEV-SNP</a>.</p>
    pub fn set_amd_sev_snp(mut self, input: ::std::option::Option<crate::types::AmdSevSnpSpecification>) -> Self {
        self.amd_sev_snp = input;
        self
    }
    /// <p>Indicates whether to enable the instance for AMD SEV-SNP. AMD SEV-SNP is supported with M6a, R6a, and C6a instance types only. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/sev-snp.html">AMD SEV-SNP</a>.</p>
    pub fn get_amd_sev_snp(&self) -> &::std::option::Option<crate::types::AmdSevSnpSpecification> {
        &self.amd_sev_snp
    }
    /// Consumes the builder and constructs a [`LaunchTemplateCpuOptionsRequest`](crate::types::LaunchTemplateCpuOptionsRequest).
    pub fn build(self) -> crate::types::LaunchTemplateCpuOptionsRequest {
        crate::types::LaunchTemplateCpuOptionsRequest {
            core_count: self.core_count,
            threads_per_core: self.threads_per_core,
            amd_sev_snp: self.amd_sev_snp,
        }
    }
}
