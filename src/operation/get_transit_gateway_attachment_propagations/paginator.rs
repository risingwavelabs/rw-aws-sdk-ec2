// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Paginator for [`GetTransitGatewayAttachmentPropagations`](crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagations)
pub struct GetTransitGatewayAttachmentPropagationsPaginator {
    handle: std::sync::Arc<crate::client::Handle>,
    builder: crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsInputBuilder,
    stop_on_duplicate_token: bool,
}

impl GetTransitGatewayAttachmentPropagationsPaginator {
    /// Create a new paginator-wrapper
    pub(crate) fn new(
        handle: std::sync::Arc<crate::client::Handle>,
        builder: crate::operation::get_transit_gateway_attachment_propagations::builders::GetTransitGatewayAttachmentPropagationsInputBuilder,
    ) -> Self {
        Self {
            handle,
            builder,
            stop_on_duplicate_token: true,
        }
    }

    /// Set the page size
    ///
    /// _Note: this method will override any previously set value for `max_results`_
    pub fn page_size(mut self, limit: i32) -> Self {
        self.builder.max_results = ::std::option::Option::Some(limit);
        self
    }

    /// Create a flattened paginator
    ///
    /// This paginator automatically flattens results using `transit_gateway_attachment_propagations`. Queries to the underlying service
    /// are dispatched lazily.
    pub fn items(
        self,
    ) -> crate::operation::get_transit_gateway_attachment_propagations::paginator::GetTransitGatewayAttachmentPropagationsPaginatorItems {
        crate::operation::get_transit_gateway_attachment_propagations::paginator::GetTransitGatewayAttachmentPropagationsPaginatorItems(self)
    }

    /// Stop paginating when the service returns the same pagination token twice in a row.
    ///
    /// Defaults to true.
    ///
    /// For certain operations, it may be useful to continue on duplicate token. For example,
    /// if an operation is for tailing a log file in real-time, then continuing may be desired.
    /// This option can be set to `false` to accommodate these use cases.
    pub fn stop_on_duplicate_token(mut self, stop_on_duplicate_token: bool) -> Self {
        self.stop_on_duplicate_token = stop_on_duplicate_token;
        self
    }

    /// Create the pagination stream
    ///
    /// _Note:_ No requests will be dispatched until the stream is used
    /// (e.g. with the [`.next().await`](aws_smithy_async::future::pagination_stream::PaginationStream::next) method).
    pub fn send(
        self,
    ) -> ::aws_smithy_async::future::pagination_stream::PaginationStream<
        ::std::result::Result<
            crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsOutput,
            ::aws_smithy_runtime_api::client::result::SdkError<
                crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsError,
                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
            >,
        >,
    > {
        // Move individual fields out of self for the borrow checker
        let builder = self.builder;
        let handle = self.handle;
        let runtime_plugins =
            crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagations::operation_runtime_plugins(
                handle.runtime_plugins.clone(),
                &handle.conf,
                ::std::option::Option::None,
            );
        ::aws_smithy_async::future::pagination_stream::PaginationStream::new(::aws_smithy_async::future::pagination_stream::fn_stream::FnStream::new(
            move |tx| {
                ::std::boxed::Box::pin(async move {
                    // Build the input for the first time. If required fields are missing, this is where we'll produce an early error.
                    let mut input = match builder
                        .build()
                        .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)
                    {
                        ::std::result::Result::Ok(input) => input,
                        ::std::result::Result::Err(e) => {
                            let _ = tx.send(::std::result::Result::Err(e)).await;
                            return;
                        }
                    };
                    loop {
                        let resp =
                            crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagations::orchestrate(
                                &runtime_plugins,
                                input.clone(),
                            )
                            .await;
                        // If the input member is None or it was an error
                        let done = match resp {
                            ::std::result::Result::Ok(ref resp) => {
                                let new_token = crate::lens::reflens_get_transit_gateway_attachment_propagations_output_output_next_token(resp);
                                let is_empty = new_token.map(|token| token.is_empty()).unwrap_or(true);
                                if !is_empty && new_token == input.next_token.as_ref() && self.stop_on_duplicate_token {
                                    true
                                } else {
                                    input.next_token = new_token.cloned();
                                    is_empty
                                }
                            }
                            ::std::result::Result::Err(_) => true,
                        };
                        if tx.send(resp).await.is_err() {
                            // receiving end was dropped
                            return;
                        }
                        if done {
                            return;
                        }
                    }
                })
            },
        ))
    }
}

/// Flattened paginator for `GetTransitGatewayAttachmentPropagationsPaginator`
///
/// This is created with [`.items()`](GetTransitGatewayAttachmentPropagationsPaginator::items)
pub struct GetTransitGatewayAttachmentPropagationsPaginatorItems(GetTransitGatewayAttachmentPropagationsPaginator);

impl GetTransitGatewayAttachmentPropagationsPaginatorItems {
    /// Create the pagination stream
    ///
    /// _Note_: No requests will be dispatched until the stream is used
    /// (e.g. with the [`.next().await`](aws_smithy_async::future::pagination_stream::PaginationStream::next) method).
    ///
    /// To read the entirety of the paginator, use [`.collect::<Result<Vec<_>, _>()`](aws_smithy_async::future::pagination_stream::PaginationStream::collect).
    pub fn send(
        self,
    ) -> ::aws_smithy_async::future::pagination_stream::PaginationStream<
        ::std::result::Result<
            crate::types::TransitGatewayAttachmentPropagation,
            ::aws_smithy_runtime_api::client::result::SdkError<
                crate::operation::get_transit_gateway_attachment_propagations::GetTransitGatewayAttachmentPropagationsError,
                ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
            >,
        >,
    > {
        ::aws_smithy_async::future::pagination_stream::TryFlatMap::new(self.0.send()).flat_map(|page| {
            crate::lens::lens_get_transit_gateway_attachment_propagations_output_output_transit_gateway_attachment_propagations(page)
                .unwrap_or_default()
                .into_iter()
        })
    }
}
