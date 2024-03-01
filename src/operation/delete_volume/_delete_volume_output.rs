// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct DeleteVolumeOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for DeleteVolumeOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl DeleteVolumeOutput {
    /// Creates a new builder-style object to manufacture [`DeleteVolumeOutput`](crate::operation::delete_volume::DeleteVolumeOutput).
    pub fn builder() -> crate::operation::delete_volume::builders::DeleteVolumeOutputBuilder {
        crate::operation::delete_volume::builders::DeleteVolumeOutputBuilder::default()
    }
}

/// A builder for [`DeleteVolumeOutput`](crate::operation::delete_volume::DeleteVolumeOutput).
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
pub struct DeleteVolumeOutputBuilder {
    _request_id: Option<String>,
}
impl DeleteVolumeOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`DeleteVolumeOutput`](crate::operation::delete_volume::DeleteVolumeOutput).
    pub fn build(self) -> crate::operation::delete_volume::DeleteVolumeOutput {
        crate::operation::delete_volume::DeleteVolumeOutput {
            _request_id: self._request_id,
        }
    }
}